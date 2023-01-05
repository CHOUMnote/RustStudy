/**
 * 스칼라 타입 변수 - 단일 값 변수
 * 복합 값 - 배열, 튜플 등등
 */
fn main() {
    //단일 값 사용 방법
    print!("Scalar 변수");
    {
        //rust 타입별 출력
        let a: i32 = 1_000_000_000; //가독성 좋은 10진수
        println!("{}", a);

        let a = 0xff; //16진수
        println!("{}", a);

        let a = 0o15; //8진수
        println!("{}", a);

        let a = 0b1111_1111; //2진수
        println!("{}", a);

        let a = b'A'; //1바이트 값(아스키코드)
        println!("{}", a);

        let a = 1.12345; //부동소수점
        println!("{}", a);

        let a = false; //boolean
        println!("{}", a);

        let a = 'A'; //유니코드
        println!("{}", a);
    }
    {
        //복합 값 사용 방법
        print!("\nCompound  변수");
        {
            let arr1 = [1, 2, 3]; //선언-> [type; size]
                                  //let arr:[i32; 5] = [1,2,3];// rust에 널은 없다! 에러!
            let arr2 = [0; 100]; //생성 및 초기화

            println!("배열의 2번째 원소는? {}", 1);
            println!("배열 출력? {:?}", arr1);
            println!("배열 길이? {}", arr2.len());
        }

        {
            //튜플은 수정 안됨
            let tuple1 = (1, 1.4, "Asv", true);
            let tuple2: (i16, f32) = (1, 3.14);

            println!("3번째 원소 출력? {}", tuple1.2);
        }
    }
}
