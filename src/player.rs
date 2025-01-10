pub mod normal {
    use std::{cmp::Ordering, collections::HashMap};
    pub struct Player {
        pub pai: Vec<Option<char>>,
        pub stat_pai: HashMap<char, usize>,
        pub cu_map: HashMap<char, usize>,
        pub uc_map: HashMap<usize, char>,
    }
    impl Player {
        pub fn new() -> Self {
            let mut cu_map: HashMap<char, usize> = HashMap::new();
            cu_map.insert('3', 3);
            cu_map.insert('4', 4);
            cu_map.insert('5', 5);
            cu_map.insert('6', 6);
            cu_map.insert('7', 7);
            cu_map.insert('8', 8);
            cu_map.insert('9', 9);
            cu_map.insert('T', 10);
            cu_map.insert('J', 11);
            cu_map.insert('Q', 12);
            cu_map.insert('K', 13);
            cu_map.insert('A', 14);
            cu_map.insert('2', 15);
            cu_map.insert('W', 16);
            cu_map.insert('w', 17);
            let mut uc_map: HashMap<usize, char> = HashMap::new();
            uc_map.insert(3, '3');
            uc_map.insert(4, '4');
            uc_map.insert(5, '5');
            uc_map.insert(6, '6');
            uc_map.insert(7, '7');
            uc_map.insert(8, '8');
            uc_map.insert(9, '9');
            uc_map.insert(10, 'T');
            uc_map.insert(11, 'J');
            uc_map.insert(12, 'Q');
            uc_map.insert(13, 'K');
            uc_map.insert(14, 'A');
            uc_map.insert(15, '2');
            uc_map.insert(16, 'W');
            uc_map.insert(17, 'w');
            Player {
                pai: Vec::new(),
                stat_pai: HashMap::new(),
                cu_map,
                uc_map,
            }
        }
        ///修改玩家手牌
        pub fn put(&mut self, v: &Vec<Option<char>>) {
            self.pai = v.clone();
            self.stat_player_pai();
        }
        pub fn add(&mut self, v: &Vec<Option<char>>) {
            self.pai.extend(v);
            self.stat_player_pai();
        }
        ///统计玩家手牌
        pub fn stat_player_pai(&mut self) {
            let mut map: HashMap<char, usize> = HashMap::new();
            for i in &self.pai {
                if let Some(i1) = i {
                    if let Some(count) = map.get_mut(&i1) {
                        *count += 1;
                    } else {
                        map.insert(*i1, 1);
                    }
                }
            }
            self.stat_pai = map
        }
        ///比较函数
        pub fn _compare_pai(&self, a: char, b: char) -> Ordering {
            let a = self.cu_map.get(&a).unwrap();
            let b = self.cu_map.get(&b).unwrap();
            if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
        /// 将玩家的手牌排序
        pub fn sort(&mut self) {
            let cu_map = &self.cu_map;
            self.pai.sort_by(|a, b| {
                let a = cu_map.get(&a.unwrap()).unwrap();
                let b = cu_map.get(&b.unwrap()).unwrap();
                a.cmp(b)
            });
        }
        ///玩家手牌转字符串
        pub fn pai_to_string(&self) -> String {
            let mut s = String::new();
            for i in &self.pai {
                if i.is_some() {
                    s.push(i.unwrap());
                }
            }
            s
        }
    }
}
