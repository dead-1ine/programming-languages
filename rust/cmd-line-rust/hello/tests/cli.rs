use std::process::Command;

#[test] //  러스트가 이 함수를 테스트 목적으로 실행하도록 지시한다. 
fn runs() {
    let mut cmd = Command::new("hello");
    let res = cmd.output();
    assert!(res.is_ok());
}
