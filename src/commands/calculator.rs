use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult, Args};
use serenity::prelude::*;
use serenity::model::channel::Message;

#[command]
async fn sum(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let mut args = args;
    let mut total: f32 = args.single::<f32>().unwrap();

    for _ in 0..args.len() {
        total = total + args.single::<f32>().unwrap();
    }

    let content = format!("{} the sum is `{}`", msg.author.mention(),total);
    msg.channel_id.say(&ctx.http, content).await?;
    Ok(())
}

#[command]
async fn prod(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let mut args = args;
    let mut total: f32 = args.single::<f32>().unwrap();

    for _ in 0..args.len() {
        total = total * args.single::<f32>().unwrap();
    }
    
    let content = format!("{} the product is `{}`", msg.author.mention(),total);
    msg.channel_id.say(&ctx.http, content).await?;
    Ok(())
}

#[command]
async fn diff(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let mut args = args;
    let mut total: f32 = args.single::<f32>().unwrap();

    for _ in 0..args.len() {
        total = total - args.single::<f32>().unwrap();
    }
    
    let content = format!("{} the difference is `{}`", msg.author.mention(),total);
    msg.channel_id.say(&ctx.http, content).await?;
    Ok(())
}

#[command]
async fn quo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let mut args = args;
    let mut total: f32 = args.single::<f32>().unwrap();

    for _ in 0..args.len() {
        total = total / args.single::<f32>().unwrap();
    }
    
    let content = format!("{} the quotient is `{}`", msg.author.mention(),total);
    msg.channel_id.say(&ctx.http, content).await?;
    Ok(())
}
