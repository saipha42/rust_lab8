use assert_cmd::Command;



#[test]
fn test_4_file() {

    let mut cmd = Command::cargo_bin("pb4_1").unwrap();
    cmd.args(&["test.csv", "output4_1.csv"]);

    cmd.assert().success();

}


#[test]
fn test_4_stdin() {

    let mut cmd = Command::cargo_bin("pb4_1").unwrap();
    cmd.write_stdin("1,2,green\n3,4,black");

    cmd.assert().stdout("1,2,green\n3,4,black\n");

}