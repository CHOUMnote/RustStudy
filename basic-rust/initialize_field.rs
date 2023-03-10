/**
 * 변수 초기화
 * rust는 NULL이 없기에 초기화 되지않은 변수를 선언할 수 없다.
 * 하지만 똑똑한 러스트는 컴파일 타임에 알 수 있기에 조금 다른 방식으로 사용할 수 있다.
 * 1. 변수 선언시 타입 지정.
 * 2. 그리고 이 변수는 반드시 초기화 한다.
*/

fn main() {
    //let t; 초기화x -> 에러
    let t:i32; //임시 선언(타입지정)
    //println!("{t}");  초기화x -> 사용 불가능
    t = 10;
    println!("{t}");    //초기화 -> 사용가능

    let tt:String;  //조금 다른 초기화 방법
    {
        tt = String::from("Hello rust!");
    }
    println!("{tt}");

}