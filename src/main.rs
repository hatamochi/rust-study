fn main() {
    let a = String::from("Rust");
    print_message(&a);//貸与
    println!("もう一回：{}", a);
}

fn print_message(meg: &String){
    println!("メッセージを受け取りました：{}", meg);
}