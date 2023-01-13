/**
 * String 객체와 &str
 * &str : 문자열 리터럴, 이미 소유하고있는(owned) 문자열을 슬라이스 한 값
 * &str = &'static str 
 * &'static? 라이프타임 표시 - 이 변수는 프로그램 끝까지 생존(정적 변수)
 * &str : 래퍼런스 타입이기 때문에 &str로 사용
 * 문자열 리터럴은 바이너리에 저장 - 참조를 통해 값을 본다!
 * String - &str보다 많은 기능 내부적으로 Vec<u8>으로 이루어짐
 * [String] api문서를 보자!
*/
fn main() {
    let a:String = String::from("i`m jiho!");
    
    slice_string(&a);
    println!();
    how_to_string();

}

///문자열 슬라이싱
fn slice_string(a:&str){
    //바이트 단위로 자름
    //만약 유니코드라면 글자 바이트에 맞춰 잘라야함
    println!("{}",&a[1..5]); //index - [1,5) = 1,2,3,4
}

fn how_to_string(){
    let base = "김지호";
    //to_owned
    let a = base.to_owned();
    let b = base.to_string(); //.to_owned.to_string();랑 같다 그럼 왜 씀?
    let c = String::from(base);
    let d = format!("안녕! {}!",base);  //문자열 접합

    let arr:[&String;4] = [&a,&b,&c,&d];

    for i in 0..arr.len(){
        println!("{}",arr[i]);
    }
}