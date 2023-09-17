use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult, Args};
use serenity::prelude::*;
use serenity::model::channel::Message;
use rand::prelude::*;

#[command]
async fn flip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {

    let mut content = format!("{} flipped `Tails`", msg.author.mention());
    
    if rand::random() { // generates a boolean
        // Try printing a random unicode code point (probably a bad idea)!
       content = format!("{} flipped `Heads`", msg.author.mention());
    }
    
    msg.channel_id.say(&ctx.http, content).await?;
    Ok(())
}

#[command]
async fn roll(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {

    let rolled = rand::thread_rng().gen_range(1..=99);

    let content = format!("{} rolled `{}`",  msg.author.mention(), rolled);
    
    msg.channel_id.say(&ctx.http, content).await?;
    Ok(())
}