use std::collections::HashMap;

use axum::body::Body;
use crud::insert_test_data;
use genshin_dictionary::app;
use http_body_util::BodyExt;
use model::{Language, NestedDictionaryItem, QueryResult};
use sqlx::PgPool;
use tower::ServiceExt;

use axum::http::{Request, StatusCode};

#[sqlx::test(migrator = "crud::MIGRATOR")]
async fn test_update_all(db: PgPool) {
    insert_test_data(&db).await;
    let app = app(db);

    let resp = app
        .oneshot(
            Request::get("/v1/translations/Kryo?page=1&page_size=20")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = resp.status();
    let b = resp.into_body().collect().await.unwrap().to_bytes();
    let result = serde_json::from_slice(&b).unwrap();

    let assertion = QueryResult {
        total_page: 1,
        results: vec![NestedDictionaryItem {
            vocabulary_id: 5532,
            target: "Kryo".to_string(),
            target_lang: Language::De,
            lan_dict: HashMap::from([
                (Language::En, "Cryo".to_string()),
                (Language::Es, "Cryo".to_string()),
                (Language::Chs, "冰".to_string()),
                (Language::De, "Kryo".to_string()),
                (Language::Cht, "冰".to_string()),
            ]),
        }],
    };

    assert_eq!(status, StatusCode::OK);
    assert_eq!(assertion, result);
}
