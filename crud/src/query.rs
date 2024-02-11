use futures::future::try_join_all;
use model::{Language, NestedDictionaryItem, QueryResult};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::CrudError;

pub async fn query_dictionary(
    word: &str,
    page_size: u64,
    page: Option<u64>,
    db: &PgPool,
) -> Result<QueryResult, CrudError> {
    let results = sqlx::query!(
        r#"
        SELECT
            "vocabulary_id",
            "language" as "language!: Language",
            "vocabulary_translation",
            COUNT(*) OVER () AS "total!"
        FROM (
                SELECT
                DISTINCT ON ("vocabulary_id")
                    "vocabulary_id",
                    "language",
                    "vocabulary_translation",
                    COUNT(*) OVER () AS "total",
                    pgroonga_score(tableoid, ctid) AS "score"
                FROM "dictionary_items"
                WHERE
                    "vocabulary_translation" &@~ $1
                ORDER BY "vocabulary_id"
            ) AS t
        ORDER BY LENGTH("vocabulary_translation"), score DESC
        LIMIT $2
        OFFSET $3
        "#,
        word,
        page_size as i64,
        (page_size as i64) * ((page.unwrap_or(1) - 1) as i64)
    )
    .fetch_all(db)
    .await?;
    let total = if let Some(total) = results.first().map(|r| r.total) {
        total
    } else if ((page.unwrap_or(1) - 1) as i64) == 0 {
        0
    } else {
        sqlx::query!(
            r#"
            SELECT
                COUNT(*) OVER () AS "total!"
            FROM (
                    SELECT
                    DISTINCT ON ("vocabulary_id")
                        "vocabulary_id",
                        "language",
                        "vocabulary_translation",
                        COUNT(*) OVER () AS "total",
                        pgroonga_score(tableoid, ctid) AS "score"
                    FROM "dictionary_items"
                    WHERE
                        "vocabulary_translation" &@~ $1
                    ORDER BY "vocabulary_id"
                ) AS t
            ORDER BY LENGTH("vocabulary_translation"), score DESC
            LIMIT 1
            "#,
            word
        )
        .fetch_optional(db)
        .await?
        .map(|r| r.total)
        .unwrap_or(0)
    };
    let total_page = (total as f64 / page_size as f64).ceil() as u64;
    let queries = results.into_iter().map(|result| {
        let db = db.clone();
        async move {
            let all_lang_results = sqlx::query!(
                r#"
                SELECT "language" as "language!: Language", "vocabulary_translation"
                FROM "dictionary_items"
                WHERE "vocabulary_id" = $1
                ORDER BY "language"
                "#,
                result.vocabulary_id
            )
            .fetch_all(&db)
            .await?;
            let lang_dict = all_lang_results
                .into_iter()
                .map(|record| (record.language, record.vocabulary_translation))
                .collect::<HashMap<_, _>>();
            let result = NestedDictionaryItem {
                vocabulary_id: result.vocabulary_id as i32,
                target: result.vocabulary_translation,
                target_lang: result.language,
                lan_dict: lang_dict,
            };
            Ok::<_, CrudError>(result)
        }
    });
    let results = try_join_all(queries).await?;
    Ok(QueryResult {
        total_page,
        results,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::insert_test_data;

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn test_query(db: PgPool) {
        insert_test_data(&db).await;
        let result = query_dictionary("冰", 20, Some(1), &db).await.unwrap();
        let assertion = QueryResult {
            total_page: 1,
            results: vec![NestedDictionaryItem {
                vocabulary_id: 5532,
                target: "冰".to_string(),
                target_lang: Language::Chs,
                lan_dict: HashMap::from([
                    (Language::En, "Cryo".to_string()),
                    (Language::Es, "Cryo".to_string()),
                    (Language::Chs, "冰".to_string()),
                    (Language::De, "Kryo".to_string()),
                    (Language::Cht, "冰".to_string()),
                ]),
            }],
        };
        assert_eq!(assertion, result);
    }
}
