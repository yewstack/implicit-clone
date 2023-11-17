#[allow(dead_code)]
#[test]
fn tests_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/function_component_attr/*-pass.rs");
}

#[allow(dead_code)]
#[rustversion::attr(stable(1.64), test)]
fn tests_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/function_component_attr/*-fail.rs");
}
