use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult, Args};
use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::model::channel::Message;

use serde::{Serialize, Deserialize};

use reqwest; 

#[derive(Debug, Serialize, Deserialize)]
struct GiveAways {
    id: u32,
    title: String,
    worth: String,
    thumbnail: String,
    image: String, 
    description: String,
    published_date: String,
    end_date: String,
    platforms: String,
    open_giveaway: String,
}

#[command]
pub async fn free(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let mut platform: String = String::new();

    match args.single::<String>() {
        Ok(value) => { 
            platform = value;
        }
        Err(_) => {
            let error_content = "please add a source (steam, epic, ubisoft, gog, itchio) `!free steam`".to_string();
            msg.channel_id.say(&ctx.http, error_content).await.unwrap();
            return Ok(()); 
        }
    }
   
    if platform == "epic" { platform = "epic-games-store".to_string() } 

    let format = format!("https://www.gamerpower.com/api/giveaways?type=game&platform={}",platform).to_string();

    let body: Vec<GiveAways> = reqwest::get(format)
            .await?
            .json()
            .await?;

    if body.len() > 0 {
        for items in body {
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(
                    |e| e
                    .color(Colour::from_rgb(171,206,236))
                    .title(items.title)
                    .thumbnail(items.thumbnail)
                    .description(items.description)
                    .field("Platforms: ", items.platforms, false)
                    .field("Worth: ", items.worth, true)
                    .field("Free until: ", items.end_date, true)
                    .url(items.open_giveaway)
                    .image(items.image)  
                )
            }).await.unwrap();
            println!("Item, {:?}", items.id);
        }
    } else {
        println!("no games found");
    }

    Ok(())
}

