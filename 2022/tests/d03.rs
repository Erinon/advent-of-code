mod utils;

#[test]
fn d03p1() {
    utils::test_command_output("d03p1", "tests/input/d03.txt", "157");
}

#[test]
fn d03p2() {
    utils::test_command_output("d03p2", "tests/input/d03.txt", "70");
}
