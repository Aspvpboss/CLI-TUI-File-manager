
use pfad_cli::cli_args::{Args, Commands};
use tempfile::tempdir;

#[test]
fn creating_file(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };

    println!("{p}");

    let args = Args{
        command: Commands::New { file_path: p.to_string() },
        flags: vec![],
    };
    let result = pfad_cli::run(args);
    assert!(result.is_ok());
}

#[test]
fn creating_dir(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };

    let args = Args{
        command: Commands::Newd { dir_path: p.to_string() },
        flags: vec![],
    };
    let result = pfad_cli::run(args);
    assert!(result.is_ok());
}

#[test]
fn deleting_file(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };

    pfad_core::create_file(p).unwrap();

    let args = Args{
        command: Commands::Del { file_path: p.to_string() },
        flags: vec![],
    };
    let result = pfad_cli::run(args);
    assert!(result.is_ok());
}

#[test]
fn deleting_dir(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };  

    pfad_core::create_dir(p).unwrap();

    let args = Args{
        command: Commands::Deld { dir_path: p.to_string(), recursion: false },
        flags: vec![],
    };

    let result = pfad_cli::run(args);
    assert!(result.is_ok()); 
}

#[test]
fn deleting_dir_with_recursion(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };  

    pfad_core::create_dir(p).unwrap();
    pfad_core::create_file(&format!("{p}/file.txt")).unwrap();

    let args = Args{
        command: Commands::Deld { dir_path: p.to_string(), recursion: true },
        flags: vec![],
    };

    let result = pfad_cli::run(args);
    assert!(result.is_ok()); 
}

#[test]
fn renaming_file(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };  

    pfad_core::create_file(p).unwrap();

    let args = Args{
        command: Commands::Rename {
            file_path: p.to_string(),
            new_file_name: "burger.txt".to_string()
        },
        flags: vec![],
    };

    let result = pfad_cli::run(args);
    println!("{:?}", result);
    assert!(result.is_ok()); 
}

#[test]
fn renaming_dir(){
    let dir = tempdir().unwrap();
    let p = dir.path().join("a");
    let Some(p) = p.to_str() else {
        panic!("I hope this doesn't happen");
    };  

    pfad_core::create_dir(p).unwrap();

    let args = Args{
        command: Commands::Rename {
            file_path: p.to_string(),
            new_file_name: "b".to_string()
        },
        flags: vec![],
    };

    let result = pfad_cli::run(args);
    assert!(result.is_ok()); 
}