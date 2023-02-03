// fn main() {
//     println!("Hello, world!");
// }

// fn greet_world() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界，你好";
//     let english = "World, hello";
//     let regions = [southern_germany, chinese, english];
//     for region in regions.iter() {
//         println!("{}", &region);
//     }
// }

// fn main() {
//     greet_world();
// }

// fn main() {
//     let penguin_data = "\
//     common name,length (cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";
 
//     let records = penguin_data.lines();
 
//     for (i, record) in records.enumerate() {
//       if i == 0 || record.trim().len() == 0 {
//         continue;
//       }
 
//       // 声明一个 fields 变量，类型是 Vec
//       // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//       // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
//       let fields: Vec<_> = record
//         .split(',')
//         .map(|field| field.trim())
//         .collect();
//       if cfg!(debug_assertions) {
//           // 输出到标准错误输出
//         eprintln!("debug: {:?} -> {:?}",
//                record, fields);
//       }
 
//       let name = fields[0];
//       // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
//       //
//       // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
//       //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
//       //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
//       //
//       // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
//       if let Ok(length) = fields[1].parse::<f32>() {
//           // 输出到标准输出
//           // println!("{}, {}cm", name, length);
//           println!("{name}, {length}cm");
//       }
//     }
//   }

// 变量绑定与解构
// struct Struct {
//   e: i32
// }
// fn main() {
//   // 使用 mut 声明可更改的变量
//   let mut num = 1;
//   println!("This num is {num}");
//   num = 2;
//   println!("This num is {num}");

//   // 未被使用的变量名前 需要加 _
//   let _str = "str";

//   // 使用 let 进行 变量解构
//   let (boola, mut boolb): (bool,bool) = (true, false);
//   // a = true,不可变; b = false，可变
//   println!("a = {:?}, b = {:?}", boola, boolb);
//   boolb = true;
//   assert_eq!(boola, boolb);

//   // 解构式赋值
//   let (a, b, c, d, e);
//   (a, b) = (1, 2);
//   // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
//   [c, .., d, _] = [1, 2, 3, 4, 5];
//   Struct { e, .. } = Struct { e: 5 };
//   assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
//   println!("{a}, {b}, {c}, {d}, {e}");

//   // 使用 const 声明 常量, 所有字母大写, 使用 _ 分割单词
//   const MAX_POINTS: u32 = 100_000;
//   println!("MAX_POINTS = {MAX_POINTS}");

//   // 变量遮蔽(shadowing)  即 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
//   let x = 5;
//   println!("The value of x is: {x}");
//   // 在main函数的作用域内对之前的x进行遮蔽
//   let x = x + 1;
//   println!("Then the value of x is: {x}");
//   {
//     // 在当前的花括号作用域内，对之前的x进行遮蔽
//     let x = x * 2;
//     println!("The value of x in the inner scope is: {x}");
//   }
//   println!("The final value of x is: {x}");

//   let guess:i32 = "42".parse().expect("Not a number!");
//   println!("The value of guess is {guess}");


// }

// 基本类型
#![allow(unused)]
fn main(){
  let a = 1;
}


