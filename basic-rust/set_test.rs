/**
 * std::collections
 * set - 키값 집합
 * HashSet<T, S>
*/

use std::collections::HashSet;

fn main(){
   let numbers = vec![1,5,9,3,1,5,10];
   let mut set = HashSet::new();

   for i in numbers{
      set.insert(i);
   }
   println!("set is {set:?}");

   //Set 데이터 접근
   for i in 0..=10{
      if let None = set.get(&i){ //== if set.get(&i).is_none(){..}
         print!("{i}, ");  //0~10까지 set에 없는 수 찾기
      }
   }
}