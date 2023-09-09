use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult, Args};
use serenity::prelude::*;
use serenity::model::channel::Message;

#[command]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    
    let content = "I am Relapse, a discord bot written in rust.";
    msg.channel_id.say(&ctx.http, content).await?;
    Ok(())
}
