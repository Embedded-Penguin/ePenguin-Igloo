mod env_info;

use std::path::PathBuf;
use std::env;
use directories::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum IglooInstType
{
	Null = -1,
	New = 0,
	Run = 1,
	Flash = 2,
	Debug = 3,
	Clean = 4,
	Erase = 5,
	GenDoc = 6,
}

#[derive(Debug)]
#[derive(PartialEq)]
/// Igloo Error Type
pub enum IglooErrType
{
	ErrNone =		 				0,
	ErrUnknown =		 			1,
	ConfigNotFound = 				2,
	ConfigFound =		 			3,
	UnknownInstType =		 		4,
	NewCalledInsideProject =	 	5,
	FolderAlreadyExists =		 	6,
	InvalidProjectName =		 	7,
	InvalidEnvInfo =		 		8,
	InvalidTarget =		 		9,
	/// Failed to load ePenguin Make Manifest
	/// This means igloo couldn't find the master
	/// make manifest
	FailedToLoadMasterMakeManifest =	10,
	/// Failed to load ePenguin Target Manifest
	/// This means igloo couldn't find the master
	/// target manifest
	FailedToLoadMasterTargetManifest = 11,
	/// This means igloo couldn't find the scripts dir
	/// which should be located within a target directory
	/// It should be impossible for igloo to generate a target
	/// inside a project without also generating a scripts directory.
	/// The likely culprit of this failure is a user has messed with the folder
	FailedToFindTargetScriptsDir = 12,
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct IglooEnvInfo
{
	// Current Working Directory
	pub cwd: PathBuf,
	// Home Directory
	pub hd: PathBuf,
	// ESF Directory
	pub esfd: PathBuf,
}

impl IglooEnvInfo
{
	/// Returns the environment information for the igloo call
	pub fn get_env_info() -> IglooEnvInfo
	{
		IglooEnvInfo
		{
			cwd: env::current_dir().unwrap(),
			hd: match UserDirs::new()
			{
				Some(v) => v.home_dir().to_owned(),
				None =>
				{
					println!("Error: Failed to get home directory.\n\
							  This should never happen. Exiting...");
					std::process::exit(1);
				}
			},
			esfd: match std::env::var("ESF_DIR")
			{
				Ok(v) =>
				{
					std::path::PathBuf::from(&v.to_owned())
				}
				Err(e) =>
				{
					// Note: Need to change new to return errors
					// instead of exiting early
					println!("Error: $ESF_DIR not defined as an environment\
							  variable -- {:?}", e);
					std::process::exit(1);
				}
			}
		}
	}
}
