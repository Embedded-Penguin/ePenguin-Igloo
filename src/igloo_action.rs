pub mod IglooAction
{
	use crate::igloo_manifest::IglooManifest;
	use crate::igloo::{Igloo, IglooErrType, IglooEnvInfo};
	use crate::igloo_prj::IglooPrj;

	pub fn run(prj_name: &str, target: &str) -> IglooErrType
	{
		let res_err: IglooErrType = IglooErrType::IGLOO_ERR_NONE;
		res_err
	}

	pub fn new(inst: &Igloo, prj_name: &str, target: &str)
			   -> IglooErrType
	{
		let mut res_err: IglooErrType = IglooErrType::IGLOO_ERR_NONE;
		// Check if we are already inside of an igloo project
		// Creating an igloo project inside an igloo project
		// is a no no
		if std::path::Path::new(".igloo").exists()
		{
			res_err = IglooErrType::IGLOO_NEW_CALLED_INSIDE_PRJ;
			return res_err
		}
		// Check if the project folder already exists
		// Don't want to accidentally overwrite anything
		if std::path::Path::new(prj_name).exists()
		{
			res_err = IglooErrType::IGLOO_FOLDER_ALREADY_EXISTS;
			return res_err
		}

		let mut project = IglooPrj::new(inst, prj_name, target);
		match project
		{
			Err(e) =>
			{
				println!("Error spawning project: {:?}", e);
				res_err = e;
				return res_err
			}
			_ => (),
		}
		let res_err = project.unwrap().populate();
		if res_err  != IglooErrType::IGLOO_ERR_NONE
		{
			return res_err
		}
		res_err
	}
}
