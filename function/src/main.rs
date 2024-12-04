fn main() {
    let call_greeting1 = gree_ting("Hunting", 10);
    println!("{}", call_greeting1);

    let call_greeting2 = gree_ting("Actack", 3);
    println!("{}", call_greeting2);
}

fn gree_ting(task: &str, time: i32) -> String{
   format!("task: {} time: {}", task,time)
}
