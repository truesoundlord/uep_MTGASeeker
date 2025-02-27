#![allow(non_snake_case)]
#![allow(unused_assignments)]

use std::env::Args;
use crate::BrowseFile::BrowseFile;

pub fn Launch(parametre :Args, OSType :bool)
{
	let filepath;
	let filename;
	let separator;
	
	let vecteuralacon: Vec<_> = parametre.skip(1).collect();
	let path = vecteuralacon.first().unwrap();
	
	if OSType
	{
		separator='\\';
	}
	else 
	{  
		separator='/';
	}
	
	let delimiter = path.rfind(separator);
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
	// let grosfilsdepute = vecteuralacon.last().unwrap().as_mut();
	BrowseFile(path,vecteuralacon.last().unwrap());
}

