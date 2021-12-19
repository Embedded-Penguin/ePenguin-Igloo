// igloo_manifest is a simple collection of functions that work with configs

use crate::Igloo;

use crate::IglooStatus;
use crate::IglooStatus::*;

use crate::IglooType;
use crate::IglooType::*;

/// USES: environment variables (home dir, esf dir, current dir)
/// DOES: brings target manifest into memory
pub fn get_master_target_manifest(inst: &mut Igloo) -> IglooStatus
{
	let mut ret: IglooStatus = IS_GOOD;

	match inst.master_target_manifest.merge(
		config::File::with_name(
			inst.env.esfd.join("manifest/target-manifest.toml").to_str.unwrap()))
	{
		Ok(_v) => (),
		Err(e) =>
		{
			println!("Error: {:?}", e);
			ret = IS_FAILED_TO_LOAD_MTM;
		}
	}
	ret
}
