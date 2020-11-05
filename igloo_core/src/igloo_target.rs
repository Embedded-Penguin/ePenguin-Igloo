use igloo_base::*;
use igloo_base::IglooErrType::*;

use crate::IglooPrj;
use crate::Igloo;

use crate::config::Config;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct IglooTarget
{
	// name, links, and includes are extracted from a manifest
	pub name: String,
	pub links: HashMap<String, config::Value>,
	pub includes: Vec<config::Value>,
	pub openocd: HashMap<String, config::Value>,
	pub make_manifest: HashMap<String, config::Value>,
	pub root: PathBuf,
}

impl IglooTarget
{
	pub fn default() -> IglooTarget
	{
		IglooTarget
		{
			name: String::from(""),
			root: PathBuf::default(),
			make_manifest: HashMap::default(),
			links: HashMap::default(),
			includes: Vec::default(),
			openocd: HashMap::default(),
		}
	}

	pub fn from(root: PathBuf, inst: &Igloo, name_in: String,
				target_make_loc: &str,
				target_man_loc: &str) -> Result<IglooTarget, IglooErrType>
	{
		// target man first
		let mut target_man = Config::new();
		target_man.merge(
			config::File::with_name(
				IglooEnvInfo::get_env_info().esfd.join(target_man_loc)
					.to_str().unwrap()))
			.unwrap();

		// now make man
		let mut makefile: HashMap<String, config::Value> = HashMap::new();
		let mut make_table_head = &target_make_loc[0..target_make_loc.len()];
		let mut b_quit: bool = false;
		loop
		{
			let mut _active_table = inst.master_make_man.get_table(&make_table_head).unwrap();
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
			links: target_man.get_table("esf.links").unwrap(),
			includes: target_man.get_table("esf.includes")
				.unwrap()
				.get("IGLOO_INCLUDES")
				.unwrap()
				.clone()
				.into_array()
				.unwrap(),
			openocd: target_man.get_table("esf.openocd")
				.unwrap(),
			root: root,
		})
	}

	/// generate all folders needed for the target
	pub fn generate(&self) -> IglooErrType
	{

		ErrNone
	}

	/// populates all folders needed for the target
	pub fn populate(&self) -> IglooErrType
	{
		ErrNone
	}

	/// generates the makefile for a target
	/// this will be updated as the user edits their project toml
	pub fn generate_makefile(&self) -> IglooErrType
	{
		ErrNone
	}
}
