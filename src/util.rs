pub fn get_peano_num(nat_num: usize) -> String {
    let mut s = "".to_string();

    for _ in 0..nat_num {
        s += "S(";
    }
    s += "Z";
    for _ in 0..nat_num {
        s += ")";
    }
    s
}

pub fn get_depth_space(depth: usize) -> String {
    let mut s = "".to_string();
    for _ in 0..depth {
        s += " ";
    }
    s
}
