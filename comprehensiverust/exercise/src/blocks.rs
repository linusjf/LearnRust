pub fn main() {
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = { 3 + 4 };
            println!("w: {w}");
            y * w
        };
        println!("z: {z}");
        z - y
    };
    println!("x: {x}");
    println!("doubled: {}", double(7));
}

fn double(x: i32) -> i32 {
    x + x
}
