pub mod ini_handler
{

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;



pub struct IniNode {
    pub filename: String,
    pub hashmap: HashMap<String, String>,
    pub hashset: HashSet<String>,
}

pub trait Methods {
    //fn print_file_name(&self);
    fn check_file_exists(&self) -> bool;
    fn create_file(&self) -> bool;
    fn process_file(&mut self) -> bool;
    fn get_key_value(&self, str_key: String, str_section: String) -> String;
    fn set_key(&mut self, str_value: String, str_key: String, str_section: String);
    fn save(&mut self);
}

impl Methods for IniNode {
    /* fn print_file_name(&self)
    {
       println!("{}",self.filename);
    }
    */

    fn check_file_exists(&self) -> bool {
        let b = std::path::Path::new(&self.filename).exists();
        return b;
    }

    fn create_file(&self) -> bool {
        let _file = File::create(&self.filename);
        return self.check_file_exists();
    }

    fn process_file(&mut self) -> bool {
        if self.check_file_exists() {
            let mut currentsection: String = "".to_string();
            let file = File::open(&self.filename).expect("file not found!");
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let stemp: String = line.unwrap().trim().to_string();
                if stemp.len() == 0 
                {
                    continue;
                }
                    
                if stemp.chars().nth(0).unwrap() == '['
                    && stemp.chars().nth(stemp.len() - 1).unwrap() == ']'
                {
                    let stemp = stemp.replace("[", "");
                    let stemp = stemp.replace("]", "");
                    //println!("found section: {}",stemp);
                    currentsection = stemp.clone();
                    self.hashset.insert(stemp.clone());
                } else {
                    let split = stemp.split("=");
                    let mut i = 0;
                    let mut key: String = "".to_string();
                    let mut val: String;

                    for s in split {
                        if i == 0 {
                            key = s.to_string();
                            i = 1;
                        } else if i == 1 {
                            val = s.to_string();
                            i = 2;
                            let mut mykey = currentsection.clone();
                            mykey.push('+');
                            mykey.push_str(&*key);
                            self.hashmap.insert(mykey, val);
                        } else {
                        }
                    }
                }
            }
            return true;
        }
        println!("File not found. Creating ...");
        return self.create_file();
    }

    fn get_key_value(&self, str_key: String, str_section: String) -> String {
        let mut key = str_section;
        key.push_str("+");
        key.push_str(&*str_key);
        let ret = self.hashmap.get(&key);
        match ret {
            Some(z) => return z.to_string(),
            None => return "".to_string(),
        }
    }

    fn set_key(&mut self, str_value: String, str_key: String, str_section: String) {
        let mut mykey = str_section.clone();
        mykey.push_str("+");
        mykey.push_str(&*str_key.clone());
        self.hashset.insert(str_section);
        self.hashmap.insert(mykey, str_value);
        self.save();
    }

    fn save(&mut self) {
        let mut file = File::create(self.filename.clone()).unwrap();

        for section in &self.hashset {
            let mut stemp = "[".to_string();
            stemp.push_str(section);
            stemp.push(']');
            writeln!(&mut file, "{}", stemp).unwrap();

            for (key, value) in &self.hashmap {
                let mut ss = section.clone();
                ss.push('+');
                if key.contains(&*ss)
                {
                    let mut wdata = key.replace(&*ss, "");
                    wdata.push('=');
                    wdata.push_str(value);
                    writeln!(&mut file, "{}", wdata).unwrap();
                }
                //println!("{} / {}", key, value);
            }
        }
    }
}
	
	
}



#[cfg(test)]
mod tests 
{
    use super::*;
	use std::collections::HashMap;
	use std::collections::HashSet;
	use crate::ini::ini_handler::Methods;	

	#[test]
    fn create_ini() 
	{
		let keys = HashMap::new();
		let set  = HashSet::new();
		let mut node = ini_handler::IniNode{filename:"d:\\config.ini".to_string(),hashmap:keys,hashset:set};		
		node.process_file();
		assert!(node.check_file_exists());		
    }
	
	#[test]	
	fn save_string_key()
	{
		let keys = HashMap::new();
		let set  = HashSet::new();
		let mut node = ini_handler::IniNode{filename:"d:\\config.ini".to_string(),hashmap:keys,hashset:set};		
		node.process_file();
		assert!(node.check_file_exists());	
		
		node.set_key("DSPCOM".to_string(),"Company".to_string(), "test_section".to_string());
		assert_eq!(node.get_key_value("Company".to_string(),"test_section".to_string()),"DSPCOM".to_string());
	}
	
	
	
}