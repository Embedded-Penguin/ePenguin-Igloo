#![allow(warnings)]
extern crate clap;
extern crate config;

/// Igloo Core

mod igloo_action;
mod igloo_project;
mod igloo_target;

use igloo_base::*;
use igloo_base::IglooInstType::*;
use igloo_base::IglooErrType::*;
use igloo_cli::*;
use igloo_manifest::*;
use igloo_project::IglooPrj;

use config::Config;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Igloo should contain a config and any important environment information.
/// Upon running igloo, an instanc is created, env information is stored, and then
/// things happen.
pub struct Igloo
{
	cli_conf: IglooCliConfig,
	master_make_man: Config,
	master_target_man: Config,
}

impl Igloo
{
	/// The new function creates an instance of igloo. The idea is I create an igloo,
	/// run start on it so I can collect environment information and process what command
	/// the user would like to run, and then I run that command or deal with errors.
	///
	/// This function handles all cli input and stores it. It is parsed for errors in the
	/// start function.
	pub fn new() -> Self
	{
		Igloo
		{
			master_make_man: Config::new(),
			master_target_man: Config::new(),
			cli_conf: IglooCliConfig::new(),
		}
	}

	/// The start function processes the command you want igloo to run
	///  It theoretically should never return an error. It should just exit.
	///  If an error was returned, It was my fault and not the users.
	///  It is really only here to help me debug.
	///
	///  The Inst Type is only returned for usage outside of this struct.
	pub fn start(&mut self) -> Result<IglooInstType, IglooErrType>
	{
		let mut res: IglooInstType = Null;
		// Load manifests first
		match IglooManifest::get_master_make_manifest(&mut self.master_make_man)
		{
			ErrNone => (),
			v =>
			{
				println!("{:?}", v);
				return Err(v)
			}
		}
		match IglooManifest::get_master_target_manifest(&mut self.master_target_man)
		{
			ErrNone => (),
			v =>
			{
				println!("{:?}", v);
				return Err(v)
			},
		}

		// Assign our instance type (new, run, flash, etc..)
		match igloo_subcommand(&self.cli_conf.raw)
		{
			Ok(v) => res = v,
			Err(e) => return Err(e),
		}

		if res == Null
		{
			return Err(ErrUnknown)
		}

		Ok(res)
	}

	/// The run function processes the request from the user.
	/// On success, it will give some string indicating the success of the operation.
	/// On failure, it will return some error type.
	pub fn run(&self, inst_type: IglooInstType) -> Result<String, IglooErrType>
	{
		let mut res_err = ErrNone;
		let mut prj: IglooPrj;
		loop { match inst_type
		{
			Null => res_err = ErrNone,
			New =>
			{
				let prj_name: &str = self
					.cli_conf
					.raw
					.subcommand()
					.unwrap().1
					.value_of("project_name")
					.unwrap();

				let target: &str = self
					.cli_conf
					.raw
					.subcommand()
					.unwrap().1
					.value_of("target")
					.unwrap();
				let res_err = igloo_action::new(
					self, prj_name, target);
				if res_err != ErrNone
				{
					return Err(res_err)
				}
			}
			Flash =>
			{

			}
			Run =>
			{

			}
			Info =>
			{
				// list current version
				println!("Igloo Version: {0}.{1}.{2}\n",
						 self.cli_conf.version_major,
						 self.cli_conf.version_minor,
						 self.cli_conf.version_patch);
				// list esf version
				// list supported mcus

				// if we're in a project, list the project info
				// list targets/boards
				println!("Info in run handler");
			}
			_ => println!("Unhandled case: {:?}", inst_type),
		} break; }
		if res_err == ErrNone
		{
			Ok(String::from("We won!"))
		}
		else
		{
			Err(res_err)
		}
	}
}
