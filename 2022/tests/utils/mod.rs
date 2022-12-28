fn run_command<S, P>(name: S, file: P) -> (std::process::ExitStatus, String)
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

    (output.status, String::from_utf8(output.stdout).unwrap())
}

pub fn test_command_output<S, P>(name: S, file: P, expected: &str)
where
    S: AsRef<str>,
    P: AsRef<std::path::Path>,
{
    let (status, mut output) = run_command(name, file);

    output.pop();

    assert!(status.success());
    assert_eq!(output, expected);
}
