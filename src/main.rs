use std::fmt::Display;

use jcalgs::unionfind::{make_set, to_dot_string, union};

#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x:{},y:{})", self.x, self.y)
    }
}

fn main() {
    let set = make_set(vec![
        Point { x: 0, y: 0},
        Point { x: 1, y: 0},
        Point { x: 2, y: 0},
        Point { x: 0, y: 1},
        Point { x: 1, y: 1},
        Point { x: 2, y: 1},
    ]);

    union(set[0].clone(), set[1].clone());
    union(set[1].clone(), set[2].clone());
    union(set[2].clone(), set[1].clone());
    union(set[3].clone(), set[4].clone());

    // The following line enables quick visualization with:
    // `cargo run` | dot -Tsvg > output.svg && firefox output.svg`
    println!("digraph {}", to_dot_string(set));
}
