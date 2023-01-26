/**
 * reference와 lifetime
 * rust는 lifetime 체크가 엄격하다. 따라서 컴파일시간에 많은 에러를 찾을 수 있다!(짱편함)
 * 1. mut ref는 오직 한개만 가능하다
 * 2. 일반 ref는 여러개 존재해도 상관 없다!
 * 3. mut ref, ref는 공존하지 못한다
*/

fn main() {
    let mut a = 100;
    let b = &mut a; //b의 주소를 갖는다
    //b = 200; b는 &mut i32지 mut i32가 아니다. 주소를 바꿀 수는 없다
    *b = 200; //b가 참조하는 값을 바꿈 '*' 연산자 (c와 같다고 생각 하면 쉽다)
    println!("바꾼 값 = {b}");    //a는 사용X b에서 바뀌므로 a에대한 무결성이 깨진다.

    println!("\n===about lifetime...===");
    //case 1 - mut ref와 ref 관계
    let mut a = 300;
    let b = &a;
    //let c = &mut a; b의 라이프 타임이 살아 있다. 따라서 새로운 ref를 사용할 수 없다.
    println!("{b}");    //b의 라이프타임은 여기 까지!
    let c = &mut a; //a에 관한 참조가 하다도 없으므로 사용 가능
    *c+=1000;
    println!("{a}");
    
    //case 2
    let a = 100;
    let a_ptr = &a;
    let a = 200; //a에 쉐도잉
    
    println!("a의 참조 : {a_ptr}"); //a의 원래 주소는 a_ptr이 참조하므로 라이프타임이 끝나지 않음
    //단 a라는 별명을 다른 공간이 사용하는 것 뿐이다.
}