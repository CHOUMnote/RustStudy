/**
 * assert
 * 다른 언어와 똑같이 true면 패스, false면 패닉이 발생하며 함수 도큐먼트나 디버깅에 사용한다.
 * 1. assert!(bool)  -  단순 bool
 * 2. assert_eq!(a,b) - a,b 비교 (같으면 ture)
 * 3. assert_ne!(a,b)  - a,b 비교 (다르면 true)
 */

 fn main() {
    let vec = vec!['a','김','3'];
    let mut iter = vec.iter();
 
    assert_eq!(iter.next(), Some(&'a'));   //next()는 Option<T>를 반환
    assert_eq!(iter.next(), Some(&'김'));
    assert_eq!(iter.next(), Some(&'3'));
    assert_eq!(iter.next(), None);   //마지막은 None
    assert_eq!(iter.next(), None);   //이후 계속 None
    assert_ne!(iter.next(), Some(&'!'));
    //assert_eq!(iter.next(), Some(&'!'), "이건 에러 메세지 입니다!");  //메시징 지정
 
    println!("완료");
 }
 