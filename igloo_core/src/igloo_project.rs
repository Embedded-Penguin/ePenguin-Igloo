use crate::Igloo;
use crate::igloo_cli::*;

use crate::IglooType;
use crate::IglooType::*;

use crate::IglooStatus;
use crate::IglooStatus::*;

use crate::igloo_target::IglooTarget;

use serde::{Serialize, Deserialize};
use config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings
{
	testvar: String,
	profile: Profile,
}

/// Basic profile settings
#[derive(Serialize, Deserialize, Debug)]
struct Profile
{
	name: String,
	targets: Vec::<String>

}

impl Profile
{
	fn default() -> Profile
	{
		Profile
		{
			name: String::new(),
			targets: Vec::new(),
		}
	}
}

impl Settings
{
	pub fn default() -> Settings
	{
		Settings
		{
			testvar: String::new(),
			profile: Profile::default(),
		}
	}
	pub fn from_project_file(self, igloo: &Igloo) -> Result<Settings, IglooStatus>
	{
		let mut config = config::Config::default();
		config.merge(
			config::File::with_name(
				igloo.env
					.cwd
					.clone()
					.join("test")
					.join("igloo.toml")
					.to_str().unwrap())).unwrap();
		let x = config.try_into::<Settings>().unwrap();
		println!("{:?}", x);
		Ok(x)

	}

	pub fn to_project_file(self, igloo: &Igloo) -> IglooStatus
	{
		IglooStatus::IS_GOOD
	}

	pub fn set_profile_name(&mut self, name: String)
	{
		self.profile.name = name;
	}

	pub fn add_target(&mut self, target_name: String)
	{
		self.profile.targets.push(target_name);
	}
}

pub struct IglooProject<'a>
{
	igloo: &'a Igloo,
	config: Settings,
}

impl<'a> IglooProject<'a>
{
	pub fn default(igloo_in: &'a Igloo) -> IglooProject
	{
		IglooProject
		{
			igloo: igloo_in,
			config: Settings::default(),

		}
	}
	/// Used to populate an IglooProject from scratch
	/// This takes input from cli and generates the project in memory
	pub fn from_new(igloo_in: &'a Igloo, project_name: String) -> Result<IglooProject, IglooStatus>
	{
		let mut settings = Settings::default();
		settings.set_profile_name(project_name);
		Ok(IglooProject
		{
			igloo: igloo_in,
			config: settings,
		})
	}

	/// Used to create an IglooProject from an existing project
	/// So this will be called when things like
	/// igloo run, push, pull, erase, etc... are called
	pub fn from_existing(igloo_in: &'a Igloo) -> Result<IglooProject, IglooStatus>
	{
		Ok(IglooProject
		   {
			   igloo: igloo_in,
			   config: Settings::default().from_project_file(igloo_in).unwrap(),
		   })
	}

	pub fn is_igloo_prj(path: &std::path::PathBuf) -> bool
	{
		if !path.join("igloo").exists()
		{
			return false
		}

		if !path.join("igloo.toml").exists()
		{
			return false
		}
		return true
	}

	/// creates project files
	/// including igloo.toml
	pub fn generate(self) -> IglooStatus
	{
		IglooStatus::IS_GOOD
	}

	pub fn add_target(&mut self, target: String) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		self.config.add_target(target);
		ret
	}
}
