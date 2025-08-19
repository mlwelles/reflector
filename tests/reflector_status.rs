use assert_cmd::Command;

fn this() -> Command {
    Command::cargo_bin("reflector_status").unwrap()
}

#[test]
#[ignore] // since the default mode requires local storage, which is not yet accompocated for...
fn noargs() {
    this().assert().success().stderr("");
}
