# 공식문서를 읽으며 얻은 유용할것 같은 내용들

`cargo doc --open` : 위 명령어는 Cargo.toml에서 의존하는 크레이트들의 문서를 로컬에서 일괄 빌드해, 브라우저에서 각 크레이트들의 사용법을 담은 문서를 탐색할 수 있게해준다. (생성된 doc폴더는 target경로에 생성된다)

**문자열 길이**

```rust
  let str = "안녕하세요.";
  let string = String::from("안녕하세요.");

  let str_length = str.chars().count();
  let string_length = string.chars().count();
```
