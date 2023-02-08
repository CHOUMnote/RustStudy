/**
 * std::collections
 * map - HashMap, BTreeMap
 * map - key:value 조합 자료구조
 * hash -> 정렬 x
 * BTree -> 정렬 o
 * rust에서는 항상 소유권 관령 issue를 신경쓰자! 소유권이 있는 값들은 반드시 소유권 이전이 일어난다!
*/

use std::collections::HashMap;
use std::collections::BTreeMap;

fn main(){
   let vec = vec![(1241, "김지호"), (2030, "김준호"), (10_001, "이민주"), (827, "길상현")];
   //컴파일 시간에 정의 되므로 추론 가능하다.
   //HashMap<K, V, S = RandomState>
   let mut hashmap = HashMap::new();
   //BTreeMap<K, V>
   let mut treemap = BTreeMap::new(); 

   for i in vec{
      hashmap.insert(i.0, i.1);
      treemap.insert(i.0, i.1);
   }
   
   println!("hashmap is {hashmap:#?}");   //random state
   println!("hashmap is {treemap:#?}");   //ordering state

   println!();
   //접근 방법
   println!("1241 is {}", hashmap[&1241]);  //인덱싱 - 없다면 패닉이다.
   println!("1241 is {:?}",hashmap.get(&1241)); //get 방식 - 일반 상수는 소유권 이전 -> 레퍼런스를 줘야한다.
   println!("1242 is {:?}",hashmap.get(&1242));

}