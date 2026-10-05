use std::assert_matches;

use fm_cli::{cli_args::Args, run};
use fm_core::*;
use tempfile::tempdir;


#[test]
fn creating_file(){
    // absolutely ripped from your tests :)  
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };

    println!("{p}");

    let args = Args{
        command: Some("new".to_string()),
        arg_one: Some(p.to_string()),
        arg_two: None,
        flags: vec![],
    };
    let result = fm_cli::run(args);

    assert!(result.is_none());
}

#[test]
fn invalid_command(){
    // absolutely ripped from your tests :)  
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };

    let args = Args{
        command: Some("some random thing".to_string()),
        arg_one: Some(p.to_string()),
        arg_two: None,
        flags: vec![],
    };
    let result = fm_cli::run(args);

    assert!(result.is_some());
}