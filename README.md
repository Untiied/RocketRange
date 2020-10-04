# RocketRange
Rocket (web framework) extension to provide video / file "range" support.

** Just like the Rocket web framework this library requries use of the nightly version of Rust. **
** Use this link to determine the best nightly setup for you. [Rust Docs](https://doc.rust-lang.org/edition-guide/rust-2018/rustup-for-managing-rust-versions.html)
The purpose of this file is to provide a top level solution to Rocket not supporting the "Range" command for files. This allows us to create a custom response for the routing system.

To currently use this extension simply put this into your Cargo.toml
[dependencies]
rocket_range = { git = "https://github.com/Untiied/RocketRange" }

As the project stands right now: 

 - Safari can play a supplied video
 - Network calls are missed (more than likely because of the bad file implmentation)
 - Chrome / Firefox will crash their Rocket thread.


Pull requests are more than welcomed. 
