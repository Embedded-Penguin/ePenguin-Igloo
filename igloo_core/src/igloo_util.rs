use crate::IglooStatus::{self, *};
use crate::IglooType::{self, *};
use crate::IglooDebugSeverity::{self, *};
pub static TRACE_LEVEL: IglooDebugSeverity = TRACE;

macro_rules! igloo_debug
{
	($severity:expr, $status:expr) =>
	{
		if cfg!(debug_assertions)
		{
			if $severity as u8 <= TRACE_LEVEL.clone() as u8
			{
				println!("[{:?}]: Line {:?} in {:?} | {:?}",
						 $severity,
						 line!(),
						 file!(),
						 $status);
			}
		}
	};

	($severity:expr, $status:expr, $($message:tt)*) =>
	{
		if cfg!(debug_assertions)
		{
			if $severity as u8 <= TRACE_LEVEL.clone() as u8
			{
				println!("[{:?}]: Line {:?} in {} | {} -- STATUS: {:?}",
						 $severity,
						 line!(),
						 file!(),
						 format_args!($($message)*),
						 $status
				);
			}
		}
	};
}

pub(crate) use igloo_debug;
