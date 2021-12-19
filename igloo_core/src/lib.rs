#![allow(warnings)]
extern crate clap;
extern crate config;

use config::Config;
use std::path::PathBuf;
use std::env;
use directories::*;

pub mod igloo_target;
mod igloo_action;
mod igloo_project;
mod igloo_manifest;
mod igloo_cli;
mod igloo_env;

use igloo_cli::IglooCliInfo;
use igloo_env::IglooEnv;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum IglooType
{
	IT_NEW = 0,
	IT_RUN,
	IT_PUSH,
	IT_PULL,
	IT_HELP,
	IT_BUILD,
	IT_ERASE,
	IT_INFO,
	IT_TARGET,
	IT_NULL,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum IglooDebugSeverity
{
	CRITICAL = 0,
	WARNING = 1,
	INFO = 2,
	TRACE = 3,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum IglooStatus
{
	IS_GOOD = 0x00,
	IS_BAD = 0x01,
	IS_UNKNOWN = 0x02,
	IS_FAILED_TO_LOAD_MTM = 0x03,
}

use IglooStatus::*;
use IglooType::*;

pub struct Igloo
{
	cli_info: IglooCliInfo,
	// manifest containing all mcu information
	master_target_manifest: Config,
	env: IglooEnv,
}

impl Igloo
{
	pub fn new() -> Self
	{
		Igloo
		{
			master_target_manifest: Config::new(),
			cli_info: IglooCliInfo::new(),
			env: IglooEnv::get_env(),
		}
	}

	pub fn start(&mut self) -> Result<IglooType, IglooStatus>
	{
		let mut res: IglooType = IT_NULL;

		match igloo_manifest::get_master_target_manifest(self)
		{
			IS_GOOD => (),
			e =>
			{
				println!("{:?}", e);
				return Err(e)
			},
		}

		// Assign instance type (new, run, push, etc)
		igloo_action::igloo_subcommand(&self.cli_info.raw)
	}

	pub fn run(&self, inst_type: IglooType) -> IglooStatus
	{
		let mut res_err = IS_GOOD;
		let mut prj: IglooProject;

		res_err
	}
}

// Tests
#[cfg (test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
