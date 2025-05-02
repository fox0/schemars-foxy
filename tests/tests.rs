use std::fs::read_to_string;

use glob::glob;
use pretty_assertions::assert_eq;

use schemars_foxy::Walker;

fn run_test(path: &str) {
    let mut walker = Walker::default();

    let pattern = path.to_owned() + "/**/*.json";
    for i in glob(pattern.as_str()).unwrap() {
        let i = i.unwrap();
        walker.parse(i);
    }

    let result = format!("{}", walker);
    assert_eq!(read_to_string(path.to_owned() + ".rs").unwrap(), result);
}

#[allow(dead_code)]
fn init() {
    colog::default_builder()
        .filter(None, log::LevelFilter::Trace)
        .default_format()
        .format_source_path(true)
        .init();
}

#[test]
fn cls_schemas() {
    // init();
    run_test("tests/ClsSchemas");
}

#[test]
fn despatch_schemas() {
    // init();
    run_test("tests/DespatchSchemas");
}

#[ignore]
#[test]
fn own_schemas() {
    init();
    run_test("tests/OwnSchemas");
}
