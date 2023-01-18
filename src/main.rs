use serenity::client::ClientBuilder;
use serenity::prelude::*;

pub(crate) mod config;
mod events;
pub(crate) mod user;

// Registers handlers and starts the bot
#[tokio::main]
async fn main()
{
	let config = config::gen_config();

	let intents = GatewayIntents::non_privileged()
		| GatewayIntents::GUILD_MEMBERS;

	let mut client = ClientBuilder::new(config.token, intents)
		.event_handler(events::HANDLER)
		.await
		.expect("Failed to create client");

	if let Err(why) = client.start().await {
		println!(
			"An error occurred while running the client: {:?}",
			why
		);
	}
}
