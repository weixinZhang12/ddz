pub mod pai {
    use std::{cmp::Ordering, collections::HashMap};

    use rand::Rng;

    pub struct Pai {
        ///正常牌
        pub normal_pai: Vec<Vec<Option<char>>>,
        ///小王和和大王
        pub unnormal_pai: Vec<Option<char>>,
    }
    impl Pai {
        pub fn new() -> Self {
            Pai {
                normal_pai: vec![
                    vec![
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
                    ],
                    vec![
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
                    ],
                    vec![
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
                    ],
                    vec![
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
                    ],
                ],
                unnormal_pai: vec![Some('W'), Some('w')],
            }
        }
        pub fn asign_pai(&mut self, length: usize) -> Vec<Option<char>> {
            let mut rng = rand::thread_rng();

            let mut temp: Vec<Option<char>> = vec![];
            while temp.len() < 17 {
                let row: usize = rng.gen_range(0..4);
                let col: usize = rng.gen_range(0..13);
                if self.normal_pai[row][col].is_some() {
                    temp.push(self.normal_pai[row][col].take());
                }
                let take_unnor = rng.gen_bool(0.019);
                if take_unnor == true && temp.len() < 17 {
                    let i = rng.gen_range(0..2);
                    if self.unnormal_pai[i].is_some() {
                        temp.push(self.unnormal_pai[i].take());
                    }
                }
            }
            temp
        }
        pub fn com(&self, a: char, b: char) -> Ordering {
            let mut map: HashMap<char, usize> = HashMap::new();
            map.insert('3', 3);
            map.insert('4', 4);
            map.insert('5', 5);
            map.insert('6', 6);
            map.insert('7', 7);
            map.insert('8', 8);
            map.insert('9', 9);
            map.insert('T', 10);
            map.insert('J', 11);
            map.insert('Q', 12);
            map.insert('K', 13);
            map.insert('A', 14);
            map.insert('2', 15);
            map.insert('W', 16);
            map.insert('w', 17);
            println!("{}",a);
            println!("{}",b);
            let a = map.get(&a).unwrap();
            let b = map.get(&b).unwrap();
            if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }

        pub fn sort(&self, pai_d:&mut  Vec<Option<char>>) {
            pai_d.sort_by(|a, b| self.com(a.unwrap(), b.unwrap()));
        }
    }
}
