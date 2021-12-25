// igloo_manifest is a simple collection of functions that work with configs

use std::collections::HashMap;
use crate::Igloo;

use serde::{Serialize, Deserialize};

use igloo_util::IglooDebugSeverity::*;
use igloo_util::IglooStatus::{self, *};
use igloo_util::TRACE_LEVEL;

/// IglooTargetManifest - a manifest file locations which contain each target's
/// settings and configuration properties
#[derive(Serialize,Deserialize,Debug)]
pub struct IglooTargetManifest
{
	pub targets: HashMap::<String, String>,
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
		igloo_debug!(TRACE,
					 IS_NONE,
					 "Reading master target manifest from {}",
					 igloo.env.esfd.join("manifest").join("target-manifest.toml").to_str().unwrap());

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

		igloo_debug!(INFO,
					 IS_NONE,
					 "Target Manifest deserialized: \n{:?}", ret);
		Ok(ret)
	}
}
