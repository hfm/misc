use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILDS;
    let client = Client::builder(&token, intents).await?;
    let channel = ChannelId::from(1335521136569745482);
    let channel_name = channel.name(&client.http).await?;
    println!("Channel name: {}", channel_name);

    Ok(())
}
