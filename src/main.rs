fn main () {
    let name = "hatamochi";
    let level: u32 = 33;
    let hitpoint : f32 = 75.5;
    let flg : bool = true;

    println!("名前：{} レベル：{} 体力：{} 生存フラグ：{}",name, level, hitpoint, flg);

    if level >= 30 {
        println!("あなたはベテランプレイヤー");
    } else {
        println!("まだ始まったばかり");
    }

    let mut count = 0;
    loop {
        count += 1;
        if count == 3{
            println!("ループ終了");
            break;
        }
    }
}