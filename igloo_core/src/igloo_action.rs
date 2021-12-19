use igloo_base::*;
use igloo_base::IglooErrType::*;

use crate::Igloo;
use crate::igloo_project::IglooPrj;

pub fn iac_run(prj_name: &str, target: &str) -> IglooErrType
{
	let res_err: IglooErrType = ErrNone;
	res_err
}

pub fn new(inst: &Igloo, prj_name: &str, target: &str)
		   -> IglooErrType
{
	let mut res_err: IglooErrType = ErrNone;
	// Check if we are already inside of an igloo project
	// Creating an igloo project inside an igloo project
	// is a no no
	if IglooPrj::is_igloo_prj(&std::env::current_dir().unwrap())
	{
		println!("Calling igloo new from igloo project...");
		res_err = NewCalledInsideProject;
		return res_err
	}

	// Check if the project folder already exists
	// Don't want to accidentally overwrite anything
	if std::path::Path::new(prj_name).exists()
	{
		res_err = FolderAlreadyExists;
		return res_err
	}

	let project = IglooPrj::new(inst, prj_name, target);
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
	if res_err  != ErrNone
	{
		return res_err
	}
	res_err
}

pub fn push(inst: &Igloo)
{

}
