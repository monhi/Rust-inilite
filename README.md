# Rust-inilite

Previously I implemented a simple ini file handler by using C++ language. 
you can find it <a href="https://github.com/monhi/inilite">here</a>
Now I find that by using the same technologies, I can implement it in Rust language.

Thus I implemented it by using Rust language.

The current version is working properly and I upgrade it step by step to make it more standard.


## How to use it

* Download the project.
* Run `cargo build` to build the project.
* Run `cargo run` to run the project.
* Run `cargo test -- --test-threads=1` to test the project. In this way, tests run one after another in single thread.
  
As I test it in Windows OS, I set d:\config.ini as the ini file name in main.rs file.

If you use it on other operating systems, remember to change the ini file path based on your requirements.

But you can change it based on your requirements.

to use this file, you are supposed to first initialize the structure by passing the required fields.

Then run process_file() to create the file or read field contents.

Then by using 'set_key' you can write new keys into ini file.

And by using 'get_key_value' you can read available keys from it. 

The whole module is implemented in ini.rs file.

I use traits to implement the necessary methods to handle ini actions.

## Unit testing 
Rust is a modern language.
One of the features that I love a lot in this language is the unit testing capability.
In this project I implement the `ini_handler` module in `src/ini.rs` file and inside of it, I test different features of that module.
Thus I have the capability to test each module separately and those test routines evolve gradually to give an error-less module.
As all my test modules use `d:\config.ini` file, I have to set the test to run one after another. 
For that, I use `--test-threads=1` flag which gives this feature to run tests sequentially.












