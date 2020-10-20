/// Contains subcommand functions for igloo actions like new, run, push, etc..
pub mod IglooAction
{
	use crate::igloo::{Igloo, IglooErrType, IglooEnvInfo};
	pub fn run(prj_name: &str, target: &str) -> IglooErrType
	{
		let res_err: IglooErrType = IglooErrType::IGLOO_ERR_NONE;
		res_err
	}

	pub fn new(prj_name: &str, target: &str)
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
		// Create new directory
		let mut active_dir = IglooEnvInfo::info().cwd;
		println!("Active Directory: {:?}", active_dir.display());
		active_dir.push(prj_name);
		match std::fs::create_dir(&active_dir)
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		println!("Active Directory: {:?}", active_dir.display());
		println!("Creating .igloo dir...");
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join(".igloo"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join("src"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join("inc"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join("cfg"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		match std::fs::create_dir(
			std::path::Path::new(&active_dir)
				.join("ESF"))
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}

		// load targets
		//create symlinks in ESF
		match std::os::unix::fs::symlink("", "")
		{
			Err(e) => println!("{:?}", e),
			_ => (),
		}
		println!("Displaying contents of {:?}", active_dir.display());
		for entry in active_dir.read_dir()
			.unwrap()
		{
			let dir = entry.unwrap();
			println!("{:?}", dir.file_name());
		}
		res_err
	}
}
