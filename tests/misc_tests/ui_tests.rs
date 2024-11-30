// HELP:
//
// use `cargo test -- ui trybuild=filter_here`
// to only  run UI tests

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    for dir in [
        "nlist_ui_tests",
    ] {
        t.compile_fail(format!("tests/misc_tests/{}/*err.rs", dir));
        t.pass(format!("tests/misc_tests/{}/*fine.rs", dir));
    }
}
