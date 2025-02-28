use fluids::*;

fn main() {
    let mut cont = Container::new_random();
    println!("created container, starting simulation...");
    cont.run(1.);
    println!("simulation finished...");
}
