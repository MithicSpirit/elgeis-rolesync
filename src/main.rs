use serenity::client::ClientBuilder;
use serenity::prelude::*;

pub(crate) mod context;
mod events;
pub(crate) mod role;
pub(crate) mod user;

// Registers handlers and starts the bot
#[tokio::main]
async fn main()
{
	let (token, data) = context::gen_context_data();
	let intents = GatewayIntents::non_privileged()
		| GatewayIntents::GUILD_MEMBERS;

	let mut client = ClientBuilder::new(token, intents)
		.event_handler(events::HANDLER)
		.type_map(data)
		.await
		.expect("Failed to create client");
	println!("Initialized bot");

	if let Err(why) = client.start().await {
		println!(
			"An error occurred while running the client: {:?}",
			why
		);
	}
}
