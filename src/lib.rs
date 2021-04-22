extern crate dissimilar;

use std::fmt;
use std::cmp::PartialEq;
use std::str::FromStr;
use std::borrow::{Borrow,ToOwned};
use dissimilar::{diff,Chunk};

#[derive(Debug)]
pub struct ParseError(String);

#[derive(Debug)]
pub enum ApplyError {
    NoMatch,
    WithMessage(String)
}

#[derive(Debug)]
pub struct EditScript<T> {
    pub mode: Mode,
    pub distance: u32,
    pub instructions: Vec<EditInstruction<T>>,
}



#[derive(Debug)]
pub enum EditInstruction<T> {
    /// An insertion
    Insertion(T),

    /// A deletion
    Deletion(T),

    /// An identity check
    Identity(T),

    /// An identity check for a specific length (a generic abstraction)
    GenericIdentity(u32),

    /// A disjunction over multiple possible insertions
    InsertionOptions(Vec<T>),

    /// A disjunction over multiple possible deletions
    DeletionOptions(Vec<T>),
    ///
    /// A disjunction over multiple possible identities
    IdentityOptions(Vec<T>),
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
            },
            EditInstruction::Insertion(s) => {
                write!(f, "+[{}]", s)
            },
            EditInstruction::Deletion(s) => {
                write!(f, "-[{}]", s)
            },
            EditInstruction::IdentityOptions(s) => {
                write!(f, "=[{}]", s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|"))
            },
            EditInstruction::InsertionOptions(s) => {
                write!(f, "+[{}]", s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|"))
            },
            EditInstruction::DeletionOptions(s) => {
                write!(f, "-[{}]", s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|"))
            }
        }
    }
}

impl FromStr for EditScript<String> {
  type Err = ParseError;

   fn from_str(editscript: &str) -> Result<Self,Self::Err> {
        let mut instructions: Vec<EditInstruction<String>> = Vec::new();
        let mut begin = 0;
        let mut distance = 0;
        for (i, c) in editscript.char_indices() {
            if c == ']' {
                let instruction = EditInstruction::<String>::from_str(&editscript[begin..i+1])?;
                if instruction.is_change() {
                    distance += 1;
                }
                instructions.push(instruction);
                begin = i+1;
            }
        }
        if instructions.is_empty() {
            return Err(ParseError(format!("Not a valid edit script, no instructions found: {}", editscript)));
        }
        Ok(EditScript {
            distance: distance,
            instructions: instructions,
            mode: Mode::Normal,
        })
    }

}

impl FromStr for EditInstruction<String> {
  type Err = ParseError;

   fn from_str(editinstruction: &str) -> Result<Self,Self::Err> {
        if editinstruction.len() <= 3 {
            return Err(ParseError(format!("String too short to describe a valid edit instruction: {}", editinstruction)));
        }
        let mut chariter = editinstruction.chars();
        let operator = chariter.next() ; //first char is operator
        let startbracket = chariter.next() ; //second char is start bracket
        if startbracket != Some('[') {
            return Err(ParseError(format!("Expected start bracket: {}", editinstruction)));
        }
        let s = &editinstruction[2..editinstruction.len()-1];
        let instruction = match operator {
                Some('+') => {
                    if s.contains("|") {
                        EditInstruction::InsertionOptions(s.split("|").collect())
                    } else {
                        EditInstruction::Insertion(s)
                    }
                }
                Some('-') => {
                    if s.contains("|") {
                        EditInstruction::DeletionOptions(s.split("|").collect())
                    } else {
                        EditInstruction::Deletion(s)
                    }
                }
                Some('=') => {
                    if s.contains("|") {
                        if s.chars().nth(0) == Some('#') && s[1..].parse::<u32>().is_ok() {
                            return Err(ParseError(format!("GenericIdentity can not take multiple values")));
                        } else {
                            EditInstruction::IdentityOptions(s.split("|").collect())
                        }
                    } else {
                        if s.chars().nth(0) == Some('#') && s[1..].parse::<u32>().is_ok() {
                            EditInstruction::GenericIdentity(s[1..].parse::<u32>().unwrap())
                        } else {
                            EditInstruction::Identity(s)
                        }
                    }
                },
                _ => return Err(ParseError(format!("Parsing editscript failed, invalid operator")))
        };
        Ok(instruction.to_owned())
    }
}

impl EditInstruction<&str> {

    /// This is technically different from using the ToOwned trait because I couldn't get the Borrow<>
    /// counterpart to work out.
    fn to_owned(&self) -> EditInstruction<String> {
        match self {
            EditInstruction::Insertion(s) => EditInstruction::Insertion(s.to_string()),
            EditInstruction::Deletion(s) => EditInstruction::Deletion(s.to_string()),
            EditInstruction::Identity(s) => EditInstruction::Identity(s.to_string()),
            EditInstruction::GenericIdentity(n) => EditInstruction::GenericIdentity(*n),
            EditInstruction::InsertionOptions(v) => EditInstruction::InsertionOptions(v.iter().map(|s| s.to_string()).collect()),
            EditInstruction::DeletionOptions(v) => EditInstruction::DeletionOptions(v.iter().map(|s| s.to_string()).collect()),
            EditInstruction::IdentityOptions(v) => EditInstruction::IdentityOptions(v.iter().map(|s| s.to_string()).collect()),
        }
    }
}

impl EditScript<&str> {
    fn to_owned(&self) -> EditScript<String> {
        EditScript {
            distance: self.distance,
            mode: self.mode,
            instructions: self.instructions.iter().map(|x| x.to_owned()).collect()
        }
    }
}


impl EditInstruction<String> {
    fn as_ref(&self) -> EditInstruction<&str> {
        match self {
            EditInstruction::Insertion(s) => EditInstruction::Insertion(s.as_str()),
            EditInstruction::Deletion(s) => EditInstruction::Deletion(s.as_str()),
            EditInstruction::Identity(s) => EditInstruction::Identity(s.as_str()),
            EditInstruction::GenericIdentity(n) => EditInstruction::GenericIdentity(*n),
            EditInstruction::InsertionOptions(v) => EditInstruction::InsertionOptions(v.iter().map(|s| s.as_str()).collect()),
            EditInstruction::DeletionOptions(v) => EditInstruction::DeletionOptions(v.iter().map(|s| s.as_str()).collect()),
            EditInstruction::IdentityOptions(v) => EditInstruction::IdentityOptions(v.iter().map(|s| s.as_str()).collect()),
        }
    }
}

impl EditScript<String> {
    fn as_ref(&self) -> EditScript<&str> {
        EditScript {
            distance: self.distance,
            mode: self.mode,
            instructions: self.instructions.iter().map(|x| x.as_ref()).collect()
        }
    }
}

impl<T> EditScript<T> {
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}

impl<T> EditInstruction<T> {
    pub fn is_change(&self) -> bool {
        match self {
            EditInstruction::Insertion(_) | EditInstruction::Deletion(_) => true,
            EditInstruction::Identity(_) => false,
            EditInstruction::GenericIdentity(_) => false,
            EditInstruction::InsertionOptions(_) | EditInstruction::DeletionOptions(_) => true,
            EditInstruction::IdentityOptions(_) => false,
        }
    }

}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Mode {
    Normal,
    Suffix,
    Prefix,

    ///Infix mode is only used when applying an edit script and means it can apply to any subpart (infix) of
    ///the string and may also apply multiple times.
    Infix,
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
                    instructions.push(EditInstruction::GenericIdentity(s.chars().count() as u32));
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
                    instructions.push(EditInstruction::GenericIdentity(s.chars().count() as u32));
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

pub trait ApplyEditScript {
    fn apply_to(&self, input: &str, mode: Option<Mode>) -> Result<String,ApplyError>;
}

impl ApplyEditScript for EditScript<String> {
    fn apply_to(&self, input: &str, mode: Option<Mode>) -> Result<String,ApplyError> {
        self.as_ref().apply_to(input, mode)
    }
}

///auxiliary internal function for apply_to() in normal/prefix mode
fn instruction_applies(instructioncontent: &str, input: &str, head: &Option<String>, tail: &Option<String>, tailchars: usize) -> Result<usize,ApplyError> {
    let instructionlength = instructioncontent.chars().count();
    if instructionlength > tailchars {
        return Err(ApplyError::NoMatch);
        //return Err(ApplyError(format!("Edit script does not match current word, prefix is longer than head (unable to remove prefix {})", prefix)));
    }
    let refcontent = if let Some(tail) = tail {
        &tail[..instructionlength]
    } else {
        &input[..instructionlength]
    };
    if refcontent != instructioncontent {
        return Err(ApplyError::NoMatch);
    } else {
        Ok(instructionlength)
    }
}


impl ApplyEditScript for EditScript<&str> {
    fn apply_to(&self, input: &str, mode: Option<Mode>) -> Result<String,ApplyError> {
        let mode = if let Some(mode) = mode {
            mode
        } else {
            self.mode
        };

        if mode == Mode::Infix {
            /////////////////////////////////// INFIX MODE

            //iterate over the input attempting to match at each stage

            for (i, _) in input.char_indices() {
                if let Ok(result) = self.apply_to(&input[i..], Some(Mode::Normal)) { //we override the mode
                    return Ok(result.to_string());
                }
            }

            Err(ApplyError::NoMatch)
        } else if mode == Mode::Suffix {
            /////////////////////////////////// SUFFIX MODE
            let mut head: String = input.to_string();
            let mut tail = String::new();
            for instruction in self.instructions.iter() {
                let headchars = head.chars().count();
                /*
                eprintln!("DEBUG: Instruction: {}", instruction);
                eprintln!("              Head: {}", head);
                eprintln!("              Tail: {}", tail);*/
                match instruction {
                    EditInstruction::Deletion(suffix) => {
                        let suffixchars = suffix.chars().count();
                        if suffixchars > headchars {
                            return Err(ApplyError::WithMessage(format!("Edit script does not match current word, suffix is longer than head (unable to remove suffix {})", suffix)));
                        }
                        let foundsuffix: String = head.chars().skip(headchars - suffixchars).take(suffixchars).collect();
                        if foundsuffix.as_str() != *suffix {
                            return Err(ApplyError::WithMessage(format!("Edit script does not match current word (unable to find and remove suffix '{}', found '{}' instead)", suffix, foundsuffix)));
                        }
                        head = head.chars().take(headchars - suffixchars).collect();
                    },
                    EditInstruction::Insertion(s) => {
                        tail.insert_str(0, s);
                    },
                    EditInstruction::GenericIdentity(keeplength) => {
                        let keeplength = *keeplength as usize;
                        if keeplength > headchars {
                            return Err(ApplyError::WithMessage(format!("Edit script does not match current word, length to keep is longer than head")));
                        }
                        tail = head.chars().skip(headchars - keeplength).take(keeplength).collect::<String>() + tail.as_str();
                        head = head.chars().take(headchars - keeplength).collect();
                    },
                    EditInstruction::Identity(suffix) => {
                        let suffixchars = suffix.chars().count();
                        if suffixchars > headchars {
                            return Err(ApplyError::WithMessage(format!("Edit script does not match current word, suffix is longer than head (unable to keep suffix {})", suffix)));
                        }
                        let foundsuffix: String = head.chars().skip(headchars - suffixchars).take(suffixchars).collect();
                        if foundsuffix.as_str() != *suffix {
                            return Err(ApplyError::WithMessage(format!("Edit script does not match current word (unable to find and keep suffix {})", suffix)));
                        }
                        tail = head.chars().skip(headchars - suffixchars).take(suffixchars).collect::<String>() + tail.as_str();
                        head = head.chars().take(headchars - suffixchars).collect();
                    },
                    EditInstruction::IdentityOptions(suffixes) => {
                        for suffix in suffixes {
                            let suffixchars = suffix.chars().count();
                            if suffixchars > headchars {
                                continue; //no match
                            }
                            let foundsuffix: String = head.chars().skip(headchars - suffixchars).take(suffixchars).collect();
                            if foundsuffix.as_str() == *suffix {
                                //match, apply
                                tail = head.chars().skip(headchars - suffixchars).take(suffixchars).collect::<String>() + tail.as_str();
                                head = head.chars().take(headchars - suffixchars).collect();
                                break;
                            }
                        }
                    }
                    EditInstruction::DeletionOptions(suffixes) => {
                        for suffix in suffixes {
                            let suffixchars = suffix.chars().count();
                            if suffixchars > headchars {
                                continue; //no match
                            }
                            let foundsuffix: String = head.chars().skip(headchars - suffixchars).take(suffixchars).collect();
                            if foundsuffix.as_str() == *suffix {
                                //match, apply
                                head = head.chars().take(headchars - suffixchars).collect();
                                break;
                            }
                        }
                    },
                    EditInstruction::InsertionOptions(_) => {
                        return Err(ApplyError::WithMessage(format!("Edit script has multiple insertion options and is therefor ambiguous, unable to apply")));
                    },
                }
            }
            head += tail.as_str();
            Ok(head)
        } else {
            /////////////////////////////////// NORMAL or PREFIX MODE

            //we use Options because we want to defer making clones and new instances until we
            //really need to
            let mut tail: Option<String> = Some(input.to_string());
            let mut head: Option<String> = Some(String::new());

            let mut matches = false;

            for instruction in self.instructions.iter() {
                let tailchars = if let Some(tail) = tail.as_ref() {
                    tail.chars().count()
                } else {
                    input.chars().count()
                };
                /*eprintln!("DEBUG: Instruction: {}", instruction);
                eprintln!("              Head: {}", head);
                eprintln!("              Tail: {}", tail);*/
                match instruction {
                    EditInstruction::Deletion(prefix) => {
                        match instruction_applies(prefix, input, &head, &tail, tailchars) {
                            Ok(matchchars) => {
                                matches = true;
                                if tail.is_none() { tail = Some(input.to_string()) }; //clone
                                tail.as_mut().map(|t| t.drain(..matchchars));
                            },
                            Err(e) => return Err(e)
                        }
                    },
                    EditInstruction::Insertion(s) => {
                        if head.is_none() { head = Some(String::new()) }; //init
                        head.as_mut().map(|h| *h += s);
                        matches = true;
                    },
                    EditInstruction::GenericIdentity(keeplength) => {
                        let keeplength = *keeplength as usize;
                        if keeplength > tailchars {
                            return Err(ApplyError::WithMessage(format!("Edit script does not match current word, length to keep is longer than head")));
                        }
                        if head.is_none() { head = Some(String::new()) }; //init
                        if tail.is_none() { tail = Some(input.to_string()) }; //clone
                        if let (Some(head), Some(tail)) = (head.as_mut(), tail.as_mut())  {
                            head.extend(tail.drain(..keeplength));
                            matches = true;
                        } else { panic!("Can't unpack head and tail for EditInstruction::GenericIdentity") } //should never happen
                    },
                    EditInstruction::Identity(prefix) => {
                        match instruction_applies(prefix, input, &head, &tail, tailchars) {
                            Ok(matchchars) => {
                                if head.is_none() { head = Some(String::new()) }; //init
                                if tail.is_none() { tail = Some(input.to_string()) }; //clone
                                if let (Some(head), Some(tail)) = (head.as_mut(), tail.as_mut())  {
                                    head.extend(tail.drain(..matchchars));
                                }
                                matches = true;
                            },
                            Err(e) => return Err(e)
                        }
                    },
                    EditInstruction::IdentityOptions(prefixes) => {
                        for prefix in prefixes {
                            if let Ok(matchchars) = instruction_applies(prefix, input, &head, &tail, tailchars) {
                                if head.is_none() { head = Some(String::new()) }; //init
                                if tail.is_none() { tail = Some(input.to_string()) }; //clone
                                if let (Some(head), Some(tail)) = (head.as_mut(), tail.as_mut())  {
                                    head.extend(tail.drain(..matchchars));
                                }
                                matches = true;
                                break;
                            }
                        }
                    }
                    EditInstruction::DeletionOptions(prefixes) => {
                        for prefix in prefixes {
                            if let Ok(matchchars) = instruction_applies(prefix, input, &head, &tail, tailchars) {
                                if tail.is_none() { tail = Some(input.to_string()) }; //clone
                                tail.as_mut().map(|t| t.drain(..matchchars));
                                matches = true;
                                break;
                            }
                        }
                    },
                    EditInstruction::InsertionOptions(_) => {
                        return Err(ApplyError::WithMessage(format!("Edit script has multiple insertion options and is therefor ambiguous, unable to apply")));
                    },
                }
            }
            if let Some(head) = head {
                Ok(head)
            } else if matches {
                Ok(String::new())
            } else {
                Err(ApplyError::NoMatch)
            }
        }
    }
}
