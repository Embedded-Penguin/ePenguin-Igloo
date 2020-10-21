#![allow(warnings)]
extern crate clap;
extern crate config;
mod igloo;
mod igloo_action;
mod igloo_prj;
mod igloo_manifest;

fn main()
{
	let ig = igloo::Igloo::new();
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
