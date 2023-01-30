/**
 * 복합값 - 배열, 튜플 ... 벡터
 * 배열 - 같은 자료형 집합
 * 벡터 - 뭔가 할수 있는게 많은 배열(문자열이 벡터로 이루어 져있다.)
 * 튜플 - 수정 불가능한 여러 타입들의 모음(파이썬과 유사)
*/

fn main() {
    let arr:[i32;10] = [7; 10]; //i32 10개 선언 및 7로 10개 초기화
    println!("배열 : {0:?}, 4번째 원소 : {1}",arr, arr[3]);
    println!("11번째 : {:?}",arr.get(10));   //좀더 안전한 슬라이싱
    println!();

    let tuple:(i32, &str, bool) = (1004, "안녕!", true);    //튜플 구조체 선언 - 서로 다른 자료들 모음
    println!("튜플 : {tuple:?}");
    println!("1번째 : {}",tuple.0);
    println!("2번째 : {}",tuple.1);
    println!("3번째 : {}",tuple.2);
    println!();

    println!("new, from 내장 함수 사용법"); //각 객체에는 from new into... 많은 기능이 내포한다. 이건 api문서를 참고하면 더 많은 정보를 얻을수 있다. 
    let vec:Vec<i32> = Vec::from(arr);  //array to vec
    println!("{:?}",vec);
    let vec:Vec<i32> = vec![1,2,3]; //매크로
    println!("{:?}",vec);
    let vec:Vec<u8> = Vec::from(String::from("HELLO"));  //String to vec
    println!("{:?}",vec);
    println!();

    //튜플을 포함한 배열의 2차배열
    let t:[[(&str,i32);10];5] = [[("rust",2021);10];5]; //튜플 10개짜리가 5개 있는 구조

    for i in t.iter(){
        println!("{i:?}");
    }

}