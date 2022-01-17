#![allow(warnings)]
use clap::ArgMatches;

use igloo_util::IglooDebugSeverity::*;
use igloo_util::IglooStatus::{self, *};
use igloo_util::IglooType::{self, *};
use igloo_util::TRACE_LEVEL;
use crate::Igloo;
use crate::igloo_project::IglooProject;
use crate::igloo_project::Settings;


pub fn igloo_subcommand(args: &ArgMatches) -> Result<IglooType, IglooStatus>
{
	let mut _res_type: IglooType = IT_NULL;
	match args.subcommand_name()
	{
		Some("new") =>
		{
			igloo_debug!(TRACE, IS_NONE, "Igloo new was called!");
			_res_type = IT_NEW;
		}
		Some("run") =>
		{
			igloo_debug!(TRACE, IS_NONE, "Igloo run was called!");
			_res_type = IT_RUN;
		}
		Some("build") =>
		{
			igloo_debug!(TRACE, IS_NONE, "Igloo build was called!");
			_res_type = IT_BUILD;
		}
		Some("push") =>
		{
			igloo_debug!(TRACE, IS_NONE, "Igloo flash was called!");
			_res_type = IT_PUSH;
		}
		Some("pull") =>
		{
			igloo_debug!(TRACE, IS_NONE, "Igloo pull was called!");
			_res_type = IT_PULL;
		}
		Some("erase") =>
		{
			igloo_debug!(TRACE, IS_NONE, "Igloo erase was called!");
			_res_type = IT_ERASE;
		}
		Some("info") =>
		{
			igloo_debug!(TRACE, IS_NONE, "Igloo info was called!");
			_res_type = IT_INFO;
		}
		Some("target") =>
		{
			igloo_debug!(TRACE, IS_NONE, "Igloo target was called");
			_res_type = IT_TARGET;
		}
		Some("debug") =>
		{
			igloo_debug!(TRACE, IS_NONE, "Igloo debug was called");
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
		ret = IS_NEW_CALLED_IN_EXISTING_PRJ;
		igloo_debug!(WARNING, ret);
		return ret
	}

	// check if project folder already exists
	if std::path::Path::new(
		&igloo.env.cwd.join(&project_name)).exists()
	{
		ret = IS_NEW_DIR_ALREADY_EXISTS;
		igloo_debug!(WARNING, ret);
		return ret
	}

	igloo_debug!(TRACE, IS_NONE, "Creating new igloo project...");
	let mut prj = match IglooProject::from_new(igloo, project_name)
	{
		Ok(v) => v,
		Err(e) =>
		{
			ret = e;
			igloo_debug!(ERROR, ret);
			return ret
		}
	};

	// add initial target to config
	prj.config.add_target(initial_target);
	prj.targets = Settings::get_targets_from_config(&prj);

	// Now populate project files
	ret = prj.generate();
	if ret != IS_GOOD
	{
		igloo_debug!(ERROR, ret);
		return ret
	}

	// now do target folders
	ret = prj.generate_targets();
	if ret != IS_GOOD
	{
		igloo_debug!(WARNING, ret);
		return ret
	}

	ret
}

pub fn ia_build(igloo: &Igloo) -> IglooStatus
{
    let mut ret: IglooStatus = IS_GOOD;

    loop
    {
        if !IglooProject::is_igloo_prj(&igloo.env.cwd)
        {
            ret = IS_NOT_IGLOO_DIRECTORY;
            break;
        }

        let mut prj = match IglooProject::from_existing(&igloo)
        {
            Ok(v) => v,
            Err(e) => 
            {
                ret = e;
                break;
            },
        };






    break;}

    if ret != IS_GOOD
    {
        igloo_debug!(ERROR, ret);
    }
    ret
}

/// Debugging function to make sure projects are being loaded correctly
pub fn ia_debug(igloo: &Igloo) -> IglooStatus
{
	let mut ret: IglooStatus = IS_GOOD;

	ret
}
