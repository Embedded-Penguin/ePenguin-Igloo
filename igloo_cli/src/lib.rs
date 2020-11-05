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


/// runs the clap initializer to get command line arguments
pub fn igloo_app() -> ArgMatches
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
		.subcommand(App::new("flash")
					.about("Flashes target mcu or multiple mcus")
					.arg(Arg::new("build_type")
						 .required(false)
						 .about("Release or Debug build type\n\
								 Defaults to Debug")),)
		.subcommand(App::new("erase")
					.about("Erases flash from target mcu or target mcus"))
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
		Some("flash") =>
		{
			println!("Igloo flash was called!");
			_res_type = Flash;
		}
		Some("erase") =>
		{
			println!("Igloo erase was called!");
			_res_type = Erase;
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
