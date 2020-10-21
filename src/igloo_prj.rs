use crate::igloo::{Igloo, IglooEnvInfo, IglooErrType};
use crate::igloo_manifest::IglooManifest;
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

	pub fn new(inst: &Igloo, nameIn: &str, targetIn: &str)
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
		match IglooManifest::target_exists(inst, targetIn)
		{
			Ok(v) =>
			{
				if v
				{
					println!("Verified target exists {}", nameIn);
				}
				else
				{
					println!("Couldn't verify target exists {}", nameIn);
					return Err(IglooErrType::IGLOO_INVALID_TARGET)
				}
			}
			Err(e) =>
			{
				return Err(e)
			}
		}


		Ok(IglooPrj
		{
			name: String::from(nameIn),
			target_bank: Vec::new(),
			env_info: IglooEnvInfo::info(),
		})
	}

	pub fn populate(&self) -> IglooErrType
	{
		// Create new directory
		let mut active_dir = IglooEnvInfo::info().cwd;
		//println!("Active Directory: {:?}", active_dir.display());
		println!("NAME: {}", self.name);
		active_dir.push(&self.name);
		match std::fs::create_dir(&active_dir)
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		//println!("Active Directory: {:?}", active_dir.display());
		println!("Creating .igloo dir...");
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join(".igloo"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join("src"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join("inc"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join("cfg"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join("ESF"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}

		// load targets
		//create symlinks in ESF
		// match std::os::unix::fs::symlink("", "")
		// {
		// 	Err(e) => println!("{:?}", e),
		// 	_ => (),
		// }
		println!("Displaying contents of {:?}", active_dir.display());
		for entry in active_dir.read_dir()
			.unwrap()
		{
			let dir = entry.unwrap();
			println!("{:?}", dir.file_name());
		}

		IglooErrType::IGLOO_ERR_NONE
	}

	pub fn env(self) -> IglooEnvInfo
	{
		self.env_info.clone()
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

}


