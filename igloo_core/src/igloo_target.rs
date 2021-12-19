struct IglooTargetLinks
{
	common: String,
	mcu: String,
	ld: String,
	cfg: String,
}

pub struct IglooTarget
{
	name: String,
	links: IglooTargetLinks,
	includes: Vec<String>,
	scripts: Vec<String>,
}

impl IglooTarget
{

}
