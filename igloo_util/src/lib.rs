#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
/// * IT_NEW: Create a new igloo project
/// * IT_RUN: build the project if needed, then run the project, defaults to default target set in your project's profile
/// * IT_PUSH: build the project if needed, then upload your binary to your target
/// * IT_PULL: extracts binary from mcu (if possible) and saves it
/// * IT_HELP: gets help
/// * IT_BUILD: builds the project for all targets unless otherwise specified
/// * IT_ERASE: erases the flash for the specified target
/// * IT_INFO: Gets information about igloo and your project.
/// * IT_NULL: Default type... used for debugging and development. More on this later
/// * IT_DEBUG: this state is useful for debugging project failures. Only to be used in debug build of igloo. More on this later
pub enum IglooType
{
	IT_NEW = 0,
	IT_RUN,
	IT_PUSH,
	IT_PULL,
	IT_HELP,
	IT_BUILD,
	IT_ERASE,
	IT_INFO,
	IT_TARGET,
	IT_NULL,
	IT_DEBUG,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IglooDebugSeverity
{
	ERROR = 0,
	WARNING = 1,
	LOG = 2,
	TRACE = 3,
	INFO = 4,
}

pub static TRACE_LEVEL: IglooDebugSeverity = IglooDebugSeverity::TRACE;
#[derive(Debug)]
#[derive(PartialEq)]
pub enum IglooStatus
{
	IS_GOOD = 								0x00,
	IS_BAD,
	IS_UNKNOWN,
	IS_MISSING_ENV_VARIABLES,
	IS_FAILED_TO_LOAD_MTM,
	IS_NEW_CALLED_IN_EXISTING_PRJ,
	IS_NEW_DIR_ALREADY_EXISTS,
	IS_FAILED_TO_CREATE_PRJ_CFG_FILE,
	IS_FAILED_TO_CREATE_DIR,
	IS_FAILED_TO_EXTRACT_MF_VAR,
	IS_FAILED_TO_WRITE_MF_VAR,
	IS_FAILED_TO_CREATE_SYMLINK,
    IS_NOT_IGLOO_DIRECTORY,
	IS_NONE,
}


#[macro_export]
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
