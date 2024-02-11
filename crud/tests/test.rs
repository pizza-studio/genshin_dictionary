use model::{DictionaryItem, Language};
use sqlx::PgPool;

#[sqlx::test(migrator = "crud::MIGRATOR")]
async fn test_db_conn(_db: PgPool) {}

#[sqlx::test(migrator = "crud::MIGRATOR")]
async fn test_insert_item(db: PgPool) {
    let voc_id = 1;
    let language = Language::Chs;
    let voc_trans = "Hello World".to_string();
    sqlx::query!(
        r#"
        INSERT INTO "dictionary_items" ("vocabulary_id", "language", "vocabulary_translation")
        VALUES ($1, $2, $3)
        "#,
        voc_id,
        language as Language,
        voc_trans
    )
    .execute(&db)
    .await
    .unwrap();
    let query = "Hello";
    let result = sqlx::query_as!(
        DictionaryItem,
        r#"
        SELECT vocabulary_id, language AS "language!: Language" , vocabulary_translation
        FROM dictionary_items
        WHERE vocabulary_translation &@* $1
        "#,
        query
    )
    .fetch_one(&db)
    .await
    .unwrap();

    assert!(result.language == language);
    assert!(voc_id == result.vocabulary_id);
    assert!(voc_trans == result.vocabulary_translation);
}

#[sqlx::test(migrator = "crud::MIGRATOR")]
#[ignore]
async fn test_update_all(db: PgPool) {
    crud::update_dictionary(&db).await.unwrap();
}
