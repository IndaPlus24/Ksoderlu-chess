use std::{clone, collections::HashMap};


#[derive(Debug)]
pub enum GameState {
    GameOver(bool),
    InProgress,
    Stalemate,
    Check(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    King,
    Rook,
    Queen,
    Pawn,
    Knight,
    Bishop,
    Null,
}





#[derive(Debug)]
pub struct Game {
    pub board : Board,
    pub game_state : GameState,
    pub turn_number : u64,
    pub legal_moves : HashMap<String, Board>,

}



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    pub white_pieces : u64,
    pub black_pieces : u64,

    kings : u64,
    knights : u64,
    rooks : u64,
    bishops : u64,
    queens : u64,
    pawns : u64,

    castling_rights : u64,
    turn : bool,

    pub illegal_moves : [u64; 64],
}



static COLUMN_H  : u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
//static COLUMN_AB : u64 = 0b11000000_11000000_11000000_11000000_11000000_11000000_11000000_11000000;
//static COLUMN_GH : u64 = 0b00000011_00000011_00000011_00000011_00000011_00000011_00000011_00000011;
static COLUMN_A  : u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;

static ROW_8     : u64 = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
//static ROW_12    : u64 = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
//static ROW_78    : u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
static ROW_1     : u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;



impl Board {

    pub fn new() -> Self{
        //Generera nytt bräde i start position
        let mut board = Self {
            white_pieces    : 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,
            black_pieces    : 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,

            kings           : 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            knights         : 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            rooks           : 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            bishops         : 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            queens          : 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            pawns           : 0b00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,

            castling_rights : 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001111,
            turn            : false,
            
            //Den här borde inte heta illegal moves 
            illegal_moves : [0;64],

        };

        board.all_moves();
        
        board
    }

    fn all_legal_moves(&mut self) -> HashMap<String, Board> {
        let mut v = HashMap::with_capacity(30);

        for index in 0..64 {
            if self.turn == ((1 << index) & self.black_pieces != 0) {
                //Den här kollar genom rätt många tomma rutor. Oh well...
                v.extend(self.is_legal(1<<index, index));
            }
        }
        v
    }
    
    fn all_moves(&mut self) {
        //genererar drag för hela brädet
        
        for index in 0..64 {
            self.illegal_moves[index as usize] = self.gen_moves(1 << index, index).unwrap();
        }
    }

    fn new_moves(&mut self, mut new_square : u64, old_square : u64) -> bool {

        new_square |= old_square;

        for index in 0..64 {
            if 0 != self.illegal_moves[index] & new_square {
                match self.gen_moves(1 << index, index as u64){
                    Some(n) => self.illegal_moves[index] = n,
                    None => return false,
                }
            }
        }
        true
    }

    fn gen_moves(&self, square : u64, index : u64) -> Option<u64> {

        //Definera färg på ens pjäser och om det är dens drag
        let (colour, to_move) = if square & self.white_pieces != 0 {(self.white_pieces, !self.turn)}
         else if square & self.black_pieces != 0 {(self.black_pieces, self.turn)} 
         else {return Some(0);};
         

        let moves = match self.square_to_piece(square) {
            PieceType::King     => self.king_moves(index),
            PieceType::Pawn     => self.pawn_moves(index),
            PieceType::Bishop   => self.bishop_moves(index),
            PieceType::Knight   => self.knight_moves(index),
            PieceType::Queen    => self.queen_moves(index),
            PieceType::Rook     => self.rook_moves(index),
            _ => panic!("AAA")
        };
        
        if 0 != moves & (!colour) & self.kings && to_move {return None;}
        return Some(moves);

    }


    fn king_moves(&self, index : u64) -> u64 {
        //Kung som står i nedre högre hörnet
        let mut multiplier : u64 = 0b_00000000_00000000_0000000_00000000_00000000_00000111_00000101_00000111; 
        if index < 9 {
            multiplier >>= 9 - index;
        }
        else {
            multiplier <<= index - 9;
        }
        if index % 8 == 7 {
            multiplier & (! COLUMN_H)
        }
        else if index % 8 == 0 {
            multiplier & (! COLUMN_A)
        }
        else {
            multiplier
        }
    }

    fn knight_moves(&self, index : u64) -> u64 {
        
        let mut multiplier: u64 = 0;
        //Om den inte går av brädet och den inte loopar runt brädet så lägger vi till draget
        if index + 17 < 64 && index % 8 < 7 {multiplier |= 1 << (17 + index);}
        if index + 15 < 64 && index % 8 > 0 {multiplier |= 1 << (15 + index);}
        if index + 10 < 64 && index % 8 < 6 {multiplier |= 1 << (10 + index);}
        if index + 6 < 64 && index % 8 > 1  {multiplier |= 1 << (6 + index);}


        //Om vi går neråt eller höger bitshitar vi istället till höger
        if index >= 17 && index % 8 < 7 {multiplier |= 1 << (index - 17);}
        if index >= 15 && index % 8 > 0 {multiplier |= 1 << (index - 15);}
        if index >= 10 && index % 8 < 6 {multiplier |= 1 << (index - 10);}
        if index >= 6  && index % 8 > 1 {multiplier |= 1 << (index - 6);}

        multiplier
    }

    fn pawn_moves(&self, index : u64)   -> u64 {

        let offset = 0 != self.black_pieces & (1 << index);

        let both_pices = self.black_pieces | self.white_pieces;

        let mut multiplier = 0;

        if (index < 16) & (!offset) || (index > 47) & offset { //Bonde har inte rört sig
            
            if 0 == (both_pices) & (1 << ((index + 8) - 16 * offset as u64)) {
                multiplier |= 1 << ((index + 16) - 32 * offset as u64);
            }
        }

        multiplier |= 1 << ((index + 8) - 16 * offset as u64);

        if index % 8 < 7 { // ta till vänster
            multiplier | (1 << ((index + 9) - 16 * offset as u64)) & both_pices
        }
        else if index % 8 > 0 { // ta till höger
            multiplier | (1 << ((index + 7) - 16 * offset as u64)) & both_pices
        }
        else {
            multiplier
        }

        
    }

    fn bishop_moves(&self, index : u64) -> u64 {
        let mut multiplier = 0;
        let mut count = 1 << index;

        //Alla pjäser förutom den vi genererar drag för
        let all_but_me = (!count) & (self.black_pieces | self.white_pieces);

        while 0 !=  count & (!( (COLUMN_H | ROW_8) | all_but_me)) {
            count <<= 9;
            multiplier |= count;
        }

        count = 1 << index;
        while 0 !=  count & (!( (COLUMN_A | ROW_1) | all_but_me))  {
            count >>= 9;
            multiplier |= count;
            
            
        }

        count = 1 << index;
        while 0 !=  count & (!((COLUMN_A | ROW_8) | all_but_me))  {
            count <<= 7;
            multiplier |= count;
            
        }

        count = 1 << index;
        while 0 !=  count & (!( (COLUMN_H | ROW_1) | all_but_me)) {
            count >>= 7;
            multiplier |= count;
        }

        multiplier   
    }

    fn rook_moves(&self, index : u64)   -> u64 {

        let mut multiplier = 0;
        let mut count = 1 << index;

        //Alla pjäser förutom den vi genererar drag för
        let all_but_me = (!count) & (self.white_pieces | self.black_pieces);

        
        while 0 !=  count & (!(COLUMN_H | all_but_me)) {
            count <<= 1;
            multiplier |= count;
            
        }

        count = 1 << index;
        while 0 !=  count & (!(COLUMN_A | all_but_me)) {
            count >>= 1;
            multiplier |= count;
            
        }

        count = 1 << index; 
        while 0 !=  count & (!(ROW_8 | all_but_me)) {
            count <<= 8;
            multiplier |= count;
            
        }

        count = 1 << index;
        while 0 !=  count & (!(ROW_1 | all_but_me)) {
            count >>= 8;
            multiplier |= count;
            
        }


        multiplier   
    }

    fn queen_moves(&self, index : u64)  -> u64 {
        self.rook_moves(index) | self.bishop_moves(index)
    }
    
    pub fn print_board(&self) {
        let mut square = 1;


        for i in 0..64 {
            if 0 == i % 8 {println!()};

            let colour = if square & self.black_pieces != 0 {32} else {0};
            
            match self.square_to_piece(square) {
                PieceType::Pawn   => print!("{}", (b'P' + colour) as char),
                PieceType::Bishop => print!("{}", (b'B' + colour) as char),
                PieceType::Knight => print!("{}", (b'N' + colour) as char),
                PieceType::Queen  => print!("{}", (b'Q' + colour) as char),
                PieceType::Rook   => print!("{}", (b'R' + colour) as char),
                PieceType::King   => print!("{}", (b'K' + colour) as char),
                PieceType::Null   => print!("."),
            }
            
            square <<= 1;
        }
        println!();
        
    }

    /*fn u64move_to_str(&self, index : u8,) -> Vec<String> {
    //Genererar alla drag från en ruta

        let from = square_to_str(index);

        let mut v = Vec::new();


        for i in 0..64 {
            if 0 != self.legal_moves[index as usize][i as usize].iter().sum::<u64>() {
                v.push(from.clone() + &square_to_str(i));

            }
        }
        v
    }

    pub fn get_legal_moves_slow(&self) -> Vec<String> {
        let mut v = Vec::new();
        for i in 0..64 {
            for j in self.u64move_to_str(i) {
                v.push(j);
            }
        }
        v
    }

    pub fn print_legal_moves(&mut self) {

        for i in self.get_legal_moves_slow() {
            print!("{i} ")
        }
        
        println!();
    } */

    
    fn push_move(&mut self, from : u64, to : u64, promote : Option<PieceType>) {


        if 0 != from & self.white_pieces {
            self.white_pieces ^= from | to;
            self.black_pieces &= !to;
        }
        else {
            self.black_pieces ^= from | to;
            self.white_pieces &= !to;
        }
        self.kings &= !to;
        self.knights &= !to;
        self.bishops &= !to;
        self.queens &= !to;
        self.pawns &= !to;
        self.rooks &= !to;

        let mut piece = self.square_to_piece(from);
        if piece == PieceType::Pawn && to >= 1<<56 || to < 1 << 8 {
            match promote {
                Some(n) => piece = n,
                None => piece = PieceType::Queen,
            }
        }

        match piece {
            PieceType::King     => {self.kings |= to; self.kings ^= from;},
            PieceType::Knight   => {self.knights |= to; self.knights ^= from;},
            PieceType::Bishop   => {self.bishops |= to; self.bishops ^= from;},
            PieceType::Queen    => {self.queens |= to; self.queens ^= from;},
            PieceType::Rook     => {self.rooks |= to; self.rooks ^= from;},
            PieceType::Pawn     => {self.pawns |= to; self.pawns ^= from;},
            _ => (),
        }

        self.turn = !self.turn;

        if self.is_check() {
            self.all_moves();
        }
        else {
            self.new_moves(to, from);
        }

        

    }


    pub fn is_legal(&mut self, square : u64, index : usize) -> HashMap<String, Board>{

        let colour = if 0 != self.white_pieces & (square) {self.white_pieces} else {self.black_pieces};

        let potential_moves = self.illegal_moves[index] & (!colour);
        let mut new_legal_moves = HashMap::new();
        let from_string = square_to_str(index as u8);

        for to_index in 0..64 {

            // itererar genom potentiella drag
            if 0 != potential_moves & (1 << to_index) {

                //Skapar temporärt bräde
                let mut temp_board = self.clone();
                temp_board.push_move(square, 1 << to_index, None);

                //Om man kan göra olagligt drag i den positionen tar vi bort draget
                if temp_board.new_moves(1 << to_index, square) {

                    new_legal_moves.insert(from_string.clone() + &square_to_str(to_index as u8),  temp_board,);
                    
                }
            }

        }
        new_legal_moves

    } 



    fn is_check(&self) -> bool {
        let king = self.kings & if self.turn {self.black_pieces} else {self.white_pieces};

        for i in 0..64 {
            if 0 != self.illegal_moves[i] & king && 0 != if self.turn {self.white_pieces} else {self.black_pieces} & (1 << i) {
                return true
            }
        }
        false

    }


    pub fn square_to_piece(&self, square : u64) -> PieceType {        
        if 0 != square & self.kings {
            PieceType::King
        }
        else if 0 != square & self.knights {
            PieceType::Knight
        }
        else if 0 != square & self.bishops {
            PieceType::Bishop
        }
        else if 0 != square & self.rooks {
            PieceType::Rook
        }
        else if 0 != square & self.queens {
            PieceType::Queen
        }
        else if 0 != square & self.pawns {
            PieceType::Pawn
        }
        else {
            PieceType::Null
        }


    }
    



}

impl Game {

    pub fn new() -> Self {
        let mut b = Board::new();
        Self {
            game_state : GameState::InProgress,
            legal_moves : b.all_legal_moves(),
            board : b,
            turn_number : 0,
        }
    }


    pub fn make_move(&mut self, new_move : String) -> bool {

        //Returnar false om draget inte funkar

        match self.legal_moves.get(&new_move) {
            Some(&b) => {
                self.board = b;
                //Om jag hade orkat hade den bara genererat lagliga drag där det behövs. Det är typ jobbigt att ta bort saker ur en hashmap dock
                self.legal_moves = self.board.all_legal_moves();

                self.turn_number += 1;
                self.update_gamestate();
                true
            },
            None => false,
        }
    }

    fn update_gamestate(&mut self) {

        if self.board.is_check() {

            //sidan som attackerar. True för svart
            let attacker = self.turn_number % 2 == 1;
            if self.legal_moves.len() == 0 {
                self.game_state = GameState::GameOver(attacker);
            }
            else {
                self.game_state = GameState::Check(attacker);
            }
        }
        else if self.legal_moves.len() == 0 {
            self.game_state = GameState::Stalemate;
        }
        else {
            self.game_state = GameState::InProgress;
        }
    }

    pub fn get_legal_moves(&self) -> Vec<String> {
        let mut moves = Vec::new();

        for s in self.legal_moves.keys() {
            moves.push(s.clone());
        }
        moves.sort();
        moves
    }

    pub fn is_game_over(&self) -> Option<String> {
        match self.game_state {
            GameState::Check(n) => None,
            GameState::InProgress => None,
            GameState::Stalemate => Some(String::from("It's a stalemate")),
            GameState::GameOver(true) => Some(String::from("White wins!")),
            GameState::GameOver(false) => Some(String::from("Black wins!"))
        }
    }

}



pub fn square_to_str(index : u8) -> String {
    format!("{}{}",(b'a' + index % 8) as char, (b'1' + index / 8) as char)
}

pub fn str_to_square(square : Vec<char>) -> u64 {

    1 << (square[0] as u8 + (square[1] as u8 - b'a') * 8 )
}




