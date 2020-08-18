extern crate clap;
extern crate config;
mod igloo;

use clap::{crate_version, crate_description, crate_authors, App, Arg};
use config::*;
use std::collections::HashMap;

fn main()
{
	let ig = igloo::Igloo::New();
	match ig.start()
	{
		Ok(d) => println!("{:?}", d),
		Err(_) => panic!("FUCK"),
	}

	
}
