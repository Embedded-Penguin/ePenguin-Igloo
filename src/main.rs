extern crate clap;
extern crate config;
mod igloo;

use clap::{crate_version, crate_description, crate_authors, App, Arg, AppSettings, ArgMatches};
use config::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[allow(unused_variables)]
#[allow(unused_imports)]
fn main()
{
	let ig = igloo::Igloo::New();
	let start_ret = match ig.start()
	{
		Ok(it) =>
		{
			match ig.run(it)
			{
				Ok(rt) => println!("{:?}", rt),
				Err(e) => println!("Run Error: {:?}", e),
			}
		}
		Err(e) => println!("Error: {:?}", e),
	};

}
