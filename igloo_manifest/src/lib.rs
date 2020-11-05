extern crate config;

use igloo_base::*;
use igloo_base::IglooErrType::*;
use config::Config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Igloo Manifest -- Responsible for all lookups in manifest files
pub fn get_master_make_manifest(man: &mut Config) -> IglooErrType
{
	let mut ret: IglooErrType = ErrNone;
	match man.merge(
		config::File::with_name(
			IglooEnvInfo::get_env_info().esfd.join("manifest/make-manifest.toml")
				.to_str()
				.unwrap()))
	{
		Ok(_v) => (),
		Err(e) =>
		{
			println!("Error: {:?}", e);
			ret = FailedToLoadMasterMakeManifest;
		}
	}

	ret
}

pub fn get_master_target_manifest(man: &mut Config) -> IglooErrType
{
	let mut ret: IglooErrType = ErrNone;
	match man.merge(
		config::File::with_name(
			IglooEnvInfo::get_env_info().esfd.join("manifest/target-manifest.toml")
				.to_str()
				.unwrap()))
	{
		Ok(_v) => (),
		Err(e) =>
		{
			println!("Error: {:?}", e);
			ret = FailedToLoadMasterTargetManifest;
		}
	}

	ret
}
/// master_mm -- Master Make Manifest
/// master_tm -- Master Target Manifest
/// name -- name of target
pub fn target_exists(master_mm: &Config, master_tm: &Config, name: &str)
					 -> Result<bool, IglooErrType>
{
	let mut ret: bool = true;
	if name.is_empty()
	{
		return Err(InvalidTarget)
	}

	match master_mm.get_table("target.make")
	{
		Ok(v) =>
		{
			match v.get(name)
			{
				Some(v) =>
				{
					println!("target.make entry for \"{}\" exists!", v);
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
			return Err(FailedToLoadMasterMakeManifest)
		}
	}

	if !ret
	{
		return Ok(ret)
	}

	let target_table = master_tm.get_table("target.manifest");
	match target_table
	{
		Ok(v) =>
		{
			match v.get(name)
			{
				Some(v) =>
				{
					println!("target.manifest entry for \"{}\" exists!", v);
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
			return Err(FailedToLoadMasterTargetManifest)
		}
	}

	Ok(ret)
}
