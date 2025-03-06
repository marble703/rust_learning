struct Struct {
    e: i32,
}

fn main() {
    // 变量绑定

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //变量解构

    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b); // 断言a和b相等
    println!("a = {:?}, b = {:?}\n", a, b);

    // 解构式赋值

    let (a, b, c, d, e);

    // 元组解构
    (a, b) = (1, 2);

    // 数组解构
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];

    // 结构体解构
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    println!(
        "a = {:?}, b = {:?}, c = {:?}, d = {:?}, e = {:?} \n",
        a, b, c, d, e
    );

    // 变量和常量之间的差异

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {} \n", MAX_POINTS);

    // 变量遮蔽

    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();

    let mut spaces = "   ";
    // spaces = spaces.len(); // 这里会报错，因为spaces是不可变的

    println!("The value of spaces is: {}", spaces);
}
