#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_unsafe)]

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, BufRead};

pub struct IniNode{
   pub filename:String,
   pub currentsection:String,
   pub hashmap :HashMap<String,String>
}

pub trait Methods {
    fn print_file_name(&self);
    fn check_file_exists(&self) -> bool;
    fn create_file(&self) -> bool;
    fn process_file(&mut self) -> bool;
    fn get_key_value(&self, param:String) -> String;
}

impl Methods for IniNode 
{
    fn print_file_name(&self)
    {
        println!("{}",self.filename);
    }
    
    fn check_file_exists(&self) -> bool {
        let b = std::path::Path::new(&self.filename).exists();
        return b;
    }

    fn create_file(&self) -> bool {
        let _file = File::create(&self.filename);  
        return self.check_file_exists();
    }

    fn process_file(&mut self) -> bool
    {
        if self.check_file_exists()
        {
            let file   = File::open(&self.filename).expect("file not found!");
            let reader = BufReader::new(file);

            for line in reader.lines() 
            {
                let stemp: String = line.unwrap().trim().to_string();
                if stemp.chars().nth(0).unwrap() == '[' &&  stemp.chars().nth(stemp.len()-1).unwrap() == ']'
                {
                    let stemp = stemp.replace("[",""); 
                    let stemp = stemp.replace("]","");
                    println!("found section: {}",stemp);
                }
                else
                {
                    let idx = stemp.find('=');
                    
                }


            }            
            //self.hashmap.insert("general-version".to_string(), "1.0".to_string());
            return true;
        }
        return self.create_file();
    }

    fn get_key_value(&self,param:String) -> String
    {
        let     ret = self.hashmap.get(&param);
        match ret 
        {
            Some(z) => return z.to_string(),
            None => return "".to_string()
        }
    }
}