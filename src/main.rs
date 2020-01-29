extern crate clap;
use clap::Arg;
use clap::App;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let matches = App::new("mypipe").version("1.0").author("hedi BOUFADEN")
   .arg(Arg::with_name("input").long("in").takes_value(true).required(true))
   .arg(Arg::with_name("output").long("out").takes_value(true).required(false))
   .get_matches();


    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let input_result = Command::new(input).stdout(Stdio::piped()).spawn().expect("error");

    Command::new(output).stdin(input_result.stdout.unwrap()).spawn().expect("error");

}
