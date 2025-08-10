use jcalgs::unionfind::{make_set, to_dot_string, union};

fn main() {
    let set = make_set(vec!["A", "B", "C", "D", "E", "F", "G"]);

    union(set[0].clone(), set[1].clone());
    union(set[1].clone(), set[2].clone());
    union(set[2].clone(), set[1].clone());
    union(set[3].clone(), set[4].clone());
    union(set[5].clone(), set[6].clone());

    // The following line enables quick visualization with:
    // `cargo run` | dot -Tsvg > output.svg && firefox output.svg`
    println!("digraph {}", to_dot_string(set));
}
