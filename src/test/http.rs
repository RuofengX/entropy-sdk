use crate::client::{http, Access};
use anyhow::Result;

// #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[tokio::test]
async fn test_ping() -> Result<()> {
    let conn = http::Connection::new("http://127.0.0.1:3333".to_string());
    conn.ping().await?;
    Ok(())
}

#[tokio::test]
async fn test_player() -> Result<()> {
    let conn = http::Connection::new("http://127.0.0.1:3333".to_string());
    conn.ping().await?;

    // register
    let name = "测试账号";
    let password = "123456";
    let p = conn
        .player_register(name, password)
        .await?;
    assert_eq!(p.name, name);
    assert_eq!(p.password, password);

    // verify
    let p_verified = conn.player_verify(p.id, password).await?;
    assert_eq!(p, p_verified);


    Ok(())
}
