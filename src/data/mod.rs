pub mod structs;

use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use structs::*;
use serde_json;

pub struct BotData {
    pub guilds: Arc<Mutex<HashMap<u64, GuildData>>>,
}

impl Default for BotData {
    fn default() -> Self {
        Self {
            guilds: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub struct BotDataKey;
impl TypeMapKey for BotDataKey {
    type Value = Arc<BotData>;
}

// Persistence helpers
pub fn save_all_data(all_data: &AllGuildData) {
    let json = serde_json::to_string_pretty(&all_data).unwrap();
    std::fs::create_dir_all("data").unwrap();
    std::fs::write("data/data.json", json).unwrap();
}

pub fn load_all_data() -> AllGuildData {
    std::fs::read_to_string("data/data.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_guild_data(guild_id: u64, data: &GuildData) {
    let mut all_data = load_all_data();
    all_data.0.insert(guild_id, data.clone());
    save_all_data(&all_data);
}

pub fn load_guild_data(guild_id: u64) -> Option<GuildData> {
    let all_data = load_all_data();
    all_data.0.get(&guild_id).cloned()
}
