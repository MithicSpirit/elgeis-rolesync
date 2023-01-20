use std::collections::HashSet;

use serenity::model::prelude::*;
// use serenity::utils::token::validate;

pub struct Config
{
	pub source:              GuildId,
	pub target:              GuildId,
	pub log_channel:         ChannelId,
	pub anchor_role:         RoleId,
	pub user_ignore:         HashSet<UserId>,
	pub source_ignore_roles: HashSet<RoleId>,
	pub target_ignore_roles: HashSet<RoleId>,
}

pub fn gen_config() -> (String, Config)
{
	let token = std::env::var("DISCORD_TOKEN").expect("Token unavailable");
	//validate(&token).expect(format!("Invalid token `{token}'").as_str());

	// hardcoded values for now; maybe read from file eventually?
	/*
	let source = GuildId(504795568313335809);
	let target = GuildId(578380339043368960);
	let log_channel = ChannelId(825045684507770891);
	let anchor_role = RoleId(646142219480334337);

	let user_ignore = [
		565360672414629888, // alt
		225775203433709569, // main
		814955773305421865, // self (precaution)
		// TODO: bots
	]
	.into_iter()
	.map(UserId)
	.collect();

	let source_ignore_roles = [
		// TODO: staff/bot roles, @everyone
		528683399070613514, // goomlandia
		516776728824381464, // crafters
	]
	.into_iter()
	.map(RoleId)
	.collect();

	let target_ignore_roles = [
		// TODO: @everyone
		616041023956582499, // *
		618118554998407178, // bot-admin
		915356172075294770, // **
		646161706535616513, // diplomat
		789254530109669457, // supreme leader
		835364277657141308, // co-supreme leader
		616046026238984211, // mayor of goomtown
		696834828561023055, // minister of econ
		648658994667323392, // minister of disc
		787731348680736780, // in faction
		578381904160620554, // goomlandian
		613553996417859584, // iskallian
		646142219480334337, // arbi
		847567208548270151, // do stuff
		798177763383050300, // grove faith
		847657407748505600, // ch of isk
		696837396284571778, // prop
		806359836517466165, // boost
		617452824480972821, // butt-saver medal
		652686626631188491, // official g medal
		617452704356368404, // turtle medal
		618118841611976777, // bot-private
		618118814772756500, // bot-public
		618110644847378453, // BOT
		774688386346778634, // gabe
		799063609459343430, // muted
	]
	.into_iter()
	.map(RoleId)
	.collect();
	*/

	let source = GuildId(1065285149345988724);
	let target = GuildId(1065285115795755049);
	let log_channel = ChannelId(1065285115795755052);
	let anchor_role = RoleId(1065822687936655480);

	let user_ignore = [
		565360672414629888, // alt
		225775203433709569, // main
		814955773305421865, // self
	]
	.into_iter()
	.map(UserId)
	.collect();

	let source_ignore_roles = [
		1065294312423821377, // bot
		1065295022330744884, // ignore1
		1065285149345988724, // @everyone
	]
	.into_iter()
	.map(RoleId)
	.collect();

	let target_ignore_roles = [
		1065294580871872586, // bot
		1065295668404551680, // ignore'1
		1065285115795755049, // @everyone
	]
	.into_iter()
	.map(RoleId)
	.collect();

	return (token, Config {
		source,
		target,
		log_channel,
		anchor_role,
		user_ignore,
		source_ignore_roles,
		target_ignore_roles,
	});
}
