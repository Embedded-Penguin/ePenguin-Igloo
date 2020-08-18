#[derive(Debug)]
pub enum IglooInstType
{
	IGLOO_NULL = -1,
	IGLOO_NEW = 0,
	IGLOO_RUN = 1,
	IGLOO_FLASH = 2,
	IGLOO_DEBUG = 3,
	IGLOO_CLEAN = 4,
	IGLOO_GENDOC = 5
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

	pub fn start(&self) -> Result<IglooInstType, String>
	{
		let matches = clap::App::new("igloo")
			.about(clap::crate_description!())
			.version(clap::crate_version!())
			.get_matches();

		Ok(IglooInstType::IGLOO_NULL)
	}

	pub fn run(&self) -> Result<String, String>
	{
		Ok(String::from("Hello, we are running!\n"))
	}
}


