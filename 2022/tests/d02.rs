mod utils;

#[test]
fn d02p1() {
    utils::test_command_output("d02p1", "tests/input/d02.txt", "15");
}

#[test]
fn d02p2() {
    utils::test_command_output("d02p2", "tests/input/d02.txt", "12");
}
