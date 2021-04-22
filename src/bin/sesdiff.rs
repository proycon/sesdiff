extern crate clap;
extern crate dissimilar;

use std::io::BufRead;
use std::str::FromStr;
use clap::{Arg, App};

use sesdiff::*;

pub fn printeditstring(s: &str, op: char, suffix: bool) {
    if suffix {
        print!("{}[{}]", op, s.to_owned().chars().rev().collect::<String>());
    } else {
        print!("{}[{}]", op, s);
    }
}

pub fn printeditstringlength(s: &str, op: char, suffix: bool) {
    if suffix {
        print!("{}[#{}]", op, s.to_owned().chars().count());
    } else {
        print!("{}[#{}]", op, s.len());
    }
}


fn main() {
    let args = App::new("sesdiff")
        .version("0.3.0") //also adapt in cargo.toml
        .author("Maarten van Gompel (proycon) <proycon@anaproy.nl>")
        .about("Generates a shortest edit script (Myers' diff algorithm) to indicate how to get from the strings in column 1 to the strings in column 2. Also provides the edit distance.")
        //snippet hints --> addargb,addargs,addargi,addargf,addargpos
        .arg(Arg::with_name("suffix")
            .long("suffix")
            .short("s")
            .help("Suffix edit string, operates on a reversed string and ignores common prefixes in the output script")
            )
        .arg(Arg::with_name("prefix")
            .long("prefix")
            .short("p")
            .help("Prefix edit string, ignores common suffices in the output script")
            )
        .arg(Arg::with_name("nosubstitutions")
            .long("nosub")
            .short("S")
            .help("Do not count substittutions/transpositions in the edit distance")
            )
        .arg(Arg::with_name("abstract")
            .long("abstract")
            .short("a")
            .help("Attempt to generate more abstract edit scripts by not explicitly registering unchanged parts, but referring to them by their length only")
            )
        .arg(Arg::with_name("apply")
            .long("apply")
            .short("A")
            .help("Apply mode; apply the edit scripts from the second column to the strings in the first column"))
        .get_matches();

    let stdin = std::io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        if let Ok(line) = line {
            let fields: Vec<&str> = line.split("\t").collect();
            if line.trim().is_empty() {
                println!();
            } else if fields.len() >= 2 {
                let mode = if args.is_present("suffix") {
                    Mode::Suffix
                } else if args.is_present("prefix") {
                    Mode::Prefix
                } else {
                    Mode::Normal
                };
                print!("{}\t{}\t", fields[0], fields[1]);
                if args.is_present("apply") {
                    match EditScript::<String>::from_str(&fields[1]) {
                        Ok(mut editscript)  => {
                            if mode == Mode::Suffix {
                                editscript.mode = Mode::Suffix;
                            }
                            match editscript.apply_to(&fields[0], None) {
                                Ok(result) => print!("\t{}", result),
                                Err(err) => eprintln!("ERROR: {:?}", err)
                           }
                        },
                        Err(err) => eprintln!("ERROR: {:?}", err)
                    }
                } else {
                    if mode == Mode::Suffix {
                        let editscript = shortest_edit_script_suffix(&fields[0], &fields[1], args.is_present("abstract"), !args.is_present("nosubstitutions"));
                        print!("\t{}\t{}",editscript, editscript.distance);
                    } else {
                        let editscript = shortest_edit_script(&fields[0], &fields[1], args.is_present("prefix"), args.is_present("abstract"), !args.is_present("nosubstitutions"));
                        print!("\t{}\t{}",editscript, editscript.distance);
                    }
                }
                if fields.len() >= 2 {
                    //retain the rest of the input columns as well
                    for j in 2..fields.len() {
                        print!("\t{}",fields[j]);
                    }
                }
                println!();
            } else {
                eprintln!("Unable to process line {}, expected two tab-separated columns", i+1);
            }
        }
    }
}
