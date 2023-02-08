/**
 * std::collections
 * Heap
 * rust는 최대힙
*/

use std::collections::BinaryHeap;

fn main() {
   let mut heap = BinaryHeap::new();
   for i in 0..10{
      heap.push(i);
      println!("{heap:?}");
   }

   //힙정렬
   let mut vec = Vec::new();
   while let Some(x) = heap.pop(){
      vec.push(x);
   }
   println!();
   println!("array is {vec:?}");
}