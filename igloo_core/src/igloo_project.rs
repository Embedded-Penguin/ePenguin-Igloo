use crate::Igloo;
use crate::igloo_cli::*;

use crate::IglooType;
use crate::IglooType::*;

use crate::IglooStatus;
use crate::IglooStatus::*;

use crate::igloo_target::IglooTarget;

pub struct IglooProject<'a>
{
	igloo: &'a Igloo,
	name: String,
	targets: Vec<IglooTarget>,
}

impl<'a> IglooProject<'a>
{
	pub fn default(igloo_in: &'a Igloo) -> IglooProject
	{
		IglooProject
		{
			igloo: igloo_in,
			name: String::new(),
			targets: Vec::default(),

		}
	}
	/// Used to populate an IglooProject from scratch
	/// This means we do not yet have any project in storage
	/// and we must generate those directories, files, and symlinks
	/// and then populate the project in memory
	pub fn from_new(igloo_in: &'a Igloo, project_name: String) -> Result<IglooProject, IglooStatus>
	{
		Ok(IglooProject
		{
			name: ich_new_get_project_name(igloo_in),
			/// targets -- a vector of targets added for this project
			targets: Vec::default(),
			igloo: igloo_in,



		})
	}

	/// Used to populate an IglooProject from an existing project
	/// So this will be called when things like
	/// igloo run, push, pull, erase, etc... are called
	pub fn from_existing(igloo: &'a Igloo) -> IglooProject
	{
		IglooProject::default(igloo)

	}

	pub fn is_igloo_prj(path: &std::path::PathBuf) -> bool
	{
		if !path.join("igloo").exists()
		{
			return false
		}

		if !path.join("igloo.toml").exists()
		{
			return false
		}
		return true
	}
}
