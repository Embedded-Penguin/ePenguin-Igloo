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
	name: String,
	common: String,
	mcu: String,
	ld: String,
	cfg: String,
	includes: Vec<String>,
	scripts: Vec<String>,
	series: String,
}

#[derive(Debug)]
pub struct IglooTarget
{
	root: std::path::PathBuf,
	makefile: HashMap<String, config::Value>,
	config: IglooTargetConfig,
}

impl IglooTargetConfig
{
	fn _default() -> IglooTargetConfig
	{
		IglooTargetConfig
		{
			name: 	String::new(),
			common: String::new(),
			mcu: 	String::new(),
			ld: 	String::new(),
			cfg: 	String::new(),
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
			makefile: HashMap::new(),
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
			makefile: HashMap::new(),
		};
		ret_target.collect_makefile(prj);
		igloo_debug!(INFO,
					 IS_NONE,
					 "Found Igloo target and deserialized it: {:?}",
					 ret_target);

		Ok(ret_target)
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

		ret = self.generate_makefile(project);
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
		writeln!(app_file, "# ePenguin Generated Variables").unwrap();
		writeln!(app_file, "PROJECT_NAME={}", project.config.profile.name).unwrap();
		writeln!(app_file, "TARGET_NAME={}", self.config.name).unwrap();

		loop
		{
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
				"ELF_FLAGS",
				&mut app_file);
			if ret != IS_GOOD
			{
				break;
			}

		break;}
		writeln!(app_file, "\n\nQUOTE:=\"").unwrap();
		ret
	}
	/*	


		match project.igloo.master_make_manifest.get("OBJCOPY")
		{
			None =>
			{
				println!("OBJCOPY Not found");
			}
			Some(v) =>
			{
				write!(app_file, "OBJCOPY=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match project.igloo.master_make_manifest.get("OBJDUMP")
		{
			None =>
			{
				println!("OBJDUMP Not found");
			}
			Some(v) =>
			{
				write!(app_file, "OBJDUMP=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match project.igloo.master_make_manifest.get("GDB")
		{
			None =>
			{
				println!("GDB Not found");
			}
			Some(v) =>
			{
				write!(app_file, "GDB=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match project.igloo.master_make_manifest.get("SIZE")
		{
			None =>
			{
				println!("SIZE Not found");
			}
			Some(v) =>
			{
				write!(app_file, "SIZE=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match project.igloo.master_make_manifest.get("AS")
		{
			None =>
			{
				println!("AS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "AS=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		writeln!(app_file, "\n").unwrap();

		// MCU Specifics now
		match project.igloo.master_make_manifest.get("MCPU")
		{
			None =>
			{
				println!("MCPU Not found");
			}
			Some(v) =>
			{
				write!(app_file, "MCPU=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match project.igloo.master_make_manifest.get("MCU")
		{
			None =>
			{
				println!("MCU Not found");
			}
			Some(v) =>
			{
				write!(app_file, "MCU=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match project.igloo.master_make_manifest.get("LD_PATH")
		{
			None =>
			{
				println!("LD_PATH Not found");
			}
			Some(v) =>
			{
				write!(app_file, "LD_PATH=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match project.igloo.master_make_manifest.get("LD_SCRIPT")
		{
			None =>
			{
				println!("LD_SCRIPT Not found");
			}
			Some(v) =>
			{
				write!(app_file, "LD_SCRIPT=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}

		writeln!(app_file, "\n").unwrap();

		// CFLAGS
		match project.igloo.master_make_manifest.get("CFLAGS")
		{
			None =>
			{
				println!("CFLAGS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "CFLAGS=").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, " \\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}
		writeln!(app_file, "\n").unwrap();
		// ELF FLAGS
		match project.igloo.master_make_manifest.get("ELF_FLAGS")
		{
			None =>
			{
				println!("ELF_FLAGS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "ELF_FLAGS=").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, " \\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}
		writeln!(app_file, "\n").unwrap();
		// HEX FLAGS
		match project.igloo.master_make_manifest.get("HEX_FLAGS")
		{
			None =>
			{
				println!("HEX_FLAGS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "HEX_FLAGS=").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, " \\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		writeln!(app_file, "\n").unwrap();
		match project.igloo.master_make_manifest.get("EEP_FLAGS")
		{
			None =>
			{
				println!("EEP_FLAGS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "EEP_FLAGS=").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, " \\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		writeln!(app_file, "\n").unwrap();
		// Write SystemRoot config stuff for cross compatibility
		let sysroot: String = String::from("
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

		writeln!(app_file, "{}", sysroot).unwrap();
		match project.igloo.master_make_manifest.get("SUB_DIRS")
		{
			None =>
			{
				println!("SUB_DIRS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "SUB_DIRS+=").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, " \\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		writeln!(app_file, "\n").unwrap();
		match project.igloo.master_make_manifest.get("OBJS")
		{
			None =>
			{
				println!("OBJS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "OBJS+=").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, " \\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		writeln!(app_file, "\n").unwrap();
		match project.igloo.master_make_manifest.get("OBJS_AS_ARGS")
		{
			None =>
			{
				println!("OBJS_AS_ARGS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "OBJS_AS_ARGS+=").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, " \\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		writeln!(app_file, "\n").unwrap();
		match project.igloo.master_make_manifest.get("DIR_INCLUDES")
		{
			None =>
			{
				println!("DIR_INCLUDES Not found");
			}
			Some(v) =>
			{
				write!(app_file, "DIR_INCLUDES+=").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, " \\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		write!(app_file, "\n\n").unwrap();
		match project.igloo.master_make_manifest.get("DEPS")
		{
			None =>
			{
				println!("DEPS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "DEPS:=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}

		write!(app_file, "\n").unwrap();
		match project.igloo.master_make_manifest.get("DEPS_AS_ARGS")
		{
			None =>
			{
				println!("DEPS_AS_ARGS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "DEPS_AS_ARGS:=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}

		writeln!(app_file, "\nvpath %.c ../../../").unwrap();
		writeln!(app_file, "vpath %.s ../../../").unwrap();
		writeln!(app_file, "vpath %.S ../../../\n").unwrap();

		writeln!(app_file, ".PHONY: debug clean\n").unwrap();

		match project.igloo.master_make_manifest.get("ALL_PREREQS")
		{
			None =>
			{
				println!("ALL_PREREQS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "all:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}
		match project.igloo.master_make_manifest.get("ALL_CMDS")
		{
			None =>
			{
				println!("ALL_CMDS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\t{}", cflag).unwrap();
				}
			},
		}

		write!(app_file, "\n\n").unwrap();
		match project.igloo.master_make_manifest.get("ELF_TARGET_PREREQS")
		{
			None =>
			{
				println!("ELF_TARGET_PREREQS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "$(PROJECT_NAME).elf:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		match project.igloo.master_make_manifest.get("ELF_TARGET_CMDS")
		{
			None =>
			{
				println!("ELF_TARGET_CMDS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\t{}", cflag).unwrap();
				}
			},
		}

		write!(app_file, "\n\n").unwrap();
		match project.igloo.master_make_manifest.get("BIN_TARGET_PREREQS")
		{
			None =>
			{
				println!("BIN_TARGET_PREREQS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "$(PROJECT_NAME).bin:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		match project.igloo.master_make_manifest.get("BIN_TARGET_CMDS")
		{
			None =>
			{
				println!("BIN_TARGET_CMDS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\t{}", cflag).unwrap();
				}
			},
		}

		write!(app_file, "\n\n").unwrap();
		match project.igloo.master_make_manifest.get("HEX_TARGET_PREREQS")
		{
			None =>
			{
				println!("HEX_TARGET_PREREQS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "$(PROJECT_NAME).hex:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		match project.igloo.master_make_manifest.get("HEX_TARGET_CMDS")
		{
			None =>
			{
				println!("HEX_TARGET_CMDS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\t{}", cflag).unwrap();
				}
			},
		}
		write!(app_file, "\n\n").unwrap();
		match project.igloo.master_make_manifest.get("EEP_TARGET_PREREQS")
		{
			None =>
			{
				println!("EEP_TARGET_PREREQS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "$(PROJECT_NAME).eep:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		match project.igloo.master_make_manifest.get("EEP_TARGET_CMDS")
		{
			None =>
			{
				println!("EEP_TARGET_CMDS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\t{}", cflag).unwrap();
				}
			},
		}
		write!(app_file, "\n\n").unwrap();
		match project.igloo.master_make_manifest.get("LSS_TARGET_PREREQS")
		{
			None =>
			{
				println!("LSS_TARGET_PREREQS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "$(PROJECT_NAME).lss:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\\").unwrap();
					write!(app_file, "{}", cflag).unwrap();
				}
			},
		}

		match project.igloo.master_make_manifest.get("LSS_TARGET_CMDS")
		{
			None =>
			{
				println!("LSS_TARGET_CMDS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					writeln!(app_file, "\t{}", cflag).unwrap();
				}
			},
		}

		// Compiler Targets
		writeln!(app_file, "").unwrap();
		writeln!(app_file, "
# Compiler targets
%.o: %.c
	@echo Building file: $<
	@echo ARM/GNU C Compiler
	$(QUOTE)$(CC)$(QUOTE) $(CFLAGS) -o $(QUOTE)$@$(QUOTE) $(QUOTE)$<$(QUOTE)
	@echo Finished building: $<").unwrap();
		writeln!(app_file, "
%.o: %.s
	@echo Building file: $<
	@echo ARM/GNU Assembler
	$(QUOTE)$(AS)$(QUOTE) $(CFLAGS) -o $(QUOTE)$@$(QUOTE) $(QUOTE)$<$(QUOTE)
	@echo Finished building: $<").unwrap();
		writeln!(app_file, "
%.o: %.S
	@echo Building file: $<
	@echo ARM/GNU Preprocessing Assembler
	$(QUOTE)$(CC)$(QUOTE) $(CFLAGS) -o $(QUOTE)$@$(QUOTE) $(QUOTE)$<$(QUOTE)
	@echo Finished building: $<").unwrap();


		writeln!(app_file, "\n").unwrap();
		writeln!(app_file, "$(SUB_DIRS):\n\t$(MK_DIR) $(QUOTE)$@$(QUOTE)").unwrap();
		writeln!(app_file, "
ifneq ($(MAKECMDGOALS),clean)
ifneq ($(strip $(DEPS)),)
-include $(DEPS)
endif
endif\n").unwrap();

		match project.igloo.master_make_manifest.get("CLEAN_PREREQS")
		{
			None =>
			{
				println!("CLEAN_TARGET_PREREQS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "clean:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(app_file, "\\").unwrap();
						write!(app_file, "{}", cflag).unwrap()
					}
				}
			},
		}

		match project.igloo.master_make_manifest.get("CLEAN_CMDS")
		{
			None =>
			{
				println!("CLEAN_CMDS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(app_file, "\t{}", cflag).unwrap()
					}
				}
			},
		}

		writeln!(app_file, "\n").unwrap();
		match project.igloo.master_make_manifest.get("DEBUG_PREREQS")
		{
			None =>
			{
				println!("DEBUG_PREREQS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "debug:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(app_file, "\\").unwrap();
						write!(app_file, "{}", cflag).unwrap()
					}
				}
			},
		}

		match project.igloo.master_make_manifest.get("DEBUG_CMDS")
		{
			None =>
			{
				println!("DEBUG_CMDS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(app_file, "\t{}", cflag).unwrap()
					}
				}
			},
		}

		writeln!(app_file, "\n").unwrap();
		match project.igloo.master_make_manifest.get("PUSH_PREREQS")
		{
			None =>
			{
				println!("PUSH_PREREQS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "push:").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(app_file, "\\").unwrap();
						write!(app_file, "{}", cflag).unwrap()
					}
				}
			},
		}

		match project.igloo.master_make_manifest.get("PUSH_CMDS")
		{
			None =>
			{
				println!("PUSH_CMDS Not found");
			}
			Some(v) =>
			{
				write!(app_file, "\n").unwrap();
				for cflag in v.clone().into_array().unwrap()
				{
					if !cflag.to_string().is_empty()
					{
						writeln!(app_file, "\t{}", cflag).unwrap()
					}
				}
			},
		}

		*/

	fn makefile_write_var(&self,
						  name: &str,
						  makefile: &mut std::fs::File) -> IglooStatus
	{
		let mut ret: IglooStatus = IS_GOOD;
		println!("PRINTING MAKEFILE {:?}", self.makefile);
		match self.makefile.get(name)
		{
			None =>
			{
				ret = IS_FAILED_TO_EXTRACT_MF_VAR;
				igloo_debug!(WARNING, ret, "Failed to write make var {} -- wasn't found", name);
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
							writeln!(makefile, " \\").unwrap();
							write!(makefile, "{}", element).unwrap();
						}
					}
					Err(e) =>
					{
						// not an array
						write!(makefile, "{}", v.clone().into_str().unwrap());
					}
				}
			},
		}
		ret
	}

	// fn makefile_write_rule(targ: &str, makefile_manifest: &config::Config, makefile: &std::fs::File)
	// {

	// 	match project.igloo.master_make_manifest.get("PUSH_CMDS")
	// 	{
	// 		None =>
	// 		{
	// 			println!("PUSH_CMDS Not found");
	// 		}
	// 		Some(v) =>
	// 		{
	// 			write!(app_file, "\n").unwrap();
	// 			for cflag in v.clone().into_array().unwrap()
	// 			{
	// 				if !cflag.to_string().is_empty()
	// 				{
	// 					writeln!(app_file, "\t{}", cflag).unwrap()
	// 				}
	// 			}
	// 		},
	// 	}
	// }

	pub fn collect_makefile(&mut self, project: &IglooProject) -> IglooStatus
	{
		let mut ret: IglooStatus = IS_GOOD;

		let (dummy, arch, family, mcu_name) = sscanf::scanf!(
			self.config.series, "{}.{}.{}.{}", String, String, String, String).unwrap();
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
						if !self.makefile.contains_key(&name)
						{
							self.makefile.insert(name, val);
						}
						else
						{
							let mut newval = val.clone().into_array().unwrap();
							let mut newvec = self.makefile.get_key_value(&name).unwrap().1.clone().into_array().unwrap();
							newvec.append(&mut newval);
							self.makefile.insert(name, config::Value::from(newvec));
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
		ret
	}
}
