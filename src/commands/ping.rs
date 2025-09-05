use serenity::prelude::*;
use serenity::model::application::*;
use serenity::builder::*;

use crate::data::BotData;
use crate::utils::check_admin;

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
        .description("Replies with Pong! (Admins only ✨)")
}

pub async fn execute(ctx: Context, command: CommandInteraction, _bot_data: &BotData) {
    if !check_admin(&ctx, &command).await { return; }

    let _ = command.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("🏓 Pong!")
            .flags(InteractionResponseFlags::EPHEMERAL)
    )).await;
}
