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
		}
	}

	pub fn new(inst: &Igloo, nameIn: &str, targetIn: &str)
			   -> Result<IglooPrj, IglooErrType>
	{
		let mut res_err = IglooErrType::IGLOO_ERR_NONE;
		if String::from(nameIn).is_empty()
		{
			res_err = IglooErrType::IGLOO_INVALID_PROJECT_NAME;
			return Err(res_err)
		}

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

		let mut _targ_make_table_name = inst.target_manifest.get_str(
			&("target.make.".to_owned() + &targetIn)).unwrap();
		let mut _targ_manifest_file_name = inst.target_manifest.get_str(
			&("target.manifest.".to_owned() + &targetIn)).unwrap();

		let mut temp: Vec<IglooTarget> = Vec::new();
		let targ = IglooTarget::from(
				inst,
				targetIn,
				&_targ_make_table_name,
				&_targ_manifest_file_name).unwrap();
		temp.push(targ);

		Ok(IglooPrj
		{
			name: String::from(nameIn),
			target_bank: temp,
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
		match std::os::unix::fs::symlink("", "")
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		println!("Displaying contents of {:?}", active_dir.display());
		for entry in active_dir.read_dir()
			.unwrap()
		{
			let dir = entry.unwrap();
			println!("{:?}", dir.file_name());
		}

		self.debugManifests();
		IglooErrType::IGLOO_ERR_NONE
	}

	pub fn debugManifests(&self)
	{
		for target in &self.target_bank
		{
			println!("Target manifest:");
			for (key, val) in &target.target_manifest
			{
				println!("{} = {:?}", key, val);
			}
			println!("\nMake Manifest:");
			for (key, val) in &target.make_manifest
			{
				println!("{} = {:?}", key, val);
			}
		}
	}

	/// Generates the target directories for all targets
	pub fn gen_targets(&self) -> IglooErrType
	{

		IglooErrType::IGLOO_ERR_NONE
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

	pub fn from(inst: &Igloo, name_in: &str,
				target_make_loc: &str,
				target_man_loc: &str) -> Result<IglooTarget, IglooErrType>
	{
		// target man first
		let mut target_man = config::Config::new();
		target_man.merge(
			config::File::with_name(
				IglooEnvInfo::info().esfd.join(target_man_loc)
					.to_str().unwrap()))
			.unwrap();

		// now make man
		let mut makefile: HashMap<String, config::Value> = HashMap::new();
		let mut make_table_head = &target_make_loc[0..target_make_loc.len()];
		println!("{}", make_table_head);
		let mut b_quit: bool = false;
		loop
		{
			let mut _active_table = inst.make_manifest.get_table(&make_table_head).unwrap();
			for (name, val) in _active_table
			{
				match val.clone().into_table()
				{
					Err(_e) =>
					{
						if !makefile.contains_key(&name)
						{
							makefile.insert(name, val);
						}
						else
						{
							let mut newval = val.clone().into_array().unwrap();
							let mut newvec = makefile.get_key_value(&name).unwrap().1.clone().into_array().unwrap();
							newvec.append(&mut newval);
							makefile.insert(name, config::Value::from(newvec));
						}
					}
					Ok(_v) => {}
				}
			}
			match make_table_head.rfind('.')
			{
				None => b_quit = true,
				Some(v) => make_table_head = &make_table_head[0..v],
			}
			if b_quit
			{
				break;
			}
		}

		Ok(IglooTarget
		{
			name: String::from(name_in),
			make_manifest: makefile,
			target_manifest: target_man.get_table("esf.links").unwrap(),

		})
	}

}


