static MAX_HEALTH: i32 = 100;
static GAME_NAME: &str = "Monster Attack";
const MYPI: f32 = 3.14;

fn main() {
    println!("The game is called {}", GAME_NAME);
    println!("The max health is set to {}", MAX_HEALTH);
    println!("The value of PI is {}", MYPI);
    println!("In the game {0} you start with {1} % health, yes you read correctly: {1} points!", GAME_NAME, MAX_HEALTH);
    println!("The address of PI is {:p}", &MYPI);
}

