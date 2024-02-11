use std::{collections::HashMap, sync::Arc};

use futures::future::try_join_all;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use lazy_static::lazy_static;

use model::Language;
use strum::IntoEnumIterator;

use tracing::info;

use sqlx::PgPool;

use crate::CrudError;

lazy_static! {
    static ref LANGUAGE_URL_MAPPING: HashMap<Language, String> = {
        Language::iter()
            .map(|lang| {
                let url = format!(
                    "https://github.com/Masterain98/GenshinData/raw/main/TextMap/TextMap{}.json",
                    lang.to_string().to_uppercase()
                );
                info!("Data url for {} is: {}", lang.to_string(), url);
                (lang, url)
            })
            .collect()
    };
}

pub async fn update_dictionary(db: &PgPool) -> Result<(), CrudError> {
    truncate_table(db).await?;
    for (lang, url) in LANGUAGE_URL_MAPPING.iter() {
        info!("Getting data for {}", lang);
        let map = reqwest::get(url)
            .await
            .map_err(|e| CrudError::UpdateData(e.into()))?
            .json::<HashMap<i64, String>>()
            .await
            .map_err(|e| CrudError::UpdateData(e.into()))?;
        info!("Updating data for {}", lang);
        let inserted_count = insert_items(*lang, map, db).await?;
        info!("Insert {}", inserted_count);
    }
    delete_duplicated_items(db).await?;
    Ok(())
}

pub async fn insert_items(
    lang: Language,
    map: HashMap<i64, String>,
    db: &PgPool,
) -> Result<usize, sqlx::Error> {
    let len = map.len();
    let bar = Arc::new(ProgressBar::new(len as u64));
    let queries = map.into_iter().map(|(voc_id, voc_trans)| {
        let style = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}\n{msg}",
        )
        .unwrap()
        .progress_chars("##-");
        let bar = bar.clone();
        bar.set_style(style);
        async move {
            bar.set_message(format!("{} {}: {}", lang, voc_id, voc_trans.chars().take(50).collect::<String>()));
            let result = sqlx::query!(
                r#"
                INSERT INTO "dictionary_items" ("vocabulary_id", "language", "vocabulary_translation")
                VALUES ($1, $2, $3)
                "#,
                voc_id,
                lang as Language,
                voc_trans
            )
            .execute(db)
            .await;
            bar.inc(1);
            result
        }
    });
    for chunk in queries.chunks(50).into_iter() {
        try_join_all(chunk).await?;
    }
    bar.finish();
    Ok(len)
}

async fn delete_duplicated_items(db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM dictionary_items
        WHERE
            vocabulary_id NOT IN (
                SELECT MIN(vocabulary_id)
                FROM (
                        SELECT vocabulary_id, STRING_AGG(vocabulary_translation, ', ' ORDER BY language) AS translations
                        FROM (
                                SELECT
                                    vocabulary_id, vocabulary_translation, language
                                FROM dictionary_items
                            ) AS sorted_items
                        GROUP BY
                            vocabulary_id
                    ) AS subquery_alias
                GROUP BY
                    translations
            )
        "#
    )
    .execute(db)
    .await?;
    Ok(())
}

async fn truncate_table(db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        TRUNCATE TABLE dictionary_items
        "#
    )
    .execute(db)
    .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_content_access() {
        let client = reqwest::Client::new();
        for (_lang, url) in LANGUAGE_URL_MAPPING.iter() {
            let res = client.head(url).send().await.unwrap();
            assert_ne!(
                res.headers()
                    .get("content-length")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
                0
            )
        }
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn test_truncate_table(db: PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO "dictionary_items" ("vocabulary_id", "language", "vocabulary_translation")
            VALUES ($1, $2, $3)
            "#,
            1,
            Language::Chs as Language,
            "Hello World"
        )
        .execute(&db)
        .await
        .unwrap();
        truncate_table(&db).await.unwrap();
        assert!(sqlx::query!(
            r#"
            SELECT "vocabulary_id", "language" AS "language!: Language", "vocabulary_translation"
            FROM dictionary_items
            "#
        )
        .fetch_optional(&db)
        .await
        .unwrap()
        .is_none());
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn test_delete_duplicate(db: PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO "dictionary_items" ("vocabulary_id", "language", "vocabulary_translation")
            VALUES ($1, $2, $3)
            "#,
            1,
            Language::Chs as Language,
            "Hello World"
        )
        .execute(&db)
        .await
        .unwrap();
        sqlx::query!(
            r#"
            INSERT INTO "dictionary_items" ("vocabulary_id", "language", "vocabulary_translation")
            VALUES ($1, $2, $3)
            "#,
            2,
            Language::Chs as Language,
            "Hello World"
        )
        .execute(&db)
        .await
        .unwrap();

        delete_duplicated_items(&db).await.unwrap();

        assert_eq!(
            sqlx::query!(
                r#"
            SELECT "vocabulary_id", "language" AS "language!: Language", "vocabulary_translation"
            FROM dictionary_items
            "#
            )
            .fetch_all(&db)
            .await
            .unwrap()
            .len(),
            1
        );
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn test_insert_data(db: PgPool) {
        let data = include_bytes!("../test_data/TextMapCHS.json");
        let map: HashMap<i64, String> = serde_json::from_slice(data).unwrap();
        let len = insert_items(Language::Chs, map, &db).await.unwrap();
        assert_eq!(29, len);
    }
}
