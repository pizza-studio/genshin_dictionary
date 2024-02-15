use model::QueryResult;
use sqlx::{types::Json, PgPool};

use crate::CrudError;

pub async fn query_dictionary(
    word: &str,
    page_size: u64,
    page: Option<u64>,
    db: &PgPool,
) -> Result<QueryResult, CrudError> {
    let result = sqlx::query!(
        r#"
        WITH
            t1 AS (
                SELECT DISTINCT
                    ON ("vocabulary_id") base."vocabulary_id", base."language" AS "target_lang", base."vocabulary_translation" AS "target", pgroonga_score (base.tableoid, base.ctid) AS "score", JSON_OBJECT_AGG(
                        q."language", q."vocabulary_translation"
                    ) OVER (
                        PARTITION BY
                            base."vocabulary_id"
                    ) "lan_dict"
                FROM
                    "dictionary_items" base
                    LEFT JOIN "dictionary_items" q ON base."vocabulary_id" = q."vocabulary_id"
                WHERE
                    base."vocabulary_translation" &@~ $1
                ORDER BY base."vocabulary_id"
            ),
            t2 AS (
                SELECT TO_JSON(t1.*) AS "result", COUNT(*) OVER () AS "total"
                FROM t1
                ORDER BY LENGTH("target"), score DESC
                LIMIT $2
                OFFSET $3
            ),
            t3 AS (
                SELECT COALESCE(JSON_AGG(t2."result"), '[]'::json) AS "results"
                FROM t2
            ),
            t4 AS (
                SELECT COUNT(*) AS "total_page"
                FROM t1
            ),
            t5 AS (
                SELECT t3."results", t4."total_page"
                FROM t3
                    FULL JOIN t4 ON TRUE
            )
        SELECT to_json(t5.*) AS "result!: Json<QueryResult>"
        FROM t5;
        "#,
        word,
        page_size as i64,
        (page_size as i64) * ((page.unwrap_or(1) - 1) as i64)
    )
    .fetch_one(db)
    .await?
    .result
    .0;
    Ok(result)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use model::{Language, NestedDictionaryItem};

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
