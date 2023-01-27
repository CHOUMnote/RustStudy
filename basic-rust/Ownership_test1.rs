/**
 * 소유권
 * 기본적으로 함수 호출시 인자들은 소유권 전달이다.
 * 따라서 다음 호출시 소유권이 없으므로 사용하지 못한다.
 * 이러한 문제는 ref, 복사, 재리턴 등 방법들이 있다.
*/

fn main() {
    let name = "Kimjiho".to_owned();
    println!("{name}"); //정상 작동
    fnc(name);  //정상 작동 && 소유권 전달
    //fnc(name);  에러 - name의 소유권이 fnc로 넘어간다. copy가 아님!
    
    //소유권 문제 해결 방법!
    //방법 1
    println!("\n===방법 1===");
    let name = "Kimjiho".to_owned();
    fnc(name.clone()); //복사 후 전달로 정상 작동
    fnc(name.clone()); //복사 후 전달로 정상 작동
    fnc(name.clone()); //복사 후 전달로 정상 작동

    //방법 2
    println!("\n===방법 2===");
    let mut t = "Hello World!".to_owned();
    t = str_return(t);
    t = str_return(t);
    t = str_return(t);

    //방법 3
    println!("\n===방법 3===");
    let country = "대한민국".to_owned();
    get_ref(&country);
    get_ref(&country);
    get_ref(&country);
}

///소유권을 받는다
fn fnc(a:String){
    println!("{a}");
}

///a넘겨받아서 출력 후 다시 소유권 전달
fn str_return(a:String) -> String{
    println!("{a}");
    a
}

//call by ref(pointer)
fn get_ref(a:& String){
    println!("{a}");
}