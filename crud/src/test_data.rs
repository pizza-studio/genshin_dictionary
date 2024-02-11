use crate::update_data::insert_items;

use model::Language;
use sqlx::PgPool;

pub async fn insert_test_data(db: &PgPool) {
    insert_items(
        Language::Chs,
        serde_json::from_slice(include_bytes!("../test_data/TextMapCHS.json")).unwrap(),
        db,
    )
    .await
    .unwrap();
    insert_items(
        Language::Cht,
        serde_json::from_slice(include_bytes!("../test_data/TextMapCHT.json")).unwrap(),
        db,
    )
    .await
    .unwrap();
    insert_items(
        Language::De,
        serde_json::from_slice(include_bytes!("../test_data/TextMapDE.json")).unwrap(),
        db,
    )
    .await
    .unwrap();
    insert_items(
        Language::En,
        serde_json::from_slice(include_bytes!("../test_data/TextMapEN.json")).unwrap(),
        db,
    )
    .await
    .unwrap();
    insert_items(
        Language::Es,
        serde_json::from_slice(include_bytes!("../test_data/TextMapES.json")).unwrap(),
        db,
    )
    .await
    .unwrap();
}
