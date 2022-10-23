#![allow(unused)]

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("aaa bbb\nccc ddd eee", "bbb", &mut result);
    assert_eq!(result, b"aaa bbb\n");
}
