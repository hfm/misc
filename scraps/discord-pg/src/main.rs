use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_VOICE_STATES;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    let shard_manager = client.shard_manager.clone();
    let http = client.http.clone();
    let cache = client.cache.clone();

    tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        };
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let guild_id = GuildId::from(env::var("GUILD_ID").unwrap().parse::<u64>()?);
    let channels = guild_id.channels(&http).await?;

    match guild_id.to_guild_cached(&cache) {
        Some(guild) => {
            for (channel_id, channel) in channels {
                if channel.kind == ChannelType::Voice {
                    println!("Channel: {:?}", channel.name);

                    let user_ids = guild
                        .voice_states
                        .iter()
                        .filter(|(_, vs)| vs.channel_id == Some(channel_id))
                        .collect::<Vec<_>>();

                    for (user_id, _) in user_ids {
                        if let Some(member) = guild.members.get(user_id) {
                            println!("Member: {:?}", member.user.name);
                        }
                    }
                }
            }
            println!("Guild: {:?}", guild.name);
        }
        None => {
            println!("Guild not found");
        }
    }

    shard_manager.shutdown_all().await;

    Ok(())
}
