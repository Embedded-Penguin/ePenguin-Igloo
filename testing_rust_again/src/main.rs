use std::fs;
fn main()
{
    println!("Hello, world!");

    
    match fs::create_dir("a")
    {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

	println!("We just made a new directory! Printing...\n");
	match fs::read_dir(".")
	{
		Err(why) => println!("! {:?}", why.kind()),
		Ok(paths) => for path in paths
		{
			println!("> {:?}", path.unwrap().path());
		},
	}

	fs::remove_dir("a").unwrap_or_else(|why| {
		println!("! {:?}", why.kind());
	});


	println!("We just removed that directory! Printing...\n");
	match fs::read_dir(".")
	{
		Err(why) => println!("! {:?}", why.kind()),
		Ok(paths) => for path in paths
		{
			println!("> {:?}", path.unwrap().path());
		},
	}

}
