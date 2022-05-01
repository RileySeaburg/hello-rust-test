use assert_cmd::Command;
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
	// Assert success 
    cmd.assert().success();
}
