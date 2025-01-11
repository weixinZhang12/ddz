pub mod normal_version {
    use std::{cmp::Ordering, collections::HashMap};

    use rand::Rng;
    pub type PaiVec = Vec<Option<char>>;

    pub struct Pai {
        ///正常牌
        pub normal_pai:PaiVec,
        ///小王和和大王
        pub cu_map: HashMap<char, usize>,
        pub uc_map: HashMap<usize, char>,
    }
    impl Pai {
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
            Pai {
                normal_pai: vec![
                    Some('A'),
                    Some('2'),
                    Some('3'),
                    Some('4'),
                    Some('5'),
                    Some('6'),
                    Some('7'),
                    Some('8'),
                    Some('9'),
                    Some('T'),
                    Some('J'),
                    Some('Q'),
                    Some('K'),
                    Some('A'),
                    Some('2'),
                    Some('3'),
                    Some('4'),
                    Some('5'),
                    Some('6'),
                    Some('7'),
                    Some('8'),
                    Some('9'),
                    Some('T'),
                    Some('J'),
                    Some('Q'),
                    Some('K'),
                    Some('A'),
                    Some('2'),
                    Some('3'),
                    Some('4'),
                    Some('5'),
                    Some('6'),
                    Some('7'),
                    Some('8'),
                    Some('9'),
                    Some('T'),
                    Some('J'),
                    Some('Q'),
                    Some('K'),
                    Some('A'),
                    Some('2'),
                    Some('3'),
                    Some('4'),
                    Some('5'),
                    Some('6'),
                    Some('7'),
                    Some('8'),
                    Some('9'),
                    Some('T'),
                    Some('J'),
                    Some('Q'),
                    Some('K'),
                    Some('W'),
                    Some('w'),
                ],
                cu_map,
                uc_map,
            }
        }
        ///分配牌
        pub fn asign_pai(&mut self, _length: usize) -> PaiVec {
            let mut rng = rand::thread_rng();
            let mut temp: PaiVec = vec![];
            // 抽取牌直到满17张
            while temp.len() < 17 {
                let row: usize = rng.gen_range(0..54);
                let col: usize = rng.gen_range(0..13);
                if self.normal_pai[row].is_some() {
                    temp.push(self.normal_pai[row].take());
                    continue;
                }
              
            }
            temp
        }
        pub fn compare_pai(&self, a: char, b: char) -> Ordering {
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
        ///将牌堆牌序
        pub fn sort(&self) {
            // pai_d.sort_by(|a, b| self.compare_pai(a.unwrap(), b.unwrap()));
        }
        ///返回底牌
        pub fn remain_pai(&mut self) -> PaiVec {
            let mut temp: PaiVec = vec![];
            for i in &mut self.normal_pai {
                    if i.is_some() {
                        temp.push(i.take());
                    }
            }

            temp
        }
        ///牌堆转换为字符串
        pub fn pai_to_string(&self) -> String {
            let mut s = String::new();
            for i in &self.normal_pai {
                    if i.is_some() {
                        s.push(i.unwrap());
                    }
            }

            s
        }
        ///旧方法，现在即将废弃
        pub fn _duizi_f(&self, s: String) -> PaiVec {
            let mut pattern = String::new();
            let mut array: PaiVec = vec![];
            for i in 3..=15 {
                let a = self.uc_map.get(&i).unwrap();
                pattern.push(*a);
                pattern.push(*a);
                if s.contains(&pattern) {
                    array.push(Some(*a));
                }
                pattern = "".to_string();
            }
            array
        }
        ///查看牌堆的牌数
        pub fn stat_player_pai(&self) -> HashMap<char, usize> {
            let mut map: HashMap<char, usize> = HashMap::new();
            for i in &self.normal_pai {
                    if let Some(i1) = i {
                        if let Some(count) = map.get_mut(&i1) {
                            *count += 1;
                        } else {
                            map.insert(*i1, 1);
                        }
                    }
            }

            map
        }
    }
}
