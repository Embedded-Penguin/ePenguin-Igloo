use crate::igloo::{Igloo, IglooEnvInfo, IglooErrType};
use crate::igloo_manifest::{IglooManifest};
use std::collections::HashMap;
// New Project
// --- Verify location
// --- Populate base folders
// --- create .igloo/<PrjName>.toml
// --- Create default target in ./igloo<PrjName>.toml
// --- Read Default Targets manifest toml
// --- generate projects core manifest toml
// --- Spawn user manifest config
pub struct IglooPrj
{
	name: String,
	target_bank: Vec<IglooTarget>,
	env_info: IglooEnvInfo,
}

pub struct IglooTarget
{
	name: String,
	make_manifest: HashMap<String, config::Value>,
	target_manifest: HashMap<String, config::Value>,
}

impl IglooPrj
{
	pub fn default() -> IglooPrj
	{
		IglooPrj
		{
			name: String::from(""),
			target_bank: Vec::default(),
			env_info: IglooEnvInfo::info(),
		}
	}

	pub fn new(nameIn: &str, targetIn: &str, env_infoIn: &IglooEnvInfo)
			   -> Result<IglooPrj, IglooErrType>
	{
		let mut res_err = IglooErrType::IGLOO_ERR_NONE;
		loop
		{
			if String::from(nameIn).is_empty()
			{
				res_err = IglooErrType::IGLOO_INVALID_PROJECT_NAME;
				break;
			}


		break; }

		if res_err != IglooErrType::IGLOO_ERR_NONE
		{
			return Err(res_err)
		}

		Ok(IglooPrj
		{
			name: String::from(nameIn),
			target_bank: Vec::new(),
			env_info: IglooEnvInfo::info(),
		})
	}
}

impl IglooTarget
{
	pub fn default() -> IglooTarget
	{
		IglooTarget
		{
			name: String::from(""),
			make_manifest: HashMap::default(),
			target_manifest: HashMap::default(),
		}
	}

	pub fn from(inst: &Igloo, nameIn: &str) -> Result<IglooTarget, IglooErrType>
	{
		if !IglooManifest::target_exists(inst, nameIn)
		{
			return Err(IglooErrType::IGLOO_INVALID_TARGET)
		}


		Ok(IglooTarget::default())
	}
}


