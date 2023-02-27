use std::sync::Arc;
use datafusion::prelude::SessionContext;

#[tokio::main]
async fn main() {
    let mut ctx = SessionContext::new();
    let table = deltalake::open_table("./data/delta-0.8.0")
        .await
        .unwrap();
    ctx.register_table("demo", Arc::new(table)).unwrap();

    let batches = ctx
        .sql("SELECT * FROM demo").await.unwrap()
        .collect()
        .await.unwrap();
}
