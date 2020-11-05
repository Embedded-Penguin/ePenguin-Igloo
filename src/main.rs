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
				Ok(rt) => println!("{:?}", rt),
				Err(e) => println!("Run Error: {:?}", e),
			}
		}
		Err(e) => println!("Error: {:?}", e),
	};

}
