use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::context::keys::*;

pub async fn ban(ctx: &Context, user: User)
{
	let data = ctx.data.read().await;
	if let Err(why) = data
		.get::<Config>()
		.unwrap()
		.target
		.ban_with_reason(&ctx.http, user, 0, "Synchronized from Elgeis")
		.await
	{
		println!("An error ocurred while banning a user: {:?}", why);
	};
}

pub async fn unban(ctx: &Context, user: User)
{
	let data = ctx.data.read().await;
	if let Err(why) = data
		.get::<Config>()
		.unwrap()
		.target
		.unban(&ctx.http, user)
		.await
	{
		println!("An error ocurred while unbanning a user: {:?}", why);
	};
}

pub async fn sync(ctx: &Context, source: Member, target: Member)
{
	todo!();
}

pub async fn clean(ctx: &Context, mut member: Member)
{
	let data = ctx.data.read().await;
	let synced_roles = &data.get::<RevRoleMap>().unwrap();
	let to_remove: Vec<RoleId> = member
		.roles
		.iter()
		.filter(|role| synced_roles.contains_key(role))
		.map(|r| *r)
		.collect();
	if let Err(why) =
		member.remove_roles(&ctx.http, to_remove.as_slice()).await
	{
		println!(
			"An error ocurred while removing a user's roles: {:?}",
			why
		)
	};
}
