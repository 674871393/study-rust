struct Struct {
    e: i32,
}

const MAX_POINTER: u32 = 100_000;

fn main() {
    // let mut x = 5;
    // println!("x = {}", x);
    // x = 3;
    // println!("x = {}", x);

    // let _x = 5;
    // let y = 3;
    // println!("x = {}", _x);

    // let (a, mut b): (bool, bool) = (true, false);
    // println!("a = {}, b = {}", a, b);
    // b = true;
    // assert_eq!(a, b);

    // let (a, b, c, d, e);
    // (a, b) = (1, 2);
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct { e, .. } = Struct { e: 5 };
    // println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);

    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x is: {}", x);

    let space = "    ";
    let space = space.len();
    println!("space = {}", space);
}
