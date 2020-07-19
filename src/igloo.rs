

use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use config::*;
use std::error::Error;

use clap::{Arg, App};
enum BuildType
{
	Release,
	Debug,
}
pub struct ProjectManager
{
	makeManifest: MakeManifest,
	mcuManifest: McuManifest,
}

impl ProjectManager
{
	pub fn get_config() -> Self
	{
		let mut conf = Config::default();
		ProjectManager
		{
			makeManifest: MakeManifest::from_config(&mut conf),
			mcuManifest: McuManifest::from_config(&mut conf),
		}
	}
}
// used for managing cli requests for new and existing igloo projects
pub struct CliManager<'b>
{
	app: clap::App<'b>,
	debug_mode: bool,
	release_mode: bool,
	fresh_mode: bool,
	version: &'b str,
	name: &'b str,
	author: &'b str,
	description: &'b str,
}

impl<'b> CliManager<'b>
{
	pub fn new() -> Self
	{
		CliManager
		{
			version: concat!("v", crate_version!()),
			name: crate_name!(),
			author: crate_authors!(),
			description: crate_description!(),
			debug_mode: true,
			release_mode: false,
			fresh_mode: false,
			app: clap::App::new(crate_name!())
				.author(crate_authors!())
				.version(concat!("v", crate_version!())),
		}
	}


}
// Make Manifest contains default flags, files, and
// include directories. User files, flags, and directories
// can be added via the user yml in the cfg folder. They will be
// appended to these lists which are read from the default make manifest
// section in the mcu series yml file
pub struct MakeManifest
{
	linker_script: String,
	src_files: Vec<config::Value>,
	inc_dirs: Vec<config::Value>,
	cflags: Vec<config::Value>,
	libs: Vec<config::Value>,
	cc: String,
	ld: String,
	ar: String,
}

impl MakeManifest
{
	fn from_config(conf: &mut config::Config) -> MakeManifest
	{
		MakeManifest
		{
			src_files: conf.get_array("MAKEFILE_DEFAULT_SRC_FILES").unwrap_or_default(),
			inc_dirs: conf.get_array("MAKEFILE_DEFAULT_INC_DIRS").unwrap_or_default(),
			cflags: conf.get_array("CFLAGS").unwrap_or_default(),
			libs: conf.get_array("EP_LIBS").unwrap_or_default(),
			cc: conf.get_str("CC").unwrap_or_default(),
			ld: conf.get_str("LD").unwrap_or_default(),
			ar: conf.get_str("AR").unwrap_or_default(),
			linker_script: conf.get_str("LINKER_SCRIPT").unwrap_or_default(),
		}
	}
}
impl std::fmt::Display for MakeManifest
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		// print src files
		for src_file in &self.src_files
		{
			write!(f, "{}\n", src_file).unwrap();
		}
		// print inc dirs
		// print cflags
		// print cc
		// print ld
		// print ar
		// print default linker script

		Ok(())
	}
}

// MCU Manifest contains the options for the mcu
// If, for example, a MCU has a USART peripheral,
// it is listed as a driver option here
// The user project manifest's mcu options are compared to the
// available options from the epsf which are stored here once read
struct McuManifest
{
	core_deps: HashMap<String, config::Value>,
	drivers: Vec<config::Value>,
	modules: Vec<config::Value>,

}

impl McuManifest
{

	pub fn from_config(conf: &mut config::Config) -> McuManifest
	{
		McuManifest
		{
			core_deps: conf.get_table("EP_DEPS").unwrap_or_default(),
			drivers: conf.get_array("DRIVERS").unwrap_or_default(),
			modules: conf.get_array("MODULES").unwrap_or_default(),
		}
	}
}

impl std::fmt::Display for McuManifest
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		// print ep deps
		write!(f, "Printing core dependencies:\n").unwrap();
		for dep in &self.core_deps
		{
			write!(f, "{}: {}\n", &dep.0, &dep.1).unwrap();
		}

		// Available drivers
		write!(f, "\nAvailable drivers:\n").unwrap();
		for driver in &self.drivers
		{
			write!(f, "{}\n", &driver).unwrap();
		}

		// Available modules
		write!(f, "\nAvailable modules:\n").unwrap();
		for module in &self.modules
		{
			write!(f, "{}\n", &module).unwrap();
		}
		Ok(())
	}
}


