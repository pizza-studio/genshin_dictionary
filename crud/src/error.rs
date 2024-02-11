#[derive(thiserror::Error, Debug)]
pub enum CrudError {
    #[error("Sqlx Error")]
    Sqlx(#[from] sqlx::Error),
    #[error("Update Data Error")]
    UpdateData(#[from] anyhow::Error),
}
