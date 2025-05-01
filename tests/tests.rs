use std::fs::read_to_string;

use pretty_assertions::assert_eq;
use glob::glob;

use schemars_foxy::Walker;

fn run_test(path: &str) {
    let mut walker = Walker::default();

    let pattern = path.to_owned() + "/**/*.json";
    for i in glob(pattern.as_str()).unwrap() {
        let i = i.unwrap();
        walker.run_parse(i);
    }

    let result = format!("{}", walker);
    assert_eq!(
        read_to_string(path.to_owned() + ".rs").unwrap(),
        result
    );
}

#[test]
fn cls_schemas() {
    run_test("tests/ClsSchemas");
}

#[test]
fn despatch_schemas() {
    run_test("tests/DespatchSchemas");
}

#[ignore]
#[test]
fn own_schemas() {
    run_test("tests/OwnSchemas");
}
