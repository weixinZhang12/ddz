use control_system::normal::System;
use pai::{
    normal_version::Pai,
    pai_p::{pattern, remove_duplicates, s_to_pattern},
};
use player::normal::Player;

pub mod control_system;
pub mod pai;
pub mod player;
fn main() {
    println!("Hello, world!");
    let mut s: System = System::new(1);
    let mut p = Pai::new(1);
    let a = p.is_shunzi("444333".to_string());
    println!("{}", a);
    s.set_player_count(3);
    s.assign_pai_for_all_player(17);
    s.sort_all_player();
    println!("{:#?}", s);
    let s = pattern(s_to_pattern(Some("33344".to_string())));
    let d=s_to_pattern(Some("33445566778899TTJJ".to_string()));
    let b = remove_duplicates(s_to_pattern(Some("333444".to_string())));
    println!("{:?}", s);
    println!("{:?}", d);
    println!("{:?}", b);
}
