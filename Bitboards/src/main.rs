use std::io;

mod chess;

fn main() {

    let mut g = chess::Game::new();
    loop {
        match g.is_game_over() {
            None => {
                g.board.print_board();
                loop {
                    let mut new_move = String::new();
                    io::stdin().read_line(&mut new_move).expect("msg");
                    if g.make_move(new_move.trim().to_string()) {break;}
                    else {
                        println!("Not a legal move! Here are all legal moves:");
                        for i in g.get_legal_moves() {
                            print!("{}, ", i)
                        }
                        println!()
                    }
                }
            }
            Some(n) =>  {
                print!("The game is over. {}", n);
                break;
            }

        }
        
    }
    

}