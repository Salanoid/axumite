#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn client_test() -> Result<()> {
    let base_url = "http://127.0.0.1:8080";
    let http_client = httpc_test::new_client(base_url)?;
    http_client.do_get("/").await?.print().await?;

    Ok(())
}
