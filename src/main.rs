extern crate clap;
extern crate config;
mod igloo;

use clap::{crate_version, crate_description, crate_authors, App, Arg, AppSettings, ArgMatches};
use config::*;
use std::collections::HashMap;
use std::path::Path;
fn main()
{
	let ig = igloo::Igloo::New();
	match ig.start()
	{
		Ok(it) => {
			match ig.run(it)
			{
				Ok(s) => println!("{}", s),
				Err(e) => println!("Run Error: {:?}", e),
			}
		}
		Err(e) => println!("Error: {:?}", e),
	}


}
