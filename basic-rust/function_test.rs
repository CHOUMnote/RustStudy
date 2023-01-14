/**
 * rust 함수
 * fn function_name(T a,...) -> T{ ... }
 * rust에는 NULL이 없다 따라서 무조건 리턴을 해야한다.
 * ->(리턴문) 생략시 empty tuple 리턴
 * return 키워드는 사용해도 되지만 생략해도 됨 단 ';'는 쓰지 않음
 * 함수 선언은 main 위 아래 어디든 상관 없음
*/

fn function1(){
    //empty tuple 리턴
}

fn main() {
    let a = function1();
    println!("{:?}",a);
    println!("{}",function2(10));
    
    return ()//empty tuple 반환! 
}

fn function2(a:i32) -> i32{
    let a = a*a;
    a   //return 생략 + ; 없음
}