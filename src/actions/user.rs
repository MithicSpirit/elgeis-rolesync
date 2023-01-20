use serenity::model::prelude::*;
use serenity::prelude::*;

use super::create_role;
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
	let data = ctx.data.read().await;
	let conf = data.get::<Config>().unwrap();
	let role_map = data.get::<RoleMap>().unwrap();

	let (found, not_found): (Vec<_>, Vec<_>) = source
		.roles
		.into_iter()
		.filter(|r| !conf.source_ignore_roles.contains(r))
		.map(|r| (r, role_map.get(&r)))
		.partition(|(_, maybe)| maybe.is_some());
	let mut roles: Vec<_> =
		found.into_iter().map(|(_, maybe)| *maybe.unwrap()).collect();
	roles.extend(futures::future::join_all(
		not_found.into_iter().map(|(r, _)| create_role(ctx, r)),
	)
	.await
	.iter()
	.filter_map(|maybe| maybe.as_ref()));

	if let Err(why) = target
		.edit(&ctx.http, |mem| {
			mem.nickname(source.nick.unwrap_or_else(|| "".into()))
				.roles(roles)
		})
		.await
	{
		println!("An error ocurred while updating a member: {:?}", why)
	};
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
