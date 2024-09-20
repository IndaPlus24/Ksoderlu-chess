use std::fmt;
use std::collections::HashMap;

pub enum Colour {
    white,
    black,
}
pub enum GameState {
    InProgress,
    Check(Colour),
    GameOver,
}

pub enum Pieces {
    pawn {colour : Colour, position : u8, has_moved : bool},
    knight{colour : Colour, position : u8}, 
    bishop{colour : Colour, position : u8},
    rook{colour : Colour, position : u8, has_moved : bool},
    queen{colour : Colour, position : u8},
    king{colour : Colour, position : u8, has_moved : bool},

    //bonde, torn och kung har en extra variabel för om de kan gå fram två steg eller göra rokad

    en_passant {colour : Colour, position : u8}, //Ruta som kan tas av bönder. Skapas när bonde går fram två steg
    null {position : u8}, //Tom ruta

}

#[derive(Debug)]
pub struct Game {
    state : GameState,
    turn_number : u64,
    board : BoardState,
    white : Player,
    black : Player,
    history : Vec<BoardState>,

}

struct Move {
    from : u8,
    to : u8,
}

struct PlayerPiece {
    piece_on : u8,
    piece_type : Pieces,
}

#[derive(Debug)]
struct Player {
    legal_moves : Vec<Move>, 
    pieces : Vec<PlayerPiece>,
    is_turn : bool,
}

#[derive(Debug)]
struct BoardState {
    board : Vec<String>,
    all_pieces : Vec<Pieces>,
    turn : Colour,
    board_occurences : u8, //Hur många ggr den här board_staten uppkommit innan
}

impl Player {
    pub fn new( board : Board_state, is_turn : bool) -> self {
        self {
            legal_moves : gen_legal_moves(&self, Board_state),
            is_turn,
            pieces : Board_state
        }
    }
    fn gen_legal_moves(&self, board : Board_state) -> Vec<u64>{

    }
}
impl BoardState {
    fn new() -> self {
        self {
            all_pieces : vec!(rook{})

            // brädet är indexerat så att 0 är a1, 8 är a2, 63 är h8. 
        }
    }
}
impl Game {


    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            
            state : GameState::InProgress,
            board : BoardState.new(), 
            turn_number : 0,
        }
    }

}

