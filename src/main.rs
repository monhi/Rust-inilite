mod ini;

use crate::ini::Methods;  
use std::collections::HashMap;

fn main() {
    let keys = HashMap::new();
    let mut node = ini::IniNode{filename:"f:\\config.ini".to_string(),hashmap:keys};
    //node.print_file_name();
    //println!("{}",node.check_file_exists());
    //node.create_file();
    //println!("{}",node.check_file_exists());
    node.process_file();  
    println!("{}",node.get_key_value("version".to_string(),"general".to_string()));
}
