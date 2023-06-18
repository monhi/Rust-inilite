mod ini;

use crate::ini::Methods;  
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let keys = HashMap::new();
    let set  = HashSet::new();
    let mut node = ini::IniNode{filename:"d:\\config.ini".to_string(),hashmap:keys,hashset:set};

    node.process_file();  
    node.set_key("Rust".to_string(),"Language".to_string(), "general".to_string());
    node.set_key("1".to_string(),"classno".to_string(), "class".to_string());
    node.set_key("1.58".to_string(),"Version".to_string(), "general".to_string());
    node.set_key("DSPCOM".to_string(),"Company".to_string(), "general".to_string());

    println!("Programming language is {}",node.get_key_value("Language".to_string(),"general".to_string()));
    println!("Current Rust version is {}",node.get_key_value("Version".to_string(),"general".to_string()));    
}
