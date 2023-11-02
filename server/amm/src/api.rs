
use axum::Router;
use sea_orm::Database;

use crate::config::Opts;

pub async fn setup(opts: &Opts) -> Router {
    let conn = Database::connect(opts.get_db_url())
        .await
        .expect("failed to connect to db");
    todo!()
}
