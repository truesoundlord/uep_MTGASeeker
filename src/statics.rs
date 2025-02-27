#![allow(dead_code)]

use std::collections::HashMap;
use lazy_static::lazy_static;

pub static YEARS :&str = "2025";
pub static PATTERN :&str = "\t• ";
pub static PTTRN_CMNT :&str = "][";

pub static QUESTION :&str = "(qm)";         // ?
pub static DBLQTS :&str = "(dq)";           // "
pub static PIPE :&str = "(pp)";             // |
pub static CLN :&str = "(dp)";              // :
pub static LBRKT :&str = "(lb)";            // <
pub static RBRKT :&str = "(rb)";            // >
pub static ASTERIX :&str = "(as)";          // *
pub static MUL :&str = "(mul)";             // https://www.amp-what.com/unicode/search/asterisk
pub static FSLASH :&str = "(sl)";           // /
pub static BSLASH :&str = "(bs)";           // \
pub static RGSTRD :&str = "(r)";
pub static TRDMRK :&str = "(tm)";
pub static SMLEY :&str = "(smiley)";        // avec lunettes noires
pub static OSLASH :&str = "(obarre)";       // sale ikeatien
pub static OSLASHM :&str = "(mobarre)";     // sale ikeacien maj
pub static ARROW :&str = "(arrow)";
pub static BARROW :&str = "(barrow)";
pub static LTRARW :&str = "(larw)";					// flèche vers la gauche
pub static RTRARW :&str = "(rarw)";					// flèche vers la droite
pub static CENTS :&str = "(cents)";					
pub static LBARRE :&str = "(lbarré)";       // biélorusse de merde
pub static MLBARRE :&str = "(mlbarré)";     // ...ou ukrainien de merde
pub static SWORD :&str = "(sword)";
pub static DELTA :&str = "(delta)";
pub static INF :&str = "(inf)";							// infini 

pub static CUP :&str = "(Cup)";             // sale tchèque de merde Č
pub static CDW :&str = "(Cdw)";							// č
pub static CZED :&str = "(Edw)";						// ě


lazy_static!
{
	pub static ref ILLEGALCHARS :HashMap<&'static str,&'static str> =
	{
		let mut mapponstoutcela = HashMap::new();
		
		mapponstoutcela.insert(QUESTION,"?");
		mapponstoutcela.insert(DBLQTS,"\"");
		mapponstoutcela.insert(PIPE,"|");
		mapponstoutcela.insert(CLN,":");
		mapponstoutcela.insert(LBRKT,"<");
		mapponstoutcela.insert(RBRKT,">");
		mapponstoutcela.insert(ASTERIX,"*");
		mapponstoutcela.insert(FSLASH,"\u{2215}");
		mapponstoutcela.insert(BSLASH,"\\");
		mapponstoutcela.insert(RGSTRD,"\u{00AE}");
		mapponstoutcela.insert(TRDMRK,"\u{2122}");
		mapponstoutcela.insert(SMLEY,"\u{1F60E}");
		mapponstoutcela.insert(OSLASH,"\u{00F8}");
		mapponstoutcela.insert(OSLASHM,"\u{00D8}");
		mapponstoutcela.insert(MUL,"\u{002A}");
		mapponstoutcela.insert(ARROW,"\u{27F6}");  	// https://www.compart.com/en/unicode/block/U+27F0
		mapponstoutcela.insert(BARROW,"\u{27F5}"); 
		mapponstoutcela.insert(CENTS,"\u{00A2}");
		mapponstoutcela.insert(LBARRE,"\u{0142}");
		mapponstoutcela.insert(MLBARRE,"\u{0141}");
		mapponstoutcela.insert(SWORD,"\u{1F52A}");  // 31 mai 2024 (100C9)
		mapponstoutcela.insert(DELTA,"\u{0394}");  	// 11 juin 2024
		mapponstoutcela.insert(LTRARW,"\u{25C4}");  // 9 août 2024
		mapponstoutcela.insert(RTRARW,"\u{25BA}");  
		mapponstoutcela.insert(CUP,"\u{010C}");     // sale tchèque de merde
		mapponstoutcela.insert(CDW,"\u{010D}");
		mapponstoutcela.insert(CZED,"\u{011B}");
		mapponstoutcela.insert(INF,"\u{221E}");	
		
		mapponstoutcela
	};
	static ref COUNT :usize = ILLEGALCHARS.len();
}
