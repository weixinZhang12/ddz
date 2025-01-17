pub mod normal_version {
    use std::{cmp::Ordering, collections::HashMap};

    use rand::Rng;
    pub type PaiVec = Vec<Option<char>>;
    #[derive(Debug)]

    pub struct Pai {
        ///正常牌
        pub normal_pai: PaiVec,
        ///小王和和大王
        pub cu_map: HashMap<char, usize>,
        pub uc_map: HashMap<usize, char>,
    }
    impl Pai {
        // 创建牌堆，参数为几副牌，例如1，为1副牌
        pub fn new(pai_c: usize) -> Self {
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
            let pai = vec![
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
            ];
            let mut normal_pai: PaiVec = vec![];
            for _ in 0..pai_c {
                for p in pai.clone() {
                    normal_pai.push(p);
                }
            }
            Pai {
                normal_pai,
                cu_map,
                uc_map,
            }
        }
        ///分配牌
        pub fn asign_pai(&mut self, _length: usize) -> PaiVec {
            let mut rng = rand::thread_rng();
            let mut temp: PaiVec = vec![];
            // 抽取牌直到满17张
            while temp.len() < _length {
                let row: usize = rng.gen_range(0..54);
                if self.normal_pai[row].is_some() {
                    temp.push(self.normal_pai[row].take());
                    continue;
                }
            }
            temp
        }
        ///不洗牌玩法
        pub fn asign_pai_no_shuffle(&mut self, _length: usize) -> PaiVec {
            let mut rng = rand::thread_rng();
            let mut temp: PaiVec = vec![];
            // 抽取牌直到满17张
            while temp.len() < _length {
                let row: usize = rng.gen_range(0..54);
                let m: usize = rng.gen_range(1..3);
                if self.normal_pai[row].is_some() {
                    let p = self.normal_pai[row].take();
                    temp.push(p);
                    for i in 0..m {
                        // 如果抽够17张牌的话直接退出
                        if temp.len() == _length {
                            break;
                        }
                        let u = self.normal_pai.iter().position(|x| *x == p);
                        if u.is_some() {
                            temp.push(self.normal_pai[u.unwrap()].take());
                        }
                    }

                    continue;
                }
            }
            temp
        }
        // 比较牌
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
        pub fn is_shunzi(&self, s: String) -> bool {
            if s.len() < 5 {
                return false;
            }
            for w in s.chars().collect::<Vec<char>>().windows(2) {
                if let [a, b] = w {
                    if self.cu_map.get(a) > self.cu_map.get(b) {
                        return false;
                    }
                }
            }

            true
        }
    }
}

pub mod pai_p {
    use std::collections::{HashMap, HashSet};

    use super::normal_version::PaiVec;
    #[derive(Debug)]
    ///提取共性，牌型可以分为以下几种
    pub enum PaiLx {
        ///单张
        Dan,
        ///递增
        Raise,
        ///相等
        Equal,
        ///递增和相等
        RaiseEq,
        ///相等和不等
        RaiseUnEq,
        ///相等与相等
        EqualEq,
        ///相等与不等
        EqualUnEq,
        Feiji,
        ///王炸
        Wang,
    }

    #[derive(Debug)]
    // 可能的错误
    pub enum PaiError {
        NotFoundPai(String),
    }
    // 统计牌
    pub fn stat_pai(v: PaiVec) -> HashMap<char, usize> {
        let mut map: HashMap<char, usize> = HashMap::new();
        for i in &v {
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
    // 获取牌种
    pub fn remove_duplicates(input: Option<String>) -> Option<String> {
        let mut seen = HashSet::new();
        if input.is_some() {
            let s = input
                .unwrap()
                .chars()
                .filter(|&c| seen.insert(c)) // 只有第一次见到的字符会被插入
                .collect();
            return Some(s);
        }
        None
    }
    /// 模式匹配
    pub fn s_to_pattern(s: Option<String>) -> Option<String> {
        let mut result = String::new();
        let mut prev_char = None; // 记录前一个字符
        let mut group_index = 0; // 记录当前分组的索引
        if s == None {
            return None;
        }
        for c in s.unwrap().chars() {
            if Some(c) != prev_char {
                // 如果当前字符与前一个字符不同，切换到下一个分组
                group_index += 1;
                if group_index > 26 {
                    panic!("超过字母表范围！"); // 防止分组超出 'z'
                }
            }
            // 追加对应的字母
            result.push((b'a' + (group_index - 1) as u8) as char);
            prev_char = Some(c);
        }

        Some(result)
    }
    ///定义所有的牌型
    pub fn pattern(s: Option<String>) -> Option<PaiLx> {
        if let Some(s) = s {
            return match s.as_str() {
                "a" => Some(PaiLx::Dan),
                "aa" => Some(PaiLx::Equal),
                "aaa" => Some(PaiLx::Equal),
                "aaaa" => Some(PaiLx::Equal),
                "aaaabc" => Some(PaiLx::EqualUnEq),
                "aaab" => Some(PaiLx::EqualUnEq),
                "aaabb" => Some(PaiLx::EqualEq),
                "aaabbbab" => Some(PaiLx::Feiji),
                "aaabbbccdd" => Some(PaiLx::Feiji),
                "abcde" => Some(PaiLx::Raise),
                "abcdef" => Some(PaiLx::Raise),
                "abcdefg" => Some(PaiLx::Raise),
                "abcdefgh" => Some(PaiLx::Raise),
                "abcdefghi" => Some(PaiLx::Raise),
                "abcdefghij" => Some(PaiLx::Raise),
                "abcdefghijk" => Some(PaiLx::Raise),
                "abcdefghijkl" => Some(PaiLx::Raise),
                "aabbcc" => Some(PaiLx::RaiseEq),
                "aabbccdd" => Some(PaiLx::RaiseEq),
                "aabbccddee" => Some(PaiLx::RaiseEq),
                "aabbccddeeff" => Some(PaiLx::RaiseEq),
                "aabbccddeeffgg" => Some(PaiLx::RaiseEq),
                "aabbccddeeffgghh" => Some(PaiLx::RaiseEq),
                "aabbccddeeffgghhii" => Some(PaiLx::RaiseEq),
                "aabbccddeeffgghhiijj" => Some(PaiLx::RaiseEq),
                "aabbccddeeffgghhiijjkk" => Some(PaiLx::RaiseEq),
                "aabbccddeeffgghhiijjkkll" => Some(PaiLx::RaiseEq),
                // 此情况仅限于大小王
                "ab" => Some(PaiLx::Wang),
                _ => None,
            };
        }
        None
    }
    ///判断是否是对子
    pub fn is_duizi(s: String) -> bool {
        // 不可能出错，因为大于2
        if s.len() >= 2 {
            let s = remove_duplicates(Some(s)).unwrap();
            if s.len() == 1 {
                return true;
            }
        }
        false
    }
}
