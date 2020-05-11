#![allow(
    unused_imports,
    dead_code,
    unused_variables,
    non_snake_case,
    non_camel_case_types,
    unused_parens
)]

extern crate smallvec;
#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate itertools;
extern crate lalrpop_util;
extern crate num_rational;
extern crate rand;
extern crate regex;
extern crate seahash;
use clap::{App, Arg, SubCommand};

#[macro_use]
mod macros;

mod cfgbuilder;
mod formatter;
mod inliner;
mod namespace;
mod ordering;
pub mod parser_output;
//mod peephole;
mod run;
mod runtime;
mod seahasher;
mod syntaxhelper;
mod validator;
mod value;
use self::run::run_path;

fn main() {
    let matches = App::new("foxhole")
        .version("0.1.0")
        .version_short("0.1")
        .author("cody laeder <codylaeder@gmail.com>")
        .about("procedural dice roll simulator")
        .set_term_width(80)
        .max_term_width(80)
        .arg(
            Arg::with_name("input")
                .index(1)
                .required(true)
                .help("path to the file to run"),
        )
        .get_matches();
    let rc = match matches.value_of("input") {
        Option::Some(path) => match run_path(path) {
            Ok(x) => {
                println!("{}", x);
                0
            }
            Err(e) => {
                eprintln!("{}", e);
                1
            }
        },
        Option::None => {
            eprintln!("run '-h' for help");
            1
        }
    };
    ::std::process::exit(rc)
}
