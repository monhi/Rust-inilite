# inilite-rust

Previously I implemented a simple ini file handler by using C++ language. 
you can find it <a href="https://github.com/monhi/inilite">here</a>
Now I find that by using same technologies, I can implement it in Rust language.

Thus I implemented it by using Rust language.

Current version is working properly but it not tested enough.

## How to use it

* Download the project.
* run cargo build to build the project.
* run cargo run to run the project.

As I test it in windows OS, I set f:\config.ini as the ini file name in main.rs file.

But you can change it based on your requirements.

to use this file, you are supposed to first initialize the structure by passing required fields.

Then run process_file() to create the file or read fild contents.

Then by using 'set_key' you can write new keys into ini file.

And by using 'get_key_value' you can read available keys from it. 





