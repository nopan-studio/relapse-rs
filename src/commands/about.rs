use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::prelude::*;
use serenity::model::channel::Message;

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let content = "I am relapse, a discord bot written in rust. \n You can check my source code at `https://github.com/nopan-studio/relapse-rs`";
    msg.channel_id.say(&ctx.http, content).await?;
    Ok(())
}
