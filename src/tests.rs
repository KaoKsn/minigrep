use super::*;
use std::io::Write;

#[test]
fn read_fails_with_io_errors() {
    let filename = "3A4m5w2.3wc";
    assert!(read_content(filename).is_err())
}
#[test]
fn reads_when_a_valid_file_exists() {
    let mut tmpfile = tempfile::NamedTempFile::new().expect("Couldn't create a temporary file!");

    write!(tmpfile, "Hello, World").expect("Couldn't write to the temporary file");

    let contents = read_content(
        tmpfile
            .path()
            .to_str()
            .expect("Couldn't convert the tmpfile to a string."),
    )
    .expect("Failed to read the file.");

    assert_eq!(contents, "Hello, World");
}

#[test]
/// Test if search returns an empty vector when query isn't present.
fn test_query_absent() {
    let query = "absent";
    let contents = "\
Hey, there:
Dennis Ritchie created the C programming language";

    assert_eq!(Vec::<&str>::new(), search(query, contents));
}

#[test]
/// Test if search returns all the valid lines that contain the query string.
/// Also tests if case sensitive is preserved when insensitivity isn't asked.
fn test_multiple_results() {
    let query = "some";
    let contents = "\
Can I get some sugar?
Sure, you could get some, but not here. Sorry!
Some people never get tired of doing the same thing.
He created the C programming language";

    assert_eq!(
        vec![
            "Can I get some sugar?",
            "Sure, you could get some, but not here. Sorry!"
        ],
        search(query, contents)
    );
}

#[test]
fn test_highlight_case_insensitive() {
    let query = "age";
    let line = "He created the C programming language";

    assert_eq!(
        Some(format!(
            "He created the C programming langu{}",
            "age".bright_yellow()
        )),
        highlight(query, line, true)
    );
}

#[test]
fn test_highlight_case_sensitive() {
    let query = "rUst";
    let line = "\
Alright. Pizza CrUsT";

    assert_eq!(None, highlight(query, line, false));
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
Pizza CrUsT";

    assert_eq!(
        vec!["Rust:", "Trust me.", "Pizza CrUsT"],
        search_case_insensitive(query, contents)
    );
}
