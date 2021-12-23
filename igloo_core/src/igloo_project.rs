use crate::Igloo;
use crate::igloo_cli::*;

use crate::IglooType;
use crate::IglooType::*;

use crate::IglooStatus;
use crate::IglooStatus::*;

use crate::igloo_project;
use crate::igloo_target::IglooTarget;

use serde::{Serialize, Deserialize};
use config::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings
{
	pub testvar: String,
	pub profile: Profile,
}

/// Basic profile settings
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile
{
	pub name: String,
	pub targets: Vec::<String>

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

	pub fn add_target(&mut self, target_name: String)
	{
		if self.profile.targets.contains(&target_name)
		{
			return
		}
		self.profile.targets.push(target_name);
	}

	pub fn get_targets_from_config(igloo: &Igloo, config: &Settings) -> Vec<IglooTarget>
	{
		let mut _targets: Vec<IglooTarget> = Vec::new();
		for target in config.profile.targets.iter()
		{
			_targets.push(IglooTarget::target_from_name(igloo, String::from(target)).unwrap());
		}
		_targets
	}
}

pub struct IglooProject<'a>
{
	pub igloo: &'a Igloo,
	pub config: Settings,
	targets: Vec::<IglooTarget>,
	pub root: std::path::PathBuf,
}

impl<'a> IglooProject<'a>
{
	pub fn default(igloo_in: &'a Igloo) -> IglooProject
	{
		IglooProject
		{
			igloo: igloo_in,
			config: Settings::default(),
			targets: Vec::new(),
			root: std::path::PathBuf::new(),
		}
	}
	/// Used to populate an IglooProject from scratch
	/// This takes input from cli and generates the project in memory
	pub fn from_new(igloo_in: &'a Igloo, project_name: String) -> Result<IglooProject, IglooStatus>
	{
		let mut settings = Settings::default();
		settings.profile.name = String::from(&project_name);
		Ok(IglooProject
		{
			igloo: igloo_in,
			config: settings,
			targets: Vec::new(),
			root: igloo_in.env.cwd.join(&project_name),
		})
	}

	/// Used to create an IglooProject from an existing project
	/// So this will be called when things like
	/// igloo run, push, pull, erase, etc... are called
	pub fn from_existing(igloo_in: &'a Igloo) -> Result<IglooProject, IglooStatus>
	{
		let _config = Settings::default().from_project_file(igloo_in).unwrap();
		let _targets = Settings::get_targets_from_config(igloo_in, &_config);
		let _root = igloo_in.env.cwd.join(&_config.profile.name);
		let ret_project = IglooProject
		{
			igloo: igloo_in,
			config: _config,
			targets: _targets,
			root: _root,
		};
		Ok(IglooProject::default(igloo_in))
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
	pub fn generate(&self) -> IglooStatus
	{
		let mut ret: IglooStatus = IS_GOOD;

		// making this root and then cloning to work with active directory
		// so i can make changes to active dir and still have my project root if i need it
		// so far i havent needed it so i may just remove this

		let active_dir = std::path::PathBuf::new().join(&self.config.profile.name);
		// create new project directory
		match std::fs::create_dir(&active_dir)
		{
			Err(e) =>
			{
				println!("{:?}", e);
				return IS_BAD
			}
			_ => (),
		}

		// create igloo directory
		match std::fs::create_dir(&active_dir.clone().join("inc"))
		{
			Err(e) =>
			{
				println!("{:?}", e);
				return IS_BAD
			}
			_ => (),
		}

		// create src directory
		match std::fs::create_dir(&active_dir.clone().join("src"))
		{
			Err(e) =>
			{
				println!("{:?}", e);
				return IS_BAD
			}
			_ => (),
		}

		match std::fs::create_dir(&active_dir.clone().join("cfg"))
		{
			Err(e) =>
			{
				println!("{:?}", e);
				return IS_BAD
			}
			_ => (),
		}

		match std::fs::create_dir(&active_dir.clone().join("esf"))
		{
			Err(e) =>
			{
				println!("{:?}", e);
				return IS_BAD
			}
			_ => (),
		}

		// project folders finished
		// now do target folders
		ret = self.generate_targets();
		if ret != IS_GOOD
		{
			return ret
		}

		ret = self.generate_igloo_header();
		if ret != IS_GOOD
		{
			return ret
		}

		ret = self.generate_igloo_main();
		if ret != IS_GOOD
		{
			return ret
		}

		return ret
	}

	pub fn add_target_to_config(&mut self, target: String) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		self.config.add_target(target);
		ret
	}

	fn generate_targets(&self) -> IglooStatus
	{
		for target in &self.targets
		{
			target.generate(self);
		}
		IS_GOOD
	}

	fn generate_igloo_header(&self) -> IglooStatus
	{
		IS_GOOD
	}

	fn generate_igloo_main(&self) -> IglooStatus
	{
		IS_GOOD
	}
}
