#![allow(unused_imports)]
extern crate seahash;
extern crate lalrpop_util;
extern crate regex;
extern crate clap;
use clap::{App,Arg,SubCommand};

mod value;
pub mod parser_output;

mod validator;
mod formatter;
mod syntaxhelper;
mod notacfg;
mod seahasher;

fn main() {

    let matches = App::new("foxhole")
        .version("0.1.0")
        .version_short("0.1")
        .author("cody laeder <codylaeder@gmail.com>")
        .about("procedural dice roll simulator")
        .set_term_width(80)
        .max_term_width(80)
        .subcommand(SubCommand::with_name("fmt")
            .version("0.1.0")
            .version_short("0.1")
            .author("cody laeder <codylaeder@gmail.com>")
            .set_term_width(80)
            .max_term_width(80)
            .about("formats the input argument")
            .arg(Arg::with_name("input")
                 .index(1)
                 .takes_value(true)
                 .required(true)
                 .validator(validator::file_path)
                 .help("input file to format")))
        .get_matches();
    match matches.subcommand() {
        ("fmt",Option::Some(arg)) => {
            match formatter::formatter(arg.value_of("input").unwrap()) {
                Ok(_) => { },
                Err(e) => println!("{}", e)
            };
        },
        (_,_) => { }
    };
}

