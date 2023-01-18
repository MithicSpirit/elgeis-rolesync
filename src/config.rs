use serenity::model::prelude::*;
use serenity::utils::token::validate;

pub struct Config
{
	pub token:       String,
	pub source:      GuildId,
	pub target:      GuildId,
	pub log_channel: ChannelId,
	pub user_ignore: Vec<UserId>,
	pub role_ignore: Vec<RoleId>,
}

pub fn gen_config() -> Config
{
	let token = std::env::var("DISCORD_TOKEN").expect("Token unavailable");
	validate(&token).expect(format!("Invalid token `{token}'").as_str());

	// hardcoded values for now; maybe read from file eventually?
	let source = GuildId(504795568313335809);
	let target = GuildId(578380339043368960);
	let log_channel = ChannelId(825045684507770891);
	let user_ignore = [
		565360672414629888, // alt
		225775203433709569, // main
		814955773305421865, // self (precaution)
	]
	.into_iter()
	.map(UserId)
	.collect::<Vec<_>>();
	let role_ignore = [
		// TODO: Staff roles
		528683399070613514, // goomlandia
		516776728824381464, // crafters
	]
	.into_iter()
	.map(RoleId)
	.collect::<Vec<_>>();

	return Config {
		token,
		source,
		target,
		log_channel,
		user_ignore,
		role_ignore,
	};
}
