extern crate dissimilar;

use std::fmt;
use std::cmp::PartialEq;
use dissimilar::{diff,Chunk};

#[derive(Clone,Debug)]
pub struct EditScript<T> {
    pub mode: Mode,
    pub distance: u32,
    pub instructions: Vec<EditInstruction<T>>,
}


#[derive(Clone,Debug)]
pub enum EditInstruction<T> {
    Insertion(T),
    Deletion(T),
    Identity(T),
    GenericIdentity(usize),
}

impl<T: std::fmt::Display> fmt::Display for EditScript<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for instruction in self.instructions.iter() {
            write!(f, "{}", instruction)?;
        }
        fmt::Result::Ok(())
    }
}

impl<T: std::fmt::Display> fmt::Display for EditInstruction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EditInstruction::GenericIdentity(s) => {
                write!(f, "=[#{}]", s)
            },
            EditInstruction::Identity(s) => {
                write!(f, "=[{}]", s)
            }
            EditInstruction::Insertion(s) => {
                write!(f, "+[{}]", s)
            }
            EditInstruction::Deletion(s) => {
                write!(f, "-[{}]", s)
            }
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Mode {
    Normal,
    Suffix,
    Prefix
}

/// Compute the shorted edit script (Myers' diff) between source and target
/// Returns an edit script with borrowed references to the original source
pub fn shortest_edit_script<'a>(source: &'a str, target: &'a str, prefix: bool, generic: bool, allow_substitutions: bool) -> EditScript<&'a str> {
    let diffchunks: Vec<Chunk> = diff(&source, &target);
    let mut prev: isize = 0;
    let mut distance = 0;
    let mut abort_at = None;
    if prefix {
        let mut tail = 0;
        for chunk in diffchunks.iter() {
            if let Chunk::Equal(_) = chunk {
                tail += 1;
            } else {
                tail = 0;
            }
        }
        abort_at = Some(diffchunks.len() - tail);
    }
    let mut instructions: Vec<EditInstruction<&'a str>> = Vec::with_capacity(abort_at.unwrap_or(diffchunks.len()));
    for (i, chunk) in diffchunks.iter().enumerate() {
        if abort_at.is_some() && i == abort_at.unwrap() {
            break;
        }
        match chunk {
            Chunk::Equal(s) => {
                if generic {
                    instructions.push(EditInstruction::GenericIdentity(s.chars().count()));
                } else {
                    instructions.push(EditInstruction::Identity(s));
                }
                prev = 0;
            }
            Chunk::Delete(s) => {
                let length: isize = s.chars().count() as isize;
                let is_substitution = prev > 0 && length == prev;
                if !is_substitution || !allow_substitutions {
                    distance += length;
                }
                instructions.push(EditInstruction::Deletion(s));
                prev = length * -1;
            }
            Chunk::Insert(s) => {
                let length: isize = s.chars().count() as isize;
                let is_substitution = prev < 0 && s.len() as isize * -1 == prev;
                if !is_substitution || !allow_substitutions {
                    distance += length;
                }
                instructions.push(EditInstruction::Insertion(s));
                prev = length;
            }
        }
    }
    EditScript {
        instructions: instructions,
        mode: match prefix {
            true => Mode::Prefix,
            false => Mode::Normal,
        },
        distance: distance as u32,
    }
}

/// Compute the shortest edit script (Myers' diff) between source and target where we look at
/// suffixes and strip common prefixes
/// Returns an edit script with owned strings
pub fn shortest_edit_script_suffix(source: &str, target: &str, generic: bool, allow_substitutions: bool) -> EditScript<String> {
    let source = source.to_owned().chars().rev().collect::<String>();
    let target = target.to_owned().chars().rev().collect::<String>();
    let diffchunks: Vec<Chunk> = diff(source.as_str(), target.as_str());
    let mut prev: isize = 0;
    let mut distance = 0;
    let abort_at = {
        let mut tail = 0;
        for chunk in diffchunks.iter() {
            if let Chunk::Equal(_) = chunk {
                tail += 1;
            } else {
                tail = 0;
            }
        }
        Some(diffchunks.len() - tail)
    };
    let mut instructions: Vec<EditInstruction<String>> = Vec::with_capacity(abort_at.unwrap_or(diffchunks.len()));
    for (i, chunk) in diffchunks.iter().enumerate() {
        if i == abort_at.unwrap() {
            break;
        }
        match chunk {
            Chunk::Equal(s) => {
                if generic {
                    instructions.push(EditInstruction::GenericIdentity(s.chars().count()));
                } else {
                    instructions.push(EditInstruction::Identity(s.to_owned().chars().rev().collect::<String>()));
                }
                prev = 0;
            }
            Chunk::Delete(s) => {
                let length: isize = s.chars().count() as isize;
                let is_substitution = prev > 0 && length == prev;
                if !is_substitution || !allow_substitutions {
                    distance += length;
                }
                instructions.push(EditInstruction::Deletion(s.to_owned().chars().rev().collect::<String>()));
                prev = length * -1;
            }
            Chunk::Insert(s) => {
                let length: isize = s.chars().count() as isize;
                let is_substitution = prev < 0 && s.len() as isize * -1 == prev;
                if !is_substitution || !allow_substitutions {
                    distance += length;
                }
                instructions.push(EditInstruction::Insertion(s.to_owned().chars().rev().collect::<String>()));
                prev = length;
            }
        }
    }
    EditScript {
        instructions: instructions,
        mode: Mode::Suffix,
        distance: distance as u32,
    }
}
