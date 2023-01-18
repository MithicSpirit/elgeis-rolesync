use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct Handler {}
pub const HANDLER: Handler = Handler {};

#[async_trait]
impl EventHandler for Handler
{
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
		_: Option<Member>,
	)
	{
		todo!();
	}

	async fn guild_member_update(
		&self,
		ctx: Context,
		old: Option<Member>,
		member: Member,
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
		data: Option<Role>,
	)
	{
		todo!();
	}

	// elgeis
	async fn guild_role_update(
		&self,
		ctx: Context,
		old: Option<Role>,
		role: Role,
	)
	{
		todo!();
	}
}
