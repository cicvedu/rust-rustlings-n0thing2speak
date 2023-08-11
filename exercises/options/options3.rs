// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main(){
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), //ref的意思就是不消耗some的所有权，这样上面的y之后还可以正常使用
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
