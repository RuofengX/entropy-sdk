use std::ops::Deref;

use anyhow::{bail, Ok, Result};
use async_trait::async_trait;
use reqwest::{header::USER_AGENT, Response, StatusCode, Url};
use serde::Serialize;
use serde_json::json;

use entropy_base::{
    entity::{Guest, GuestInfo, Player, PlayerInfo},
    grid::navi,
};

use super::{Access, PhantomRead, Play, Visit};

pub struct RequestBuilder(reqwest::RequestBuilder);
impl RequestBuilder {
    fn auth(self, player: &Player) -> Self {
        RequestBuilder(self.0.basic_auth(player.id, Some(player.password.clone())))
    }
    async fn send(self) -> Result<Response> {
        Ok(self.0.send().await?)
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    base_url: String,
    client: reqwest::Client,
}

impl Connection {
    pub const USER_AGENT: &'static str =
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    pub fn new(url: String) -> Connection {
        Connection {
            base_url: url,
            client: reqwest::ClientBuilder::new()
                .user_agent(USER_AGENT)
                .build()
                .unwrap(),
        }
    }

    pub fn renew(self) -> Self {
        Self::new(self.base_url)
    }

    fn build_get<P: AsRef<str>>(&self, path: P) -> Result<RequestBuilder> {
        let mut url = Url::parse(&self.base_url)?;
        url.set_path(path.as_ref());
        let req = self.client.get(url);
        Ok(RequestBuilder(req))
    }

    fn build_post<P, T>(&self, path: P, json: T) -> Result<RequestBuilder>
    where
        P: AsRef<str>,
        T: Serialize,
    {
        let mut url = Url::parse(&self.base_url)?;
        url.set_path(path.as_ref());
        let req = self.client.post(url).json(&json);
        Ok(RequestBuilder(req))
    }
}

#[async_trait]
impl Access for Connection {
    fn from_url(server: String) -> Result<Self> {
        Ok(Connection::new(server))
    }

    async fn server_ping(&self) -> Result<()> {
        let resp = self.build_get("/")?.send().await?;
        if resp.status() != StatusCode::OK {
            bail!("ping fail")
        };
        Ok(())
    }

    async fn player_info(&self, id: i32) -> Result<PlayerInfo> {
        let resp = self.build_get(format!("/player/{id}"))?.send().await?;
        let p = match resp.status() {
            StatusCode::OK => resp.json::<PlayerInfo>().await?,
            _ => bail!("ping fail"),
        };
        Ok(p)
    }

    async fn player_register(&self, name: String, password: String) -> Result<Player> {
        let resp = self
            .build_post(
                "/player/register",
                json!({
                    "name": name,
                    "password": password
                }),
            )?
            .send()
            .await?;

        let p = match resp.status() {
            StatusCode::OK => resp.json::<Player>().await?,
            _ => bail!("ping fail"),
        };
        Ok(p)
    }

    async fn player_verify(&self, name: String, password: String) -> Result<Player> {
        let resp = self
            .build_post(
                "/player/veirfy",
                json!({
                    "name": name,
                    "password": password
                }),
            )?
            .send()
            .await?;

        let p = match resp.status() {
            StatusCode::OK => resp.json::<Player>().await?,
            _ => bail!("ping fail"),
        };
        Ok(p)
    }

    async fn play(&self, name: String, password: String) -> Result<PlayerControl> {
        let player = self.player_verify(name, password).await?;
        Ok(PlayerControl {
            conn: self.clone(),
            player,
        })
    }
}

pub struct PlayerControl {
    conn: Connection,
    player: Player,
}

impl Deref for PlayerControl {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.player
    }
}

#[async_trait]
impl Play for PlayerControl {
    async fn list_guest(&self) -> Result<Vec<Guest>> {
        let resp = self
            .conn
            .build_get("/player/guest")?
            .auth(&self.player)
            .send()
            .await?;
        let gs = resp.json::<Vec<Guest>>().await?;
        Ok(gs)
    }
    async fn spawn_guest(&self) -> Result<Guest> {
        let resp = self
            .conn
            .build_get("/player/guest/spawn")?
            .auth(&self.player)
            .send()
            .await?;
        let g = resp.json::<Guest>().await?;
        Ok(g)
    }
    async fn visit<'g>(&'g self, guest_id: i32) -> Result<GuestControl<'g>> {
        let resp = self
            .conn
            .build_get(format!("/guest/{guest_id}"))?
            .auth(&self.player)
            .send()
            .await?;
        let g = resp.json::<Guest>().await?;
        Ok(GuestControl {
            player: &self,
            guest: g,
        })
    }
}

pub struct GuestControl<'g> {
    player: &'g PlayerControl,
    guest: Guest,
}

impl<'g> Deref for GuestControl<'g> {
    type Target = Guest;

    fn deref(&self) -> &Self::Target {
        &self.guest
    }
}

#[async_trait]
impl<'g> PhantomRead for GuestControl<'g> {
    async fn refresh(&mut self) -> Result<()> {
        self.guest = self
            .player
            .conn
            .build_get(format!("/guest/{}", self.guest.id))?
            .auth(&self.player)
            .send()
            .await?
            .json::<Guest>()
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<'g> Visit<'g> for GuestControl<'g> {
    async fn walk(&mut self, to: navi::Direction) -> Result<()> {
        let resp = self
            .player
            .conn
            .build_post(
                format!("/guest/walk/{}", self.guest.id),
                json!(
                    {
                        "to": to
                    }
                ),
            )?
            .auth(&self.player)
            .send()
            .await?;
        let g = resp.json::<Guest>().await?;
        self.guest = g;
        Ok(())
    }

    async fn harvest(&mut self, at: usize) -> Result<()> {
        let resp = self
            .player
            .conn
            .build_post(
                format!("/guest/harvest/{}", self.guest.id),
                json!(
                    {
                        "at": at
                    }
                ),
            )?
            .auth(&self.player)
            .send()
            .await?;
        let g = resp.json::<Guest>().await?;
        self.guest = g;
        Ok(())
    }

    async fn arrange(&mut self, transfer_energy: i64) -> Result<GuestControl> {
        let resp = self
            .player
            .conn
            .build_post(
                format!("/guest/arrange/{}", self.guest.id),
                json!(
                    {
                        "transfer_energy": transfer_energy
                    }
                ),
            )?
            .auth(&self.player)
            .send()
            .await?;
        let new_g = resp.json::<Guest>().await?;
        self.refresh().await?;

        let mut gc = GuestControl {
            player: &self.player,
            guest: new_g,
        };
        gc.refresh().await?;

        Ok(gc)
    }

    async fn detect(&self) -> Result<Vec<GuestInfo>> {
        let resp = self
            .player
            .conn
            .build_get(format!("/guest/detect/{}", self.guest.id))?
            .auth(&self.player)
            .send()
            .await?;
        let gis = resp.json::<Vec<GuestInfo>>().await?;
        Ok(gis)
    }

    async fn heat(&mut self, at: usize, energy: i64) -> Result<()> {
        let resp = self
            .player
            .conn
            .build_post(
                format!("/guest/heat/{}", self.guest.id),
                json!(
                    {
                        "at": at,
                        "energy": energy,
                    }
                ),
            )?
            .auth(&self.player)
            .send()
            .await?;
        let g = resp.json::<Guest>().await?;
        self.guest = g;
        Ok(())
    }
}
