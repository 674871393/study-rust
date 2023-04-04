fn main() {
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

    let x: &str = "hello";
    let y = x;
    println!("x={},y={}", x, y);

    // 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。
    // 所有整数类型，比如 u32
    // 布尔类型，bool，它的值是 true 和 false
    // 所有浮点数类型，比如 f64
    // 字符类型，char
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
    // 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的
}
