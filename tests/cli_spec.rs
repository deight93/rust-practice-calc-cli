use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn run_then_quit() {
    let mut cmd = Command::cargo_bin("rust-practice-calc-cli").unwrap();
    cmd.write_stdin("\n")
        .assert()
        .success()
        .stdout(contains("bye"));
}

#[test]
fn add_case() {
    let mut cmd = Command::cargo_bin("rust-practice-calc-cli").unwrap();
    cmd.write_stdin("3 + 5\n3+5\n\n")
        .assert()
        .success()
        .stdout(contains("8").count(2));
}

#[test]
fn bad_format() {
    let mut cmd = Command::cargo_bin("rust-practice-calc-cli").unwrap();
    cmd.write_stdin("3 +\n\n")
        .assert()
        .success()
        .stderr(contains("형식 오류 입니다. 예: 3 + 5"));
}

#[test]
fn bad_op() {
    let mut cmd = Command::cargo_bin("rust-practice-calc-cli").unwrap();
    cmd.write_stdin("3 & 6\n\n")
        .assert()
        .success()
        .stderr(contains("지원하지 않는 연산자 &"));
}

#[test]
fn div_by_zero() {
    let mut cmd = Command::cargo_bin("rust-practice-calc-cli").unwrap();
    cmd.write_stdin("10 / 0\n\n")
        .assert()
        .success()
        .stderr(contains("0으로 나눌 수 없습니다."));
}
