/**
 * std::collections
 * map 다루기
 * 1. insert, [key] - 중복시 값 변경
 * 2. entry.or_insert(key) - 해당 키가 없다면 넣고, 있으면 무시 그리고 해당 &`a mut를 반환한다
 *  Entry<'_, K, V> 
*/

use std::collections::BTreeMap;

fn main() {
   let mut map = BTreeMap::new();
   map.insert(1, "김".to_string());
   map.insert(2, "이".to_string());
   map.insert(3, "박".to_string());
   map.entry(1).or_insert("최".to_string());
   map.entry(4).or_insert("최".to_string());
   println!("map is {map:#?}");
   println!();

   //.or_insert() -> &mut 반환이므로 역참조가 가능하다.
   let entry = map.entry(1);
   let temp:&mut String = entry.or_insert("안".to_owned());
   println!("{temp}");
   *temp = "안".to_owned();
   println!("{temp}");
}