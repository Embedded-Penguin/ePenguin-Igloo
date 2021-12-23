use clap::ArgMatches;

use crate::IglooType::{self, *};
use crate::IglooStatus::{self, *};
use crate::IglooDebugSeverity::{self, *};
use crate::Igloo;
use crate::igloo_project::IglooProject;
use crate::igloo_util::*;

pub fn igloo_subcommand(args: &ArgMatches) -> Result<IglooType, IglooStatus>
{
	let mut _res_type: IglooType = IT_NULL;
	match args.subcommand_name()
	{
		Some("new") =>
		{
			igloo_debug!(TRACE, IS_GOOD, "Igloo new was called!");
			_res_type = IT_NEW;
		}
		Some("run") =>
		{
			igloo_debug!(TRACE, IS_GOOD, "Igloo run was called!");
			_res_type = IT_RUN;
		}
		Some("build") =>
		{
			igloo_debug!(TRACE, IS_GOOD, "Igloo build was called!");
			_res_type = IT_BUILD;
		}
		Some("push") =>
		{
			igloo_debug!(TRACE, IS_GOOD, "Igloo flash was called!");
			_res_type = IT_PUSH;
		}
		Some("pull") =>
		{
			igloo_debug!(TRACE, IS_GOOD, "Igloo pull was called!");
			_res_type = IT_PULL;
		}
		Some("erase") =>
		{
			igloo_debug!(TRACE, IS_GOOD, "Igloo erase was called!");
			_res_type = IT_ERASE;
		}
		Some("info") =>
		{
			igloo_debug!(TRACE, IS_GOOD, "Igloo info was called!");
			_res_type = IT_INFO;
		}
		Some("target") =>
		{
			igloo_debug!(TRACE, IS_GOOD, "Igloo target was called");
			_res_type = IT_TARGET;
		}
		Some("debug") =>
		{
			igloo_debug!(TRACE, IS_GOOD, "Igloo debug was called");
			_res_type = IT_DEBUG;
		}
		None => unreachable!(),
		_ => unreachable!(),
	}

	if _res_type == IT_NULL
	{
		return Err(IS_UNKNOWN)
	}

	Ok(_res_type)
}

// this will eventually be implemented so that projects can be created without an initial target
// for now it's necessary
pub fn ia_new(igloo: &Igloo, project_name: String, initial_target: String) -> IglooStatus
{
	let mut ret: IglooStatus = IS_GOOD;

	// is igloo project
	if IglooProject::is_igloo_prj(&igloo.env.cwd)
	{
		println!("Calling igloo new from inside igloo project...");
		ret = IS_BAD;
		return ret
	}

	// check if project folder already exists
	if std::path::Path::new(
		&igloo.env.cwd.join(&project_name)).exists()
	{
		ret = IS_BAD;
		return ret
	}

	let mut prj = match IglooProject::from_new(igloo, project_name)
	{
		Ok(v) => v,
		Err(e) =>
		{
			println!("{:?}", e);
			panic!();
		}
	};

	prj.add_target_to_config(initial_target);

	// Now populate
	prj.generate();



	ret
}

/// Debugging function to make sure projects are being loaded correctly
pub fn ia_debug(igloo: &Igloo) -> IglooStatus
{
	let mut ret = IS_GOOD;

	ret
}
