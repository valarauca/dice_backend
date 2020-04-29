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
mod peephole;
mod runtime;
mod seahasher;
mod syntaxhelper;
mod validator;
mod value;

fn main() {
    let matches = App::new("foxhole")
        .version("0.1.0")
        .version_short("0.1")
        .author("cody laeder <codylaeder@gmail.com>")
        .about("procedural dice roll simulator")
        .set_term_width(80)
        .max_term_width(80)
        .subcommand(
            SubCommand::with_name("fmt")
                .version("0.1.0")
                .version_short("0.1")
                .author("cody laeder <codylaeder@gmail.com>")
                .set_term_width(80)
                .max_term_width(80)
                .about("formats the input argument")
                .arg(
                    Arg::with_name("input")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .validator(validator::file_path)
                        .help("input file to format"),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        ("fmt", Option::Some(arg)) => {
            match formatter::formatter(arg.value_of("input").unwrap()) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            };
        }
        (_, _) => {}
    };
}
