use std::{self, mem}; //c++ using, c# use

/**
 * -print 매크로 함수 알아보기
 * cli출력을 위해서 rust에서 제공하는 메크로 함수 여러 기능 알아보기
 * 1. 대부분 파이썬과 유사하다. f-string, r-string...
 * ex) ("{1} {0} {2}",~~,~~,~~), r"~~~"
 * 2. 파이썬과 유사하게 출력 조작이 가능하다.
 * 3. {:?}로 디버그 출력이 가능하다(정말 유용하다) - 파이썬은 기본적으로 __str__()을 호출해 버리거나 toString()을 출력하지만 디버그 출력으로 더욱 편하고, 원하는 데이터인지를 판별 할 수 있다.
 */
fn main() {
    let name = String::from("김지호");
    let num = 24;

    show_title("기본 출력");
    basic_print_macro(&name, num);
    show_title("출력 조작");
    basic_print_manip(&name, num);
    show_title("디버그 프린트");
    _basic_print_debug(&name, num);
}

///기본 출력 형식
fn basic_print_macro(a: &String, b: i32) {
    println!("{} {} {}", "이름과 나이", a, b * 2); //기본 - 순서대로 입력된다.
    println!("{1} {2} {0}", "이름과 나이", a, b); //인덱스 - 인자가 인덱스에 따라 입력된다..
    println!("{string} {a} {b}", string = "이름과 나이"); //naming - 변수를 입력한다.

    let script = "안녕하세요~
    저는
김지호입니다";
    println!("{}", script); //여러줄 출력 - \t 주의!
}

///출력 형식 조정
fn basic_print_manip(a: &String, b: i32) {
    println!("주소 : {:p}",a);   //주소
    println!("16진수 : {:x}",b*2014);   //16진수 {:X},{:O} -> 대문자 출력
    println!("8진수 : {:o}",b);   //8진수
    println!("2진수 : {:b}",b);   //2진수
}

///디버그 프린트
fn _basic_print_debug(a: &String, b: i32) {
    println!("{:?}", a);   //:? -> 디버그 프린트
    println!("{:?}", b);      
    let arr:[i8;10] = [7;10];
    println!("{:?}", arr);
    println!("{:?}", a.chars());
}

fn show_title(a:&str){
    println!();
    println!("{:-^20}",a);  //20칸 '-'로 채우고 인자값 가운데 정렬
}
