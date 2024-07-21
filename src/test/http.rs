use crate::client::{http, Access, Guide, Play, Visit};
use anyhow::Result;
use entropy_base::heat::carnot_efficiency;

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
    let mut visiting_guest = p.visit(g.id).await?;
    assert_eq!(*visiting_guest, g);

    // get_node
    let map = conn.guide().await?;
    let node_origin = map.get_node((0, 0)).await?;

    // harvest
    let at = 1;
    let cell_zero = node_origin.data.get(at).unwrap();
    let eff = carnot_efficiency(cell_zero, g.temperature);

    let delta = g.temperature.abs_diff(cell_zero);
    let delta = (eff * delta as f32).div_euclid(2.0) as u8;

    let expect_temp = if g.temperature > cell_zero {
        g.temperature.saturating_sub_unsigned(delta)
    } else if g.temperature < cell_zero {
        g.temperature.saturating_add_unsigned(delta)
    } else {
        g.temperature
    };

    visiting_guest.harvest(at).await?;
    assert_eq!(expect_temp, visiting_guest.temperature);

    // harvest all the node
    for (at, _) in node_origin.data.to_bytes().iter().enumerate() {
        visiting_guest.harvest(at).await?;
    }

    // walk
    assert_eq!(visiting_guest.pos, (0, 0));
    visiting_guest.walk((1, 0)).await?;
    assert_eq!(visiting_guest.pos, (1, 0));
    visiting_guest.walk((0, 1)).await?;
    assert_eq!(visiting_guest.pos, (1, 1));
    visiting_guest.walk((-1, 0)).await?;
    assert_eq!(visiting_guest.pos, (0, 1));
    visiting_guest.walk((0, -1)).await?;
    assert_eq!(visiting_guest.pos, (0, 0));

    assert!(visiting_guest.walk((2, 0)).await.is_err());
    assert!(visiting_guest.walk((10000, 0)).await.is_err());
    assert!(visiting_guest.walk((0, 0)).await.is_err());

    // arrange
    let g2 = visiting_guest.arrange(0).await?.clone();
    let mut gs = p.list_guest().await?;
    gs.sort_by(|a, b| a.id.cmp(&b.id));
    assert_eq!(gs, vec![visiting_guest.clone(), g2]);

    Ok(())
}
