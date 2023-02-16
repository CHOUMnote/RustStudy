/**
 * trait
 * 다른 언어의 인터페이스의 역할(약간 다르지만 비슷하다)
 * 공통 메소드 원형을 제시하고 trait를 받는 구조체의 구현부에서 구현한다
 * default 메소드도 가능하다. 재선언을 할 수도 있다.
 * '#' 어노테이션 또는 impl [trait] for [T]로 구현한다.
 */

 fn main(){
    let dog = Dog{};
    let cat = Cat{};
 
    Animal::print(&dog);
    dog.sound();
    cat.print();
    cat.sound();
 }
 
 trait Animal {
    //디폴트 메소드
    fn print(&self){
       println!("i am animal");
    }
    //공통 메서드
    fn sound(&self);
 }  
 
 //# 어노테이션 - trait 선언
 #[derive(Debug)]
 struct Dog{}
 #[derive(Debug)]
 struct Cat{}
 
 impl Animal for Dog{
    fn sound(&self){
       println!("멍멍");
    }
 }
 impl Animal for Cat{
    fn print(&self){  //기본 메서드 재정의
       println!("i am Cat")
    }
    fn sound(&self){
       println!("야옹");
    }
 }