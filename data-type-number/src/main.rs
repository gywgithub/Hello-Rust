// use std::rc::Rc;
// use std::collections::HashMap;

// use std::io;
// use std::cmp::Ordering;

// type Table = HashMap<String, Vec<String>>;

// use std::error::Error;
// use std::io::{Write, stderr};

// use std::fs;
// use std::io;
// use std::path::Path;

// use std::io::{self, BufRead};
// type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
// type GenericResult<T> = Reulst<T, GenericError>;

// fn main() -> Result<(), TideCalcError> {
//     let tides = calculate_tides()?;
//     print_tides(tides);
//     Ok(())
// }
// #[derive(Debug, Clone)]
// pub struct JsonError {
//     pub message: String,
//     pub line: usize,
//     pub column: usize,
// }

// #[test]
// fn math_works() {
//     let x: i32 = 1;
//     assert!(x.is_positive());
//     assert_eq!(x + 1, 2);
// }

// #[test]
// #[allow(unconditional_panic, unused_must_use)]
// #[should_panic(expected = "divide by zero")]
// fn test_divide_by_zero_error() {
//     1 / 0;
// }

// use std::num::ParseIntError;

// #[test]
// fn explicit_radix() -> Result<(), ParseIntError> {
//     i32::from_str_radix("1024", 10)?;
//     Ok(())
// }

// fn roughly_equal(a: f64, b: f64) -> bool {
//     (a - b).abs() < 1e-6
// }

// #[test]
// fn trig_works() {
//     use std::f64::consts::PI;
//     assert!(roughly_equal(PI.sin(), 0.0))
// }

// struct GrayscaleMap {
//     pixels: Vec<u8>,
//     size: (usize, usize)
// }

// let width = 1024;
// let height = 576;
// let image = GrayscaleMap {
//     pixels: vec![0; width * height],
//     size: (width, height)
// }

// fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
//     assert_eq!(pixels.len(), size.0 * size.1);
//     GrayscaleMap { pixels, size }
// }

// assert_eq!(image.size, (1024, 576));
// assert_eq!(image.pixels.len(), 1024 * 576);

pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

fn main() {
    println!("Hello, world!");

    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();

    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    // q.push('0');
    // q.push('1');
    // assert_eq!(q.pop(), Some('0'));

    // q.push('∞');
    // assert_eq!(q.pop(), Some('1'));
    // assert_eq!(q.pop(), Some('∞'));
    // assert_eq!(q.pop(), None);

    // assert!(q.is_empty());
    // q.push('⊙');
    // assert!(!q.is_empty())

    // struct Bounds(usize, usize);

    // let image_bounds = Bounds(1024, 768);

    // assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    // fn Bounds(elem0: usize, elem1: usize) -> Bounds { ... }

    // ---
    // struct Broom {
    //     name: String,
    //     height: u32,
    //     health: u32,
    //     position: (f32, f32, f32),
    //     intent: BroomIntent,
    // }

    // #[derive(Copy, Clone)]
    // enum BroomIntent {
    //     FetchWater,
    //     DumpWater,
    // }

    // fn chop(b: Broom) -> (Broom, Broom) {
    //     let mut broom1 = Broom {
    //         height: b.height / 2,
    //         ..b
    //     };

    //     let mut broom2 = Broom {
    //         name: broom1.name.clone(),
    //         ..broom1
    //     };

    //     broom1.name.push_str(" I");
    //     broom2.name.push_str(" II");

    //     (broom1, broom2)
    // }

    // let hokey = Broom {
    //     name: "Hokey".to_string(),
    //     height: 60,
    //     health: 100,
    //     position: (100.0, 200.0, 0.0),
    //     intent: BroomIntent::FetchWater,
    // };

    // let (hokey1, hokey2) = chop(hokey);

    // assert_eq!(hokey1.name, "Hokey I");
    // assert_eq!(hokey1.height, 30);
    // assert_eq!(hokey1.health, 100);

    // assert_eq!(hokey2.name, "Hokey II");
    // assert_eq!(hokey2.height, 30);
    // assert_eq!(hokey2.health, 100);

    // fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
    //     let mut numbers = vec![];
    //     for line_result in file.lines() {
    //         let line = line_result?;
    //         numbers.push(line.parse()?);
    //     }
    //     Ok(numbers)
    // }

    // fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    //     for entry_result in src.read_dir()? {
    //         let entry = entry_result?;
    //         let dst_file = dst.join(entry.file_name());
    //         fs::rename(entry.path(), dst_file)?;
    //     }
    //     Ok(())
    // }

    // let weather = get_weather(hometown).ok()?;

    // let weather = get_weather(hometown)?;

    // let weather = match get_weather(hometown) {
    //     Ok(success_value) => success_value,
    //     Err(err) => return Err(err)
    // };

    // fn print_error(mut err: &dyn Error) {
    //     let _ = writeln!(stderr(), "error: {}", err);
    //     while let Some(source) = err.source() {
    //         let _ = writeln!(stderr(), "caused by: {}", source);
    //         err = source;
    //     }
    // }

    // const THE_USUAL: WeatherReport = WeatherReport::Sunny(72);

    // let report = get_weather(los_angeles).unwrap_or(THE_USUAL);

    // display_weather(los_angeles, &report);

    // println!("error querying the weather: {}", err);

    // match get_weather(hometown) {
    //     Ok(report) => {
    //         display_weather(hometown, &report);
    //     }
    //     Err(err) => {
    //         println!("error querying the weather: {}", err);
    //         schedule_weatcher_retry();
    //     }
    // }

    // let is_even = |x| x % 2 == 0;

    // let is_even = |x: u64| -> bool { x % 2 == 0 };
    // assert_eq!(is_even(14), true);

    // let x = 17;
    // let index = x as usize;
    // println!("{}", index);

    // let padovan: Vec<u64> = compute_padovan_sequence(n);
    // for elem in &padovan {
    //     draw_triangle(turtle, *elem);
    // }

    // fn quicksort<T: Ord>(slice: &mut [T]) {
    //     if slice.len() <= 1 {
    //         return;
    //     }
    //     let pivot_index = partition(slice);
    //     quicksort(&mut slice[.. pivot_index]);
    //     quicksort(&mut slice[pivot_index + 1 ..]);
    // }

    // return Vec::with_capacity(10);

    // let ramp: Vec<i32> = (0 .. n).collect();

    // return Vec::<i32>::with_capacity(1000);

    // let ramp = (0 .. n).collect::<Vec<i32>>();

    // server
    //     .bind("127.0.0.1").expect("error binding server to address")
    //     .run().expect("error running server");

    // let mut numbers = Vec::new();

    // fn serve_forever(socket: ServerSocket, handler: ServerHandler) -> ! {
    //     socket.listen();
    //     loop {
    //         let s = socket.accept();
    //         handler.handle(s);
    //     }
    // }

    // fn wait_for_process(process: &mut Process) -> i32 {
    //     while true {
    //         if process.wait() {
    //             return process.exit_code();
    //         }
    //     }
    // }

    // let output = match File::create(filename) {
    //     Ok(f) => f,
    //     Err(err) => return Err(err)
    // };

    // fn f() {
    //     return;
    // }

    // let output = File::create(filename)?;

    // let sqrt = 'outer: loop {
    //     let n = next_number();
    //     for i in 1.. {
    //         let square = i * i;
    //         if square == n {
    //             break 'outer i;
    //         }
    //         if square > n {
    //             break;
    //         }
    //     }
    // };

    // 'search:
    // for room in apartment {
    //     for spot in room.hiding_spots() {
    //         if spot.contains(keys) {
    //             println!("Your keys are {} in the {}", spot, room);
    //             break 'search;
    //         }
    //     }
    // }

    // for line in input_lines {
    //     let trimmed = trim_comments_and_whitespace(line);
    //     let trimmed.is_empty() {
    //         continue;
    //     }
    // }

    // let answer = loop {
    //     if let Some(line) = next_line() {
    //         if line.starts_with("answer: ") {
    //             break line;
    //         }
    //     } else {
    //         break "answer: nothing";
    //     }
    // };

    // while condition {
    //     block
    // }

    // while let pattern = expr {
    //     block
    // }

    // loop {
    //     block
    // }

    // for pattern in iterable {
    //     block
    // }

    // if let Some(cookie) = request.session_cookie {
    //     return restore_session(cookie);
    // }

    // if let Err(err) = show_cheesy_anti_robot_task() {
    //     log_robot_attempt(err);
    //     politely_accuse_user_of_being_a_robot();
    // } else {
    //     session.mark_as_human();
    // }

    // if let pattern = expr {
    //     block1
    // } else {
    //     block2
    // }

    // struct CardObj {
    //     rank: String
    // }

    // let card = CardObj {
    //     rank: "Jack".to_string(),
    // };
    // let score = match card.rank {
    //     Jack => 10,
    //     Queen => 10,
    //     Ace => 11
    // };

    // let code: i32 = 6;
    // match code {
    //     0 => println!("OK"),
    //     1 => println!("Wires Tangled"),
    //     2 => println!("User Asleep"),
    //     _ => println!("Unrecognized Error {}", code)
    // }

    // match parmas.get("name") {
    //     Some(name) => println!("Hello, {}!", name),
    //     None => println!("Greetings, stranger.")
    // }

    // if condition1 {
    //     block1
    // } else if condition2 {
    //     block2
    // } else {
    //     block_n
    // }

    // fn show_files() -> io::Result<()> {
    //     let mut v = vec![];
    //     fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
    //         a.timestamp.cmp(&b.timestamp)
    //             .reverse()
    //             .then(a.path.cmp(&b.path))
    //     }
    //     v.sort_by(cmp_by_timestamp_then_name)
    // }

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

    // #[derive(Copy, Clone)]
    // struct Label { number: u32 }

    // fn print(l: Label) { println!("STAMP: {}", l.number); }

    // let l = Label { number: 3 };

    // print(l);

    // println!("My label number is: {}", l.number);

    // --- 第五章 引用 ---

    // let mut table = Table::new();

    // table.insert("Gesualdo".to_string(),
    //                 vec!["many madrigals".to_string(),
    //                      "Tenebrae Responsoria".to_string()]);
    // table.insert("Caravaggio".to_string(),
    //                 vec!["The Musicians".to_string(),
    //                      "The Calling of St. Matthew".to_string()]);
    // table.insert("Cellini".to_string(),
    //                 vec!["Perseus with the head of Medusa".to_string(),
    //                      "a salt cellar".to_string()]);
    // // show(&table);

    // show(&table);
}

// fn show(table: Table) {
//     for (artist, works) in table {
//         println!("works by {}:", artist);
//         for work in works {
//             println!("  {}", work);
//         }
//     }
// }

// fn build_vector() -> Vec<i16> {
//     let mut v = Vec::new();
//     v.push(10);
//     v.push(20);
//     v
// }
