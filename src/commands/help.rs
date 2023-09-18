use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::utils::Colour;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let commands = " `!in` `!out` `!about` `!help` `!sum` `!diff` `!prod` `!quo` `!flip` `!roll`, '!free `epic/steam/gog`'";
     // send message
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(
            |e| e
            .color(Colour::from_rgb(171,206,236))
            .title(":clock3: **Relapse Commands** ")
            .field("commands", commands, false)  
            .description("hello, I am Relapse bot.")
        )
    }).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}
