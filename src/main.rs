#[macro_use]
extern crate clap;
use clap::{Arg, App, ArgGroup};
use std::str::FromStr;
use std::process;
use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};
use std::os::unix;
enum BuildTypes
{
	Release,
	Debug,
}

struct Igloo
{
	debug_mode: bool,
	release_mode: bool,
	fresh_mode: bool,
	version: String,
	name: String,
	author: String,
	description: String,
}

fn main()
{
	let mut _igloo: Igloo = Igloo
	{
		debug_mode: true,
		release_mode: false,
		fresh_mode: false,
		version: "v".to_owned() + crate_version!(),
		name: String::from("Igloo"),
		author: crate_authors!().to_owned(),
		description: crate_description!().to_owned(),
	};
	let matches = App::new(&*_igloo.name)
		.version(&*_igloo.version)
		.author(&*_igloo.author)
		.about(&*_igloo.description)
		.arg(Arg::with_name("VERSION")
			 .short('v')
			 .multiple(true)
			 .about("Sets the level of verbosity"))
		.subcommand(App::new("new")
					.about("Creates a new igloo project")
					.arg(
						Arg::new("NAME")
							.required(true)
							.about("The name of your new project")))
		.subcommand(App::new("run")
					.about("Builds project on target selected in config file")
					.arg(Arg::new("RELEASE")
						 .short('R')
						 .long("release")
						 .about("builds in release mode"))
					.arg(Arg::new("DEBUG")
						 .short('D')
						 .long("debug")
						 .about("builds in debug mode"))
					.arg(Arg::new("FRESH")
						 .short('F')
						 .long("fresh")
						 .about("Clean project, then builds project")))
		.subcommand(App::new("build")
					.about("Builds project on target selected in config file")
					.arg(Arg::new("RELEASE")
						 .short('R')
						 .long("release")
						 .about("builds in release mode"))
					.arg(Arg::new("DEBUG")
						 .short('D')
						 .long("debug")
						 .about("builds in debug mode"))
					.arg(Arg::new("FRESH")
						 .short('F')
						 .long("fresh")
						 .about("Clean project, then builds project")))
		.subcommand(App::new("clean")
					.about("Cleans project")
					.version("0.0")
					.arg(Arg::new("verbose")
						 .short('v')
						 .about("cleans project and prints extra info")))
		.get_matches();


	match matches.subcommand()
	{
		("new", Some(new_matches)) =>
		{
			igloo_new(&_igloo, new_matches.value_of("NAME").unwrap());
		}

		("run", Some(run_matches)) =>
		{

			if run_matches.is_present("FRESH")
			{
				println!("Building fresh project");
				_igloo.fresh_mode = true;
			}

			if run_matches.is_present("RELEASE") && run_matches.is_present("DEBUG")
			{
				println!("Can't run in debug and release mode...");
				process::exit(1);
			}
			else if run_matches.is_present("DEBUG")
			{
				_igloo.debug_mode = true;
			}
			else if run_matches.is_present("RELEASE")
			{
				_igloo.release_mode = true;
				_igloo.debug_mode = false;
			}

			igloo_run(&_igloo);


		}
		("build", Some(build_matches)) =>
		{
			if build_matches.is_present("FRESH")
			{
				println!("Building fresh project");
				_igloo.fresh_mode = true;
			}

			if build_matches.is_present("RELEASE") && build_matches.is_present("DEBUG")
			{
				println!("Can't run in debug and release mode...");
				process::exit(1);
			}
			else if build_matches.is_present("DEBUG")
			{
				_igloo.debug_mode = true;
			}
			else if build_matches.is_present("RELEASE")
			{
				_igloo.release_mode = true;
				_igloo.debug_mode = false;
			}
		}
		("", None) => println!("No subcommand was used"),
		_ => unreachable!(),
	}
}
fn igloo_new_with_dir(igloo_inst: &Igloo, prj_name: &str, prj_dir: &str)
{
	// WIP
}
fn igloo_new(igloo_inst: &Igloo, prj_name: &str)
{
	let path = Path::new(prj_name);
	if path.exists()
	{
		println!("Project already exists. Exiting...");
		process::exit(1);
	}

	println!("Making new project named {}", path.display());
	match fs::create_dir(prj_name)
	{
		Err(why) => println!("! {:?}", why.kind()),
		Ok(_) => {},
	}

	if cfg!(target_family = "unix")
	{
		println!("You are on unix!\n");
	}
	else
	{
		println!("only unix is currently supported!");
	}
}
fn igloo_run(igloo_inst: &Igloo)
{
	
}

fn igloo_build(igloo_inst: &Igloo)
{

}

fn igloo_clean(igloo_inst: &Igloo)
{

}

fn igloo_init(igloo_inst: &Igloo)
{

}

fn igloo_search(igloo_inst: &Igloo)
{

}

fn igloo_test(igloo_inst: &Igloo)
{

}

fn igloo_doc(igloo_inst: &Igloo)
{

}
