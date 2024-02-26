// use std::rc::Rc;

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

    // // 饱和运算 saturating_add 饱和整数加法
    // // saturating v.使饱和，浸透，使充满
    // assert_eq!(32760_i16.saturating_add(10), 32767);

    // assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // // 溢出运算 返回元组(result, overflowd), overflowd 表示是否溢出
    // // sub 减法
    // // assert_eq!(255_u8.overflowing_sub(2), (253, false));
    // // assert_eq!(255_u8.overflowing_add(3), (2, true));

    // // assert_eq!(5_u16.overflowing_shl(17), (10, true));

    // // --- 3.1.3 浮点类型 ---
    // // assert!((-1. / f32::INFINITY).is_sign_negative());

    // // assert_eq!(-f32::MIN, f32::MAX)

    // // sqrt() 平方根
    // // square root 平方根
    // // assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    // // assert_eq!((-1.01f64).floor(), -2.0);

    // println!("{}", (2.0_f64).sqrt());
    // println!("{}", f64::sqrt(2.0));

    // // --- 3.2 布尔类型 ---
    // assert_eq!(false as i32, 0);
    // assert_eq!(true as i32, 1);

    // // --- 3.4 元组 ---
    // let text = "I see the eigenvalue in thine eye";
    // let (head, tail) = text.split_at(21);
    // assert_eq!(head, "I see the eigenvalue ");
    // assert_eq!(tail, "in thine eye");

    // // --- 3.6 数组、向量和切片 ---
    // let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    // let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    // assert_eq!(lazy_caterer[3], 7);
    // assert_eq!(taxonomy.len(), 3);

    // let mut sieve = [true; 10000];
    // for i in 2..100 {
    //     if sieve[i] {
    //         let mut j = i * i;
    //         while j < 10000 {
    //             sieve[j] = false;
    //             j += i;
    //         }
    //     }
    // }
    // assert!(sieve[211]);
    // assert!(!sieve[9876]);

    // let mut chaos = [3, 5, 4, 1, 2];
    // chaos.sort();
    // assert_eq!(chaos, [1, 2, 3, 4, 5]);

    // let mut primes = vec![2, 3, 5, 7];
    // assert_eq!(primes.iter().product::<i32>(), 210);

    // primes.push(11);
    // primes.push(13);
    // assert_eq!(primes.iter().product::<i32>(), 30030);

    // // fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    // //     vec![0; rows * cols]
    // // }

    // let mut pal = Vec::new();
    // pal.push("step");
    // pal.push("on");
    // pal.push("no");
    // pal.push("pets");
    // assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    // let v: Vec<i32> = (0..5).collect();
    // assert_eq!(v, [0, 1, 2, 3, 4]);

    // let mut v = vec![10, 20, 30, 40, 50];
    // v.insert(3, 35);
    // assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    // v.remove(1);
    // assert_eq!(v, [10, 30, 35, 40, 50]);

    // let mut v = vec!["Snow Puff", "Glass Gem"];
    // assert_eq!(v.pop(), Some("Glass Gem"));
    // assert_eq!(v.pop(), Some("Snow Puff"));
    // assert_eq!(v.pop(), None);

    // let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    // let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    // let sv: &[f64] = &v;
    // let sa: &[f64] = &a;

    // fn print(n: &[f64]) {
    //     for elt in n {
    //         println!("{}", elt);
    //     }
    // }

    // print(&a);
    // print(&v);

    // // --- 3.7 字符串类型 ---
    // let method = b"GET";
    // assert_eq!(method, &[b'G', b'E', b'T']);

    // let noodles = "noodles".to_string();
    // let oodless = &noodles[1..];
    // let poodless = "ಠ_ಠ";

    // println!("{}", noodles);

    // // --- 4 所有权与移动 ---
    // struct Person {
    //     name: String,
    //     birth: i32,
    // }

    // let mut composers = Vec::new();
    // composers.push(Person {
    //     name: "Palestrina".to_string(),
    //     birth: 1525,
    // });
    // composers.push(Person {
    //     name: "Downland".to_string(),
    //     birth: 1563,
    // });
    // composers.push(Person {
    //     name: "Lully".to_string(),
    //     birth: 1632,
    // });

    // for composer in &composers {
    //     println!("{}, born {}", composer.name, composer.birth);
    // }

    // let s: Rc<String> = Rc::new("shirataki".to_string());
    // let t: Rc<String> = s.clone();
    // let u: Rc<String> = s.clone();

    // struct Person { name: String, birth: i32 }

    // let mut composers = Vec::new();

    // composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    // composers.push(Person { name: "Download".to_string(), birth: 1563 });
    // composers.push(Person { name: "Lully".to_string(), birth: 1632 });

    // for composer in &composers {
    //     println!("{}, born {}", composer.name, composer.birth);
    // }

    // --- 4.2.1 移动 ---
    // let mut s = "Govinda".to_string();
    // let t = s;
    // // println!("{}", s);

    // println!("{}", t);
    // s = "Siddhartha".to_string();
    // println!("{}", s);

    // let mut v = Vec::new();
    // for i in 101 .. 106 {
    //     v.push(i.to_string());
    // }

    // let third = &v[2];
    // println!("{}", third);

    // struct Person { name: Option<String>, birth: i32 }

    // let mut composers = Vec::new();
    // composers.push(Person { name: Some("Palestrina".to_string()), birth: 1525 });

    // let first_name = std::mem::replace(&mut composers[0].name, None);
    // assert_eq!(first_name, Some("Palestrina".to_string()));
    // assert_eq!(composers[0].name, None);

    #[derive(Copy, Clone)]
    struct Label { number: u32 }

    fn print(l: Label) { println!("STAMP: {}", l.number); }

    let l = Label { number: 3 };

    print(l);

    println!("My label number is: {}", l.number);
}

// fn build_vector() -> Vec<i16> {
//     let mut v = Vec::new();
//     v.push(10);
//     v.push(20);
//     v
// }
