/*
zipping iterator
*/

use std::collections::HashMap;

fn main() {
    let numbers = vec![1,2,3,4,5,6];
    let mut strings:Vec<String>=Vec::new();
    for i in 1..=5{
        strings.push(i.to_string());
    }

    print!("i32:stirng hashmap is");

    let mut hashmap:HashMap<i32,String> = numbers
    .into_iter()
    .zip(strings.into_iter())
    .collect(); //갯수가 안맞더라도 남는 원소들은 버려짐

    print!("{hashmap:#?}");
}
 