use assert_cmd::Command;

fn this() -> Command {
    Command::cargo_bin("reflector_status").unwrap()
}

#[test]
fn noargs() {
    this().assert().success().stderr("");
}
