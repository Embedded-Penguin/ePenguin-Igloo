use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use std::fmt::Display;
use config::*;

struct McuManifest
{
	ep_deps: HashMap<String, config::Value>,
	makefile_default_src_files: Vec<config::Value>,
	makefile_default_inc_dirs: Vec<config::Value>,
	drivers: Vec<config::Value>,
	modules: Vec<config::Value>,
}
impl Display for McuManifest
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		// print ep deps
		write!(f, "Printing EP DEPS:\n").unwrap();
		for dep in &self.ep_deps
		{
			write!(f, "{}: {}\n", &dep.0, &dep.1).unwrap();
		}

		// print makefile default src files
		write!(f, "\nDefault makefile sources:\n").unwrap();
		for src_file in &self.makefile_default_src_files
		{
			write!(f, "{}\n", &src_file).unwrap();
		}

		// print makefile default inc dirs
		for inc_dir in &self.makefile_default_inc_dirs
		{
			write!(f, "{}\n", &inc_dir).unwrap();
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

fn yml_to_mcu_manifest(conf: &mut config::Config) -> McuManifest
{
	McuManifest
	{
		ep_deps: conf.get_table("EP_DEPS").unwrap_or_default(),
		makefile_default_inc_dirs: conf.get_array("MAKEFILE_DEFAULT_INC_DIRS").unwrap_or_default(),
		makefile_default_src_files: conf.get_array("MAKEFILE_DEFAULT_SRC_FILES").unwrap_or_default(),
		drivers: conf.get_array("DRIVERS").unwrap_or_default(),
		modules: conf.get_array("MODULES").unwrap_or_default(),
	}
}
