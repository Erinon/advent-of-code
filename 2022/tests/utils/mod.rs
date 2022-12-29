fn run_command<S, P>(name: S, file: P) -> (std::process::ExitStatus, Vec<u8>)
where
    S: AsRef<str>,
    P: AsRef<std::path::Path>,
{
    let output = assert_cmd::Command::cargo_bin(name)
        .unwrap()
        .pipe_stdin(file)
        .unwrap()
        .output()
        .unwrap();

    (output.status, output.stdout)
}

pub fn test_command_output<S, P>(name: S, file: P, expected: &str)
where
    S: AsRef<str>,
    P: AsRef<std::path::Path>,
{
    let (status, output) = run_command(name, file);

    let mut output = String::from_utf8(output).unwrap();
    output.pop();

    assert!(status.success());
    assert_eq!(output, expected);
}
