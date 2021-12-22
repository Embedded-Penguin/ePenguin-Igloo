// igloo_manifest is a simple collection of functions that work with configs

use std::collections::HashMap;
use std::vec::Vec;
use crate::Igloo;

use crate::IglooStatus;
use crate::IglooStatus::*;

use crate::IglooType;
use crate::IglooType::*;

use crate::igloo_target::IglooTarget;
use serde::{Serialize, Deserialize};
use config::Config;

/// IglooTargetManifest - a manifest file locations which contain each target's
/// settings and configuration properties
#[derive(Serialize,Deserialize,Debug)]
pub struct IglooTargetManifest
{
	targets: HashMap::<String, String>,
}


impl IglooTargetManifest
{
	pub fn default() -> IglooTargetManifest
	{
		IglooTargetManifest
		{
			targets: HashMap::new(),
		}
	}
	pub fn get(igloo: &Igloo) -> Result<IglooTargetManifest, IglooStatus>
	{
		let mut target_manifest = config::Config::default();
		target_manifest.merge(
			config::File::with_name(
				igloo.env
					.esfd
					.clone()
					.join("manifest")
					.join("target-manifest.toml")
					.to_str().unwrap()
			)).unwrap();
		let ret = target_manifest.try_into::<IglooTargetManifest>().unwrap();
		Ok(ret)
	}
}
