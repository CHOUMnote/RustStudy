/**
 * impl block
 * struct, enum에 대한 실질적인 행동(method)를 구현한다.
 * 연관 함수 - static method
 * 메소드 - member method
 */

 fn main() {
    let mut kim = Person::new("jiho".to_owned(), 24, Sex::male); //연관 함수
    kim.set_name("Kimjiho"); //멤버 함수
    println!("i`m {kim:#?}");
    kim.sex.check(); //enum 구현 함수
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    sex: Sex,
}

///첫 인자가 self(self:&Self)일 경우 멤버 함수,아니라면 연관 함수
impl Person {
    //Person 구현...
    ///연관 함수
    fn new(name: String, age: i32, sex: Sex) -> Self {
        //또는 Person
        Person { name, age, sex }
    }

    fn set_name(&mut self, t: &str) {
        self.name = t.to_owned();
    }
}

#[derive(Debug)]
enum Sex {
    male,
    female,
}

impl Sex {
    //enum 구현
    fn check(&self) {
        use Sex;
        match self {
            male => println!("Sex is Male"),
            female => println!("Sex is Male"),
        }
    }
}