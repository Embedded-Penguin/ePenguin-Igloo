use clap::ArgMatches;

use crate::IglooType;
use crate::IglooType::*;
use crate::IglooStatus;
use crate::IglooStatus::*;

use crate::Igloo;
use crate::igloo_project::IglooProject;

pub fn igloo_subcommand(args: &ArgMatches) -> Result<IglooType, IglooStatus>
{
	let mut _res_type: IglooType = IT_NULL;
	match args.subcommand_name()
	{
		Some("new") =>
		{
			println!("Igloo new was called!");
			_res_type = IT_NEW;
		}
		Some("run") =>
		{
			println!("Igloo run was called!");
			_res_type = IT_RUN;
		}
		Some("build") =>
		{
			println!("Igloo build was called!");
			_res_type = IT_BUILD;
		}
		Some("push") =>
		{
			println!("Igloo flash was called!");
			_res_type = IT_PUSH;
		}
		Some("pull") =>
		{
			println!("Igloo pull was called!");
			_res_type = IT_PULL;
		}
		Some("erase") =>
		{
			println!("Igloo erase was called!");
			_res_type = IT_ERASE;
		}
		Some("info") =>
		{
			println!("Igloo info was called!");
			_res_type = IT_INFO;
		}
		Some("target") =>
		{
			println!("Igloo target was called");
			_res_type = IT_TARGET;
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

	let prj = match IglooProject::from_new(igloo, project_name)
	{
		Ok(v) => v,
		Err(e) =>
		{
			println!("{:?}", e);
			panic!();
		}
	};

	// Now populate
	// created_project.populate()



	ret
}
