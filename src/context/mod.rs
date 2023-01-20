use std::collections::HashMap;
use std::sync::Arc;

use serenity::http::client::Http;
use serenity::model::prelude::*;
use serenity::prelude::*;

mod config;
use keys::*;

pub fn gen_context_data() -> (String, TypeMap)
{
	let mut data = TypeMap::new();
	let (token, conf) = config::gen_config();
	data.insert::<Config>(conf);

	return (token, data);
}

pub async fn populate_context(ctx: &Context)
{
	let mut data = ctx.data.write().await;
	let config = data
		.get::<Config>()
		.expect("Context data was not initialized correctly");

	let (target_roles_name_id, source_roles_id_name) = tokio::join!(
		target_roles(config, &ctx.http),
		source_roles(config, &ctx.http),
	);

	//dbg!(source_roles_id_name);
	//dbg!(target_roles_name_id);

	let mut role_map: HashMap<RoleId, RoleId> = HashMap::new();
	let mut rev_role_map: HashMap<RoleId, RoleId> = HashMap::new();
	for (source_role, target_role) in
		source_roles_id_name.iter().filter_map(|(id, name)| {
			target_roles_name_id.get(name).map(|x| (*id, *x))
		}) {
		role_map.insert(source_role, target_role);
		rev_role_map.insert(target_role, source_role);
	}

	data.insert::<RoleMap>(role_map);
	data.insert::<RevRoleMap>(rev_role_map);
}

async fn target_roles(
	config: &Config,
	http: &Arc<Http>,
) -> HashMap<String, RoleId>
{
	config.target
		.roles(http)
		.await
		.expect("Error getting roles from target server")
		.iter()
		.filter(|(id, _)| !config.target_ignore_roles.contains(id))
		.map(|(id, role)| (role.name.clone(), *id))
		.collect()
}
async fn source_roles(
	config: &Config,
	http: &Arc<Http>,
) -> HashMap<RoleId, String>
{
	config.source
		.roles(http)
		.await
		.expect("Error getting roles from source server")
		.iter()
		.filter(|(id, _)| !config.source_ignore_roles.contains(id))
		.map(|(id, role)| (*id, role.name.clone()))
		.collect()
}

pub mod keys
{
	use serenity::model::prelude::*;
	use serenity::prelude::*;

	pub use super::config::Config;
	use super::HashMap;

	impl TypeMapKey for Config
	{
		type Value = Config;
	}

	pub struct RoleMap {}
	impl TypeMapKey for RoleMap
	{
		type Value = HashMap<RoleId, RoleId>;
	}

	pub struct RevRoleMap {}
	impl TypeMapKey for RevRoleMap
	{
		type Value = HashMap<RoleId, RoleId>;
	}
}
