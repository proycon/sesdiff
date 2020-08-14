extern crate clap;
extern crate dissimilar;

use std::io::BufRead;
use std::borrow::Cow;
use dissimilar::{diff,Chunk};
use clap::{Arg, App};

pub fn printeditstring(s: &str, op: char, suffix: bool) {
    if suffix {
        print!("{}[{}]", op, s.to_owned().chars().rev().collect::<String>());
    } else {
        print!("{}[{}]", op, s);
    }
}

fn main() {
    let args = App::new("sesdiff")
        .version("0.1")
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
        .get_matches();

    //hints: matches.is_present() , matches.value_of()

    let stdin = std::io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        if let Ok(line) = line {
            let fields: Vec<&str> = line.split("\t").collect();
            if fields.len() == 2 {
                let left = if args.is_present("suffix") {
                    //operate on reverse string
                    Cow::from(fields[0].to_owned().chars().rev().collect::<String>())
                } else {
                    Cow::from(fields[0])
                };
                let right = if args.is_present("suffix") {
                    Cow::from(fields[1].to_owned().chars().rev().collect::<String>())
                } else {
                    Cow::from(fields[1])
                };
                let diffchunks: Vec<Chunk> = diff(&left, &right);
                print!("{}\t{}\t", fields[0], fields[1]);
                let mut prev: isize = 0;
                let mut distance = 0;
                let mut abort_at = 0;
                if args.is_present("suffix") || args.is_present("prefix") {
                    let mut tail = 0;
                    for chunk in diffchunks.iter() {
                        if let Chunk::Equal(_) = chunk {
                            tail += 1;
                        } else {
                            tail = 0;
                        }
                    }
                    abort_at = diffchunks.len() - tail;
                }
                for (i, chunk) in diffchunks.iter().enumerate() {
                    if abort_at != 0 && i == abort_at {
                        break;
                    }
                    match chunk {
                        Chunk::Equal(s) => {
                            printeditstring(s, '=', args.is_present("suffix"));
                            prev = 0;
                        }
                        Chunk::Delete(s) => {
                            let length: isize = s.chars().count() as isize;
                            let is_substitution = prev > 0 && length == prev;
                            if !is_substitution || args.is_present("nosubstitutions") {
                                distance += length;
                            }
                            printeditstring(s, '-', args.is_present("suffix"));
                            prev = length * -1;
                        }
                        Chunk::Insert(s) => {
                            let length: isize = s.chars().count() as isize;
                            let is_substitution = prev < 0 && s.len() as isize * -1 == prev;
                            if !is_substitution || args.is_present("nosubstitutions") {
                                distance += length;
                            }
                            printeditstring(s, '+', args.is_present("suffix"));
                            prev = length;
                        }
                    }
                }
                println!("\t{}",distance);
            } else {
                eprintln!("Unable to process line {}, expected two columns", i);
            }
        }
    }
}

