use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::utils::Colour;
use chrono_tz::Tz;

const DTR_CHANNEL: &str = "relapse-time";  
const TIME_ZONE: &str = "Asia/Manila";

pub async fn time_in(ctx: &Context, msg: &Message){
    let time = chrono::Utc::now();
    let tz: Tz = TIME_ZONE.parse().unwrap();
    let time: chrono::DateTime<Tz> = time.with_timezone(&tz);
    let time = time.format("%m/%d/%Y [%H:%M]").to_string();

    // deletes the command.
    if let Err(why) = &msg.delete(&ctx.http).await {
        println!("Message can't be deleted | {}", why);
  
    };

    // set channel to DTR_CHANNEL
    let channel = msg.guild(&ctx.cache).unwrap()
        .channel_id_from_name(&ctx.cache, DTR_CHANNEL).unwrap();

    // check if a time out wasn't called after last time in.
    let last_out = get_last_time_out(ctx, msg).await;
    let error_content = format!("It looks like {} still have not timed out. please use !out",msg.author.mention());
    if last_out == "None" { channel.say(&ctx.http, error_content).await.unwrap(); return}

    // send message
    if let Err(why) = channel.send_message(&ctx.http, |m| {
        m.embed(
            |e| e
            .color(Colour::from_rgb(171,206,236))
            .title(":clock3: **Relapse DTR** ")
            .field("User:", &msg.author.mention(), true)  
            .field("Action:", "in", true)  
            .field("Duration:", " ", false)  
            .field("Time-In:", &time, false)  
            .field("Time-Out:", " ", false)  
            .description("this user is going through some relapse.")
        )
    }).await {
        println!("Error sending message: {:?}", why);
    }
}

pub async fn time_out(ctx: &Context, msg: &Message){

    // deletes the command given.
    if let Err(why) = &msg.delete(&ctx.http).await {
        println!("Message can't be deleted | {}", why);
    };

    // set channel to DTR_CHANNEL
    let channel = msg.guild(&ctx.cache).unwrap()
        .channel_id_from_name(&ctx.cache, DTR_CHANNEL).unwrap();
    
    // get time
    let time = chrono::prelude::Local::now();
    let format = "%m/%d/%Y [%H:%M]";

    let time = chrono::Utc::now();
    let tz: Tz = TIME_ZONE.parse().unwrap();
    let time: chrono::DateTime<Tz> = time.with_timezone(&tz);

    // get last time in
    let time_in = get_last_time_in(ctx, msg).await;
    let time_out = time.format(format).to_string();

    let error_content = format!("I cannot find {} last time-in record.",msg.author.mention());
    // check if last time in is available
    if time_in == "None" { channel.say(&ctx.http, error_content).await.unwrap(); return }

    let parsed_in = chrono::NaiveDateTime::parse_from_str(&time_in, &format).unwrap();
    let parsed_out = chrono::NaiveDateTime::parse_from_str(&time_out, &format).unwrap();
    
    let duration = parsed_in - parsed_out;
    let duration_days = duration.num_days();
    let duration_hours = duration.num_hours() % 24 * -1;
    let duration_minutes = duration.num_minutes() % 60 * -1;

    let duration = format!("{} Days, {} Hours, {} Minutes", duration_days, duration_hours, duration_minutes);

    // send the embeded message.
    let message = channel.send_message(&ctx.http, |m| {
        m.embed(
            |e| e
            .color(Colour::from_rgb(171,206,236))
            .title(":clock3: **Relapse DTR** ")
            .field("User:", &msg.author.mention(), true)  
            .field("Action:", "out", true)  
            .field("Duration:", duration, false)  
            .field("Time-in:", &time_in, false)  
            .field("Time-out:", &time_out, false)  
            .description("this user is going through some relapse.")
        )
    }).await.unwrap();

    //message.react(&ctx.http, '🥇').await.unwrap();
    //message.pin(&ctx.http).await.unwrap();
   
}

async fn get_messages(ctx: &Context, msg: &Message) -> Vec<Message> {
    let channel = msg.guild(&ctx.cache).unwrap()
        .channel_id_from_name(&ctx.cache, DTR_CHANNEL).unwrap();

    channel
        .messages(
            &ctx.http, 
            |m| m
        ).await.unwrap()
}

async fn get_last_time_in(ctx: &Context, msg: &Message) -> String {
    let messages = get_messages(ctx, msg).await;

    for m in messages {
        if m.embeds.len() != 0 {
            let id: &String = &m.embeds[0].fields[0].value;
            let id: String = id.chars().filter(|&c| c.is_digit(10)).collect();
            let id = id.parse::<u64>().unwrap();
            if id.to_string() != msg.author.id.to_string() { continue; }

            let action: &String = &m.embeds[0].fields[1].value;
            if action != "in" { continue; } 
    
            return m.embeds[0].fields[3].value.clone();
        }
    }

    String::from("None")
}

async fn get_last_time_out(ctx: &Context, msg: &Message) -> String {
    let messages = get_messages(ctx, msg).await;

    for m in messages {
        if m.embeds.len() != 0 {
            let id: &String = &m.embeds[0].fields[0].value;
            let id: String = id.chars().filter(|&c| c.is_digit(10)).collect();
            let id = id.parse::<u64>().unwrap();
            let action: &String = &m.embeds[0].fields[1].value;
            
            if id.to_string() != msg.author.id.to_string() {  continue}
  
            if action != "out" { break } 

            return m.embeds[0].fields[3].value.clone();
        }
    }
    
    String::from("None")
}

