#![allow(warnings)]
extern crate clap;
extern crate config;
mod igloo;
mod igloo_action;

use clap::{crate_version, crate_description, crate_authors, App, Arg, AppSettings, ArgMatches};
use config::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::os::unix::fs;



fn main()
{
	let ig = igloo::Igloo::new();
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
