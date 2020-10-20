/// Igloo Manifest -- Responsible for all lookups in manifest files
pub mod IglooManifest
{
	use crate::igloo::{Igloo, IglooErrType, IglooEnvInfo};
	pub fn target_exists(inst: &Igloo, name: &str) -> bool
	{
		let mut ret: bool = false;
		loop
		{
			if name.is_empty()
			{
				ret = false;
				break;
			}

			let make_table = inst.target_manifest.get_table("target.make").unwrap();
			let manifest_table = inst.target_manifest.get_table("target.manifest").unwrap();

			match make_table.get(name)
			{
				Some(v) =>
				{
					println!("target.make entry for \"{}\" exists!", name);
					ret = true;
				}
				None =>
				{
					ret = false;
				}
			}

			if !ret
			{
				break;
			}

			match manifest_table.get(name)
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
		break; }
		ret
	}
}
