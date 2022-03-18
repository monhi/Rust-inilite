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
   pub hashmap :HashMap<String,String>
}

pub trait Methods {
    fn print_file_name(&self);
    fn check_file_exists(&self) -> bool;
    fn create_file(&self) -> bool;
    fn process_file(&mut self) -> bool;
    fn get_key_value(&self, str_key:String, str_section:String) -> String;
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
            let mut currentsection:String = "".to_string();
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
                    currentsection = stemp;
                }
                else
                {
                    let split = stemp.split("=");                    
                    let mut i          = 0;
                    let mut key:String = "".to_string();
                    let mut val:String = "".to_string();

                    for s in split {
                        if i == 0
                        {
                            key = s.to_string();
                            i = 1;
                        }
                        else if i == 1
                        {
                            val = s.to_string();
                            i = 2;
                            let mut mykey = currentsection.clone();
                            mykey.push('+');
                            mykey.push_str(&*key);     
                            self.hashmap.insert(mykey, val);
                        }
                        else
                        {}
                    }
                }
            }            
            return true;
        }
        println!("File not found. Creating ...");
        return self.create_file();
    }

    fn get_key_value(&self,str_key:String,str_section:String) -> String
    {
        let mut    key = str_section;
        key.push_str("+");
        key.push_str(&*str_key);
        let     ret = self.hashmap.get(&key);
        match ret 
        {
            Some(z) => return z.to_string(),
            None => return "".to_string()
        }
    }
}