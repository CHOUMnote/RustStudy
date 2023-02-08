/**
 * 많은 함수들이 Option 또는 Result 반환을 한다.
 * Some(T), None 을 반환한다.
 * Some(T).unwrap()을 통해 값을 얻는다.
 * None은 unwrap을 할수 없다. - None은 None일 뿐이다.
 */

//안전한 메소드
fn get_fifth(vec:Vec<i32>) -> Option<i32>{
    if vec.len() < 5{
       return None
    } else{
      return Some(vec[4])
    }
 }
 
 fn main() {
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![1,2,3];
 
    let some1 = get_fifth(v1);
    let some2 = get_fifth(v2);
    println!("{:?}", some1);
    println!("{}", some1.unwrap());
    println!("{:?}", some2);
 
    println!();
 
    //Option은 Some 이거나 None이거나 둘중 하나이다
    match some2{
       Some(x) => println!("Some({x})입니다."),
       None =>  println!("None입니다.")
    }
 }