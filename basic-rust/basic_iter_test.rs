/**
 * Iterator - 반복자
 * .next() 호출이 가능한 것
 * 1. .iter() - &T iterator
 * 2. .iter_mut() - &mut T iterator
 * 3. .into_iter() - cunsuming iteraor
 */

 fn main() {
    let mut vec = vec![1,2,3,4,5];
    
    //map 함수는 모든 원소에 대해 클로저를 적용한다.
    let iter1 = vec.iter().map(|x| x+1).collect::<Vec<i32>>();
    //let iter1:Vec<i32> = vec.iter().map(|x| x+1).collect();   자료형 명시
    println!("iter1 is {:?}",iter1);
 
    let iter2 = vec.iter_mut();
    iter2.for_each(|x| *x*=2); //ref이기 때문에 *연산자 사용  
    println!("iter2 is {:?}",vec);
 
    let iter3 = vec.into_iter().map(|x| x+1).collect::<Vec<i32>>();;
    //println!("vec is {:?}",vec); vec의 소유권은 사라졌기에 사용 불가
    println!("iter3 is {:?}",iter3); //iter2가 vec을 수정했다. 따라서 수정된 벡터에 함수를 적용시킨다.
 }
 