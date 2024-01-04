fn main() {
    println!("Hello, world!");

    // assert_eq!(2_u16.pow(4), 16); // 求幂
    // assert_eq!((-4_i32).abs(), 4); // 求绝对值
    // assert_eq!(0b101101_u8.count_ones(), 4); // 求二进制1的个数

    // println!("123 {}", 4);

    // println!("{}", i32::abs(-4));
    // println!("{}", (-4_i32).abs());

    // let mut i: i32 = 1;
    // loop {
    //     i = i.checked_mul(10).expect("multiplication overflowed")
    // }

    // 检查运算 checked_add 加法
    // assert_eq!(10_u8.checked_add(20), Some(30));

    // assert_eq!(100_u8.checked_add(200), None);

    // let sum = x.checked_add(y).unwrap();

    // assert_eq!((-128_i8).checked_div(-1), None);

    // 回绕运算 wrapping_mul 乘法
    // assert_eq!(100_u16.wrapping_mul(200), 20000);
    // assert_eq!(500_u16.wrapping_mul(500), 53392);

    // assert_eq!(500_i16.wrapping_mul(500), -12144);

    // assert_eq!(5_i16.wrapping_shl(17), 10);

    // 饱和运算 saturating_add 饱和整数加法
    // saturating v.使饱和，浸透，使充满
    assert_eq!(32760_i16.saturating_add(10), 32767);

    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // 溢出运算 返回元组(result, overflowd), overflowd 表示是否溢出
    // sub 减法
    // assert_eq!(255_u8.overflowing_sub(2), (253, false));
    // assert_eq!(255_u8.overflowing_add(3), (2, true));

    // assert_eq!(5_u16.overflowing_shl(17), (10, true));

    // --- 3.1.3 浮点类型 ---
    // assert!((-1. / f32::INFINITY).is_sign_negative());

    // assert_eq!(-f32::MIN, f32::MAX)

    // sqrt() 平方根
    // square root 平方根
    // assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    // assert_eq!((-1.01f64).floor(), -2.0);

    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));
}

// fn build_vector() -> Vec<i16> {
//     let mut v = Vec::new();
//     v.push(10);
//     v.push(20);
//     v
// }
