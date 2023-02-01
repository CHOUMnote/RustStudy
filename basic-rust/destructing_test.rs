/**
 * destructing(unpacking)
 * 구조체 데이터를 변수로 받기
*/

fn main() {
    //일반적인 튜플 언팩킹
    let t = ("kim", 10, 30, true);
    let (a, b, c, d) = t; //destruct
    println!("{}, {}, {}, {}", a, b, c, d);
    println!();

    //구조체 데이터 접근
    let kim = Person {
        name: "kim".to_owned(),
        age: 24,
        grade: 2,
        male: true,
    };
    println!(
        "struct data : name:{}, age:{}, grade:{}, male:{}",
        kim.name, kim.age, kim.grade, kim.male
    );
    println!();

    //구조체 언팩킹
    let Person {
        //destructing kim(Person)
        name: a,
        age: b,
        grade: c,
        male: d,
    } = kim;
    println!("unpack data : name:{a}, age:{b}, grade:{c}, male:{d}");
    println!();

    //간결한 언팩킹 - 구조체의 변수 이름을 그대로 사용 한다면...
    /* let Person {
        name,
        age,
        grade,
        male,
    } = kim;
    println!("unpack data : name:{name}, age:{age}, grade:{grade}, male:{male}"); */
}

struct Person {
    name: String,
    age: i32,
    grade: i32,
    male: bool,
}
