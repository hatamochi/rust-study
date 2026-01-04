fn main() {
    // 1. 名前を定義（後で変更はしません）
    let name = "hatamochi";

    // 2. 体力を定義（途中で減らしたいです）
    let mut hp: i32 = 100;

    println!("プレイヤー：{}、残りHP：{}", name, hp);

    // ダメージを受ける関数を呼び出す
    let new_hp = take_damage(hp);

    println!("ダメージを受けた！ 残りHP：{}", new_hp);
}

// ダメージを計算する関数（引数から10引いた値を返したい）
fn take_damage(current_hp: i32) -> i32 {
    current_hp - 10
}