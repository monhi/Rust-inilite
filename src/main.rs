mod ini;

use crate::ini::ini_handler::Methods;  
use std::collections::HashMap;
use std::collections::HashSet;

fn main() 
{
    let keys = HashMap::new();
    let set  = HashSet::new();
    let mut node = ini::ini_handler::IniNode{filename:"d:\\config.ini".to_string(),hashmap:keys,hashset:set};

    node.process_file();  
	
	println!("Writing Rust as Language key in general section of config.ini file.");
    node.set_key("Rust".to_string(),"Language".to_string(), "general".to_string());
	
	println!("Writing 1 as classno key in class section of config.ini file.");
    node.set_key("1".to_string(),"classno".to_string(), "class".to_string());
	
	println!("Writing 1.58 as Version key in general section of config.ini file.");
    node.set_key("1.58".to_string(),"Version".to_string(), "general".to_string());
	
	println!("Writing DSPCOM as Company key in general section of config.ini file.");
    node.set_key("DSPCOM".to_string(),"Company".to_string(), "general".to_string());

    println!("Programming language is {}",node.get_key_value("Language".to_string(),"general".to_string()));
    println!("Current Rust version is {}",node.get_key_value("Version".to_string(),"general".to_string()));    
}

