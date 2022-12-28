mod utils;

#[test]
fn d01p1() {
    utils::test_command_output("d01p1", "tests/input/d01.txt", "24000");
}

#[test]
fn d01p2() {
    utils::test_command_output("d01p2", "tests/input/d01.txt", "45000");
}
