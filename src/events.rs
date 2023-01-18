use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

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
		todo!();
	}

	// elgeis
	async fn guild_ban_removal(
		&self,
		ctx: Context,
		guild: GuildId,
		user: User,
	)
	{
		todo!();
	}

	// client
	async fn guild_member_addition(&self, ctx: Context, member: Member)
	{
		todo!();
	}

	// elgeis
	async fn guild_member_removal(
		&self,
		ctx: Context,
		guild: GuildId,
		user: User,
	)
	{
		todo!();
	}

	async fn guild_member_update(
		&self,
		ctx: Context,
		update: GuildMemberUpdateEvent,
	)
	{
		todo!();
	}

	// elgeis
	async fn guild_role_delete(
		&self,
		ctx: Context,
		guild: GuildId,
		role: RoleId,
	)
	{
		todo!();
	}

	// elgeis
	async fn guild_role_update(&self, ctx: Context, role: Role)
	{
		todo!();
	}
}
