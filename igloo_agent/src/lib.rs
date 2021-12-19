use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use igloo_base::*;
use igloo_core::igloo_target;
fn openocd_thread(sender: Sender<String>, child: std::process::Child)
{
	let mut f = BufReader::new(child.stdout.unwrap());
	loop
	{
		let mut buf = String::new();
		match f.read_line(&mut buf)
		{
			Ok(_) =>
			{
				if !buf.is_empty()
				{
					sender.send(buf).unwrap();
				}
			}
			Err(e) => println!("error: {:?}", e),
		}
	}
}

fn gdb_thread(sender: &Sender<String>, child: std::process::Child)
{
	let mut f = BufReader::new(child.stdout.unwrap());
	loop
	{
		let mut buf = String::new();
		match f.read_line(&mut buf)
		{
			Ok(_) =>
			{
				if !buf.is_empty()
				{
					sender.send(buf).unwrap();
				}
			}
			Err(e) => println!("error: {:?}", e),
		}
	}
}

fn start_openocd_listener(sender: Sender<String>, target: &igloo_target::IglooTarget)
{
	let child = Command::new("openocd")
		.args(["-f", board_cfg_file])
		.stdout(Stdio::piped())
		.spawn()
		.expect("Failed to start openocd process");

	println!("Started openocd process: {}", child.id());

	thread::spawn(move || openocd_thread(sender, child));
}

fn ia_push(target: &igloo_target::IglooTarget) -> Result<String, igloo_base::IglooErrType>
{
	let (oo_tx, oo_rx) = channel();
	start_openocd_listener(oo_tx, target);
	let (gdb_tx, gdb_rx) = channel();

	Ok(String::from("working"))
}
