// Note to self: It makes way more sense for the target bank in our project
// to be a hashmap of type <String,IglooTarget> so that every time I want to
// BUT rust deserialization seems to favor these structs which includes the name
// which means I'm storing the name in the target and not outside of it.
// so for now I'm going to do it this way until I do the serialization myself (switch to syncing instead of rwwr),
// which is what I did in 0.1, before the core rewrite
// I'm only doing it this way because serialization seems to be easier this way
// After I get all core features (new, push, pull, erase, etc...) completed,
// I'll revisit this part of the project and change it to a more sensible management method
use crate::Igloo;
use crate::IglooStatus;
use crate::IglooStatus::*;
use crate::IglooProject;
struct IglooTargetLinks
{
	common: String,
	mcu: String,
	ld: String,
	cfg: String,
}

pub struct IglooTarget
{
	name: String,
	links: IglooTargetLinks,
	includes: Vec<String>,
	scripts: Vec<String>,
}

impl IglooTarget
{
	fn default() -> IglooTarget
	{
		IglooTarget
		{
			name: String::new(),
			links: IglooTargetLinks::default(),
			includes: Vec::new(),
			scripts: Vec::new(),
		}
	}

	/// takes the targets name and looks up the path
	/// deserializes the targets manifest file and creates the target
	pub fn from(igloo: &Igloo, name: String) -> Result<IglooTarget, IglooStatus>
	{

		Ok(IglooTarget::default())
	}

	/// Creates the target's configuration file from itself
	/// the target must be valid at this point or else the file will be junk
	pub fn generate(&self, project: &IglooProject) -> IglooStatus
	{
		let mut ret = IS_GOOD;

		ret
	}

	pub fn generate_makefile(&self, project: &IglooProject) -> IglooStatus
	{
		let mut ret = IS_GOOD;
		let prj_root = 

		ret
	}
}

impl IglooTargetLinks
{
	pub fn default() -> IglooTargetLinks
	{
		IglooTargetLinks
		{
			common: String::new(),
			mcu: String::new(),
			ld: String::new(),
			cfg: String::new(),
		}
	}
}
