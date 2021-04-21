//#[macro_use]
//extern crate matches;

use sesdiff::*;


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

