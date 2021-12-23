use crate::IglooStatus::{self, *};
use crate::IglooType::{self, *};
use crate::IglooDebugSeverity::{self, *};
pub static TRACE_LEVEL: IglooDebugSeverity = TRACE;

macro_rules! igloo_debug
{
	($severity:expr, $status:expr) =>
	{
		if TRACE_LEVEL.clone() as u8 <= $severity as u8
		{
			println!("[{:?}]: Line {:?} in {:?} | {:?}",
					 $severity,
					 line!(),
					 file!(),
					 $status);
		}
	};

	($severity:expr, $status:expr, $($message:tt)*) =>
	{
		if TRACE_LEVEL.clone() as u8 <= $severity as u8
		{
			println!("[{:?}]: Line {:?} in {:?} | {:?} -- {}",
					 $severity,
					 line!(),
					 file!(),
					 $status,
					 format_args!($($message)*)
			);
		}
	};
}

pub(crate) use igloo_debug;
