#![allow(warnings)]
extern crate clap;
extern crate config;

use config::Config;
use clap::{Arg, App, AppSettings, ArgMatches};

use igloo_core::{Igloo, IglooErrType, IglooInstType};
use igloo_cli;
use igloo_make;
use igloo_manifest;
use igloo_agent;
fn main()
{
	let mut ig = Igloo::new();
	let _start_ret = match ig.start()
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
