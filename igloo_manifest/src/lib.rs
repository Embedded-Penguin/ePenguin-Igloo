/// Igloo Manifest is a subproject responsible for working with manifests.
/// Manifests are anything from config files to giant lists or ... manifests.
/// For now, all functionality is going to sit in this lib.rs until I figure out
/// how I want to structure manifests
extern crate config;
extern crate sscanf;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod IglooManifest
{
	use igloo_base::*;
	use igloo_base::IglooErrType::*;
	use config::Config;

	/// master_mm -- Master Make Manifest
	/// master_tm -- Master Target Manifest
	/// name -- name of target
	pub fn target_is_valid(master_mm: &Config, master_tm: &Config, name: &str)
						   -> Result<bool, IglooErrType>
	{
		let mut ret: bool = true;
		if name.is_empty()
		{
			return Err(InvalidTarget)
		}

		let mut target_make_name: String = String::default();
		// Confirm the target.make table exists in the master target manifest
		match master_tm.get_table("target.make")
		{
			Ok(v) =>
			{
				// Confirm the target exists in the target.make table
				// What this actually means is make sure we can use the target name
				// to acquire the target's name in the master make manifest
				match v.get(name)
				{
					Some(v) =>
					{
						// Now we've confirmed the target has an entry in the target.make table
						println!("target.make entry for \"{}\" exists!", v);
						// store the target's full name for use in the master make manifest later
						target_make_name = v.to_string();
					}
					None =>
					{
						// if we've gotten to this point and failed, it simply means the target doesn't have
						// a full name set in the target.make table. We need this for accessing it's makefile parameters
						// later, so we'll need to go add that now.
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

		// Now confirm the target has an entry in the master make manifest
		// strip the name for usable pieces of information
		let (dummy, arch, family, mcu_name) = sscanf::scanf!(
			target_make_name, "{}.{}.{}.{}", String, String, String, String).unwrap();
		// verify an entry exists for the arch
		match master_mm.get_table(&format!("{}.{}", dummy, arch))
		{
			Ok(_v) =>
			{
				println!("Make parameters found for arch");
			}
			Err(e) =>
			{
				println!("Make parameters not found: {}", e);
				ret = false;
			}
		}

		// verify an entry exists for the mcu family
		// later this will be family, then series, then mcu
		match master_mm.get_table(&format!("{}.{}.{}", dummy, arch, family))
		{
			Ok(_v) =>
			{
				println!("Make parameters found for mcu family");
			}
			Err(e) =>
			{
				println!("Make parameters not found: {}", e);
				ret = false;
			}
		}

		// finally, ver
		match master_mm.get_table(&format!("{}.{}.{}.{}", dummy, arch, family, mcu_name))
		{
			Ok(_v) =>
			{
				println!("Make parameters found for mcu family");
			}
			Err(e) =>
			{
				println!("Make parameters not found: {}", e);
				ret = false;
			}
		}

		Ok(ret)
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
}

