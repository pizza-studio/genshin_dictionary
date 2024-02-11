use clap::Parser;
use crud::{establish_conn, update_dictionary};

use tokio::net::TcpListener;

use tracing::info;
use tracing_unwrap::ResultExt;

mod app;
mod init_tracing;
mod search_dictionary;

use app::app;
use init_tracing::init_tracing;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    update: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let _guards = init_tracing();
    info!("Args: {args:?}");

    info!("Establishing database connection");

    if args.update {
        let db = establish_conn(false).await.unwrap_or_log();
        info!("Updating dictionary data");
        update_dictionary(&db).await.unwrap_or_log();
    }

    let db = establish_conn(true).await.unwrap_or_log();

    info!("Starting server...");

    let app = app(db);

    let addr = "0.0.0.0:3001";
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap_or_log();
}
