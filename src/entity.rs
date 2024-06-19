use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub password: String,
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guest {
    pub id: i32,
    pub energy: i64,
    pub pos: (i16, i16),
    pub temperature: i8,
    pub master_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestInfo {
    pub id: i32,
    pub temperature: i16,
    pub pos: (i16, i16),
    pub master_id: i32,
}
