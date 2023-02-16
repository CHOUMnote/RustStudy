/**
 * blanket trait
 * 뭔가를 감싸는 trait 라는 뜻으로 js의 prototype?
 * bound된 모든 것들은 이 trait를 사용할 수 있게 된다.
 * 말로는 어려우니 밑 예제를 보자
 */

 use std::fmt::*;
 #[derive(Debug)]
 struct person{name:String}
 
 ///1. 모든 대상에게 적용
 trait print {
    fn print(&self) {
        println!("print");
    }
 }
 impl<T> print for T{}   //T는 trait bound가 없기에 모든 대상
 
 ///2. 특정 trait가 포함된 대상에게 적용
 trait display{
    fn display(){
       println!("display");
    }
 }
 impl<T:print+Debug> display for T{
 
 
 }
 fn main(){
    println!("case 1");
    String::new().print();
 
    println!("\ncase 2");
    String::display();
    person::display();
 }