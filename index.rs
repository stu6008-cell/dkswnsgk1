fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("==============================");
    println!("Rust 기초 예제");
    println!("==============================");

    // 1
    println!("\n1. Hello World");
    println!("Hello, World!");

    // 2
    println!("\n2. 변수");
    let x = 10;
    println!("x = {}", x);

    // 3
    println!("\n3. 가변 변수");
    let mut y = 5;
    println!("변경 전 = {}", y);
    y = 20;
    println!("변경 후 = {}", y);

    // 4
    println!("\n4. 자료형");
    let a: i32 = 10;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'A';
    println!("{} {} {} {}", a, b, c, d);

    // 5
    println!("\n5. 문자열");
    let s = "Rust";
    println!("{}", s);

    // 6
    println!("\n6. 사칙연산");
    println!("10 + 5 = {}", 10 + 5);
    println!("10 - 5 = {}", 10 - 5);
    println!("10 * 5 = {}", 10 * 5);
    println!("10 / 5 = {}", 10 / 5);

    // 7
    println!("\n7. if문");
    let score = 80;

    if score >= 60 {
        println!("합격");
    } else {
        println!("불합격");
    }

    // 8
    println!("\n8. match");
    let n = 2;

    match n {
        1 => println!("하나"),
        2 => println!("둘"),
        _ => println!("기타"),
    }

    // 9
    println!("\n9. while문");
    let mut i = 1;

    while i <= 5 {
        println!("{}", i);
        i += 1;
    }

    // 10
    println!("\n10. for문");

    for i in 1..6 {
        println!("{}", i);
    }

    // 11
    println!("\n11. loop");

    let mut n = 1;

    loop {
        println!("{}", n);

        if n == 5 {
            break;
        }

        n += 1;
    }

    // 12
    println!("\n12. 배열");

    let arr = [10, 20, 30];

    println!("{:?}", arr);

    // 13
    println!("\n13. 벡터");

    let mut v = Vec::new();

    v.push(10);
    v.push(20);
    v.push(30);

    println!("{:?}", v);

    // 14
    println!("\n14. 함수");

    println!("3 + 5 = {}", add(3, 5));

    // 15
    println!("\n15. 튜플");

    let t = (100, "Rust", true);

    println!("{:?}", t);

    // 16
    println!("\n16. 문자열");

    let mut text = String::from("Hello");

    text.push('!');
    text.push_str(" Rust");

    println!("{}", text);

    // 17
    println!("\n17. 참조");

    let value = 100;
    let reference = &value;

    println!("{}", reference);

    // 18
    println!("\n18. Option");

    let op: Option<i32> = Some(50);

    println!("{:?}", op);

    // 19
    println!("\n19. Result");

    let result: Result<i32, &str> = Ok(100);

    println!("{:?}", result);

    // 20
    println!("\n20. 형변환");

    let num = 10;
    let decimal = num as f64;

    println!("{}", decimal);

    println!("\n==============================");
    println!("모든 예제가 실행되었습니다!");
    println!("==============================");
}