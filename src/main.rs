use colored::Colorize;

mod game;
mod generator;
mod poke;
mod util;

#[tokio::main]
async fn main() {
    use colored::Color;
    use generator::Generator;

    let generator = match Generator::new() {
        Ok(generator) => generator,
        Err(e) => {
            println!("{}", e.color(Color::Red));
            return;
        }
    };

    match generator.generate().await {
        Ok(mut game) => game.run(),
        Err(e) => println!("{}", e.color(Color::Red)),
    }
}
