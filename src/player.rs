pub mod normal {
    use crate::pai::{
        normal_version::{Pai, PaiVec},
        pai_p::{remove_duplicates, PaiError},
    };
    use std::{cmp::Ordering, collections::HashMap, error::Error};
    #[derive(Clone, Debug)]
    pub struct Player {
        pub pai: PaiVec,
        pub stat_pai: HashMap<char, usize>,
        pub cu_map: HashMap<char, usize>,
        pub uc_map: HashMap<usize, char>,
        pub len: usize,
        pub is_dz: bool,
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
            cu_map.insert('w', 16);
            cu_map.insert('W', 17);
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
                len: 0,
                is_dz: false,
            }
        }
        ///修改玩家手牌
        pub fn put(&mut self, v: &PaiVec) {
            self.pai = v.clone();
            self.stat_player_pai();
            self.auto_mod_len();
        }
        ///通过字符串修改手牌
        pub fn put_by_s(&mut self, v: &String) {
            let mut pai: PaiVec = vec![];
            for i in v.chars() {
                pai.push(Some(i));
            }
            self.pai = pai;
            self.stat_player_pai();
            self.auto_mod_len();
        }
        ///添加手牌
        pub fn add(&mut self, v: &PaiVec) {
            self.pai.extend(v);
            self.stat_player_pai();
            self.auto_mod_len();
        }
        /// 自动修改手牌数量
        pub fn auto_mod_len(&mut self) {
            let len = self.get_length();
            self.len = len
        }
        /// 获取手牌数
        pub fn get_length(&self) -> usize {
            self.pai_to_string().len()
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
        /// 寻找最小的牌
        pub fn find_min(&self) -> Option<String> {
            let one = self.have_one();
            if let Some(v) = one.iter().min() {
                return Some(v.clone());
            }
            None
        }
        // 尝试压死对手，没有可以出的牌，返回None
        pub fn chooose_by_second(&self, s: &Option<String>) -> Option<String> {
            if let Some(v) = s {
                let a = self.cu_map.get(&v.chars().next().unwrap()).unwrap();
                for i in self.have_one() {
                    let b = self.cu_map.get(&i.chars().next().unwrap()).unwrap();
                    if b > a {
                        return Some(i);
                    }
                }
            } else {
                let min = self.find_min();
                if min.is_some() {
                    return min;
                }
            }
            None
        }

        ///打出牌，无法打出的时候会返回一个错误
        pub fn del(&mut self, s: String) -> Result<String, PaiError> {
            let c_arr: Vec<char> = s.chars().collect();
            // 创建副本，如果在副本尝试出牌无法打出，直接放弃，否则才会修改真正的手牌
            let mut t = self.stat_pai.clone();
            let mut temp_pai = self.pai.clone();
            // 遍历玩家输入的
            for i in c_arr.iter() {
                let get_value = t.get(i);
                // 如果手里有这张牌
                if get_value.unwrap_or(&0) > &0 {
                    // 遍历玩家的牌，找到这张牌
                    for j in &mut temp_pai {
                        if let Some(v) = j {
                            if *v == *i {
                                *j = None;
                                t.insert(*i, get_value.unwrap() - 1).unwrap();
                                break;
                            }
                        }
                    }
                } else {
                    return Err(PaiError::NotFoundPai(
                        "无法找到牌，因此该牌型无法打出".to_string(),
                    ));
                }
            }
            for i in c_arr {
                let t = &self.stat_pai;
                if t.get(&i).unwrap() > &0 {
                    for j in &mut self.pai {
                        if let Some(v) = j {
                            if *v == i {
                                *j = None;
                                self.stat_pai
                                    .insert(i, *self.stat_pai.get(&i).unwrap() - 1)
                                    .unwrap();

                                break;
                            }
                        }
                    }
                }
            }
            self.auto_mod_len();
            Ok(s)
        }
        ///获取所有单张的牌
        pub fn have_one_repeated(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];
            println!("{:?}", self.stat_pai);
            for i in &self.stat_pai {
                let mut temp2: String = String::new();
                for j in 0..*i.1 {
                    temp.push(i.0.to_string());
                }
            }
            temp
        }

        ///获取所有单张的牌，不重复
        pub fn have_one(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];
            for i in &self.stat_pai {
                if *i.1 >= 1 {
                    let mut temp2: String = String::new();
                    temp2.push(*i.0);
                    temp.push(temp2);
                }
            }
            temp
        }
        ///获取所有对子牌
        pub fn have_double(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];
            for i in &self.stat_pai {
                if *i.1 >= 2 {
                    let mut temp2: String = String::new();
                    temp2.push(*i.0);
                    temp2.push(*i.0);
                    temp.push(temp2);
                }
            }
            temp
        }
        ///获取所有三张的牌
        pub fn have_three(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];
            for i in &self.stat_pai {
                if *i.1 >= 3 {
                    let mut temp2: String = String::new();
                    temp2.push(*i.0);
                    temp2.push(*i.0);
                    temp2.push(*i.0);
                    temp.push(temp2);
                }
            }
            temp
        }
        ///获取所有三带一的牌
        pub fn have_three_and_one(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];

            let mut three = self.have_three();
            let mut one = self.have_one();
            for i in three {
                for j in &one {
                    if &i[0..1] != j {
                        temp.push(i.clone() + j);
                    }
                }
            }
            temp
        }
        /// 获取三带二
        pub fn have_three_and_double(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];

            let mut three = self.have_three();
            let mut one = self.have_double();
            for i in three {
                for j in &one {
                    if &i[0..1] != &j[0..1] {
                        temp.push(i.clone() + j);
                    }
                }
            }
            temp
        }
        /// 获取三带二
        pub fn have_three_and_three(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];

            let mut three = self.have_three();
            let mut one = self.have_three();
            for i in three {
                for j in &one {
                    if i == "2" || j == "2" {
                        continue;
                    }
                    let i1 = self.cu_map.get(&i.chars().next().unwrap()).unwrap();
                    let j1 = self.cu_map.get(&j.chars().next().unwrap()).unwrap();
                    if (*i1 as isize) - (*j1 as isize) == 1 || (*i1 as isize) - (*j1 as isize) == -1
                    {
                        temp.push(i.clone() + j);
                    }
                }
            }
            temp
        }
        /// 三带三带三
        pub fn have_ttt(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];

            let mut three = self.have_three();
            let mut three2 = self.have_three();
            let mut three3 = self.have_three();
            for i in &three {
                for j in &three2 {
                    for k in &three3 {
                        if i == j || j == k || i == k {
                            continue;
                        }
                        if i == "2" || j == "2" || k == "2" {
                            continue;
                        }
                        let i1 = *self.cu_map.get(&i.chars().next().unwrap()).unwrap() as isize;
                        let j1 = *self.cu_map.get(&j.chars().next().unwrap()).unwrap() as isize;
                        let k1 = *self.cu_map.get(&k.chars().next().unwrap()).unwrap() as isize;
                        if (i1 - j1 == 1 || i1 - j1 == -1) && (j1 - k1 == 1 || j1 - k1 == -1)
                        // && (i1 - k1 == 1 || i1 - k1 == -1)
                        {
                            temp.push(i.clone() + j + k);
                        }
                    }
                }
            }
            temp
        }
        /// 三带三带三带三
        pub fn have_tttt(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];

            let mut three = self.have_three();
            let mut three2 = self.have_three();
            let mut three3 = self.have_three();
            let mut three4 = self.have_three();
            for i in &three {
                for j in &three2 {
                    for k in &three3 {
                        for l in &three4 {
                            if i == j || i == k || i == l || j == k || j == l || k == l {
                                continue;
                            }
                            if i == "2" || j == "2" || k == "2" || l == "2" {
                                continue;
                            }
                            let i1 = *self.cu_map.get(&i.chars().next().unwrap()).unwrap() as isize;
                            let j1 = *self.cu_map.get(&j.chars().next().unwrap()).unwrap() as isize;
                            let k1 = *self.cu_map.get(&k.chars().next().unwrap()).unwrap() as isize;
                            let l1 = *self.cu_map.get(&l.chars().next().unwrap()).unwrap() as isize;
                            if (i1 - j1 == 1 || i1 - j1 == -1)
                                && (j1 - k1 == 1 || j1 - k1 == -1)
                                && (l1 - k1 == 1 || l1 - k1 == -1)
                            {
                                temp.push(i.clone() + j + k + l);
                            }
                        }
                    }
                }
            }
            temp
        }
        /// 三带三带三带三带三
        pub fn have_ttttt(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];

            let mut three = self.have_three();
            let mut three2 = self.have_three();
            let mut three3 = self.have_three();
            let mut three4 = self.have_three();
            let mut three5 = self.have_three();
            for i in &three {
                for j in &three2 {
                    for k in &three3 {
                        for l in &three4 {
                            for u in &three5 {
                                if i == j
                                    || i == k
                                    || i == l
                                    || i == u
                                    || j == k
                                    || j == l
                                    || j == u
                                    || k == l
                                    || k == u
                                {
                                    continue;
                                }
                                if i == "2" || j == "2" || k == "2" || l == "2" || u == "2" {
                                    continue;
                                }
                                let i1 =
                                    *self.cu_map.get(&i.chars().next().unwrap()).unwrap() as isize;
                                let j1 =
                                    *self.cu_map.get(&j.chars().next().unwrap()).unwrap() as isize;
                                let k1 =
                                    *self.cu_map.get(&k.chars().next().unwrap()).unwrap() as isize;
                                let l1 =
                                    *self.cu_map.get(&l.chars().next().unwrap()).unwrap() as isize;
                                let u1 =
                                    *self.cu_map.get(&u.chars().next().unwrap()).unwrap() as isize;
                                if (i1 - j1 == 1 || i1 - j1 == -1)
                                    && (j1 - k1 == 1 || j1 - k1 == -1)
                                    && (l1 - k1 == 1 || l1 - k1 == -1)
                                    && (u1 - l1 == 1 || u1 - l1 == -1)
                                {
                                    temp.push(i.clone() + j + k + l + u);
                                }
                            }
                        }
                    }
                }
            }
            temp
        }
        /// 飞机
        pub fn have_tttooo(&self) -> Vec<String> {
            let ttt: Vec<String> = self.have_ttt();
            let one: Vec<String> = self.have_one();
            let mut three_head: Vec<&str> = vec![];
            let mut temp: Vec<String> = vec![];
            for i in &ttt {
                let t = remove_duplicates(Some(i.clone()));
            }

            temp
        }
        /// 炸弹
        pub fn have_boom(&self) -> Vec<String> {
            let mut temp: Vec<String> = vec![];
            for i in &self.stat_pai {
                if *i.1 == 4 {
                    let mut temp2: String = String::new();
                    temp2.push(*i.0);
                    temp2.push(*i.0);
                    temp2.push(*i.0);
                    temp2.push(*i.0);
                    temp.push(temp2);
                }
            }
            temp
        }
    }
}
