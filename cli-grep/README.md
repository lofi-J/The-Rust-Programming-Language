# I/O 프로젝트 : 커맨드 라인 프로그램 만들기

고전적인 커맨드 라인 검색 도구인 grep(global search a regular expresstion and print)을 직접 구현한 버전

## 작성하며 알게된 내용

1. std::env::args 함수는 유효하지 않은 유니코드를 인수로 받은 경우 panic을 일으킨다.
   만일 프로그램이 유효하지 않은 유니코드를 포함하는 인수들을 받을 필요가 있다면 std::env::args_os를 사용해야한다.

2. std::env::args 함수는 순회 가능한 이터레이터를 반환한다.
   첫 번째 요소로 컴파일된 바이너리 파일의 이름이 온다. (target/debug/cli-grep)

3. 바이너리 프로젝트에 대한 관심사 분리
   프로그램을 main.rs와 lib.rs로 분리하고 프로그램 로직을 lib.rs로 옮기기

4. 프로그램에서 panic!은 유저의 사용의 문제보다는 프로그램 자체의 문제에 더 적합하다. (예상치 못한 panic!보다는 Result를 통해 핸들링을 해주는것이 좋다)

5. 트레이트 객체 `Box<dyn Error>`
   `Box<dyn Error>`는 해당 함수가 Error트레이트를 구현한 어떤 타입임을 의미함을 알림
