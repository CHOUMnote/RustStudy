/**
 * impl Iterator
 */

#[derive(Debug)]
struct Book {
    items: Vec<String>,
}
impl Book {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, t: &str) {
        self.items.push(t.to_string());
    }
}
impl Iterator for Book{ //Iterator의 next 구현
   type Item = String; //iterable 타입 지정

   fn next(&mut self) -> Option<Self::Item> {
       match self.items.pop(){
         Some(T) => Some(T),
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

   for i in books{
      println!("{i}");
   }

}