#![allow(non_snake_case)]
#![allow(unused_assignments)]

use std::process::exit;
use std::fs;
use std::io::Read;
use text_colorizer::{ColoredString, Colorize};
use crate::statics::{ILLEGALCHARS, PATTERN, PTTRN_CMNT};

pub fn BrowseFile(param_target :&str, param_filsdepute :&str)
{
	let mut bTrouve = false;
	let mut Buffer = String::new();
	let mut uncommentaire= "";

	let lefichier = fs::File::open(param_target);
	match lefichier
	{
		Ok(_) => 
			{
				println!("Opened file...");
			}
		Err(erreurdemerde) => 
			{
				println!("[{}]",erreurdemerde.to_string());
				exit(-2);
			}
	}
	
		
	let result = lefichier.unwrap().read_to_string(&mut Buffer);
	match result
	{
		Ok(_) =>
			{
				println!("\t...buffer filled...");
			}
		Err(erreurdemerde) => 
			{
				println!("[{}]",erreurdemerde.to_string());
				exit(-3);
			}
	}

	let mut thispattern= format!("[{}]",param_filsdepute);

	for element in ILLEGALCHARS.clone().into_iter()
	{
		if param_filsdepute.contains(element.0)
		{
			thispattern = param_filsdepute.replace(element.0,element.1);
			break;
		}
	}

	println!("\tseeking for {}...",thispattern.bold());

	let positionDebut = Buffer.rfind(&thispattern);
	match positionDebut 
	{
		None => 
			{
				println!("{} not found :{{",thispattern);
				return;
			}
		Some(_) => 
			{
				println!("\t {} found !!!",ColoredString::from(thispattern.to_string()).bold());
			}
	}
	
	// On fait quoi maintenant ?
	// Récupérer le nombre de poutrages
	
	let (mut tuple_caca,mut bon) = Buffer.split_at(positionDebut.unwrap());
	let mut position;
	loop
	{
		position = bon.find(PATTERN).unwrap_or(0);
		(tuple_caca,bon) = bon.split_at(position+PATTERN.len());
		if bTrouve == true
		{
			break;
		}
		bTrouve=true;
	}
	tuple_caca="";
	position = bon.find('\n').unwrap_or(0);
	let (couillonade,mut str_commentaires) = bon.split_at(position);
	let str_nbcommentaires=couillonade.trim_end();
	
	let mut nbCommentaires = u32::from_str_radix(str_nbcommentaires,10).unwrap_or(0); 
	println!("\t {} comment(s)...",nbCommentaires);
	
	bon="";
	position = str_commentaires.find("\n").unwrap_or(0);
	(tuple_caca,str_commentaires) = str_commentaires.split_at(position+1);
	loop 
	{
		position	= str_commentaires.find(PTTRN_CMNT).unwrap_or(0);
		(tuple_caca,str_commentaires) = str_commentaires.split_at(position+PTTRN_CMNT.len());
		if position == 0
		{
			break;
		}
		else 
		{  
			nbCommentaires-=1;
		}
		position = str_commentaires.find(PTTRN_CMNT).unwrap_or(0);
		(uncommentaire,str_commentaires) = str_commentaires.split_at(position);
		println!("{}",uncommentaire.trim_end().bright_white());
		position = str_commentaires.find('\n').unwrap_or(0);
		(tuple_caca,str_commentaires) = str_commentaires.split_at(position+1);
		if nbCommentaires == 0
		{
			break;
		}
	}
}