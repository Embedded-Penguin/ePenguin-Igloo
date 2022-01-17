// #![allow(warnings)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use igloo_core::Igloo;

#[macro_use] extern crate igloo_util;
use igloo_util::IglooDebugSeverity::{self, *};
use igloo_util::IglooStatus::{self, *};
use igloo_util::IglooType::{self, *};
use igloo_util::TRACE_LEVEL;

fn main()
{
	let mut ig = Igloo::new();
	let _start_ret = match ig.start()
	{
		Ok(it) =>
		{
			match ig.run(it)
			{

				IS_GOOD => (),
				// this is actually so dumb and should not exist in any language
				ANYTHING_ELSE => igloo_debug!(ERROR, ANYTHING_ELSE),
			}
		}
		Err(e) => println!("Error: {:?}", e),
	};

}
