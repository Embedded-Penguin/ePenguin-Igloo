use igloo_base::*;
use igloo_base::IglooErrType::*;

use crate::IglooPrj;
use crate::Igloo;

use crate::config::Config;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;

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
		// Create target root directory
		match std::fs::create_dir(&self.root)
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}

		// Create target scripts directory
		match std::fs::create_dir(&self.root.join("scripts"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}

		ErrNone
	}

	/// populates all folders needed for the target
	pub fn populate(&self) -> IglooErrType
	{
		let mut target_scripts_dir: PathBuf = PathBuf::from(
			self.root.join("scripts"));
		// Read the gdb scripts directory in ESF
		let esf_target_scripts_dir = std::fs::read_dir(
			&(String::from(
				IglooEnvInfo::get_env_info()
					.esfd.to_str()
					.unwrap()) + "/scripts"))
			.unwrap();

		// Creating a vector to hold our gdb script file names
		let mut gdb_scripts: std::vec::Vec<std::path::PathBuf>
			= std::vec::Vec::new();

		// Grab the files only
		for entry in esf_target_scripts_dir
		{
			match &entry
			{
				Ok(v) => if !v.path().is_dir() {
					gdb_scripts.push(v.path()) },
				Err(e) => println!("{:?}", e),
			}
		}

		// Generate each GDB script
		for file in gdb_scripts
		{
			std::os::unix::fs::symlink(
				&file, &target_scripts_dir.join(&file.file_name().unwrap())).unwrap();
		}

		// Populate the project ESF folder with our targets relevant files
		let mut prj_esf_dir = self.root
			.parent().unwrap()
			.parent().unwrap()
			.parent().unwrap().join("ESF");
		println!("PRINTING {:?}", prj_esf_dir);
		for (sym_dir, loc_in_esf) in &self.links
		{
			let link_to_dir = IglooEnvInfo::get_env_info()
				.esfd
				.join(&loc_in_esf.clone().into_str().unwrap());
			std::os::unix::fs::symlink(link_to_dir, prj_esf_dir.join(sym_dir)).unwrap();
		}


		ErrNone
	}

	/// generates the makefile for a target
	/// this will be updated as the user edits their project toml
	pub fn generate_makefile(&self) -> IglooErrType
	{
		ErrNone
	}

	/// generates the openocd config for a target
	/// this will be updated as the user edits their project toml
	pub fn generate_openocd_config(&self) -> IglooErrType
	{
		let mut openocd_cfg = PathBuf::from(&self.root);
		openocd_cfg.push("scripts");
		openocd_cfg.push(&self.name);
		if openocd_cfg.with_extension("cfg").exists()
		{
			std::fs::remove_file(openocd_cfg.with_extension("cfg")).unwrap();
		}

		std::fs::File::create(
			openocd_cfg.with_extension("cfg")).unwrap();
		let mut ocfg_file = OpenOptions::new()
			.write(true)
			.append(true)
			.open(openocd_cfg.with_extension("cfg"))
			.unwrap();

		writeln!(ocfg_file, "#\n# ePenguin Generated OpenOCD \
							 Config Script\n#\n").unwrap();

		writeln!(ocfg_file, "\n# Transport Select").unwrap();
		writeln!(ocfg_file, "source [find interface//{}.cfg]", self
				 .openocd.get("transport_cfg")
				 .unwrap()
				 .clone()
				 .into_str()
				 .unwrap()).unwrap();
		writeln!(ocfg_file, "transport select {}", self
				 .openocd.get("transport")
				 .unwrap()
				 .clone()
				 .into_str()
				 .unwrap()).unwrap();

		writeln!(ocfg_file, "\n# Chip Information").unwrap();
		writeln!(ocfg_file, "set CHIPNAME {}", self.name).unwrap();
		writeln!(ocfg_file, "source [find target//{}.cfg]", self
				 .openocd.get("chip_name_cfg")
				 .unwrap()
				 .clone()
				 .into_str()
				 .unwrap()).unwrap();

		ErrNone
	}
}
