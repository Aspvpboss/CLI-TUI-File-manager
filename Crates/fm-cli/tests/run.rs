use fm_cli::{cli_args::Args, cli_args::ArgFlags};
use fm_core::*;
use tempfile::tempdir;


#[test]
fn invalid_command(){
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

    assert!(result.is_err());
}

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
    assert!(result.is_ok());
}

#[test]
fn creating_dir(){
    // absolutely ripped from your tests :)  
    let dir = tempdir().unwrap();
    let p = dir.path().join("a");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };

    let args = Args{
        command: Some("newf".to_string()),
        arg_one: Some(p.to_string()),
        arg_two: None,
        flags: vec![],
    };
    let result = fm_cli::run(args);
    assert!(result.is_ok());
}

#[test]
fn deleting_file(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };

    fm_core::create_file(p).unwrap();

    let args = Args{
        command: Some("del".to_string()),
        arg_one: Some(p.to_string()),
        arg_two: None,
        flags: vec![],
    };
    let result = fm_cli::run(args);
    assert!(result.is_ok());
}

#[test]
fn deleting_dir(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };  

    fm_core::create_dir(p).unwrap();

    let args = Args{
        command: Some("delf".to_string()),
        arg_one: Some(p.to_string()),
        arg_two: None,
        flags: vec![],
    };

    let result = fm_cli::run(args);
    assert!(result.is_ok()); 
}

#[test]
fn deleting_dir_with_recursion(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };  

    fm_core::create_dir(p).unwrap();
    fm_core::create_file(&format!("{p}/file.txt")).unwrap();

    let args = Args{
        command: Some("delf".to_string()),
        arg_one: Some(p.to_string()),
        arg_two: None,
        flags: vec![ArgFlags::USE_RECURSION],
    };

    let result = fm_cli::run(args);
    assert!(result.is_ok()); 
}

#[test]
fn renaming_file(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };  

    fm_core::create_file(p).unwrap();

    let args = Args{
        command: Some("rename".to_string()),
        arg_one: Some(p.to_string()),
        arg_two: Some(format!("{p}2")),
        flags: vec![],
    };

    let result = fm_cli::run(args);
    assert!(result.is_ok()); 
}


#[test]
fn renaming_dir(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };  

    fm_core::create_dir(p).unwrap();

    let args = Args{
        command: Some("rename".to_string()),
        arg_one: Some(p.to_string()),
        arg_two: Some(format!("{p}2")),
        flags: vec![],
    };

    let result = fm_cli::run(args);
    assert!(result.is_ok()); 
}