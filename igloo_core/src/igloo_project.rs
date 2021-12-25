use crate::Igloo;

use crate::igloo_target::IglooTarget;

use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions};
use igloo_util::IglooDebugSeverity::*;
use igloo_util::IglooStatus::{self, *};
use igloo_util::TRACE_LEVEL;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings
{
	pub testvar: Option<String>,
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
			testvar: Option::default(),
			profile: Profile::default(),
		}
	}
	pub fn from_project_file(prj: &IglooProject) -> Result<Settings, IglooStatus>
	{
		let mut config = config::Config::default();
		config.merge(
			config::File::with_name(
				prj.igloo.env
					.cwd
					.clone()
					.join("test")
					.join("igloo.toml")
					.to_str().unwrap())).unwrap();
		let x = config.try_into::<Settings>().unwrap();
		println!("{:?}", x);
		Ok(x)

	}

	pub fn to_project_file(prj: &IglooProject) -> IglooStatus
	{
		let prj_cfg_path = prj
			.root
			.clone()
			.join("igloo")
			.join("igloo.toml");
		std::fs::File::create(&prj_cfg_path).unwrap();
		let mut prj_cfg_file = OpenOptions::new()
			.write(true)
			.append(true)
			.open(&prj_cfg_path)
			.unwrap();

		let contents = toml::to_string(&prj.config).unwrap();
		igloo_debug!(TRACE, IS_NONE, "{}", contents);
		println!("PRINTING THIS ON ITS OWN: {}", contents);
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

	/// This function is labeled .._from_config, but the project contains
	/// the environment vars (from &Igloo) and config already
	pub fn get_targets_from_config(prj: &IglooProject) -> Vec<IglooTarget>
	{
		let mut _targets: Vec<IglooTarget> = Vec::new();
		for target in prj.config.profile.targets.iter()
		{
			_targets.push(IglooTarget::target_from_name(prj.igloo, String::from(target)).unwrap());
		}
		_targets
	}
}

pub struct IglooProject<'a>
{
	pub igloo: &'a Igloo,
	pub config: Settings,
	pub targets: Vec::<IglooTarget>,
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
		igloo_debug!(TRACE, IS_NONE, "Creating new igloo project named {}", project_name);
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
		// These vars need to be acquired in this order when creating a project from an existing project
		// The config requires the &Igloo
		// targets requires config and &Igloo
		// root just requires the project name, but its best to do it last to make sure everything else is valid
		let mut ret_project = IglooProject::default(igloo_in);
		ret_project.config = Settings::from_project_file(&ret_project).unwrap();
		ret_project.targets = Settings::get_targets_from_config(&ret_project);
		ret_project.root = igloo_in.env.cwd.join(&ret_project.config.profile.name);
		Ok(ret_project)
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

		let active_dir = self.root.clone();
		// create new project directory
		match std::fs::create_dir(&active_dir)
		{
			Err(e) =>
			{
				ret = IS_FAILED_TO_CREATE_DIR;
				igloo_debug!(ERROR, ret, "Failed to create dir: {:?} | {:?}", &active_dir, e);
				return ret
			}
			_ => (),
		}

		// create igloo directory
		match std::fs::create_dir(&active_dir.clone().join("igloo"))
		{
			Err(e) =>
			{
				ret = IS_FAILED_TO_CREATE_DIR;
				igloo_debug!(ERROR, ret, "Failed to create dir: {:?} | {:?}", &active_dir.clone().join("igloo"), e);
				return ret
			}
			_ => (),
		}
		match std::fs::create_dir(&active_dir.clone().join("inc"))
		{
			Err(e) =>
			{
				ret = IS_FAILED_TO_CREATE_DIR;
				igloo_debug!(ERROR, ret, "Failed to create dir: {:?} | {:?}", &active_dir.clone().join("inc"), e);
				return ret
			}
			_ => (),
		}

		// create src directory
		match std::fs::create_dir(&active_dir.clone().join("src"))
		{
			Err(e) =>
			{
				ret = IS_FAILED_TO_CREATE_DIR;
				igloo_debug!(ERROR, ret, "Failed to create dir: {:?} | {:?}", &active_dir.clone().join("src"), e);
				return ret
			}
			_ => (),
		}

		match std::fs::create_dir(&active_dir.clone().join("cfg"))
		{
			Err(e) =>
			{
				ret = IS_FAILED_TO_CREATE_DIR;
				igloo_debug!(ERROR, ret, "Failed to create dir: {:?} | {:?}", &active_dir.clone().join("cfg"), e);
				return ret
			}
			_ => (),
		}

		match std::fs::create_dir(&active_dir.clone().join("esf"))
		{
			Err(e) =>
			{
				ret = IS_FAILED_TO_CREATE_DIR;
				igloo_debug!(ERROR, ret, "Failed to create dir: {:?} | {:?}", &active_dir.clone().join("esf"), e);
				return ret
			}
			_ => (),
		}

		// project folders finished
		// create project settings file (igloo.toml)
		ret = self.generate_project_config();
		if ret != IS_GOOD
		{
			igloo_debug!(WARNING, ret);
			return ret
		}

		// now do target folders
		ret = self.generate_targets();
		if ret != IS_GOOD
		{
			igloo_debug!(WARNING, ret);
			return ret
		}


		return ret
	}

	fn generate_targets(&self) -> IglooStatus
	{
		for target in &self.targets
		{
			target.generate(self);
		}
		IS_GOOD
	}

	pub fn generate_igloo_header(&self) -> IglooStatus
	{
		IS_GOOD
	}

	pub fn generate_igloo_main(&self) -> IglooStatus
	{
		IS_GOOD
	}

	pub fn generate_project_config(&self) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		Settings::to_project_file(self);
		ret
	}
}
