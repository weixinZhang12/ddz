mod pai;
fn main() {
    println!("Hello, world!");
    let mut p = pai::pai::Pai::new();
    let mut pl1 = p.asign_pai(17);
    let mut pl2 = p.asign_pai(17);
    let mut pl3 = p.asign_pai(17);
    println!("{:?}", pl1);
    println!("{:?}", pl2);
    println!("{:?}", pl3);
    p.sort(&mut pl1);
    p.sort(&mut pl2);
    p.sort(&mut pl3);
    println!("{:?}", pl1);
    println!("{:?}", pl2);
    println!("{:?}", pl3);
    // println!("{:?}", p.unnormal_pai);
    // println!("{:?}", p.normal_pai);
}
