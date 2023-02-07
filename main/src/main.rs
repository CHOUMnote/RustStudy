/**
 * 많은 함수들이 Option 또는 Result 반환을 한다.
 * Some(T), None 을 반환한다.
 * Some(T).unwrap()을 통해 값을 얻는다.
 * None은 unwrap을 할수 없다. - None은 None일 뿐이다.
 */

//안전하고
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

   let some = get_fifth(v1);
   println!("{:?}", some);
   println!("{}", some.unwrap());
   println!("{:?}", get_fifth(v2));
}