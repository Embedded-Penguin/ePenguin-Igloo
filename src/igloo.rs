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
	IGLOO_ERR_UNKNOWN = 0,
	IGLOO_CONFIG_NOT_FOUND = 1,
	IGLOO_CONFIG_FOUND = 2,
}

pub struct Igloo
{
	inst_type: IglooInstType,
	conf: config::Config,
}

impl Igloo
{
	pub fn New() -> Igloo
	{
		Igloo
		{
			inst_type: IglooInstType::IGLOO_NULL,
			conf: config::Config::default(),
		}
	}

	pub fn start(&self) -> Result<IglooInstType, IglooErrType>
	{
		let mut res_error = IglooErrType::IGLOO_ERR_UNKNOWN;
		let mut res_type = IglooInstType::IGLOO_NULL;
		let matches = clap::App::new("igloo")
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
			    .get_matches();

		match matches.subcommand_name()
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

	pub fn run(&self) -> Result<String, String>
	{
		Ok(String::from("Hello, we are running!\n"))
	}
}


