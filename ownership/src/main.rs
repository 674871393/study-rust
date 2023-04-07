/* fn main() {
    // 所有权原则
    //  Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    // 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    // 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

    // let mut s = String::from("hello");
    // s.push_str(",world!");
    // println!("{}", s);

    // 转移所有权
    // let x = 5;
    // let y = x;
    // println!("x={},y={}", x, y);

    // let s1 = String::from("hello");
    // let s2 = s1;

    // let x: &str = "hello";
    // let y = x;
    // println!("x={},y={}", x, y);

    // 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。
    // 所有整数类型，比如 u32
    // 布尔类型，bool，它的值是 true 和 false
    // 所有浮点数类型，比如 f64
    // 字符类型，char
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
    // 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1={},s2={}", s1, s2);

    // 函数传值与返回
    /*  let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x */

    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 移给 s1
    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中,
                                       // 它也将返回值移给 s3
    println!("s1={},s3={}", s1, s3);
}

/* fn takes_ownership (some_string: String) {
    println!("{}", some_string);
} // 这里，some_string 进入作用域并离开作用域。所以 drop 被调用，内存被释放。

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // 这里，some_integer 进入作用域并离开作用域。不会有特殊操作。 */

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

// takes_and_gives_back 将传入的 String 返回给调用者
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
 */

fn main() {
    // let x = 5;
    // let y = &x;
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // 不可变引用
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // 可变引用,可变引用有一个很大的限制，我们只能拥有一个可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

fn change(s: &mut String) {
    s.push_str(",world!");
}
