use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::context::keys::*;

pub async fn delete(ctx: &Context, role: RoleId)
{
	let data = ctx.data.read().await;
	if let Err(why) = data
		.get::<Config>()
		.unwrap()
		.target
		.delete_role(&ctx.http, role)
		.await
	{
		println!("An error ocurred while deleting a role: {:?}", why);
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

pub(super) async fn create_role(ctx: &Context, source: Role)
{
	let data = ctx.data.read().await;
	let conf = data.get::<Config>().unwrap();
	let position = match conf.anchor_next_position(
		conf.target.roles(&ctx.http).await.unwrap(),
	) {
		Some(p) => p,
		None => {
			println!("Could not find anchor role; defaulting to position 1.");
			0
		},
	};
	if let Err(why) = conf
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
		println!("An error ocurred while creating a role: {:?}", why);
	};
}
