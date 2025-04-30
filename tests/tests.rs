use pretty_assertions::assert_eq;
use std::fs::{read_dir, read_to_string};

use schemars_foxy::Walker;

fn run_test(path: &str) {
    let paths = read_dir(path).unwrap();

    let mut v: Vec<_> = paths
        .map(|o| o.unwrap())
        .filter(|a| a.file_name().into_string().unwrap().ends_with(".json"))
        .collect();
    v.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let mut walker = Walker::default();

    for i in v {
        walker.run_parse(i.path());
    }

    let result = format!("{}", walker);
    assert_eq!(
        read_to_string(path.to_owned() + "/_models.rs").unwrap(),
        result
    );
}

#[test]
fn schemas1() {
    run_test("tests/schemas1");
}
