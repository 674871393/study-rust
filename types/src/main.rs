fn main() {
    // 类型推导与标注
    /*  let guess: i32 = "42".parse().expect("Not a number!");
    println!("{}", guess); */

    // 数值类型: 有符号整数 (i8, i16, i32, i64, isize)、
    //      无符号整数 (u8, u16, u32, u64, usize) 、
    //          浮点数 (f32, f64)、
    //              以及有理数、复数。
    // 字符串：字符串字面量和字符串切片 &str
    // 布尔类型： true和false
    // 字符类型: 表示单个 Unicode 字符，存储为 4 个字节
    // 单元类型: 即 () ，其唯一的值也是 ()

    // let a: u8 = 255;
    // let b = a.wrapping_add(29);
    // println!("{}", b);

    // 整型溢出
    // 要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：
    //  使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    //  如果使用 checked_* 方法时发生溢出，则返回 None 值
    //  使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    //  使用 saturating_* 方法使值达到最小值或最大值

    // let a: u8 = 255;
    // let b = a.wrapping_add(20);
    // println!("{}", b);

    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let remainder = 43 % 5;

    /* let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    // 只有同样的类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{}", addition);

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:.2}", forty_twos[0]); */

    // for i in 1..=5 {
    //     println!("{}", i);
    // }

    // for i in 'A'..='Z' {
    //     println!("{}", i);
    // }

    // fn add_with_extra(x: i32, y: i32) -> i32 {
    //     let x = x + 1; // 语句
    //     let y = y + 5; // 语句
    //     x + y // 表达式，表达式可以用作函数的返回值
    // }
    //     let y = {
    //         let x = 3;
    //         x + 1
    //     };

    // let x = plus_or_minus(5);
    // println!("{}", x);
}
// fn ret_unit_type() {
//     let x = 1;
//     let y = if x % 2 == 1 { "odd" } else { "even" };
// }

// fn add(i:i32,j:i32)->i32{
//     i+j
// }

// fn plus_or_minus(x: i32) -> i32 {
//     if x > 5 {
//         return x - 5;
//     }
//     x + 5
// }
