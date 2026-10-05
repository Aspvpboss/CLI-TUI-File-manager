use std::assert_matches;

use fm_cli::{cli_args::Args, run};
use fm_core::*;


#[test]
fn creating_file(){
    
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");

    let args = Args{
        command: Some("new".to_string()),
        arg_one: Some("test.txt".to_string()),
        arg_two: None,
        flags: vec![],
    };
    let result = fm_cli::run(args);

    assert_matches!(result, None);
}