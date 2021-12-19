// igloo_manifest is a simple collection of functions that work with configs

use std::collections::HashMap;
use std::vec::Vec;
use crate::Igloo;

use crate::IglooStatus;
use crate::IglooStatus::*;

use crate::IglooType;
use crate::IglooType::*;

use crate::igloo_target::IglooTarget;

/// Igloo Manifest Helper functions
/// USES: environment variables (home dir, esf dir, current dir)
/// DOES: brings target manifest into memory
pub fn imh_get_master_target_manifest(inst: &mut Igloo) -> IglooStatus
{
	let mut ret: IglooStatus = IS_GOOD;

	match inst.master_target_manifest.merge(
		config::File::with_name(
			inst.env.esfd.join("manifest/target-manifest.toml").to_str.unwrap()))
	{
		Ok(_v) => (),
		Err(e) =>
		{
			println!("Error: {:?}", e);
			ret = IS_FAILED_TO_LOAD_MTM;
		}
	}
	ret
}

pub fn imh_get_project_name(inst: &Igloo) -> String
{
	let project_config: config::Config = config::Config::new();
	match project_config.merge(
		config::File::with_name(
			inst.env.cwd.clone().join("igloo.toml").to_str().unwrap()))
	{
		Ok(v) =>
		{
			return v.deserialize::<HashMap<String, String>>().unwrap()["Project"]
		}
		Err(e) => panic!(),
	}
}

pub fn imh_get_targets(inst: &Igloo) -> Vec<IglooTarget>
{
	let project_config: config::Config = config::Config::new();
	match project_config.merge(
		config::File::with_name(
			inst.env.cwd.clone().join("igloo.toml").to_str().unwrap()))
	{
		Ok(v) =>
		{
			match v.get("Targets")
			{
				Some(v2) =>
				{
					for target in 
					{

					}
				}
				None => panic!(),
			}
		}
	}
}
