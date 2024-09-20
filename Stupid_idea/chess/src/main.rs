use std::fmt::Debug;
use std::num::Wrapping;
use std::io;
fn main() {

    let mut newgaem = Game::new();
    newgaem.get_possible_moves();
    loop {
        let mut input = String::new();
        (io::stdin().read_line(&mut input)).unwrap();
        newgaem.make_move(str_sq(input).unwrap());
        newgaem.board.print_board();
        newgaem.get_possible_moves();
    }

}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Colour {
    White,
    Black,
    Null,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    InProgress,
    Check(Colour),
    GameOver(Colour),
}


#[derive(Debug)]
pub struct Game {
    state : GameState,
    turn_number : u64,
    board : BoardState,
    white : Player,
    black : Player,
    history : Vec<BoardState>
}

#[derive(Debug)]
struct Player {
    colour : Colour,
    r_castle : bool,
    l_castle : bool,
}

impl Player {
    fn new(colour : Colour) -> Self {
        Self {colour, r_castle : true, l_castle : true,}
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Move(i8, i8);

fn sq_str(square : u8) -> String {
    format!("{}{}", (b'a' + square % 16) as char, (b'1' + square / 16) as char)
}
fn str_sq(square : String) -> Option<Move> {
    let bytes = square.as_bytes();
    Some(Move((bytes[0] - b'a' + bytes[1] - b'1') as i8,(bytes[2] - b'a' + bytes[3] - b'1') as i8 ))
}

#[derive(Debug, Clone)]
struct BoardState {
    board : Vec<char>,
    legal_moves : Vec<Move>,
    turn : Colour,
    board_occurences : u8, //Hur många ggr den här board_staten uppkommit innan
}


impl BoardState {
    fn new() -> Self {
        let mut board =vec!['0'; 256]; //'0' är out of bounds

        //Det som är "in bounds" är alltid mindre än 127 och mindre än 8 (mod 16)
        //Om vi representerar en i8 som 16 * 16 kvadrat är brädet en 8 * 8 kvadrat i nedersta vänstra hörnet
        //a1 är 0, h1 är 7, a2 är 16, h2 är 23, upp till h8 som är 119. 
        //Det här gör att vilket än håll man går av brädet från, gör att ens index blir ogiltig
        //Det här visade sig också vara ett misstag eftersom rust inte gillar integer overflows. Dock är det för sent nu. Orkar inte ändra
        let mut index = 0;
        for i in "RNBQKBNR".chars() {
            board[index] = i;
            index += 1;
        }

        index = 16;
        for i in "PPPPPPPP".chars() {
            board[index] = i;
            index += 1;
        }

        for i in 0..8 {
            for j in 2..6 {
                board[i+16*j] = '.';
            }
        } 

        index = 96;
        for i in "pppppppp".chars() {
            board[index] = i;
            index += 1;
        }

        index = 112;
        for i in "rnbqkbnr".chars() {
            board[index] = i;
            index += 1;
        }

        //sätter alla pjäser på sina positioner

        Self {
            board,
            turn : Colour::White,
            board_occurences : 1,
            legal_moves : Vec::new()
        }
    }
    fn gen_legal_moves(&self) -> Option<Vec<Vec<Move>>> {
        let mut new_moves = vec![vec![], vec![]];

        if self.board_occurences == 3 {
            return Some(new_moves);
        }
        let mut position = -1;
        for &square in self.board.iter(){
            if position == 120 {break;}
            else {position += 1;}
            if square == '.' || square == '0' || square == '!' {continue;} //tom ruta, OOB eller en passent

            let offset = (square as u32 > 90) as i8;
            //De olika funktionerna tar lägger till potentiellt legala dag till new_moves i sin färg
            //offset tas också in som antingen -1 eller 1 som colour för att det blir lättare att jobba med
            //Förra kommentaren är en lögn
            if square == (b'P' + offset as u8 * 32) as char {self.pawn(position, &mut new_moves[offset as usize], (offset * 2 -1) *-1 )}
            else if square == (b'N' + offset as u8 * 32) as char {self.knight(position, &mut new_moves[offset as usize], offset * 2 -1)}
            else if square == (b'B' + offset as u8 * 32) as char {self.march(position, &mut new_moves[offset as usize], offset * 2 -1, true, false)}
            else if square == (b'R' + offset as u8 * 32) as char {self.march(position, &mut new_moves[offset as usize],offset * 2 -1, false, true)}
            else if square == (b'Q' + offset as u8 * 32) as char {self.march(position, &mut new_moves[offset as usize],offset * 2 -1, true, true)}
            else if square == (b'K' + offset as u8 * 32) as char {self.king(position, &mut new_moves[offset as usize], offset * 2 -1)}
            else {panic!("weird square!!")}
            
        }
        //Implementera att man inte kan ta kungen!!

        //varför gjorde jag det här för båda sidorna?
        return Some(new_moves);

    }
    fn march(&self, position : i8,  move_list : &mut Vec<Move>, offset : i8, diagonal : bool, straight : bool) {
        let directions = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
        let mut count = if diagonal {1} else {0};
        let step = if straight == diagonal {1} else {2};
        
        while count < 8 {
            let mut potential_move = position.clone();
            loop {
                potential_move = (Wrapping(potential_move) + Wrapping(directions[count].0 + directions[count].1 * 16)).0;

                if potential_move < 0 {break;}
                if !self.check_moves(position, move_list, offset, potential_move) {break;}
            }
            count += step;
        
        }
       
    }
    fn pawn(&self, position : i8, move_list : &mut Vec<Move>, offset : i8) {
        
        if position > 8 && position < 112{ //Om bonden är vid kanten av brädet ska den promota

            if self.board[(position + 16 * offset) as usize] == '.' {
                //Om En ruta uppåt eller neråt är tom. Pusha drag

                move_list.push(Move(position as i8, (position + 16 * offset) as i8));

                if self.board[(Wrapping(position) + Wrapping(16 * offset)).0 as usize] == '.' && (position/16 <= 1 || position/16 >= 6) {
                    //Om en till ruta är tom och man är på start raden
                    move_list.push(Move(position as i8, (position + 32 * offset) as i8));
                }
            }
            let diag_v = self.board[(Wrapping(position) + Wrapping(15 * offset)).0 as usize].clone();
            let diag_h  = self.board[(Wrapping(position) + Wrapping(17 * offset)).0 as usize].clone();

            for i in "PNBRQ!".chars() {
                //Förlåt för den här
                if (i as u8 + ((offset + 1)/2 * 32) as u8) as char == diag_h {move_list.push(Move(position as i8, (position + 17 * offset) as i8));}
                if (i as u8 + ((offset + 1)/2 * 32) as u8) as char == diag_v {move_list.push(Move(position as i8, (position + 15 * offset) as i8));}
            }

        }
        



    }
    fn knight(&self, position : i8, move_list : &mut Vec<Move>, offset : i8) {
        //Ett steg upp är +16, vänster är +1, två vänster, en upp blir 18
        //Steg neråt blir 256-16, höger är 256-1, 
        for i in [18_i8, 14, 33, 31, - 18,  - 14, - 33, - 31] {
            self.check_moves(position as i8, move_list, offset, i);
        }
        
        
    }
    fn king(&self, position : i8, move_list : &mut Vec<Move>, offset : i8 ) {
        for i in [16_i8, 17, 15, 1,  - 16, - 15, - 17, - 1] {
          self.check_moves(position as i8, move_list, offset, i);  
        }
    }
    fn check_moves(&self, position : i8, move_list : &mut Vec<Move>, offset : i8, potential_move : i8,) -> bool {
        
        if (Wrapping(position) + Wrapping(potential_move)).0 <= 0 {return false;}

        //Wrapping tillåter integer overflows
        let move_to = (Wrapping(position) + Wrapping(potential_move)).0 as usize;
        let c = self.board[move_to];

        if c == '.' {
            move_list.push( //Lägger till ett drag om den går till en tom ruta
            Move(position, move_to as i8));
            return true;
        }

        if c == '0' || (c as i8 * -offset < 90 * -offset){return false;}
        //Det här kollar om man försöker ta en egen pjäs eller om man går OOB. Exercise for the reader att lista ut varför

        else {
            move_list.push( //Lägger till ett drag om den tar motsåndar pjäs
            Move (position, move_to as i8,));
        }
        return true;
    
    }
    fn push_move(&mut self, made_move : Move) {
        println!("{:?}", made_move);
        let a =self.board[made_move.1 as usize];
        
        self.board[made_move.1 as usize] = a.clone();
        if (made_move.0 - made_move.1).abs() == 32 && (a == 'p' || a == 'P') {
            self.board[made_move.0 as usize] = '!';
        }
        else {self.board[made_move.0 as usize] = '.';}
        self.legal_moves = self.gen_legal_moves().unwrap()[if self.turn == Colour::White {0} else {1}].clone();
    }

    fn print_board(&self) {
        for j in 0..8{
            for i in 16*j..16*j+8 {
                print!("{}", self.board[i])
            }
            println!();
        }
    }
        
}



impl Game {


    pub fn new() -> Game {

        let board = BoardState::new();
        Game {
            
            state : GameState::InProgress,
            white : Player::new(Colour::White),
            black : Player::new(Colour::Black),
            board,
            turn_number : 0,
            history : Vec::new(),
            
        }   
    }
    pub fn make_move(&mut self, made_move : Move) -> Option<GameState> {
        if self.board.legal_moves[..].contains(&made_move) {
            self.board.push_move(made_move);    
            return Some(self.state);
        }

        None
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self) -> Option<Vec<String>> {
        for i in self.board.gen_legal_moves().unwrap(){
            for j in i {
                print!("{}{} ,", sq_str(j.0 as u8), sq_str(j.1 as u8))
            }
        }
        println!("{:?}", self.board.legal_moves);
        None
    }


}
