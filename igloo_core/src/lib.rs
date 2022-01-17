// #![allow(non_snake_case)]
// #![allow(non_camel_case_types)]

pub extern crate clap;
pub extern crate config;
pub extern crate toml;
pub extern crate serde;

use config::Config;
use std::path::PathBuf;
use std::env;
use directories::*;

pub mod igloo_target;
pub mod igloo_action;
pub mod igloo_project;
pub mod igloo_manifest;
pub mod igloo_cli;
pub mod igloo_env;

use igloo_cli::IglooCliInfo;
use igloo_env::IglooEnv;
use igloo_project::IglooProject;
use igloo_manifest::IglooTargetManifest;

#[macro_use] extern crate igloo_util;
use igloo_util::IglooDebugSeverity::*;
use igloo_util::IglooStatus::{self, *};
use igloo_util::IglooType::{self, *};
use igloo_util::TRACE_LEVEL;


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
		// get master target manifest
		self.master_target_manifest = IglooTargetManifest::get(self).unwrap();

		igloo_debug!(TRACE,
					 IS_NONE,
					 "Reading master makefile manifest from {}",
					 self.env
					 .esfd
					 .clone()
					 .join("manifest")
					 .join("make-manifest.toml")
					 .to_str().unwrap());

		// get master make manifest
		// this is a hacky way of doing it until
		// i can figure out a proper structure for deserializing
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
				res_err = igloo_action::ia_new(self,
								  igloo_cli::ich_new_get_project_name(self),
								  igloo_cli::ich_new_get_target_name(self));
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
                res_err = igloo_action::ia_build(self);
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

		if res_err != IS_GOOD
		{
			igloo_debug!(ERROR, res_err, "Igloo action failed...");
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
