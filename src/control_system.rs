pub mod normal {
    use crate::{pai::normal_version::PaiVec, player::normal::Player};

    pub struct System {
        player: Vec<Player>,
        rubi: PaiVec,
        last_pai: Option<String>,
    }
    impl System {
        pub fn new()->Self{
            System{
                player:Vec::new(),
                rubi:Vec::new(),
                last_pai:None
            }
        }
        pub fn set_player(&mut self,v:Vec<Player>){
            self.player=v;
        }
        pub fn clear_rubi(&mut self) {
            self.rubi.clear();
        }

        pub fn start(&mut self) {
            let mut flag: bool = true;
            let mut is_find_dz:bool=false;
            for i in &mut self.player {
               
                if i.is_dz {
                    is_find_dz=true;
                    let min = i.find_min();
                    if min.is_some() {
                        self.last_pai = Some(i.del(min.unwrap()).unwrap());
                        println!("{:?}",self.last_pai);
                        continue;
                    }
                }
                if is_find_dz==false {
                   continue; 
                }
                let sc = i.chooose_by_second(&self.last_pai);
                self.last_pai = Some(i.del(sc.unwrap()).unwrap());
                println!("{:?}",self.last_pai);

            }
            while flag {
                for player in &mut self.player {
                    let sc = player.chooose_by_second(&self.last_pai);
                    self.last_pai = Some(player.del(sc.unwrap()).unwrap());
                    println!("{:?}",self.last_pai);

                }
            }
        }
    }
}
