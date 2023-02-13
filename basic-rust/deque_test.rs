/**
 * Deque
 * 스택+큐+벡터
 * 벡터를 스텍으로 사용한다면 O(1)로 연산을 할 수있다.
 * 벡터를 큐로 사용한다면 O(N)만큼 걸린다
*/

use std::collections::VecDeque;

fn main() {
   let mut a = vec![1,2,3,4,5,6];
   println!("{}",a.pop().unwrap()); //상수시간만큼 걸린다
   println!("{}",a.remove(0)); //O(N) 만큼 걸린다
   println!("{:?}",a);

   //push front,back - pop front, back 연산
   let mut deque = VecDeque::new(); //앞뒤로 넣기는 O(1)
   deque.push_back(1);  //1
   deque.push_front(0); //0 1
   deque.push_back(2);  //0 1 2
   println!("{:?}",deque);
   println!("{:?}",deque.pop_front()); //0
   println!("{:?}",deque.pop_back());  //2
   println!("{:?}",deque);
}