extern crate clap;
use clap::{App, Arg};
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    let matches = App::new("My Own Pipe")
        .version("1.0")
        .author("Ilès Benkoussa <ilesbnk@hotmail.fr>")
        .about("Pipe to executable")
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .value_name("PROG1")
                .help("Takes the first programm")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .value_name("PROG2")
                .help("Takes the second programm")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let in1 = matches.value_of("in").unwrap_or("default");
    let out = matches.value_of("out").unwrap_or("default");

    let mut cmd_ls = Command::new(in1)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut cmd_grep = Command::new(out)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(ref mut stdout) = cmd_ls.stdout {
        if let Some(ref mut stdin) = cmd_grep.stdin {
            let mut buf: Vec<u8> = Vec::new();
            stdout.read_to_end(&mut buf).unwrap();
            stdin.write_all(&buf).unwrap();
        }
    }

    let res = cmd_grep.wait_with_output().unwrap().stdout;
    let end = String::from_utf8(res).unwrap();

    print!("{}", end);
}
