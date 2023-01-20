use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::context::keys::*;

pub async fn delete(ctx: &Context, role: RoleId)
{
	let mut data = ctx.data.write().await;
	match data
		.get::<Config>()
		.unwrap()
		.target
		.delete_role(&ctx.http, role)
		.await
	{
		Ok(()) => {
			data.get_mut::<RevRoleMap>()
				.unwrap()
				.remove(&role)
				.map(|r| {
					data.get_mut::<RoleMap>()
						.unwrap()
						.remove(&r)
				});
		},
		Err(why) =>
			println!(
				"An error ocurred while deleting a role: {:?}",
				why
			),
	};
}

pub async fn sync(ctx: &Context, source: Role, target_id: RoleId)
{
	let data = ctx.data.read().await;
	let conf = data.get::<Config>().unwrap();
	let Some(target) = conf
		.target
		.roles(&ctx.http)
		.await
		.unwrap()
		.remove(&target_id) else {
			return;
		};
	if let Err(why) = target
		.edit(&ctx.http, |r| {
			r.colour(source.colour.0.into()).name(source.name)
		})
		.await
	{
		println!("An error ocurred while editing a role: {:?}", why);
	};
}

pub(super) async fn create_role(
	ctx: &Context,
	source_id: RoleId,
) -> Option<RoleId>
{
	let mut data = ctx.data.write().await;
	let conf = data.get::<Config>().unwrap();
	let (target_roles_maybe, source_roles_maybe) = tokio::join!(
		conf.target.roles(&ctx.http),
		conf.source.roles(&ctx.http),
	);
	let position =
		match conf.anchor_next_position(target_roles_maybe.unwrap()) {
			Some(p) => p,
			None => {
				println!("Could not find anchor role; defaulting to position 1.");
				0
			},
		};
	let source = source_roles_maybe.unwrap().remove(&source_id).unwrap();
	match conf
		.target
		.create_role(&ctx.http, |r| {
			r.mentionable(true)
				.hoist(false)
				.permissions(Permissions::empty())
				.position(position)
				.colour(source.colour.0.into())
				.name(source.name)
		})
		.await
	{
		Ok(role) => {
			data.get_mut::<RoleMap>()
				.unwrap()
				.insert(source.id, role.id);
			data.get_mut::<RevRoleMap>()
				.unwrap()
				.insert(role.id, source.id);
			return Some(role.id);
		},
		Err(why) => {
			println!(
				"An error ocurred while creating a role: {:?}",
				why
			);
			return None;
		},
	};
}
