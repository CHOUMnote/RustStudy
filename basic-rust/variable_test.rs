/**
 * 변수!    
 * rust는 변수 타입을 자동으로 추론한다!(c++:auto) 컴파일러는 i32, f64를 기본으로 정의 한다.
 * 기본적으로 모든 변수는 불변성(Immutable)을 갖는다. 하지만 mut키워드로 가변적(mutable)이게 선언한다.
 * 상수(not 'let') : 반드시 명시적 타입
 * rust에는 shadowing 기능이 있다.
 * shadowing? 한 스코프에서 동일한 let을 여러번 정의 가능하다.
 */
fn main() {
    let a = 10;
    let a = 100;
    {
        println!("{}",a);
    }
    //let b:i128 = 128;   //unused is warring
    let _c = 3.14; //unused var -> _var - it is safety
    {
        // this Shadowing
        let a = a + a;
        println!("{}", a);
    }
    println!("{}", a);

    //a = 1; it is error
    let mut _muteable_var = 0;
    {
        _muteable_var += 1000;
    }
    print!("{}", _muteable_var);

    const _PI: f32 = 3.14;
}
