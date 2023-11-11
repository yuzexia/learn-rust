const MAX: i32 = 64;

fn main() {
    let x = 1;
    let condition = true;
    let string_no_type = "hello world!";
    let string_with_type: &str = "hello world!";
    let y = 2.0; //f64
    let z: f32 = 3.0; //f32
    let bool_true = true;
    let bool_false = false;
    let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("{}{}", x, MAX);
    println!("{}{}", string_no_type, string_with_type);
    println!("{}{}", y, z);
    println!("{}{}", bool_true, bool_false);
    println!("{}", array[0]);
    if x < 5 {
        println!("条件成立");
    } else {
        println!("条件不成立");
    }
    let mut number = if condition { 5 } else { 6 };
    println!("{}", number);
    while number != 0 {
        println!("hello Rust!");
        if number == 2 {
            break;
        }
        number -= 1;
    }

    for element in array.iter() {
        println!("值为：{}", element)
    }
}
