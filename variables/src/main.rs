fn main() {
    /*
    // 变量可变性
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);*/

    /*  // 使用下划线开头忽略未使用的变量
    let _x = 5;
    let _y = 1; */

    /* // 变量解构
    let (a, mut b): (bool, bool) = (false, true);
    println!("a={:?},b={:?}", a, b);

    b = false;
    assert_eq!(a, b) */

    // 结构式赋值
    struct Struct {
        e: i32,
    }

    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，用来占位
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
