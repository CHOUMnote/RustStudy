/**
 * trait bound, impl 문법
 * 함수나 파라미터에 trait를 전달하는 방식
 * trait를 통해 다형성 구축
 * c++에서 정적 다형성을 하기엔 난해하다 하지만 rust에서 정적 다형성은 간단하다.
 */

 fn main(){
    let dog = dog_return(); //다형성
    let cat = Cat{};
 
    dog.print();
    intro1(&dog);
    cat.print();
    intro2(&cat);
 }
 
 //trait bound 문법
 fn intro1<T:Animal>(a:&T){
    a.sound();
 }
 
 fn intro2(a:&impl Animal){
    a.sound();
 }
 
 //trait 반환
 /**
    ```c++
       Animal *t = new Dog;
    ```
 */
 fn dog_return() -> impl Animal {
    Dog{}
 }
 trait Animal {
    fn print(&self){
       println!("i am animal");
    }
    fn sound(&self);
 }  
 
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
    fn print(&self){ 
       println!("i am Cat")
    }
    fn sound(&self){
       println!("야옹");
    }
 }