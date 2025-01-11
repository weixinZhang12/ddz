use player::normal::Player;

mod pai;
mod player;
fn main() {
    println!("Hello, world!");
    let mut pai_dui = pai::normal_version::Pai::new();
    let mut player1 = Player::new();
    let mut player2 = Player::new();
    let mut player3 = Player::new();
    let player1pai = pai_dui.asign_pai(17);
    let player2pai = pai_dui.asign_pai(17);
    let player3pai = pai_dui.asign_pai(17);
    player1.put(&player1pai);
    player2.put(&player2pai);
    player3.put(&player3pai);
    println!("底牌为：{:?}", pai_dui.pai_to_string());

    player3.add(&pai_dui.remain_pai());
    player1.sort();
    player2.sort();
    player3.sort();

    println!("玩家1排序后：{:?}", player1.pai_to_string());
    println!("玩家2排序后：{:?}", player2.pai_to_string());
    println!("玩家3排序后：{:?}", player3.pai_to_string());
    let dz = &player1.stat_pai;
    println!("玩家1手牌统计{:?}", dz);
    player1.del("459".to_string());
    println!("玩家1出牌后：{:?}", player1.pai_to_string());
    println!("玩家1手牌数量：{:?}", player1.len);
    println!("玩家一可以出的单张{:?}",player1.have_one());
    println!("玩家一可以出的对子{:?}",player1.have_double());
    println!("玩家一可以出的三张{:?}",player1.have_three());
    println!("玩家一可以出的炸弹{:?}",player1.have_boom());
    println!("玩家一可以出的三带{:?}",player1.have_three_and_one());
    // println!("玩家一可以出的对子{:?}",player1.have_double());

}
