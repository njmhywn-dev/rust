fn main() {
    let x = 100;
    let y = 10.5;

    let z = x as f64 + y;

    let msg = String::from("Hello Wolrd!");
    let msg2 = "Hello World2".to_string();
    let msg3 = "Hello World3";
    let msg4 = format!("Pointer: {}, {}",x, y);

    println!("{}", z);
    println!("{}", msg);
    println!("{}", msg2);
    println!("{}", msg3);
    println!("{}",msg4);
}
