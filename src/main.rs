use std::env;

use serenity::client::ClientBuilder;
use serenity::prelude::*;

mod events;

// Registers handlers and starts the bot
#[tokio::main]
async fn main()
{
	let token = env::var("DISCORD_TOKEN").unwrap_or_default();
	// validate(&token).expect(format!("Invalid token `{token}'").as_str());
	// ^ doesn't work for some reason

	let intents = GatewayIntents::non_privileged()
		| GatewayIntents::GUILD_MEMBERS;

	let mut client = ClientBuilder::new(token, intents)
		.await
		.expect("Failed to create client");

	if let Err(why) = client.start().await {
		println!(
			"An error occurred while running the client: {:?}",
			why
		);
	}
}
