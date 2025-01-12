use control_system::normal::System;
use player::normal::Player;

mod control_system;
mod pai;
mod player;
fn main() {
    println!("Hello, world!");
    let mut pai_dui = pai::normal_version::Pai::new();
    let mut player1 = Player::new();
    let mut player2 = Player::new();
    let mut player3 = Player::new();
    player3.is_dz = true;
    let player_vec: Vec<Player> = vec![player1.clone(), player2.clone(), player3.clone()];
    let mut s = System::new();
    s.set_player(player_vec);
    let player1pai = pai_dui.asign_pai_no_shuffle(17);
    let player2pai = pai_dui.asign_pai_no_shuffle(17);
    let player3pai = pai_dui.asign_pai_no_shuffle(17);
    player1.put(&player1pai);
    //    player1.put_by_s(&"333344445555666677778888".to_string());
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
    println!("玩家一可以出的单张{:?}", player1.have_one());
    println!("玩家一可以出的对子{:?}", player1.have_double());
    println!("玩家一可以出的三张{:?}", player1.have_three());
    println!("玩家一可以出的炸弹{:?}", player1.have_boom());
    println!("玩家一可以出的三带1{:?}", player1.have_three_and_one());
    println!("玩家一可以出的三带2{:?}", player1.have_three_and_double());
    println!("玩家一可以出的三带3{:?}", player1.have_three_and_three());
    println!("玩家一可以出的三带3带3{:?}", player1.have_ttt());
    println!("玩家一可以出的三带3带3带3{:?}", player1.have_tttt());
    println!("玩家一可以出的三带3带3带3带3{:?}", player1.have_ttttt());
    println!("玩家一可以出的aaabbbcccdef{:?}", player1.have_tttooo());
    s.start();
    // println!("玩家一可以出的对子{:?}",player1.have_double());
}
