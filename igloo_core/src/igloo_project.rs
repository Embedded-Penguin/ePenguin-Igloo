use igloo_base::*;
use igloo_base::IglooErrType::*;
use igloo_manifest::*;

use crate::Igloo;
use crate::igloo_target::IglooTarget;

use std::vec::Vec;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;

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
	project_dir: std::path::PathBuf,
}


impl IglooPrj
{
	pub fn default() -> IglooPrj
	{
		IglooPrj
		{
			name: String::from(""),
			target_bank: Vec::default(),
			project_dir: std::path::PathBuf::default(),
		}
	}

	pub fn new(inst: &Igloo, nameIn: &str, targetIn: &str)
			   -> Result<IglooPrj, IglooErrType>
	{
		let mut res_err = ErrNone;
		if String::from(nameIn).is_empty()
		{
			res_err = InvalidProjectName;
			return Err(res_err)
		}

		if res_err != ErrNone
		{
			return Err(res_err)
		}
		match target_exists(&inst.master_make_man, &inst.master_target_man, targetIn)
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
					return Err(InvalidTarget)
				}
			}
			Err(e) =>
			{
				return Err(e)
			}
		}

		let mut _targ_make_table_name = inst.master_target_man.get_str(
			&("target.make.".to_owned() + &targetIn)).unwrap();
		let mut _targ_manifest_file_name = inst.master_target_man.get_str(
			&("target.manifest.".to_owned() + &targetIn)).unwrap();

		let mut temp: Vec<IglooTarget> = Vec::new();
		let targ = IglooTarget::from(
				inst,
				String::from(targetIn),
				&_targ_make_table_name,
				&_targ_manifest_file_name).unwrap();
		temp.push(targ);

		Ok(IglooPrj
		{
			name: String::from(nameIn),
			target_bank: temp,
			project_dir: IglooEnvInfo::get_env_info().cwd.join(nameIn),
		})
	}

	pub fn populate(&self) -> IglooErrType
	{

		// Create new directory
		let mut active_dir = IglooEnvInfo::get_env_info().cwd;
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
				.join(".igloo/target"))
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



		self.gen_targets();
		self.gen_igloo_header();
		self.gen_igloo_main();
		//self.debugManifests();
		ErrNone
	}

	pub fn debugManifests(&self)
	{
		for target in &self.target_bank
		{
			println!("Target manifest:");
			for (key, val) in &target.links
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
		let mut prj_root = self.project_dir.join(".igloo");
		for target in &self.target_bank
		{
			let mut target_root = prj_root.join(&("target/".to_owned() + &target.name));
			println!("{:?}", target_root.display());
			match std::fs::create_dir(&target_root)
			{
				Err(e) => println!("{:?}", e),
				_ => (),
			}

			// create project scripts dir
			let mut scripts_dir = target_root.join("scripts");
			match std::fs::create_dir(&scripts_dir)
			{
				Err(e) => println!("{:?}", e),
				_ => (),
			}

			// populate scripts dir
			//sym link gdb scripts
			let mut gdb_scripts_paths = std::fs::read_dir(
				&(String::from(
					IglooEnvInfo::get_env_info()
						.esfd.to_str()
						.unwrap()) + "/scripts"))
				.unwrap();

			let mut gdb_scripts: std::vec::Vec<std::path::PathBuf>
				= std::vec::Vec::new();
			for entry in gdb_scripts_paths
			{
				match &entry
				{
					Ok(v) => if !v.path().is_dir() {
						gdb_scripts.push(v.path()) },
					Err(e) => println!("{:?}", e),
				}
			}

			for file in gdb_scripts
			{
				println!("Project Scripts Dir: {:?}", scripts_dir);
				println!("ePenguin Scripts Dir: {:?}", file);
				std::os::unix::fs::symlink(
					&file, &scripts_dir.join(&file.file_name().unwrap()));
			}


			let mut prj_esf_dir = self.project_dir.join("ESF");
			for (sym_dir, loc_in_esf) in &target.links
			{
				let link_to_dir = IglooEnvInfo::get_env_info()
					.esfd
					.join(&loc_in_esf.clone().into_str().unwrap());
				std::os::unix::fs::symlink(link_to_dir, prj_esf_dir.join(sym_dir)).unwrap();
			}

			self.gen_openocd_config(&target);
			self.gen_makefile(&target);
		}
		ErrNone
	}

	pub fn gen_openocd_config(&self, target: &IglooTarget) -> IglooErrType
	{
		let mut ret: IglooErrType = ErrNone;
		let mut openocd_cfg = self.project_dir.join(".igloo/target");
		openocd_cfg.push(&target.name);
		openocd_cfg.push("scripts");
		openocd_cfg.push(&self.name);
		if openocd_cfg.with_extension("cfg").exists()
		{
			std::fs::remove_file(openocd_cfg.with_extension("cfg"));
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
		writeln!(ocfg_file, "source [find interface//{}.cfg]", target
				 .openocd.get("transport_cfg")
				 .unwrap()
				 .clone()
				 .into_str()
				 .unwrap()).unwrap();
		writeln!(ocfg_file, "transport select {}", target
				 .openocd.get("transport")
				 .unwrap()
				 .clone()
				 .into_str()
				 .unwrap()).unwrap();

		writeln!(ocfg_file, "\n# Chip Information").unwrap();
		writeln!(ocfg_file, "set CHIPNAME {}", target.name);
		writeln!(ocfg_file, "source [find target//{}.cfg]", target
				 .openocd.get("chip_name_cfg")
				 .unwrap()
				 .clone()
				 .into_str()
				 .unwrap()).unwrap();
		ret


	}

	/// Generates a makefile for a target
	pub fn gen_makefile(&self, target: &IglooTarget) -> IglooErrType
	{
		let mut prj_root = self.project_dir.join(".igloo/target");
		let mut target_root = prj_root.join(&target.name);
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
		writeln!(app_file, "PROJECT_NAME={}", self.name).unwrap();
		writeln!(app_file, "TARGET_NAME={}", target.name).unwrap();

		match target.make_manifest.get("TOOLCHAIN")
		{
			None =>
			{
				println!("TOOLCHAIN Not found");
			}
			Some(v) =>
			{
				write!(app_file, "TOOLCHAIN=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match target.make_manifest.get("CC")
		{
			None =>
			{
				println!("CC Not found");
			}
			Some(v) =>
			{
				write!(app_file, "CC=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match target.make_manifest.get("CXX")
		{
			None =>
			{
				println!("CXX Not found");
			}
			Some(v) =>
			{
				write!(app_file, "CXX=").unwrap();
				writeln!(app_file, "{}", v.to_string()).unwrap();
			},
		}
		match target.make_manifest.get("OBJCOPY")
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
		match target.make_manifest.get("OBJDUMP")
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
		match target.make_manifest.get("GDB")
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
		match target.make_manifest.get("SIZE")
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
		match target.make_manifest.get("AS")
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
		match target.make_manifest.get("MCPU")
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
		match target.make_manifest.get("MCU")
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
		match target.make_manifest.get("LD_PATH")
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
		match target.make_manifest.get("LD_SCRIPT")
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
		match target.make_manifest.get("CFLAGS")
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
		match target.make_manifest.get("ELF_FLAGS")
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
		match target.make_manifest.get("HEX_FLAGS")
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
		match target.make_manifest.get("EEP_FLAGS")
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
		match target.make_manifest.get("SUB_DIRS")
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
		match target.make_manifest.get("OBJS")
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
		match target.make_manifest.get("OBJS_AS_ARGS")
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
		match target.make_manifest.get("DIR_INCLUDES")
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
		match target.make_manifest.get("DEPS")
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
		match target.make_manifest.get("DEPS_AS_ARGS")
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

		match target.make_manifest.get("ALL_PREREQS")
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
		match target.make_manifest.get("ALL_CMDS")
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
		match target.make_manifest.get("ELF_TARGET_PREREQS")
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

		match target.make_manifest.get("ELF_TARGET_CMDS")
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
		match target.make_manifest.get("BIN_TARGET_PREREQS")
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

		match target.make_manifest.get("BIN_TARGET_CMDS")
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
		match target.make_manifest.get("HEX_TARGET_PREREQS")
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

		match target.make_manifest.get("HEX_TARGET_CMDS")
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
		match target.make_manifest.get("EEP_TARGET_PREREQS")
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

		match target.make_manifest.get("EEP_TARGET_CMDS")
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
		match target.make_manifest.get("LSS_TARGET_PREREQS")
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

		match target.make_manifest.get("LSS_TARGET_CMDS")
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

		match target.make_manifest.get("CLEAN_PREREQS")
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

		match target.make_manifest.get("CLEAN_CMDS")
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
		match target.make_manifest.get("DEBUG_PREREQS")
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

		match target.make_manifest.get("DEBUG_CMDS")
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
		match target.make_manifest.get("PUSH_PREREQS")
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

		match target.make_manifest.get("PUSH_CMDS")
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

		writeln!(app_file, "\n\nQUOTE:=\"").unwrap();
		ErrNone
	}

	pub fn gen_igloo_header(&self) -> IglooErrType
	{
		let mut inc_dir = self.project_dir.join("inc");
		if inc_dir.join("igloo.h").exists()
		{
			std::fs::remove_file(inc_dir.join("igloo.h")).unwrap();
		}

		File::create(inc_dir.join("igloo.h")).unwrap();
		let mut igloo_h_file = OpenOptions::new()
			.write(true)
			.append(true)
			.open(inc_dir.join("igloo.h"))
			.unwrap();

		for target in &self.target_bank
		{
			match &target.make_manifest.get("MCU")
			{
				None =>
				{
					println!("MCU definition not found in make manifest.\
							  \nCould not generate igloo.h");
					return ErrUnknown
				}
				Some(v) =>
				{
					writeln!(igloo_h_file, "#ifdef {}",
							 v.to_string()).unwrap();
				}
			}
			for inc_file in &target.includes
			{
				writeln!(igloo_h_file, "\t#include \"{}\"", inc_file).unwrap();
			}
			writeln!(igloo_h_file, "#endif").unwrap();
		}
		ErrNone
	}

	pub fn gen_igloo_main(&self) -> IglooErrType
	{
		let mut src_dir = self.project_dir.join("src");
		if src_dir.join("main.c").exists()
		{
			std::fs::remove_file(src_dir.join("main.c")).unwrap();
		}

		File::create(src_dir.join("main.c")).unwrap();
		let mut igloo_main_c_file = OpenOptions::new()
			.write(true)
			.append(true)
			.open(src_dir.join("main.c"))
			.unwrap();

		writeln!(igloo_main_c_file, "#include \"igloo.h\"").unwrap();
		writeln!(igloo_main_c_file, "\n\nint main()\n{{\n\treturn 0;\n}}").unwrap();
		ErrNone
	}

}
