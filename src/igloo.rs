#[derive(Debug)]
#[derive(PartialEq)]
pub enum IglooInstType
{
	IGLOO_NULL = -1,
	IGLOO_NEW = 0,
	IGLOO_RUN = 1,
	IGLOO_FLASH = 2,
	IGLOO_DEBUG = 3,
	IGLOO_CLEAN = 4,
	IGLOO_ERASE = 5,
	IGLOO_GENDOC = 6,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum IglooErrType
{
	IGLOO_ERR_NONE = 				0,
	IGLOO_ERR_UNKNOWN = 			1,
	IGLOO_CONFIG_NOT_FOUND = 		2,
	IGLOO_CONFIG_FOUND = 			3,
	IGLOO_UNKNOWN_INST_TYPE = 		4,
	IGLOO_NEW_CALLED_INSIDE_PRJ = 	5,
	IGLOO_FOLDER_ALREADY_EXISTS = 	6,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct IglooEnvInfo
{
	// Current Working Directory
	cwd: std::path::PathBuf,
	// Home Directory
	hd: std::path::PathBuf,
	// ESF Directory
	esfd: std::path::PathBuf,
}

use IglooInstType::*;
use IglooErrType::*;

/// Igloo should contain a config and any important environment information.
/// Upon running igloo, an instanc is created, env information is stored, and then
/// things happen.
pub struct Igloo
{
	conf: config::Config,
	cli_conf: clap::ArgMatches,
	env_info: IglooEnvInfo,
}

impl Igloo
{
	/// The New function creates an instance of igloo. The idea is I create an igloo,
	/// run start on it so I can collect environment information and process what command
	/// the user would like to run, and then I run that command or deal with errors.
	///
	/// This function handles all cli input and stores it. It is parsed for errors in the
	/// start function.
	pub fn New() -> Igloo
	{
		Igloo
		{
			conf: config::Config::default(),
			env_info:
			{
				IglooEnvInfo
				{
					cwd: std::env::current_dir().unwrap(),
					hd: std::env::home_dir().unwrap(),
					esfd: match std::env::var("ESF_DIR")
					{
						Ok(v) =>
						{
							std::path::PathBuf::from(v.to_owned() + "/ePenguin-Software-Framework")
						}
						Err(e) =>
						{
							// Note: Need to change New to return errors instead of exiting early
							println!("Error: $ESF_DIR not defined as an environment variable -- {:?}", e);
							std::process::exit(1);
						}
					}
				}
			},
			cli_conf: clap::App::new("igloo")
				.about(clap::crate_description!())
				.version(clap::crate_version!())
				.setting(clap::AppSettings::SubcommandRequiredElseHelp)
				.subcommand(
					clap::App::new("new")
						.about("Creates a new igloo project")
						.arg(
							clap::Arg::new("project_name")
								.required(true)
								.about("The name of the project to be created"),
						),
				)
				.subcommand(
					clap::App::new("run")
						.about("Compiles if needed, Flashes mcu and runs current project on default target")
						.arg(
							clap::Arg::new("build_type")
								.required(false)
								.about("Release or Debug build type\nDefaults to Debug"),
						),
				)
				.subcommand(
					clap::App::new("flash")
						.about("Flashes target mcu or multiple mcus")
						.arg(
							clap::Arg::new("build_type")
								.required(false)
								.about("Release or Debug build type\nDefaults to Debug"),
						),
				)
				.subcommand(
					clap::App::new("clean")
						.about("Cleans project build files")
				)
				.subcommand(
					clap::App::new("erase")
						.about("Erases flash from target mcu or target mcus")
				)
			    .get_matches()
		}
	}

	/// The start function processes the command you want igloo to run
	///  It theoretically should never return an error. It should just exit.
	///  If an error was returned, It was my fault and not the users.
	///  It is really only here to help me debug.
	///
	///  The Inst Type is only returned for usage outside of this struct.
	pub fn start(&self) -> Result<IglooInstType, IglooErrType>
	{
		let mut res_error = IGLOO_ERR_NONE;
		let mut res_type = IGLOO_NULL;
		println!("Current Directory: {:?}", self.env_info.cwd.display());
		println!("ESF Directory: {:?}", self.env_info.esfd.display());
		println!("Home Directory: {:?}", self.env_info.hd.display());
		match self.cli_conf.subcommand_name()
		{
			Some("new") =>
			{
				println!("Igloo new was called!");
				res_type = IglooInstType::IGLOO_NEW;
			}
			Some("run") =>
			{
				println!("Igloo run was called!");
				res_type = IglooInstType::IGLOO_RUN;
			}
			Some("flash") =>
			{
				println!("Igloo flash was called!");
				res_type = IglooInstType::IGLOO_FLASH;
			}
			Some("erase") =>
			{
				println!("Igloo erase was called!");
				res_type = IglooInstType::IGLOO_ERASE;
			}
			None => unreachable!(),
			_ => unreachable!(),
		}
		if res_type != IglooInstType::IGLOO_NULL
		{
			Ok(res_type)
		}
		else
		{
			Err(res_error)
		}
	}

	/// The run function processes the request from the user.
	/// On success, it will give some string indicating the success of the operation.
	/// On failure, it will return some error type.
	pub fn run(&self, inst_type: IglooInstType) -> Result<String, IglooErrType>
	{
		let mut res_err = IGLOO_ERR_NONE;
		loop { match inst_type
		{
			IGLOO_NULL => res_err = IGLOO_ERR_UNKNOWN,
			IGLOO_NEW =>
			{
				if let ("new", new_matches) = self.cli_conf.subcommand()
				{
					let prj_name: &str = new_matches.unwrap().value_of("project_name").unwrap();

					// Check if we are already inside of an igloo project
					// Creating an igloo project inside an igloo project
					// is a no no
					if std::path::Path::new(".igloo").exists()
					{
						res_err = IGLOO_NEW_CALLED_INSIDE_PRJ;
						break;

					}

					// Check if the project folder already exists
					// Don't want to accidentally overwrite anything
					if std::path::Path::new(prj_name).exists()
					{
						res_err = IGLOO_FOLDER_ALREADY_EXISTS;
						break;
					}

					// Create new directory
					let mut active_dir = self.env_info.cwd.clone();

					println!("Active Directory: {:?}", active_dir.display());
					active_dir.push(prj_name);
					match std::fs::create_dir(&active_dir)
					{
						Err(e) => println!("{:?}", e),
						_ => (),
					}
					println!("Active Directory: {:?}", active_dir.display());
					println!("Creating .igloo dir...");
					match std::fs::create_dir(std::path::Path::new(&active_dir).join(".igloo"))
					{
						Err(e) => println!("{:?}", e),
						_ => (),
					}
					match std::fs::create_dir(std::path::Path::new(&active_dir).join("src"))
					{
						Err(e) => println!("{:?}", e),
						_ => (),
					}
					match std::fs::create_dir(std::path::Path::new(&active_dir).join("inc"))
					{
						Err(e) => println!("{:?}", e),
						_ => (),
					}
					match std::fs::create_dir(std::path::Path::new(&active_dir).join("cfg"))
					{
						Err(e) => println!("{:?}", e),
						_ => (),
					}
					match std::fs::create_dir(std::path::Path::new(&active_dir).join("ESF"))
					{
						Err(e) => println!("{:?}", e),
						_ => (),
					}
					println!("Displaying contents of {:?}", active_dir.display());
					for entry in active_dir.read_dir().unwrap()
					{
						let dir = entry.unwrap();
						println!("{:?}", dir.file_name());
					}
				}
				else
				{
					panic!("Unknown error?");
				}
			}
			IGLOO_FLASH =>
			{

			}
			IGLOO_RUN =>
			{

			}
			_ => println!("Unhandled case: {:?}", inst_type),
		} break; }
		if res_err == IGLOO_ERR_NONE
		{
			Ok(String::from("We won!"))
		}
		else
		{
			Err(res_err)
		}
	}
}


