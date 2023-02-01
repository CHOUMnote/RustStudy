/**
 * 구조체
 * rust는 OOP가 아니다! 일련의 데이터 그룹을 위한 구조체
 * 구조체는 데이터만 갖는다.
 * 구조체의 연관된 함수는 밖에서 구현한다.
 * 선언은 다른 구조체와 비슷하게 선언한다.  
*/

fn main() {
    println!("nuit 구조체");    //데이터가 없는 구조체
    let t = Temp;
    println!("This is {t:?}");
    println!();

    let red:Color = Color("Red".to_owned(),255,0,0);    //튜플 구조체
    println!("튜플 구조체");
    println!("Color is {red:?}");
    println!();

    println!("일반적인 구조체");    //다른 언어와 비스무리 하다. 단 차이점을 기억하자!
    let name = "Kim".to_owned();
    let age = 24;
    //let Person = Person{name:name.clone(), age:age}; 
    //파라미터 이름이 같다면 다음과 같이 사용 가능
    let person = Person{name, age,}; 
    println!("This is {person:#?}");
    println!();
    
    //구조체 update
    //구조체를 복사하는데 일부만 복사하려는 경우...
    //.. syntax -> don`t care, .. T syntax -> T와 같이 처리한다
    println!("Struct Update");
    let person2 = Person{
        age:10,
        ..person    //person의 name은 소유권 이전으로 더이상 사용 불가능 하다!
    };
    println!("another is {person2:#?}");
}   

///예외... 유닛 구조체
#[derive(Debug)]
struct Temp;    //아무것도 없다는 뜻

///1. tuple struct
#[derive(Debug)]
struct Color(String,i32,i32,i32);   //하나의 튜플과 같다.

///2. named struct
#[derive(Debug)]
struct Person{  //열거식으로 선언한다.
    name:String,
    age:i32,
}