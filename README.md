# RocketRange
Rocket (web framework) extension to provide video / file "range" support for Safari.
The purpose of this project is to provide a top-level solution to Rocket not supporting the "Range" command for Safari.

Just like the Rocket web framework this library requires the use of the nightly version of Rust.
Use this link to determine the best nightly setup for you [Rust Docs](https://doc.rust-lang.org/edition-guide/rust-2018/rustup-for-managing-rust-versions.html)


To currently use this extension, simply put this into your Cargo.toml
```sh
[dependencies]
rocket_range = { git = "https://github.com/Untiied/RocketRange", branch = "main" }
```

As the project stands right now: 

 - Safari, Chromium, and Firefox can play a supplied video
 - Network calls can be missed sometimes when the client is requesting multiple Range requests.
 - RocketRange is just as production ready as Rocket itself is.

