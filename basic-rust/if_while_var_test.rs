/**
 * if let, while let
 * rust에 대입 연산자는 단순 대입이 아니라 가능한지에 따라 연산이 달라진다.
 * if let - 조건에 따른 대입
 * while let - 반복에 따른 대입
 * ex) if let Some(x) = fnc(){...}
 * 해석해 본다면 만약 fnc()가 Some()이라면... -> 조건 실행
 */

 fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    //.get() -> Option<T> - 다른 언어와 같이 오류를 내뱉지 않고 처리 가능하게 리턴값을 반환 한다.
    //있다면 Some(x) 없다면 None
    let some1 = vec.get(3);
    let some2 = vec.get(5);
    println!("4번째 원소:{:?}, 5번째 원소:{:?}", some1, some2);

    //일반 Option과 match
    println!("\n===일반 match===");
    for i in 0..10 {
        match vec.get(i) {
            Some(x) => println!("{i}번째 원소 : {x}"),
            None => println!("잘못된 접근..."),
        }
    }

    //if let 문법
    println!("\n===if let match===");
    for i in 0..10 {
        if let Some(x) = vec.get(i) {
            println!("{i}번째 원소 : {x}");
        }
    }

    let vec = [
        vec!["한글", "영어", "123", "-7", "일본어", "89"],
        vec!["대한민국", "미국", "123", "-7", "일본", "89"],
    ];

    //while let match
    println!("\n===while let===");
    for mut i in vec {
        //i에서 pop이 가능하면 반복 - pop()의 명세를 보면 Option 반환
        while let Some(x) = i.pop() {
            if let Ok(x) = x.parse::<i32>() {
                //x가
                println!("str->number : {}", x)
            }
        }
    }
}
