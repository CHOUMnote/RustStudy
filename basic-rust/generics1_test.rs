/**
 * 제네릭
 * 템플릿 프로그래밍
 * 다른 언어와 비슷하다.
 */

 use std::fmt::Debug;

 //제네릭 클래스
 #[derive(Debug)]
struct Point<T1, T2>{
    x:T1,
    y:T2,
}


//몇몇 기능은 컴파일러에게 알려줘야한다. - 제네릭타입은 자체로는 안전하지 않기 때문이다.
fn test<T:Debug>(a:T) -> T{ //T는 Debug가 있다는걸 컴파일러에 알려준다. 없다면 에러다
    println!("this is {a:?}");
    a
}

fn main() {
    let point1 = Point{x:"AA".to_owned(), y:20};
    let point2 = Point{x:30, y:"BB".to_owned()};
    println!("1번 객체 : {point1:?}");
    println!("2번 객체 : {point2:?}");

    println!();

    test("문자열!".to_owned());
    test(100);
    test(&point1);  //Debug가 있다. 없다면?
}
