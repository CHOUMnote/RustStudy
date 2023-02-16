/**
 * from trait
 * From<T>에 해당하는 trait의 from을 구현 하여 from(..)을 사용할 수 있다.
 * From trait가 구현 되어있다면 .into() 를 사용할 수 있다(묵시적 자동 컴파일)
 * 하지만 into를 구현한다고 From을 사용하는 것은 아니다.
 */

 use std::fmt;
 struct Cat{
    name:String,
    age:u8,
 }
 impl From<(String,u8)> for Cat{
    fn from(value: (String,u8)) -> Self {
       Cat { name: value.0, age: value.1 }
    }
 }
 impl From<&Cat> for Cat{
    fn from(value: &Cat) -> Self {
       Cat { name: value.name.clone(), age: value.age }
    }
 }
 impl fmt::Display for Cat{
    //Display trait를 구현해야한다.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
       write!(f, "name : {}\n age : {}", self.name, self.age)
    }
 }
 fn main(){
    let a = Cat::from(("냥".to_owned(), 30));
    let b = Cat::from(&a);
    let c:Cat = (&a).into();   //into는 컴파일 시간에 자료형을 알 수 없다. 따라서 명시적 선언이 필요하다.
 
    println!("{a}");
    println!("{b}");
    println!("{c}");
 }