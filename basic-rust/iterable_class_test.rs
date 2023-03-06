/**
 * impl Iterator
 * Iteraotr trait를 구현
 * next() 구현시 자동 호출
 */

 #[derive(Debug)]
 struct Book {
     items: Vec<String>,
     index: u32,
     size: u32,
 }
 impl Book {
     fn new() -> Self {
         Self { items: Vec::new(), index: 0, size:0}
     }
 
     fn add(&mut self, t: &str){
         self.items.push(t.to_string());
         self.size+=1;
     }
 }
 impl Iterator for Book {
     //Iterator의 next 구현
     type Item = String; //iterable 타입 지정
 
     fn next(&mut self) -> Option<Self::Item> {
         let result = self.items.get(self.index as usize);
         self.index+=1;
         
         match result {
             Some(T) => Some(T.clone()),
             None => None,
         }
     }
 }
 
 fn main() {
     let mut books = Book::new();
     books.add("자바의 정석");
     books.add("easy rust");
     books.add("Jump to Django");
     println!("books is {:?}", books);
 
     for i in books {
         println!("{i}");
     }
 }
 