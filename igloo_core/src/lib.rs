#![allow(warnings)]
extern crate clap;
extern crate config;
extern crate toml;
extern crate serde;

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
mod igloo_util;

use igloo_cli::IglooCliInfo;
use igloo_env::IglooEnv;
use igloo_project::IglooProject;
use igloo_manifest::IglooTargetManifest;
use igloo_util::*;
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
	IT_DEBUG,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IglooDebugSeverity
{
	ERROR = 0,
	WARNING = 1,
	LOG = 2,
	TRACE = 3,
	INFO = 4,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum IglooStatus
{
	IS_GOOD = 						0x00,
	IS_BAD = 						0x01,
	IS_UNKNOWN = 					0x02,
	IS_FAILED_TO_LOAD_MTM = 		0x03,
	IS_NEW_CALLED_IN_EXISTING_PRJ = 0x04,
	IS_NEW_DIR_ALREADY_EXISTS = 	0x05,
	IS_NONE = 						0xFF,
}

use IglooStatus::*;
use IglooType::*;
use IglooDebugSeverity::*;

pub struct Igloo
{
	pub master_target_manifest: IglooTargetManifest,
	pub master_make_manifest: Config,
	pub cli_info: IglooCliInfo,
	pub env: IglooEnv,
}

impl Igloo
{
	pub fn new() -> Igloo
	{
		Igloo
		{
			cli_info: IglooCliInfo::new(),
			env: IglooEnv::get_env(),
			master_target_manifest: IglooTargetManifest::default(),
			master_make_manifest: Config::new(),
		}
	}

	pub fn start(&mut self) -> Result<IglooType, IglooStatus>
	{
		let mut res: IglooType = IT_NULL;

		// get master target manifest
		self.master_target_manifest = IglooTargetManifest::get(self).unwrap();

		// get master make manifest
		// this is a hacky way of doing it until
		// i can figure out a proper structure for deserializing
		igloo_debug!(TRACE,
					 IS_NONE,
					 "Reading master makefile manifest from {}",
					 self.env
					 .esfd
					 .clone()
					 .join("manifest")
					 .join("make-manifest.toml")
					 .to_str().unwrap());

		self.master_make_manifest.merge(
			config::File::with_name(
				self.env
					.esfd
					.clone()
					.join("manifest")
					.join("make-manifest.toml")
					.to_str().unwrap())).unwrap();

		igloo_debug!(INFO, IS_NONE, "Read master makefile manifest: \n{:?}", self.master_make_manifest);
		// Assign instance type (new, run, push, etc)
		igloo_action::igloo_subcommand(&self.cli_info.raw)
	}

	pub fn run(&self, inst_type: IglooType) -> IglooStatus
	{
		let mut res_err = IS_GOOD;

		match inst_type
		{
			IT_NEW =>
			{
				return igloo_action::ia_new(self,
								  igloo_cli::ich_new_get_project_name(self),
								  igloo_cli::ich_new_get_target_name(self))
			}
			IT_RUN =>
			{

			}
			IT_PUSH =>
			{

			}
			IT_PULL =>
			{

			}
			IT_HELP =>
			{

			}
			IT_BUILD =>
			{

			}
			IT_ERASE =>
			{

			}
			IT_INFO =>
			{

			}
			IT_TARGET =>
			{

			}
			IT_NULL =>
			{

			}
			IT_DEBUG =>
			{

			}
		}
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
