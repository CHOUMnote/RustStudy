### Rust 컴파일과 실행

`rustc` 명령어로 rust 파일을 컴파일 한다.

```
$ rustc main.rs
$ ./main  
```



### Cargo 사용하기

Cargo는 rust 프로젝트 매니저이자 빌드 도구이다. 그리고 많은 기능을 제공한다.

npm, pip와 비슷하다고 볼 수 있다.

```
$ cargo new {프로젝트}
$ cd {프로젝트}
```

`cargo new` 명령어로 새로운 rust 프로젝트를 생성할 수 있다.

이때 `Cargo.toml`  파일이 생성된다. 이 파일은 nodejs의 package.json 처럼 종속 패키지 목록을 관리하며 rust 프로젝트의 메타데이터를 갖고 있다.



```
[package]
name = "main"
version = "0.1.0"
edition = "2021"

[dependencies]
~~~
~~~
~~~
```

[package] 는 rust 프로젝트의 메타데이터를 포함한다.

[dependencies] 는 rust 프로젝트에 사용할 패키지들 목록이다.

이때 다운받은 패키지들은 `Cargo.lock` 에 있다. 다음 빌드시 이를 참조하여 있는거는 스킵해서 빠르게 빌드 한다.





```
$ cargo run
$ cargo run arg1 arg2
$ cargo run --arg1 -- arg2
```

`cargo run` 은 rust프로젝트 빌드+실행을 한다.

`cargo run arg1 arg2` 은 프로그램에 인자가 필요할 때 사용하게 된다. 

`cargo run --arg1 -- arg2` 은 `--` 을 기준으로 앞에는(`--arg1`) cargo 인자  뒤에는(`arg2`)는 프로그램의 인자이다.



### Cargo 명령어

```
$ cargo check
```

이 명령어는 빌드를 하지 않고 컴파일이 되는지를 체크한다. rust 컴파일러는 매우 똑똑하기 때문에 컴파일 단계에 많은 에러를 찾을 수 있기때문에 매우 유용한 명령어이다.



```
$ cargo update
$ cargo upgrade
```

이 명령어는 패키지를 최신버전으로 유지 한다.

update : Cargo.lock을 최신화

upgrade : 패키지 최신화



```
$ cargo doc --open
```

이 명령어는 프로젝트의 함수들을 문서화 시킨다. 이 문서를 통해 패키지 함수들을 쉽게 살펴 볼 수 있다.

`--open` 인자로 브라우저에서 열어 볼 수 있다.



```
$ cargo clean
```

이 명령어는 프로젝트에 target폴더를 날려버린다!

build, doc등 다양한걸 하다보면 target폴더에 많은양의 데이터가 누적된다. 이걸로 날려 버릴 수 있다!



```
cargo install {패키지}
```

[crate](https://crates.io/)를 설치 할 수 있따.



### 좋은 툴

`rustfmt`, `clippy` 가 쩌는 편의성을 제공한다.



`rustfmt` 는 자동 포맷팅 툴이다.

```
rustup component add rustfmt
```

설치

```
cargo fmt
```

코드를 포맷팅



`clippy`는 린트테스트 도구이다.(코드 분석기)

[자세한 내용](https://github.com/rust-lang/rust-clippy)

