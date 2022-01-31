#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::str_to_string)]
#![deny(rust_2018_idioms)]

//! A GitHub API client written in Rust.
//!
//! Getting started? Take a look at the [examples](https://github.com/octocat-rs/octocat-rs/tree/main/examples) folder in the project repository!

pub use github::*;

pub mod github;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}