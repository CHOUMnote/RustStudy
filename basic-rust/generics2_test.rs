/**
 * 제네릭 심화
 * 여러 제네릭 타입과 제네릭 타입에 대한 정보를 컴파일러에게 알려준다.
 * - 제네릭 타입에 대한 trait를 선언
 * ex) Debug, Display, PartialOrd;
 * 
 * where -> 제네릭이 길어지면 복잡해진다. 가독성 증가를 위해 사용한다.
 */

 use std::fmt::Display;
 use std::cmp::PartialOrd;

 //여러 trait를 사용
 fn compare<T:Display, U:Display+PartialOrd>(a:T, num1:U, num2:U){
    println!("{} {} > {}? {}",a,num1,num2,num1>num2);
 }

 //where 문
 fn test<T, U>(a:T, num1:U, num2:U)
 where
    T:Display,
    U:Display+PartialOrd,
 {

    println!("{} {} > {}? {}",a,num1,num2,num1>num2);
 }


fn main() {
    compare("num1과 num2를 비교 : ".to_string(), 10, 30);
    test("num1과 num2를 비교 : ".to_string(), 10, -1);
}
