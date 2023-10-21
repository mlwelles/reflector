use assert_cmd::Command;

fn this() -> Command {
    Command::cargo_bin("reflector_status").unwrap()
}

#[test]
fn noargs() {
    let mut c = this();
    c.assert().success().stderr("");
}
