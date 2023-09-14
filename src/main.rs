use anyhow::anyhow;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

// STANDARD FRAMEWORK
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Bot;

// command modules.
mod commands {
    pub mod dtr;
    pub mod calculator;
    pub mod help;
    pub mod about;
    pub mod other;
}
use commands::dtr;
use commands::help::*;
use commands::about::*;
use commands::calculator::*;
use commands::other::*;

#[group]
#[commands(about, help, sum, quo, diff, prod, flip, roll)]
struct General;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!in"{
            dtr::time_in(&ctx, &msg).await;
        }
        else if msg.content == "!out"{
            dtr::time_out(&ctx, &msg).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, intents)
        .event_handler(Bot)
        .framework(framework)
        .await.expect("Err creating client");

    Ok(client.into())
}
