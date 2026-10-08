
// Claude is responsible for most tests in this file

use fm_core::*;

// Entry

#[test]
fn entry_display_marks_directories_with_a_slash() {
    let file = Entry {
        name: "a.txt".to_string(),
        size: 5,
        is_dir: false,
    };
    let sub = Entry {
        name: "sub".to_string(),
        size: 0,
        is_dir: true,
    };

    assert_eq!(file.to_string(), "a.txt - 5 bytes");
    assert_eq!(sub.to_string(), "sub/ - 0 bytes");
}
