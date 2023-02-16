/**
 * ? orperator
 * ? 연산은 Result<T> 에 대해서 Ok거나 Err로 반환한다
 * ? 연산은 다시 Result를 반환 하므로 함수 연속해서 ?연산으로 체이닝을 할 수 있다.
*/
use std::num::*;

fn parse_str(input: &str) -> Result<i32, ParseIntError> { 
   let result = input.parse::<i32>()?.to_string().parse::<i32>()?; //체이닝
   Ok(result)
}

fn main() {
   let strs = ["11a","212","kim","k9"];
   let mut vec = Vec::new();   
   for i in strs{
      vec.push(parse_str(i));
   }

   println!("{vec:#?}");
}