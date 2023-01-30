/**
 * match
 * OOP의 if도 지원하지만 이보다 더 강력한 match
 * 입력되는 값의 모든 경우 조건 분기를 검사한다.
 * 리턴이 있다면 반드시 모든 분기의 리턴은 갖아야 한다.
*/

fn main() {
    let a = 100;
    let b =
        match a {
            10 => 20,
            20 => 30,
            100 => 200,
            _ => -1 //남은 모든 경우를 처리한다. 없다면 에러 - 모든 경우를 처리 하지 않았으므로 에러
            //_가 맨 위에있다면 바로 탈출
        };

    println!("{b}");

    let a = ("kim",24);
    match a {//구조체 비교
        ("kim",22) => println!("X"),
        ("lee",24) => println!("X"),
        _ => println!("안녕!")
    }

    let (r,g,b) = (10, 200, 40);
    match (r,g,b){//인자값은 순서대로 들어가고 임이의 이름으로 사용 가능하다.
        (a,_,_) if a==20 => println!("r!"),
        (_,b,_) if b==100 => println!("g!"),
        (_,_,c) if c==40 => println!("b!"),
        _ => println!("X")
    }

    let a = 30;
    match a {//범위 연산! - '@' 연산자
        0 => println!("정답"),
        a @ 1..=30 => println!("정답"),
        _ => println!("오답")
    }
}   