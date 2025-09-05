use serenity::prelude::*;
use serenity::model::gateway::Ready;
use crate::utils::log_info;

pub async fn on_ready(_ctx: &Context, ready: &Ready) {
    log_info(&format!("✅ Bot is online as {}", ready.user.name));
}
