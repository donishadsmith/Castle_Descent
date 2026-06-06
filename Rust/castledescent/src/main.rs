use castledescent::castle::Castle;

// Will be used as an initializer of a few things
fn initialize() -> Castle {
    let castle = Castle::generate();

    castle
}

fn main() {
    let castle = initialize();
    //println!("{:?}", castle.layout);
    for i in 0..castle.width {
        for j in 0..castle.depth {
            for k in 0..castle.floors {
                println!("{}", castle.check_object(i, j, k));
            }
        }
    }
    let mut x = vec![1, 1, 1];
    let mut y = &mut x;
    let z = &mut y;
    (*y)[0] += 1;
    println!("{:?}", x)
}
