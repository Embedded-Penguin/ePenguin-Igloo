extern crate clap;
extern crate config;

mod igloo_action;
mod igloo_project;
mod igloo_target;
mod igloo_manifest;
mod igloo_cli;

pub enum IglooType
{
	IT_NEW = 0,
	IT_RUN,
	IT_PUSH,
	IT_PULL,
	IT_HELP,
	IT_BUILD,
	IT_NULL,
}

pub enum IglooStatus
{
	IS_GOOD = 0x00,
	IS_BAD = 0x01,
	IS_UNKNOWN = 0x02,
}

pub struct Igloo
{
	cli_conf: IglooCliConfig,
	master_target_manifest: Config,

}

impl Igloo
{
	pub fn new() -> Self
	{
		Igloo
		{
			master_target_manifest: Config::new(),
			cli_conf: IglooCliConfig::new(),
		}
	}

	pub fn start(&mut self) -> Result<IglooInstType, IglooErrType>
	{
		let mut res: IglooInstType = Null;
	}
}

// Tests
#[cfg (test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
