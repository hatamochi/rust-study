fn main () {
    let mut level : u32 = 33;
    println!("今のレベル：{}",  level);

    level = level_up(level); //関数を呼び出して結果を代入

    println!("レベルアップ！：{}", level);
}

fn level_up(level : u32) -> u32 {
    level + 1
}