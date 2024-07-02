use std::ops::Deref;

use crate::client::{http, Access, Play};
use anyhow::Result;
use entropy_base::entity::Guest;

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
    let p = conn.player_register(name, password).await?;
    assert_eq!(p.name, name);
    assert_eq!(p.password, password);

    // verify
    let verified_account = conn.player_verify(p.id, password).await?;
    assert_eq!(p, verified_account);

    // play
    let p = conn.play(p.id, password).await?;

    // list_guest empty
    let g_list = p.list_guest().await?;
    assert_eq!(g_list, vec![]);

    // spawn first guest
    let g = p.spawn_guest().await?;
    assert_eq!(g.pos, (0, 0));

    // list_guest 1 guest
    let g_list = p.list_guest().await?;
    assert_eq!(g_list, vec![g]);

    // get_guest
    let g_again = p.visit(g.id).await?;
    assert_eq!(*g_again, g);

    // get_node






    Ok(())
}
