// Note to self: It makes way more sense for the target bank in our project
// to be a hashmap of type <String,IglooTarget> so that every time I want to
// BUT rust deserialization seems to favor these structs which includes the name
// which means I'm storing the name in the target and not outside of it.
// so for now I'm going to do it this way until I do the serialization myself (switch to syncing instead of rwwr),
// which is what I did in 0.1, before the core rewrite
// I'm only doing it this way because serialization seems to be easier this way
// After I get all core features (new, push, pull, erase, etc...) completed,
// I'll revisit this part of the project and change it to a more sensible management method
//
// Update to this: The deserialization doesn't even work with the scheme I was using...
// Going back to the old way until I learn more.
// I could make the deserialization work by default by adding a billion different
// structs, but this is honestly just a trash way of doing it and I think the idea
// of doing it that way is only an ideal solution. It isn't very practical.
use igloo_util::*;
use crate::IglooProject;
use serde::{Serialize, Deserialize};
use std::vec::Vec;
use igloo_util::IglooDebugSeverity::*;
use igloo_util::IglooStatus::{self, *};
use igloo_util::TRACE_LEVEL;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct IglooTargetConfig
{
	pub name: String,
	links: Vec<String>,
	pub includes: Vec<String>,
	scripts: Vec<String>,
	series: String,
}

#[derive(Debug)]
pub struct IglooTarget
{
	root: std::path::PathBuf,
	pub makeopts: HashMap<String, config::Value>,
	pub config: IglooTargetConfig,
}

impl IglooTargetConfig
{
	fn _default() -> IglooTargetConfig
	{
		IglooTargetConfig
		{
			name: 	String::new(),
			links: Vec::new(),
			includes: Vec::new(),
			scripts: Vec::new(),
			series: String::new(),
		}
	}
}
impl IglooTarget
{
	fn _default() -> IglooTarget
	{
		IglooTarget
		{
			root: std::path::PathBuf::default(),
			makeopts: HashMap::new(),
			config: IglooTargetConfig::_default(),
		}
	}

	/// takes the targets name and looks up the path
	/// deserializes the targets manifest file and creates the target
	pub fn target_from_name(prj: &IglooProject, name: String) -> Result<IglooTarget, IglooStatus>
	{
		let target_path = prj
			.igloo
			.env
			.esfd
			.clone()
			.join(&prj.igloo.master_target_manifest.targets[&name]);

		// We have to read in the file first so we can replace all variables with values
		let targ_templ = match std::fs::read_to_string(&target_path)
		{
			Ok(v) => v,
			Err(e) =>
			{
				igloo_debug!(ERROR,
						  IS_BAD,
						  "Failed to read {} | Error: {:?}",
						  &target_path.to_str().unwrap(),
							 e);
				return Err(IS_BAD)
			}
		};

		// replace all variables
		// this will be expanded on later and more variables will be added
		let final_targ_str = targ_templ.replace("${TARGET}", &name);

		// create config from our string
		let mut target_config: config::Config = config::Config::default();
		target_config.merge(
			config::File::from_str(
				&final_targ_str, config::FileFormat::Toml)).unwrap();

		// get [esf] (which is technically a table...) from our config
		let target_table: config::Value = target_config.get("esf").unwrap();

		// turn it into an IglooTargetConfig
		let ret_target_config = target_table.try_into::<IglooTargetConfig>().unwrap();

		// Finally, create our IglooTarget
		let mut ret_target = IglooTarget
		{
			root: prj.root.clone()
				.join("igloo")
				.join("targets")
				.join(&name),
			config: ret_target_config,
			makeopts: HashMap::new(),
		};
		ret_target.collect_makefile(prj);
		igloo_debug!(INFO,
					 IS_NONE,
					 "Found Igloo target and deserialized it: {:?}",
					 ret_target);

		Ok(ret_target)
	}

    // Verifies target is valid and exists within the project
    // This means the target must have a folder in the targets folder
    // and a <target_name>.toml
    pub fn target_from_existing(prj: &IglooProject, name: String) -> Result<IglooTarget, IglooStatus>
    {

    }

	/// Creates the target's configuration file from itself
	/// the target must be valid at this point or else the file will be junk
	pub fn generate(&self, project: &IglooProject) -> IglooStatus
	{
		let mut ret = IS_GOOD;

		if !self.root.exists()
		{
			// Make self target dir
			match std::fs::create_dir_all(&self.root)
			{
				Ok(_v) => (),
				Err(e) =>
				{
					ret = IS_FAILED_TO_CREATE_DIR;
					igloo_debug!(ERROR, ret, "Failed to create {} -- {}", self.root.to_str().unwrap(), e);
					return ret
				}
			}

		}

		if !self.root.join("scripts").exists()
		{
			// make scripts dir
			match std::fs::create_dir(&self.root.join("scripts"))
			{
				Ok(_v) => (),
				Err(e) =>
				{
					ret = IS_FAILED_TO_CREATE_DIR;
					igloo_debug!(ERROR, ret, "Failed to create {} -- {}", self.root.join("scripts").to_str().unwrap(), e);
					return ret
				}
			}
		}

		// Create symlinks for esf scripts
		ret = self.gather_esf_gdb_scripts(project);
		if ret != IS_GOOD
		{
			return ret
		}

		// Create symlinks required by the target
		// these links need to be formatted like so
		// arch/...guts.../link_name
		// igloo will use the guts to create the directories needed to make the symlinks
		// also need to handle these unwraps
		for link in &self.config.links
		{
			// name of the link is the final name in the path, i.e. arch/arm/"common"
			let link_name = String::from(&link[link.rfind('/').unwrap() + 1..link.len()]);
			let guts = String::from(&link[link.find('/').unwrap() + 1..link.rfind('/').unwrap()]);
			let guts_path = project.root.join("esf").join(std::path::PathBuf::from(&guts));
			// paths for the symlink creation
			let from_path = project.igloo.env.esfd.join(&link);
			let to_path = guts_path.join(&link_name);
			match std::fs::create_dir_all(&guts_path)
			{
				Ok(_v) =>
				{
					match std::os::unix::fs::symlink(&from_path, &to_path)
					{
						Ok(__v) => (),
						Err(e) =>
						{
							ret = IS_FAILED_TO_CREATE_SYMLINK;
							igloo_debug!(ERROR,
										 ret,
										 "Failed to create symlink from {} to {} -- {}",
										 &from_path.to_str().unwrap(),
										 &to_path.to_str().unwrap(),
										 e);
						}
					}
				}
				Err(e) =>
				{
					ret = IS_FAILED_TO_CREATE_DIR;
					igloo_debug!(ERROR,
								 ret,
								 "Failed to create dir {} -- {}",
								 &guts_path.to_str().unwrap(),
								 e);
					return ret;
				}
			}
		}

		// Write out our targets config to its config file located at its root
		// ex. project/igloo/targets/<targetname>/<targetname>.toml
		ret = self.generate_config(project);
		if ret != IS_GOOD
		{
			igloo_debug!(ERROR, ret, "Failed to create target config...");
			return ret;
		}

		ret = self.generate_makefile(project);
		if ret != IS_GOOD
		{
			igloo_debug!(ERROR, ret, "Failed to generate makefile...");
		}
		ret
	}

	pub fn generate_makefile(&self, project: &IglooProject) -> IglooStatus
	{

		let mut ret = IS_GOOD;
		let target_root = self.root.clone();
		// If the Makefile already exists, trash it
		if target_root.join("Makefile").exists()
		{
			std::fs::remove_file(target_root.join("Makefile")).unwrap();
		}

		// Make our Makefile, set it to append mode
		std::fs::File::create(target_root.join("Makefile")).unwrap();
		let mut app_file = OpenOptions::new()
			.write(true)
			.append(true)
			.open(target_root.join("Makefile"))
			.unwrap();
		//
		writeln!(app_file, "## ePenguin Generated Makefile").unwrap();
		writeln!(app_file, "PROJECT_NAME={}", project.config.profile.name).unwrap();
		writeln!(app_file, "TARGET_NAME={}", self.config.name).unwrap();

		loop
		{
			write!(app_file, "\n## Toolchain Variables").unwrap();
			ret = self.makefile_write_var(
				"TOOLCHAIN",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}
			ret = self.makefile_write_var(
				"CC",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}
			ret = self.makefile_write_var(
				"CXX",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"OBJCOPY",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"OBJDUMP",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"GDB",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"SIZE",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"AS",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			writeln!(app_file, "").unwrap();

			// Now we write some mcu specifics
			write!(app_file, "\n## MCU Specific Variables").unwrap();
			ret = self.makefile_write_var(
				"MCPU",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"MCU",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"LD_PATH",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"LD_SCRIPT",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			writeln!(app_file, "").unwrap();

			// Write out our compiler flags
			write!(app_file, "\n## Compiler Flags").unwrap();
			ret = self.makefile_write_var(
				"CFLAGS",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			writeln!(app_file, "").unwrap();
			ret = self.makefile_write_var(
				"ELF_FLAGS",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			writeln!(app_file, "").unwrap();
			ret = self.makefile_write_var(
				"HEX_FLAGS",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"EEP_FLAGS",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			// Write mkdir compatibility
			ret = self.makefile_write_compatibility_mkdir(&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_var(
				"SUB_DIRS",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}


			writeln!(app_file, "").unwrap();
			ret = self.makefile_write_var(
				"OBJS",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			writeln!(app_file, "").unwrap();
			ret = self.makefile_write_var(
				"OBJS_AS_ARGS",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			// Write our DEPS and DEPS_AS_ARGS vars
			writeln!(app_file, "\n").unwrap();
			writeln!(app_file, "DEPS=$(OBJS:%.o=%.d)").unwrap();
			writeln!(app_file, "DEPS_AS_ARGS=$(OBJS_AS_ARGS:%.o=%.d)").unwrap();
			
			writeln!(app_file, "").unwrap();
			ret = self.makefile_write_var(
				"DIR_INCLUDES",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			writeln!(app_file, "\n\nvpath %.c ../../../").unwrap();
			writeln!(app_file, "vpath %.s ../../../").unwrap();
			writeln!(app_file, "vpath %.S ../../../\n").unwrap();
			writeln!(app_file, ".PHONY: debug push clean\n").unwrap();


			// Write core rules
			// These rules+vars must exist, NOT OPTIONAL
			ret = self.makefile_core_write_rules(&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			ret = self.makefile_write_compiler_targets(&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			// Write specific rules (clean, push, debug)
			ret = self.makefile_write_rule_clean(&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			writeln!(app_file, "").unwrap();
			ret = self.makefile_write_rule_push(&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			writeln!(app_file, "").unwrap();
			ret = self.makefile_write_rule_debug(&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

			writeln!(app_file, "\n\nQUOTE:=\"").unwrap();
		break;}

		if ret != IS_GOOD
		{
			igloo_debug!(ERROR,
						 ret,
						 "Failed to write some var to the makefile for target {}",
						 self.config.name);
		}
		ret
	}

	fn makefile_write_var(&self,
						  name: &str,
						  makefile: &mut std::fs::File) -> IglooStatus
	{
		let mut ret: IglooStatus = IS_GOOD;
		igloo_debug!(INFO, IS_NONE, "Writing var {} to makefile at {}",
					 name,
					 self.root.join("Makefile").to_str().unwrap());
		match self.makeopts.get(name)
		{
			None =>
			{
				ret = IS_FAILED_TO_EXTRACT_MF_VAR;
				igloo_debug!(WARNING,
							 ret,
							 "Failed to write make var {} -- wasn't found",
							 name);
			}
			Some(v) =>
			{
				write!(makefile, "\n{}", String::from(name) + "=").unwrap();
				match v.clone().into_array()
				{
					Ok(arr) =>
					{
						// is an array
						for element in arr
						{
							writeln!(makefile, "\\").unwrap();
							write!(makefile, "{}", element).unwrap();
						}
					}
					Err(_e) =>
					{
						// not an array
						write!(makefile, "{}", v.clone().into_str().unwrap()).unwrap();
					}
				}
			},
		}
		ret
	}

	// This is just so mkdir is compatible with multiple systems
	fn makefile_write_compatibility_mkdir(&self, makefile: &mut std::fs::File) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		let sysroot: String = String::from("\n
ifdef SystemRoot
	SHELL = cmd.exe
	MK_DIR = mkdir
else
	ifeq ($(shell uname), Linux)
		MK_DIR = mkdir -p
	endif

	ifeq ($(shell uname | cut -d _ -f 1), CYGWIN)
		MK_DIR = mkdir -p
	endif

	ifeq ($(shell uname | cut -d _ -f 1), MINGW32)
	MK_DIR = mkdir -p
	endif

	ifeq ($(shell uname | cut -d _ -f 1), MINGW64)
	MK_DIR = mkdir -p
	endif

	ifeq ($(shell uname | cut -d _ -f 1), DARWIN)
	MK_DIR = mkdir -p
	endif
endif");

		// need to handle these unwraps eventually
		writeln!(makefile, "{}", sysroot).unwrap();

		ret
	}


	// Write all makefile rules
	// Eventually this will be moved into a write_makefile_rule(x) where x is the rule to be written
	// I don't know how I want to structure that yet so for now we're just writing them all at once
	// as hardcoded rules
	fn makefile_core_write_rules(&self, makefile: &mut std::fs::File) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		writeln!(makefile, "").unwrap();

		let mut makevar: &str;
		loop
		{
			// Writing all rule
			makevar = "ALL_PREREQS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "all:").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\\").unwrap();
						write!(makefile, "{}", cflag).unwrap();
					}
				},
			}
			makevar = "ALL_CMDS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "\n").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\t{}", cflag).unwrap();
					}
				},
			}

			// Writing $(PROJECT_NAME).elf rule
			write!(makefile, "\n\n").unwrap();
			makevar = "ELF_TARGET_PREREQS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "$(PROJECT_NAME).elf:").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\\").unwrap();
						write!(makefile, "{}", cflag).unwrap();
					}
				},
			}

			makevar = "ELF_TARGET_CMDS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "\n").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\t{}", cflag).unwrap();
					}
				},
			}

			write!(makefile, "\n\n").unwrap();
			makevar = "BIN_TARGET_PREREQS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "$(PROJECT_NAME).bin:").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\\").unwrap();
						write!(makefile, "{}", cflag).unwrap();
					}
				},
			}

			makevar = "BIN_TARGET_CMDS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "\n").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\t{}", cflag).unwrap();
					}
				},
			}

			write!(makefile, "\n\n").unwrap();
			makevar = "HEX_TARGET_PREREQS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "$(PROJECT_NAME).hex:").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\\").unwrap();
						write!(makefile, "{}", cflag).unwrap();
					}
				},
			}

			makevar = "HEX_TARGET_CMDS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "\n").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\t{}", cflag).unwrap();
					}
				},
			}

			write!(makefile, "\n\n").unwrap();
			makevar = "EEP_TARGET_PREREQS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "$(PROJECT_NAME).eep:").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\\").unwrap();
						write!(makefile, "{}", cflag).unwrap();
					}
				},
			}

			makevar = "EEP_TARGET_CMDS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "\n").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\t{}", cflag).unwrap();
					}
				},
			}

			write!(makefile, "\n\n").unwrap();
			makevar = "LSS_TARGET_PREREQS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "$(PROJECT_NAME).lss:").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\\").unwrap();
						write!(makefile, "{}", cflag).unwrap();
					}
				},
			}

			makevar = "LSS_TARGET_CMDS";
			match self.makeopts.get(makevar)
			{
				None =>
				{
					ret = IS_FAILED_TO_EXTRACT_MF_VAR;
					break;
				}
				Some(v) =>
				{
					write!(makefile, "\n").unwrap();
					for cflag in v.clone().into_array().unwrap()
					{
						writeln!(makefile, "\t{}", cflag).unwrap();
					}
				},
			}

		break;}

		if ret != IS_GOOD
		{
			igloo_debug!(ERROR, ret, "Failed to extract make var {}", makevar);
		}
		ret
	}

	fn makefile_write_compiler_targets(&self, makefile: &mut std::fs::File) -> IglooStatus
	{
		let mut ret = IS_GOOD;

		// Compiler Targets
		writeln!(makefile, "\n
# Compiler targets
%.o: %.c
	@echo Building file: $<
	@echo ARM/GNU C Compiler
	$(QUOTE)$(CC)$(QUOTE) $(CFLAGS) -o $(QUOTE)$@$(QUOTE) $(QUOTE)$<$(QUOTE)
	@echo Finished building: $<").unwrap();
		writeln!(makefile, "
%.o: %.s
	@echo Building file: $<
	@echo ARM/GNU Assembler
	$(QUOTE)$(AS)$(QUOTE) $(CFLAGS) -o $(QUOTE)$@$(QUOTE) $(QUOTE)$<$(QUOTE)
	@echo Finished building: $<").unwrap();
		writeln!(makefile, "
%.o: %.S
	@echo Building file: $<
	@echo ARM/GNU Preprocessing Assembler
	$(QUOTE)$(CC)$(QUOTE) $(CFLAGS) -o $(QUOTE)$@$(QUOTE) $(QUOTE)$<$(QUOTE)
	@echo Finished building: $<").unwrap();


		writeln!(makefile, "\n").unwrap();
		writeln!(makefile, "$(SUB_DIRS):\n\t$(MK_DIR) $(QUOTE)$@$(QUOTE)").unwrap();
		writeln!(makefile, "
ifneq ($(MAKECMDGOALS),clean)
ifneq ($(strip $(DEPS)),)
-include $(DEPS)
endif
endif\n").unwrap();
		ret
	}

	fn makefile_write_rule_clean(&self, makefile: &mut std::fs::File) -> IglooStatus
	{
		let mut ret = IS_GOOD;

		let mut makevar: &str = "CLEAN_PREREQS";

		match self.makeopts.get(makevar)
		{
			None =>
			{
				ret = IS_FAILED_TO_EXTRACT_MF_VAR;
				igloo_debug!(ERROR, ret, "Failed to extract makevar {}", makevar);
				return ret;
			}
			Some(v) =>
			{
				write!(makefile, "clean:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(makefile, "\\").unwrap();
						write!(makefile, "{}", cflag).unwrap()
					}
				}
			},
		}

		makevar = "CLEAN_CMDS";
		match self.makeopts.get(makevar)
		{
			None =>
			{
				ret = IS_FAILED_TO_EXTRACT_MF_VAR;
				igloo_debug!(ERROR, ret, "Failed to extract makevar {}", makevar);
				return ret;
			}
			Some(v) =>
			{
				write!(makefile, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(makefile, "\t{}", cflag).unwrap()
					}
				}
			},
		}
		ret
	}

	fn makefile_write_rule_push(&self, makefile: &mut std::fs::File) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		let mut makevar: &str = "PUSH_PREREQS";

		match self.makeopts.get(makevar)
		{
			None =>
			{
				ret = IS_FAILED_TO_EXTRACT_MF_VAR;
				igloo_debug!(ERROR, ret, "Failed to extract makevar {}", makevar);
				return ret;
			}
			Some(v) =>
			{
				write!(makefile, "push:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(makefile, "\\").unwrap();
						write!(makefile, "{}", cflag).unwrap()
					}
				}
			},
		}

		makevar = "PUSH_CMDS";
		match self.makeopts.get(makevar)
		{
			None =>
			{
				ret = IS_FAILED_TO_EXTRACT_MF_VAR;
				igloo_debug!(ERROR, ret, "Failed to extract makevar {}", makevar);
				return ret;
			}
			Some(v) =>
			{
				write!(makefile, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(makefile, "\t{}", cflag).unwrap()
					}
				}
			},
		}
		ret
	}

	fn makefile_write_rule_debug(&self, makefile: &mut std::fs::File) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		let mut makevar: &str = "DEBUG_PREREQS";
		match self.makeopts.get(makevar)
		{
			None =>
			{
				ret = IS_FAILED_TO_EXTRACT_MF_VAR;
				igloo_debug!(ERROR, ret, "Failed to extract makevar {}", makevar);
				return ret;
			}
			Some(v) =>
			{
				write!(makefile, "debug:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(makefile, "\\").unwrap();
						write!(makefile, "{}", cflag).unwrap()
					}
				}
			},
		}

		makevar = "DEBUG_CMDS";
		match self.makeopts.get(makevar)
		{
			None =>
			{
				ret = IS_FAILED_TO_EXTRACT_MF_VAR;
				igloo_debug!(ERROR, ret, "Failed to extract makevar {}", makevar);
				return ret;
			}
			Some(v) =>
			{
				write!(makefile, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(makefile, "\t{}", cflag).unwrap()
					}
				}
			},
		}
		ret
	}


	fn gather_esf_gdb_scripts(&self, project: &IglooProject) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		let target_scripts_dir = self.root.join("scripts");
		for _script in &self.config.scripts
		{
			// Making so many vars here for
			// 1.) readability and
			// 2.) to prevent "temporary value dropped while borrowed" ??
			// I should revisit this to make sure there are no shenanigans
			// Only commenting this because I don't get it. They look exactly the same. ??
			// original:
			/*
				- let absolute_script_path = std::path::PathBuf::from(&_script);
				- let file_name = absolute_script_path.file_name().unwrap();
				+ let file_name = std::path::PathBuf::from(&script).file_name().unwrap();
			*/
			let absolute_script_path = std::path::PathBuf::from(&_script);
			let file_name = absolute_script_path.file_name().unwrap();
			let from_path = std::path::PathBuf::from(project.igloo.env.esfd.join(&_script));
			let to_path = std::path::PathBuf::from(target_scripts_dir.join(&file_name));
			match std::os::unix::fs::symlink(&from_path, &to_path)
			{
				Ok(_v) => (),
				Err(e) =>
				{
					ret = IS_FAILED_TO_CREATE_SYMLINK;
					igloo_debug!(ERROR,
								 ret,
								 "Failed to create symlink from {} to {} -- Error: {}",
								 &from_path.to_str().unwrap(),
								 &to_path.to_str().unwrap(),
								 e);
					return ret
				}
			}
		}
		ret
	}

	pub fn collect_makefile(&mut self, project: &IglooProject) -> IglooStatus
	{
		let mut ret: IglooStatus = IS_GOOD;

		// NOTE: Add
		// 
		// DEPS = "$(OBJS:%.o=%.d)"
		// DEPS_AS_ARGS = "$(OBJS_AS_ARGS:%.o=%.d)"
		// Convert OBJS to OBJS_AS_ARGS
		// add esf/...guts.../mcu/src to SUB_DIRS
		/*let (dummy, arch, family, mcu_name) = sscanf::scanf!(
		self.config.series, "{}.{}.{}.{}", String, String, String, String).unwrap(); */

		// collect makefile data
		let mut make_table_head = &self.config.series[0..self.config.series.len()];
		let mut b_quit: bool = false;
		loop
		{
			let mut _active_table = project.igloo.master_make_manifest.get_table(&make_table_head).unwrap();
			for(name, val) in _active_table
			{
				match val.clone().into_table()
				{
					// I have no idea why I did this in this way. Need to revisit...
					Err(_e) =>
					{
						if !self.makeopts.contains_key(&name)
						{
							self.makeopts.insert(name, val);
						}
						else
						{
							let mut newval = val.clone().into_array().unwrap();
							let mut newvec = self.makeopts.get_key_value(&name).unwrap().1.clone().into_array().unwrap();
							newvec.append(&mut newval);
							self.makeopts.insert(name, config::Value::from(newvec));
						}
					}
					Ok(_v) => (),
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

		if ret != IS_GOOD
		{
			igloo_debug!(ERROR, ret);
			return ret;
		}

		// Generate the remaining variables from the makefile data
		// Convert OBJS to OBJS_AS_ARGS
		let mut objs_as_args: Vec<String> = Vec::new();
		let mut sub_dirs: Vec<String> = Vec::new();
		let objs = self.makeopts.get("OBJS").unwrap();
		for obj in objs.clone().into_array().unwrap()
		{
			let obj_as_arg_string: String = format!(
				"$(QUOTE){}$(QUOTE)",
				obj.clone().into_str().unwrap());
			let mut sub_dir_as_string = String::from(&obj.into_str().unwrap());
			sub_dir_as_string = String::from(
				&sub_dir_as_string[0..sub_dir_as_string.rfind('/').unwrap()]);
			if !objs_as_args.contains(&obj_as_arg_string)
			{
				objs_as_args.push(obj_as_arg_string);
			}

			if !sub_dirs.contains(&sub_dir_as_string)
			{
				sub_dirs.push(sub_dir_as_string);
			}
		}
		self.makeopts.insert("OBJS_AS_ARGS".to_owned(), config::Value::from(objs_as_args));
		self.makeopts.insert("SUB_DIRS".to_owned(), config::Value::from(sub_dirs));
		// generate SUB_DIRS
		ret
	}

	// this needs to be changed to convert the esf table contents into an actual table
	// named esf
	// right now im just writing [esf]\n<table_contents>
	// the drawback of this is user configurations will be overwritten every time a target
	// config needs to be regenerated
	pub fn generate_config(&self, project: &IglooProject) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		let cfg_path = self.root.join(
			String::from(
				&self.config.name) + ".toml");


		let esf_table_contents = toml::to_string(&self.config).unwrap();
		let mut target_cfg_file = std::fs::File::create(&cfg_path).unwrap();
		target_cfg_file.write("[esf]\n".as_bytes()).unwrap();
		target_cfg_file.write_all(esf_table_contents.as_bytes()).unwrap();
		target_cfg_file.write("\n[user]\n".as_bytes()).unwrap();
		ret
	}
}
