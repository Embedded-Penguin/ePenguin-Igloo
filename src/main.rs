#![allow(warnings)]

use igloo_core::Igloo;

fn main()
{
	let mut ig = Igloo::new();
	let _start_ret = match ig.start()
	{
		Ok(it) =>
		{
			match ig.run(it)
			{
				IS_GOOD => println!("success"),
				_ => println!("??"),
			}
		}
		Err(e) => println!("Error: {:?}", e),
	};

}
