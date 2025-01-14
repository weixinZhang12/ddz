pub mod normal {
    use crate::{
        pai::normal_version::{Pai, PaiVec},
        player::{self, normal::Player},
    };
    ///游戏控制系统
    /// 系统的功能主要有：
    /// 1、自动创建所有玩家
    /// 2、设置玩家数量
    /// 3、向所有玩家分发手牌
    /// 4、地主牌分发（分发底牌）
    /// 5、收集玩家打出的所有牌
    /// 6、自动按照顺序让玩家出牌
    /// 7、开始打牌
    #[derive(Debug)]
    pub struct System<'a> {
        // 储存游玩的玩家
        player: Vec<Player>,
        // 垃圾篓，玩家打出的牌会出现在这里
        rubi: PaiVec,
        // 玩家上次打出牌,例如AAA，Player1
        last_pai: (Option<String>, Option<&'a Player>),
        pai: Pai,
    }
    impl<'a> System<'a> {
        /// 创建系统，参数为几副牌，例如1即为1副牌
        pub fn new(count: usize) -> Self {
            System {
                player: Vec::new(),
                rubi: Vec::new(),
                last_pai: (None, None),
                pai: Pai::new(count),
            }
        }
        /// 设置玩家的数量，如果玩的是斗地主，请将其设置为3
        pub fn set_player_count(&mut self, count: usize) {
            for _ in 0..count {
                let player = Player::new();
                self.player.push(player);
            }
        }
        pub fn sort_all_player(&mut self) {
            for player in &mut self.player {
                player.sort();
            }
        }
        ///返回当前所有玩家
        pub fn get_all_player(&self) -> &Vec<Player> {
            return &self.player;
        }
        ///给所有玩家分配牌
        pub fn assign_pai_for_all_player(&mut self, count: usize) {
            for player in &mut self.player {
                let pai = self.pai.asign_pai(count);
                player.put(&pai);
            }
        }
        /// 设置玩家
        pub fn set_player(&mut self, v: Vec<Player>) {
            self.player = v;
        }
        /// 清除垃圾篓
        pub fn clear_rubi(&mut self) {
            self.rubi.clear();
        }
        /// 开始游戏
        pub fn play(&self) {}
    }
}
