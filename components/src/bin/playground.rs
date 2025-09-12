use components::components::{Component, Uniform};

fn main() {
    let uniform = Uniform::new(5);
    println!("Hello World! {}", uniform.get(()));
}
