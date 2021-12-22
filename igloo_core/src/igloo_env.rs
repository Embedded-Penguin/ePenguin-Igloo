use crate::{PathBuf, env, UserDirs};

#[derive(Debug, PartialEq, Clone)]
pub struct IglooEnv
{
	// Current Working Directory
	pub cwd: PathBuf,
	// Home Directory
	pub hd: PathBuf,
	// ESF Directory
	pub esfd: PathBuf,
}

impl IglooEnv
{
	pub fn get_env() -> IglooEnv
	{
		IglooEnv
		{
			cwd: match env::current_dir()
			{
				Ok(v) => v,
				Err(e) => panic!(),
			},
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
					// Note : Need to change new to return actual errors
					// instead of exiting early
					println!("Error: $ESF_DIR not defined as an environment\
							  variable -- {:?}", e);
					std::process::exit(1);
				}
			}
		}
	}
}
