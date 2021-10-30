extern crate clap;
extern crate config;

use igloo_base::*;
use igloo_base::IglooInstType::*;
use igloo_base::IglooErrType::*;

use clap::{App, Arg, ArgMatches};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// IglooCliConfig stores information about the igloo command being run.
/// It is all handled in active memory because we only care about this
/// information during the execution of that command.
pub struct IglooCliConfig
{
	pub raw: clap::ArgMatches,
	pub version_major: i8,
	pub version_minor: i8,
	pub version_patch: i8,
	pub description: String,
}

impl IglooCliConfig
{
	pub fn new() -> Self
	{
		Self
		{
			raw: igloo_app(),
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
fn igloo_app() -> clap::ArgMatches
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


pub fn igloo_subcommand(args: &ArgMatches) -> Result<IglooInstType, IglooErrType>
{
	let mut _res_type: IglooInstType = Null;
	match args.subcommand_name()
	{
		Some("new") =>
		{
			println!("Igloo new was called!");
			_res_type = New;
		}
		Some("run") =>
		{
			println!("Igloo run was called!");
			_res_type = Run;
		}
		Some("push") =>
		{
			println!("Igloo flash was called!");
			_res_type = Push;
		}
		Some("erase") =>
		{
			println!("Igloo erase was called!");
			_res_type = Erase;
		}
		Some("info") =>
		{
			println!("Igloo info was called!");
			_res_type = Info;
		}
		Some("target") =>
		{
			println!("Igloo target was called");
			_res_type = Target;
		}
		None => unreachable!(),
		_ => unreachable!(),
	}

	if _res_type == Null
	{
		return Err(ErrUnknown)
	}

	Ok(_res_type)
}
