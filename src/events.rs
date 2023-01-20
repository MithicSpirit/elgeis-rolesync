use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::actions;
use crate::context::keys::*;
use crate::context::populate_context;

pub struct Handler {}
pub const HANDLER: Handler = Handler {};

#[async_trait]
impl EventHandler for Handler
{
	async fn ready(&self, ctx: Context, _: Ready)
	{
		println!("Bot started!");
		populate_context(&ctx).await;
		let data = ctx.data.read().await;
		let roles = data
			.get::<RoleMap>()
			.expect("Bot initialized incorrectly");
		println!("Bot initialized");
		dbg!(roles);
	}

	// elgeis
	async fn guild_ban_addition(
		&self,
		ctx: Context,
		guild: GuildId,
		user: User,
	)
	{
		let data = &ctx.data.read().await;
		let config = data.get::<Config>().unwrap();
		if guild != config.source
			|| config.user_ignore.contains(&user.id)
		{
			return;
		}
		actions::ban(&ctx, user).await;
	}

	// elgeis
	async fn guild_ban_removal(
		&self,
		ctx: Context,
		guild: GuildId,
		user: User,
	)
	{
		let data = &ctx.data.read().await;
		let config = data.get::<Config>().unwrap();
		if guild != config.source
			|| config.user_ignore.contains(&user.id)
		{
			return;
		}
		actions::unban(&ctx, user).await;
	}

	// client
	async fn guild_member_addition(&self, ctx: Context, member: Member)
	{
		let data = &ctx.data.read().await;
		let config = data.get::<Config>().unwrap();
		if member.guild_id != config.target
			|| config.user_ignore.contains(&member.user.id)
		{
			return;
		}
		let Ok(source_member) = config
			.source
			.member(&ctx.http, &member.user.id)
			.await else {
			return;
		};
		actions::sync_user(&ctx, source_member, member).await;
	}

	// elgeis
	async fn guild_member_removal(
		&self,
		ctx: Context,
		guild: GuildId,
		user: User,
	)
	{
		let data = &ctx.data.read().await;
		let config = data.get::<Config>().unwrap();
		if guild != config.source
			|| config.user_ignore.contains(&user.id)
		{
			return;
		}
		let Ok(member) = config
			.target
			.member(&ctx.http, &user.id)
			.await else {
			return;
		};
		actions::clean(&ctx, member).await;
	}

	// elgeis
	async fn guild_member_update(
		&self,
		ctx: Context,
		event: GuildMemberUpdateEvent,
	)
	{
		let data = &ctx.data.read().await;
		let config = data.get::<Config>().unwrap();
		if event.guild_id != config.source
			|| config.user_ignore.contains(&event.user.id)
		{
			return;
		}
		let (Ok(source_member), Ok(target_member)) = tokio::join!(
			config.source.member(&ctx.http, &event.user.id),
			config.source.member(&ctx.http, &event.user.id),
		) else {
			return;
		};
		actions::sync_user(&ctx, source_member, target_member).await;
	}

	// elgeis
	async fn guild_role_delete(
		&self,
		ctx: Context,
		guild: GuildId,
		role: RoleId,
	)
	{
		let data = &ctx.data.read().await;
		let config = data.get::<Config>().unwrap();
		let rolemap = data.get::<RoleMap>().unwrap();
		if guild != config.source {
			return;
		}
		let Some(target_role) = rolemap.get(&role) else {
			return;
		};
		actions::delete(&ctx, *target_role).await;
	}

	// elgeis
	async fn guild_role_update(&self, ctx: Context, role: Role)
	{
		let data = &ctx.data.read().await;
		let config = data.get::<Config>().unwrap();
		let rolemap = data.get::<RoleMap>().unwrap();
		if role.guild_id != config.source {
			return;
		}
		let Some(target_role) = rolemap.get(&role.id) else {
			return;
		};
		actions::sync_role(&ctx, role, *target_role).await;
	}
}
