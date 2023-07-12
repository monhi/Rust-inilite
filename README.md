# inilite-rust

Previously I implemented a simple ini file handler by using C++ language. 
you can find it <a href="https://github.com/monhi/inilite">here</a>
Now I find that by using the same technologies, I can implement it in Rust language.

Thus I implemented it by using Rust language.

The current version is working properly and I upgrade it step by step to make it more standard.


## How to use it

* Download the project.
* Run cargo build to build the project.
* Run cargo run to run the project.
* Run cargo test to test the project.
  
As I test it in Windows OS, I set d:\config.ini as the ini file name in main.rs file.

If you use it on other operating systems, remember to change the ini file path based on your requirements.

But you can change it based on your requirements.

to use this file, you are supposed to first initialize the structure by passing the required fields.

Then run process_file() to create the file or read field contents.

Then by using 'set_key' you can write new keys into ini file.

And by using 'get_key_value' you can read available keys from it. 

The whole module is implemented in ini.rs file.

I use traits to implement the necessary methods to handle ini actions.







