#![allow(non_snake_case)]
#![allow(unused_assignments)]

use std::env::Args;
use text_colorizer::Colorize;
use crate::BrowseFile::BrowseFile;

pub fn Launch(parametre :Args, OSType :bool)
{
	let filepath;
	let filename;
	
	
	let vecteuralacon: Vec<_> = parametre.skip(1).collect();
	let path = vecteuralacon.first().unwrap();
	println!("{}",path);
	
	let delimiter = path.rfind('/');
	match delimiter 
	{
		None => 
			{
				println!("Invalid path name {}",path);
				return;
			}
		Some(_) => 
			{
				(filepath,filename) = path.split_at(delimiter.unwrap()+1); 
			}
	}
	println!("{} <-> {}",filepath.italic(),filename.bright_white().bold());
	let grosfilsdepute = vecteuralacon.last().unwrap();
	println!("{}",grosfilsdepute.italic().bold());
	
	BrowseFile(path,grosfilsdepute);
}

