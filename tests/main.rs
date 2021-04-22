//#[macro_use]
//extern crate matches;

use sesdiff::*;
use std::str::FromStr;


#[test]
fn test0001_diff_normal() {

    assert_eq!(
        format!("{}",shortest_edit_script("hablaron", "hablar", false, false, false)),
        "=[hablar]-[on]"
    );

    assert_eq!(
        format!("{}",shortest_edit_script("pidieron", "pedir", false, false, false)),
        "=[p]-[i]+[e]=[di]-[eron]+[r]"
    );

    assert_eq!(
        format!("{}",shortest_edit_script("говорим", "говорить", false, false, false)),
        "=[говори]-[м]+[ть]"
    );
}

#[test]
fn test0002_diff_suffix() {

    assert_eq!(
        format!("{}",shortest_edit_script_suffix("hablaron", "hablar",  false, false)),
        "-[on]"
    );

    assert_eq!(
        format!("{}",shortest_edit_script_suffix("pidieron", "pedir",  false, false)),
        "-[eron]+[r]=[di]-[i]+[e]"
    );

    assert_eq!(
        format!("{}",shortest_edit_script_suffix("говорим", "говорить", false, false)),
        "-[м]+[ть]"
    );
}

#[test]
fn test0003_diff_generic() {
    assert_eq!(
        format!("{}",shortest_edit_script_suffix("pidieron", "pedir",  true, false)),
        "-[eron]+[r]=[#2]-[i]+[e]"
    );
}


#[test]
fn test0004_diff_components() {
    let editscript = shortest_edit_script("hablaron", "hablar", false, false, false);
    assert_eq!(editscript.len(), 2);
    assert_eq!(format!("{}",editscript.instructions.get(0).unwrap()),"=[hablar]");
    assert_eq!(format!("{}",editscript.instructions.get(1).unwrap()),"-[on]");
    assert_eq!(editscript.instructions.get(0).unwrap().is_change(),false);
    assert_eq!(editscript.instructions.get(1).unwrap().is_change(),true);
}

#[test]
fn test0005_parse() {
    let editscript = EditScript::from_str("+[ver]=[sta]-[a]=[n]+[d]").unwrap();
    assert_eq!(format!("{}",editscript.instructions.get(0).unwrap()),"+[ver]");
    assert_eq!(format!("{}",editscript.instructions.get(1).unwrap()),"=[sta]");
    assert_eq!(format!("{}",editscript.instructions.get(2).unwrap()),"-[a]");
    assert_eq!(format!("{}",editscript.instructions.get(3).unwrap()),"=[n]");
    assert_eq!(format!("{}",editscript.instructions.get(4).unwrap()),"+[d]");
}

#[test]
fn test0006_apply() {
    let editscript: EditScript<String> = EditScript::from_str("+[ver]=[sta]-[a]=[n]+[d]").unwrap();
    assert_eq!(format!("{}",editscript.apply_to("staan", None).unwrap() ),"verstand");
}

#[test]
fn test0007_apply() {
    let editscript: EditScript<String> = EditScript::from_str("-[ver]=[sta]+[a]=[n]-[d]").unwrap();
    assert_eq!(format!("{}",editscript.apply_to("verstand", None).unwrap() ),"staan");
}

#[test]
fn test0008_apply() {
    let editscript: EditScript<String> = EditScript::from_str("=[p]-[i]+[e]=[di]-[eron]+[r]").unwrap();
    assert_eq!(format!("{}",editscript.apply_to("pidieron", None).unwrap() ),"pedir");
}

#[test]
fn test0009_apply_suffix() {
    let editscript: EditScript<String> = EditScript::from_str("-[on]").unwrap();
    assert_eq!(format!("{}",editscript.apply_to("hablaron", Some(Mode::Suffix)).unwrap() ),"hablar");
}

#[test]
fn test0010_apply_suffix2() {
    let editscript: EditScript<String> = EditScript::from_str("-[eron]+[r]=[#2]-[i]+[e]").unwrap();
    assert_eq!(format!("{}",editscript.apply_to("pidieron", Some(Mode::Suffix)).unwrap() ),"pedir");
}

#[test]
fn test0011_noapply() {
    let editscript: EditScript<String> = EditScript::from_str("-[ver]=[sta]+[a]=[n]-[d]").unwrap();
    assert!(editscript.apply_to("nachtvlinder", None).is_err() );
}

#[test]
fn test0007_apply_infix() {
    let editscript: EditScript<String> = EditScript::from_str("-[a]+[e]").unwrap();
    assert_eq!(format!("{}",editscript.apply_to("hahaha", Some(Mode::Infix)).unwrap() ),"hehehe");
}

#[test]
fn test0008_apply_infix_context() {
    let editscript: EditScript<String> = EditScript::from_str("=[l]-[a]+[o]").unwrap();
    assert_eq!(format!("{}",editscript.apply_to("halaha", Some(Mode::Infix)).unwrap() ),"haloha");
}

