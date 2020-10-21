/// Igloo Manifest -- Responsible for all lookups in manifest files
pub mod IglooManifest
{
	use crate::igloo::{Igloo, IglooErrType, IglooEnvInfo};
	pub fn target_exists(inst: &Igloo, name: &str) -> Result<bool, IglooErrType>
	{
		let mut ret: bool = true;
		let mut res_err = IglooErrType::IGLOO_ERR_NONE;
		if name.is_empty()
		{
			return Err(IglooErrType::IGLOO_INVALID_TARGET)
		}

		let make_table = inst.target_manifest.get_table("target.make");
		match make_table
		{
			Ok(v) =>
			{
				match v.get(name)
				{
					Some(v) =>
					{
						println!("target.make entry for \"{}\" exists!", name);
					}
					None =>
					{
						println!("target.make entry for \"{}\" does not exist", name);
						ret = false;
					}
				}

			}
			Err(e) =>
			{
				println!("{:?}", e);
				return Err(IglooErrType::IGLOO_FAILED_TO_LOAD_MAKE_MAN)
			}
		}

		if !ret
		{
			return Ok(ret)
		}

		let target_table = inst.target_manifest.get_table("target.manifest");
		match target_table
		{
			Ok(v) =>
			{
				match v.get(name)
				{
					Some(v) =>
					{
						println!("target.manifest entry for \"{}\" exists!", name);
					}
					None =>
					{
						ret = false;
					}
				}
			}
			Err(e) =>
			{
				println!("{:?}", e);
				return Err(IglooErrType::IGLOO_FAILED_TO_LOAD_TARG_MAN)
			}
		}

		Ok(ret)
	}



}
