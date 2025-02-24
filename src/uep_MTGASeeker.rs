#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod statics;
mod fonctions;
mod BrowseFile;

use crate::statics::YEARS;

use std::{env};
use std::process::exit;
use text_colorizer::*;
use crate::fonctions::Launch;

const NAME :&str = env!("CARGO_PKG_NAME");
const VERSION :&str = env!("CARGO_PKG_VERSION");
const AUTHOR :&str = env!("CARGO_PKG_AUTHORS");

fn main()
{
	let mut bOSType = false;
	println!("{} (version {} {YEARS} by {}) RUST {}", NAME.bright_cyan().bold(),VERSION.bold(),AUTHOR.bold(),rustc_version_runtime::version().to_string().bold());
	if cfg!(windows)
	{
		println!("{}",String::from("Windows detected !!").italic());
		bOSType = true;
	}
	else if cfg!(unix)
	{
		println!("{}",String::from("Unix alike detected !!").italic());
	}

	let Parametres = env::args();
	if Parametres.len() == 3
	{
		Launch(Parametres,bOSType);
	}
	else
	{
		println!("USAGE: {} <file name> <ennemy>",NAME.italic().bold());
		exit(1);
	}
	exit(0);
}
