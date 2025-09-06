use serenity::Client;
use serenity::all::GatewayIntents;
use tokio::sync::Mutex;
use crate::handlers;
use crate::data::{BotData, BotDataKey, load_all_data};
use std::sync::Arc;

pub async fn run(token: String) {
    let all_data = load_all_data();
    let guilds_map = Arc::new(Mutex::new(all_data.0));
    let bot_data = Arc::new(BotData {
        guilds: guilds_map,
    });
    let handler = handlers::Handler::new(bot_data.clone());

    let intents = GatewayIntents::GUILDS;
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    {
        client.data.write().await.insert::<BotDataKey>(bot_data);
        crate::utils::log_info("Bot startup complete!");
    }

    if let Err(e) = client.start().await {
        crate::utils::log_error(&format!("Client error: {:?}", e));
    }
}
