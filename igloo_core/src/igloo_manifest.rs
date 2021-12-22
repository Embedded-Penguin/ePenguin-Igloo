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

#[derive(Serialize, Deserialize, Debug)]
pub struct IglooProjectManifest
{
	name: String,
	targets: Vec::<String>
}

impl IglooProjectManifest
{
	pub fn default() -> IglooProjectManifest
	{
		IglooProjectManifest
		{
			name: String::from(""),
			targets: Vec::default(),

		}
	}
	pub fn from_project_file(self, igloo: &Igloo) -> Result<IglooProjectManifest, IglooStatus>
	{
		let mut config = config::Config::default();
		config.merge(
			config::File::with_name(
				igloo.env
					.cwd
					.clone()
					.join("igloo.toml")
					.to_str().unwrap())).unwrap();

		let z = config.deserialize::<IglooProjectManifest>().unwrap();
		println!("{:?}", z);

		Ok(IglooProjectManifest::default())
	}

	pub fn to_project_file(self, igloo: &Igloo) -> IglooStatus
	{
		IglooStatus::IS_GOOD
	}
}

