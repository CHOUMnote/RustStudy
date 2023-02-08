/**
 * Result - Option의 친구
 * Result - Ok, Err
 * Option - Some, None
 * Result는 '?'연산자로 체이닝을 할수 있다.
 * Option과 같이 많은 함수에서 리턴값으로 사용한다.
 * Ok는 unwrap 가능 하지만 Err은 값이 있어보여도 unwrap 불가능 하다.
 * ex) enum Result<T,E> - T - is_ok(), E - is_err()
 */

 //Ok 또는 Err을 반환
 fn check_error(a:i32) -> Result<(),()>{
    if a%2==0{
       Ok(())
    }else{
       Err(())
    }
  }
 
 fn get_even(i:i32) -> Result<i32, String>{
    if i%2==0{
       Ok(i)
    }else{
       Err("nope".to_owned())
    }
 }
 
 //특별한 error를 같는 Result -
 fn parse_int(number:String) -> Result<i32, std::num::ParseIntError>{
    //몇 함수는 이미 정해진 에러를 내뱉는다.
    number.parse() // &str -> int
 }
 
 fn main() {
    let a = check_error(10);   //ok
    let b = check_error(21);   //err
 
    println!("a={:?}, a is {:?}", a, a.unwrap());
    println!("b = {:?}",b);
    //println!("b is {:?}",b.unwrap()); 패닉
    println!();
 
    let mut vec = Vec::new();
    for i in 0..11{
       vec.push(get_even(i));
    }
 
    for i in vec.iter(){
       //Result와 match - Ok거나 Err이다.
       match i{
          Ok(x) => println!("짝수 : {x}"),   //if i.is_ok()
          Err(_) => ()
       }
    }
 
    println!();
 
    let mut vec = Vec::new();
    vec.push(parse_int("1212".to_owned()));
    vec.push(parse_int("l2l2".to_owned()));
    vec.push(parse_int("1212".to_owned()));
 
    for i in vec{
       match i{
          Ok(x) => println!("it is {i:?}, number is {x}"),
          Err(x) => println!("Error! {x}"),
       }
    }
 }