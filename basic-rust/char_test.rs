use std::{self, mem}; //c++ using, c# use

/**
 * -char형 데이터 알아보기
 * 다양한 문자 표현 방법
 * rust에서 char는 유니코드가 지원되기 때문에 4바이트
 * 다른 언어와 같이 1바이트 char는 u8을 사용해야한다.
 */
fn main() {
    let a = 'A'; //정형화된 char형
    let b: u8 = 'a' as u8; //정형화 되지 않은 문자
    let cat = '😺'; //이모지도 지원 한다.

    println!("'A' to char = {a}\n'a'to u8 = {b}\n'cat' to emoji = {cat}");
    println!();

    //자료형 메모리 체크 -> std::mem::size_of::<T>()
    println!("char(default) mem : {}", mem::size_of::<char>());
    println!("char(u8) mem : {}", mem::size_of::<u8>());
}
