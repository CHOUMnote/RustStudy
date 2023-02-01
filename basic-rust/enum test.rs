/**
 * enum
 * 다른 언어의 enum과 다르게 상수가 아닌 구조체도 선언 가능하다!
 * 보통 enum은 0,1,2... 처럼 상수로 취급하지만 c++의 enum class처럼 상수 취급을 하지 않는다.
*/

fn main(){
    let red = Color::Red;
    println!("{red:?}");
    //if red == 0{};    error
    if red as u8 == 0{  println!("red(int) is equal 0");};
    println!();

    let t = temp(Color::Blue);
    println!("color is {t}");
    println!();

    println!("{}",number::three as i32); 
    println!();

    let a = data::C(false);
    match a{    //구조체를 같는 enum과 match
        data::A(a,b) => println!("a is {a},{b}"),
        data::B => println!("a is single number"),
        data::C(a) => println!("a is single {a}"),
    }
}

///enum을 이용한 match
fn temp(other:Color) -> String{
    match other{
        Color::Red => "Red".to_owned(),
        Color::Green => "Red".to_owned(),
        Color::Blue => "Blue".to_owned(),
    }
}

#[derive(Debug)]
enum Color{
    Red, Green, Blue,   //0, 1, 2...
}

///다른 언어의 enum과 같이 three는 201이다
enum number{
    one = 100,
    two = 200,
    three,
}

///여러가지 데이터 구조를 같는 enum
enum data{
    A(i32,i32),
    B,
    C(bool)
}