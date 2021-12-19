use clap::{App, Arg, ArgMatches};

use crate::Igloo;
use crate::IglooType;
use crate::IglooType::*;
use crate::IglooStatus;
use crate::IglooStatus::*;

/// Information input via cli will be stored here for the lifetime of the process
pub struct IglooCliInfo
{
	pub raw: clap::ArgMatches,
	pub version_major: i8,
	pub version_minor: i8,
	pub version_patch: i8,
	pub description: String,
}

impl IglooCliInfo
{
	pub fn new() -> Self
	{
		Self
		{
			raw: igloo_run_cli(),
			version_major: env!("CARGO_PKG_VERSION_MAJOR")
				.to_string()
				.parse()
				.unwrap(),
			version_minor: env!("CARGO_PKG_VERSION_MINOR")
				.to_string()
				.parse()
				.unwrap(),
			version_patch: env!("CARGO_PKG_VERSION_PATCH")
				.to_string()
				.parse()
				.unwrap(),
			description: clap::crate_description!().to_string(),
		}
	}
}

/// runs the clap initializer to get command line arguments
fn igloo_run_cli() -> clap::ArgMatches
{
	let ret_app = App::new("igloo")
		.about(clap::crate_description!())
		.version(clap::crate_version!())
		.setting(clap::AppSettings::SubcommandRequiredElseHelp)
		.subcommand(App::new("new")
					.about("Creates a new igloo project")
					.arg(Arg::new("project_name")
						 .required(true)
						.about("The name of the project to be created"),)
					.arg(Arg::new("target")
						.required(true)
						.about("MCU Target")
						.short('t')
						.long("target")
						.takes_value(true)),)
		.subcommand(App::new("run")
					.about("Compiles if needed. Flashes MCU and runs \
							current project on default target.")
					.arg(Arg::new("build_type")
						 .required(false)
						 .about("Release or Debug build type\n\
								 Defaults to Debug")),)
		.subcommand(App::new("push")
					.about("Pushes/flashes target(s)")
					.arg(Arg::new("build_type")
						 .required(false)
						 .about("Release or Debug build type\n\
								 Defaults to Debug")),)
		.subcommand(App::new("pull")
					.about("Reads .hex or .bin from mcu and\
							stores it in specified path")
					.arg(Arg::new("location")
						 .required(false)
						 .about("Specifies the name of the file. \
								 Will be stored in project root as this name")))
		.subcommand(App::new("erase")
					.about("Erases flash from target mcu or target mcus"))
		.subcommand(App::new("target")
					.about("Target subcommands")
					.subcommand(App::new("add")
								.arg(Arg::new("target_name")
								.required(true)
								.about("name of the target to be added")))
					.subcommand(App::new("remove")
								.arg(Arg::new("target_name")
								.required(true)
								.about("name of the target to be removed"))))
		.subcommand(App::new("info")
					.about("Provides info about various parts of igloo")
					.subcommand(App::new("list")
								.arg(Arg::new("supported-mcus")
									 .required(false)
									 .about("List of supported MCUs for the current version"),)
								.arg(Arg::new("supported-boards")
									 .required(false)
									 .about("List of supported boards for the current version"),)))
		.get_matches();
	ret_app
}

/// Igloo CLI Helper functions
/// These functions take some raw cli input and give us some helpful values
/// Putting these here so I don't have to pollute other code with this
pub fn ich_new_get_project_name(igloo: &Igloo) -> String
{
	return String::from(igloo
						.cli_info
						.raw
						.subcommand()
						.unwrap().1
						.value_of("project_name")
						.unwrap())
}

pub fn ich_new_get_target_name(igloo: &Igloo) -> String
{
	return String::from(igloo
						.cli_info
						.raw
						.subcommand()
						.unwrap().1
						.value_of("target")
						.unwrap())
}
