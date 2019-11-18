use std::io;

pub fn run() {
    println!("Welcome to the Brain Games!");
    println!("May I have your name?");

    let mut name = String::new();
    let mut gamenumber = String::new();
    let list_of_games = "\n
1.Calc
2.Prime
3.Even
4.GCD
5.Progression
6.Balance\n";

    io::stdin().read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}", name);
    println!("List of games: {}", list_of_games);
    println!("Enter number of game:\n");

    io::stdin().read_line(&mut gamenumber)
        .expect("Failed to read line");

    let gamenumber: u32 = gamenumber.trim().parse()
        .expect("Please type a number!");

    match gamenumber {
        1 => calc_game(),
        _ => println!("ooops!"),
    }

}

pub fn calc_game() {
    println!("bla bla");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let result = 12;

        assert_eq!(
            result,
            sum(2, 4)
        );
    }
}
