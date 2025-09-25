# rust-practice-calc-cli

Rust 공부 프로젝트 CLI 사칙연산 계산기

Rust의 소유권, 문자열 처리, `Result`/`Option` 에러 처리, `match` 패턴 매칭 등을
학습하기 위해 작성되었습니다.

## 실행 방법

```bash
# 빌드 & 실행
cargo run
````

프로그램이 실행되면 다음과 같이 수식을 입력할 수 있습니다:

```
간단 계산기. 종료시 빈 줄 입력 후 Enter:
> 3 + 5
= 8
> 10 / 0
에러: 0으로 나눌 수 없습니다.
> 
bye
```

## 테스트

```bash
cargo test
```

## 학습 포인트

* `String` vs `&str` 변환
* `Result`/`Option` 기반 에러 처리
* `match` 분기 처리
