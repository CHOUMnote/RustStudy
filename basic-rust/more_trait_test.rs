/**
 * 라이브러리 trait
 * # 어노테이션은 보통 많이 사용하는 것들에 대해서 자동 구현 되어있다.
 * display, operator 등등 제시는 되어있지만 직접 구현해야 한다.
 */

 use std::fmt;

 struct Cat{
    name:String,
    age:u8,
 }
 impl fmt::Display for Cat{
    //Display trait를 구현해야한다.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
       write!(f, "name : {}\n age : {}", self.name, self.age)
    }
 }
 
 fn main(){
    let cat = Cat{ name:"냥".to_string(), age:24};
    //println!("{cat:?}"); Debug 미구현
    println!("{cat}"); //Display trait 보유
 }
 
 