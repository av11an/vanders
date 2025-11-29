mod checkerboard;
mod ball;
mod glsl;

fn main() -> std::io::Result<()> {
    println!("Running cool GFX!");
    //checkerboard::run()
    ball::run()
}
