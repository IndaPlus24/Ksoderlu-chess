use std::{io, collections::HashMap};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    GameOver(bool), //sant om vit vinner
    InProgress,
    Stalemate,
    Check(bool),   //sat om vit schackar
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Rook,
    Queen,
    Pawn,
    Knight,
    Bishop,
    Null,
}

#[derive(PartialEq)]
pub struct Game {
    pub board : Board,
    pub game_state : GameState,
    pub turn_number : u64,
    pub legal_moves : HashMap<String, Board>,
    //Du kanske tänkar att det är en bättre idé att uppdatera brädet istället spara alla möjliga lagliga drag med brädet som de leder till
    //Don't worry. Jag gör båda
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
    en_passant : u64,

    castling_rights : u64,

    turn : bool,

    //Den här borde inte heta illegal moves utan det är alla rutor som pjäsen på en ruta attackerar oavsett färg
    illegal_moves : [u64; 64],
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

            //Där det finns vita pjäser eller svarta pjäser
            white_pieces    : 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,
            black_pieces    : 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,

            //Där alla typer av pjäser finns
            kings           : 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            knights         : 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            rooks           : 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            bishops         : 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            queens          : 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            pawns           : 0b00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
            en_passant      : 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,

            castling_rights : 0b01101110_00000000_00000000_00000000_00000000_00000000_00000000_01101110, 
            // Så många magiska nummer. Det är rutorna som ska vara tomma om man kan castla till: 
            //svart kingside, svart queenside,  vit kingside, vit queenside

            //True om det är svarts tur
            turn            : false,
            illegal_moves : [0;64],

        };
        //För att vi behöver veta hur brädet ser ut när vi genererar drag skapar jag först ett bräde utan drag och genererar sen alla drag
        board.all_moves();
        board
    }

    pub fn from_FEN(fen : Vec<char>) -> Self{ //Importera fen kod. Man borde egentligen använda metoden i game men den är pub för debugging
        let mut board = Self {
            white_pieces    : 0,
            black_pieces    : 0,
            kings           : 0,
            knights         : 0,
            rooks           : 0,
            bishops         : 0,
            queens          : 0,
            pawns           : 0,
            castling_rights : 0b01101110_00000000_00000000_00000000_00000000_00000000_00000000_01101110,
            turn            : false,
            illegal_moves : [0;64],
            en_passant : 0,
        };
        let mut count = 0;
        
        for i in 0..8 {
            //;ed hur FEN är upplagt får mig att tro att jag spegelvänt mina bitboards. Jag kan inget om bitboards. Min kod kommer rat från hjärtat
            //Jag hoppas verkligen inte att man ska skriva formella kommentarer för det här skrivs kl 4 på torsdag natt
            let mut index = (7 - i) * 8;

            //Börjar uppifrån och går höger till vänster
    
            while index < (8 - i) * 8 {
                
                if fen[count] == '/' {
                    count += 1;
                    continue;
                }
                if fen[count] as u8  - b'0' <= 8 {
                    index += fen[count] as u8  - b'0';
                    count += 1;
                    continue;
                }
                

                if (fen[count] as u8 - b'A') < 32 {
                    board.white_pieces |= 1 << index;
                } 
                else {
                    board.black_pieces |= 1 << index;
                }
                match char_to_piece(fen[count]).unwrap() {
                    PieceType::King     => board.kings   |= 1 << index,
                    PieceType::Pawn     => board.pawns   |= 1 << index,
                    PieceType::Bishop   => board.bishops |= 1 << index,
                    PieceType::Knight   => board.knights |= 1 << index,
                    PieceType::Queen    => board.queens  |= 1 << index,
                    PieceType::Rook     => board.rooks   |= 1 << index,
                    _ => panic!("CCC"),
                }
                
                count += 1;
                index += 1; 
            }
        }

        if fen[count+1] == 'b' {
            board.turn = true;
        }
        board.all_moves();
        board

    }

    //Ok det här gäller för de flesta funktionerna i Board
    //index är ett tal mellan 0 och 64 som representerar en plats på brädet
    //square är 1 << index. Det är den rutan som index representerar
    //colour är alla pjäser med samma färg som pjäsen eller rutan som man kollar på

    //from och to är rutan som man går från och till


    fn all_moves(&mut self) -> bool {
        //genererar drag för hela brädet. Om man kan ta kungen returnas false
        
        for index in 0..64 {
            match self.gen_moves(1 << index, index as u64){
                Some(n) => self.illegal_moves[index] = n,
                None => return false,
            }
        }
        true
    }

    /*fn new_moves(&mut self, mut new_square : u64, old_square : u64) -> bool {

        //genererar alla drag för pjäser som ser två positioner. Om man kan ta kungen så returnar den false
        //Jag borde nog bara ha genererat alla drag hela tiden för att det gör debugging lättare

        let square = new_square | old_square;

        for index in 0..64 {

            //Illegal moves innehåller alla positioner som en pjäs kan gå till oavsett färg
            if 0 != self.illegal_moves[index] & new_square {

                match self.gen_moves(1 << index, index as u64){
                    Some(n) => self.illegal_moves[index] = n,
                    None => return false,
                }
            }
        }
        true
    }  Något fel och jag vet inte var, nu är den endast en kommentar*/

    fn gen_moves(&self, square : u64, index : u64) -> Option<u64> {

        //Definera färg på ens pjäser och om det är dens drag. Screw whitespace
        let (colour, to_move) = if square & self.white_pieces != 0 {(self.white_pieces, !self.turn)}
         else if square & self.black_pieces != 0 {(self.black_pieces, self.turn)} 
         else {return Some(0);};
         
        if !to_move {return Some(0);}

        let moves = match self.square_to_piece(square) {
            PieceType::King     => self.king_moves(index),
            PieceType::Pawn     => self.pawn_moves(index),
            PieceType::Bishop   => self.bishop_moves(index),
            PieceType::Knight   => self.knight_moves(index),
            PieceType::Queen    => self.queen_moves(index),
            PieceType::Rook     => self.rook_moves(index),
            _ => {self.print_board(); panic!("AAA")} //Något har gått väldigt fel
        } & (!colour);
        
        if 0 != moves & self.kings {return None;}
        //Om man kan ta kungen och det är ens drag returnar vi None annars var den pjäsen kan gå till
        return Some(moves & !colour);
    }

    fn king_moves(&self, index : u64) -> u64 {
        //Kung som står i nedre högre hörnet
        
        let mut multiplier : u64 = 0b00000111_00000101_00000111;
        //Vi bitshiftar sen den till där kungen faktiskt står        

        if index < 9 {
            multiplier >>= 9 - index;
        }
        else {
            multiplier <<= index - 9;
        }

        //Castling
        let mut castle = (!(self.white_pieces | self.black_pieces)) & (self.castling_rights);
        if 0 != castle {

            if self.turn {
                castle &= !ROW_1;
            }
            else {
                castle &= !ROW_8;
            }
            multiplier |= castle & (!((1 << 57) | (1 << 1)));
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

        //Den här variabeln hade hetat is_black om jag kunde trycka på f2 på mitt tangentbord
        let is_black = 0 != self.black_pieces & (1 << index);

        // *pieces
        let both_pieces = self.black_pieces | self.white_pieces | self.en_passant;

        let mut multiplier = 0;

        if (index < 16) & (!is_black) || (index > 47) & is_black { //Bonde har inte rört sig
            //(1 << ((index + 8) - 16 * offset as u64)) är rutan framför bonden
            //Om det inte står något där kollar den om den kan gå fram två steg

            if 0 == (both_pieces) & (1 << ((index + 8) - 16 * is_black as u64)) {
                multiplier |= (1 << ((index + 16) - 32 * is_black as u64)) & !both_pieces;
            }
        }

        multiplier |= (1 << ((index + 8) - 16 * is_black as u64)) & !both_pieces;

        if index % 8 < 7 { // ta till vänster
            multiplier |= (1 << ((index + 9) - 16 * is_black as u64)) & both_pieces;
        }
        if index % 8 > 0 { // ta till höger
            multiplier | ((1 << ((index + 7) - 16 * is_black as u64)) & both_pieces)
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

        //Om den inte håller på att gå av brädet eller om den står på en pjäs avsltar den. Vi vill ha med alla pjäser den attakerar oavsett färg
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

        //Om den inte håller på att gå av brädet eller om den står på en pjäs avsltar den. Vi vill ha med alla pjäser den attakerar oavsett färg
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
        //Game debugging är bättre. Dont use dumb code
        //Brädet den här printar är flippad. Not sorry
        let mut square = 1;


        for i in 0..64 {
            if 0 == i % 8 {println!()};

            let colour = if square & self.black_pieces != 0 {32} else {0};
            //32 är avståndet mellan stora och små bokstäver i unicode
            
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

    /* Väldigt gammal kod som är mig kär. Mitt hjärta blöder för dessa rader som inte längre kan tjäna sitt syfte
    fn u64move_to_str(&self, index : u8,) -> Vec<String> {
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


    fn all_legal_moves(&mut self) -> HashMap<String, Board> {
        let mut v = HashMap::with_capacity(30);
        //Man har väl sällan mer drag än 30

        for index in 0..64 {
            //if statementet left as exercise for reader
            if self.turn == ((1 << index) & self.black_pieces != 0) {
                
                //Den här kollar genom rätt många tomma rutor. Oh well...
                v.extend(self.is_legal(1<<index, index));
            }
        }
        v
    }

    pub fn is_legal(&mut self, square : u64, index : usize) -> HashMap<String, Board>{

        let colour = if 0 != self.white_pieces & (square) {self.white_pieces} else {self.black_pieces};
        //Jag använder inte self.turn här för att få vilka pjäser man sak. Den här funktionen gjorde andra saker innan
        //så mycket kod som man skulle förstå i tidigare versioner. Om det funkar så funkar det dock

        let potential_moves = self.illegal_moves[index] & (!colour); //tar bort att man kan på egna pjäser. "Allt som colour används för. Skitkod." -Karl
        
        let mut new_legal_moves = HashMap::new();
        let from_string = index_to_str(index as u8);
        let piece = self.square_to_piece(square);

        for to_index in 0..64 {

            // itererar genom potentiella drag
            if 0 != potential_moves & (1 << to_index) {
                
                let promote_list = if piece == PieceType::Pawn && (to_index >= 56 || to_index <  8) { //Om vi ska promota
                    vec![Some(PieceType::Queen), Some(PieceType::Rook), Some(PieceType::Bishop), Some(PieceType::Knight)]}//Pjäser vi kan promota till
                    else {vec![None]}; 
                
                for promotions in promote_list  {
                    //Skapar temporärt bräde
                    let mut temp_board = self.clone();
                    temp_board.push_move(square, 1 << to_index, promotions);

                    //Om man kan göra olagligt drag i den positionen tar vi bort draget
                    if temp_board.all_moves() {
                        new_legal_moves.insert(from_string.clone() + &index_to_str(to_index as u8) + (&match promotions {
                            Some(n) => piece_to_char(n).to_string(),
                            None => String::new(),
                        }),  temp_board,);
                        
                    } 
                }
            }

        }

        new_legal_moves
    } 


    
    fn push_move(&mut self, from : u64, to : u64, promote : Option<PieceType>) {

        if self.kings.count_ones() > 2 { //Castlat förra turen
            if self.turn {
                //extra kungarna står på nån av rutorna som är noll
                self.kings &= !0b00111000;
                self.white_pieces &= !0b00010000;
            }
            else { //56 är avståndet mellan första och sista raden
                self.kings &= !(0b00111000 << 56);
                self.black_pieces &= !(0b00010000 << 56);
            }
        }

        if !self.turn {
            self.white_pieces ^= from | to; //går till ny ruta
            self.black_pieces &= !to;       //tar bort där den var
        }
        else {
            self.black_pieces ^= from | to;
            self.white_pieces &= !to;
        }

        //Om den tar en pjäs tar vi bort den
        self.kings &= !to;
        self.knights &= !to;
        self.bishops &= !to;
        self.queens &= !to;
        self.pawns &= !to;
        self.rooks &= !to;

        let mut piece = self.square_to_piece(from);
        if piece == PieceType::Pawn {
            if to >= 1 << 56 || to < 1 << 8 { //Om vi ska promota
                match promote {
                    Some(n) => piece = n,
                    None => piece = PieceType::Queen,
                }
            } 
            else if 0 != self.en_passant & to { //en_passant
                self.pawns &= if self.turn {!(to << 8)} else {!(to >> 8)};
                self.white_pieces &= if self.turn {!(to << 8)} else {!(to >> 8)};
                self.black_pieces &= if self.turn {!(to << 8)} else {!(to >> 8)};
            }

            self.en_passant = 0;

            if from / to + to / from > 1 << 15 {
                
                self.en_passant |= if self.turn {from >> 8} else {from << 8};
            }

        }
        else {
            self.en_passant = 0;
        }

        match piece {
            PieceType::King     => {self.kings |= to; self.kings &= !from;},
            PieceType::Knight   => {self.knights |= to; self.knights &= !from;},
            PieceType::Bishop   => {self.bishops |= to; self.bishops &= !from;},
            PieceType::Queen    => {self.queens |= to; self.queens &= !from;},
            PieceType::Rook     => {self.rooks |= to; self.rooks &= !from;},
            PieceType::Pawn     => {self.pawns |= to;},
            _ => (),
        }

        if piece == PieceType::King { 
            if from / to + to / from == 4 { //magiska nummer go brrr
                if from / to == 4 {         //shiftar höger / queenside
                    //Vi lägger till två kungar som inte kan röra sig för att de inte står på white_pieces. Kungen som står på torn kanske kan röra sig...
                    self.kings |= from | from >> 1;
                    self.rooks &= !(to >> 2);
                    self.rooks |= from >> 1;
                    if from > 1 << 8 {
                        self.black_pieces ^= 0b00001101 << 56;
                    }
                    else {
                        self.white_pieces ^= 0b00001101;
                    }
                }
                else { // kingside
                    self.kings |= from | from << 1;
                    self.rooks &= !(to << 1);
                    self.rooks |= from << 1;
                    if from > 1 << 8 {
                        self.black_pieces ^= 0b11100000 << 56;
                    }
                    else {
                        self.white_pieces ^= 0b11100000;
                    }
                }
            }

            if 0 != from & ROW_1 {
                self.castling_rights &= !ROW_1;
            }
            if 0 != from & ROW_8 {
                self.castling_rights &= !ROW_8;
            }
        }
        else if piece == PieceType::Rook {
            if from == 1 {
                self.castling_rights &= !(0b111 << 1)
            }
            else if from == 1 << 7 {
                self.castling_rights &= !(0b11 << 5)
            }
            else if from == 1 << 56  {
                self.castling_rights &= !(0b111 << 57)
            }
            else if from == 1 << 63 {
                self.castling_rights &= !(0b11 << 61)
            }
        }

        self.pawns &= !from; //för att promotions är skrivet som det är måste den här vara med

        self.turn = !self.turn;
        //self.all_moves();

    }




    fn is_check(&self) -> bool {

        let mut temp_board = self.clone();
        temp_board.turn ^= true; 
        if temp_board.all_moves() {
            false
        }
        else {
            true
        }
    }


    pub fn square_to_piece(&self, square : u64) -> PieceType {        
        if 0 != square & self.rooks { //Rook behöver vara före kung för castling
            PieceType::Rook
        }
        else if 0 != square & self.knights {
            PieceType::Knight
        }
        else if 0 != square & self.bishops {
            PieceType::Bishop
        }
        else if 0 != square & self.kings {
            PieceType::King
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

    pub fn import_fen(fen : &str) -> Self {
        let mut b = Board::from_FEN(fen.chars().collect());
        let mut g = Game {
            game_state : GameState::InProgress,
            legal_moves : b.all_legal_moves(),
            board : b,
            turn_number : fen.split_whitespace().last().unwrap().parse().unwrap(),
        };
        g.update_gamestate();
        g
    }


    pub fn make_move(&mut self, new_move : &String) -> Option<GameState> {
        //Returnar false om draget inte funkar

        match self.legal_moves.get(new_move) {
            Some(&b) => {

                self.board = b;                                                                                       //Wow ser du hur mycket lättere det definitivt blev när man gör det på mitt sätt. En tab för varje gång jag ångrar att jag skrev detta

                //Om jag hade orkat hade den bara genererat lagliga drag där det behövs. Det är typ jobbigt att ta bort saker ur en hashmap dock
                self.legal_moves = self.board.all_legal_moves();

                self.turn_number += 1;
                self.update_gamestate();
                Some(self.game_state)
            },
            None => None,
        }
    }

    fn update_gamestate(&mut self) {

        if self.board.is_check() {

            //sidan som attackerar. True för vit
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

    pub fn get_possible_moves(&self) -> Vec<String> {
        let mut moves = Vec::new();

        for s in self.legal_moves.keys() {
            moves.push(s.clone());
        }
        moves.sort();
        moves
    }

    pub fn is_game_over(&self) -> Option<String> { //Igga den här funktionen asså. så jävla dum. Kan typ bara användas i setup wizard
        match self.game_state {
            GameState::Check(_) => None,
            GameState::InProgress => None,
            GameState::Stalemate => Some(String::from("It's a stalemate")),
            GameState::GameOver(true) => Some(String::from("White wins!")),
            GameState::GameOver(false) => Some(String::from("Black wins!"))
        }
    }

    pub fn setup_wizard(&mut self) {
        //enkel setup. fixar schackmatch och tar stdin som den gör om till annat
        //Bra för debugging
        loop {
            match self.is_game_over() {
                None => {
                    self.board.print_board();
                    loop {
                        let mut new_move = String::new();
                        io::stdin().read_line(&mut new_move).expect("msg");
                        new_move = new_move.trim().to_string();

                        match self.make_move(&new_move) {
                            Some(_) => break,
                            None => ()
                        }
                        if new_move == "help" {
                            println!("I believe in you!");
                        }
                        else {
                            println!("Not a legal move or command! Here are all {} legal moves:", self.legal_moves.len());
                            for i in self.get_possible_moves() {
                                print!("{}, ", i)
                            }
                            println!();
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

}



pub fn index_to_str(index : u8) -> String {
    format!("{}{}",(b'a' + index % 8) as char, (b'1' + index / 8) as char)
}

pub fn str_to_square(input : &str) -> u64 { //Ingen hade väl vart så taskig att de ger dålig input till den här funktionen
    let b = input.as_bytes();

    1 << (b[0] - b'a' + 8 *( b[1] - 49)) as u64 //Jag vet inte varför b'1' inte funkade fast nu är det istället 49
}

fn char_to_piece(c: char) -> Option<PieceType> {
    match c.to_ascii_uppercase() {
        'P' => Some(PieceType::Pawn),
        'B' => Some(PieceType::Bishop),
        'N' => Some(PieceType::Knight),
        'Q' => Some(PieceType::Queen),
        'R' => Some(PieceType::Rook),
        'K' => Some(PieceType::King),
        _ => None,
    }
}

fn piece_to_char(piece: PieceType) -> char {
    match piece {
        PieceType::Pawn => 'P',
        PieceType::Bishop => 'B',
        PieceType::Knight => 'N',
        PieceType::Queen => 'Q',
        PieceType::King => 'K',
        PieceType::Rook => 'R',
        _ => ' ',
    }
}


impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* Den är wack men jag tycker det är för roligt för att fixa */

        let mut board_str = String::from("|-----------------|");
        for i in 0..8 {
            board_str +=                       "\n|                 |";
            
            let mut index = (7 - i) * 8;

            while index < (8 - i) * 8 { 
                board_str.push(piece_to_char(self.board.square_to_piece(1 << index)));
                board_str.push(' ');
                index += 1;
            }
        }
        board_str += "\n|-----------------|\n\n";

        let mut move_str = String::new();
        for i in self.get_possible_moves() {
            move_str += &i;
            move_str += " ";
        }

        write!(f, "{}{}\n", board_str, move_str)
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn it_works() { //Stulen kod...
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.game_state, GameState::InProgress);
    }

    #[test]
    fn fen_code() { //Om jag hade castling hade den här inte funkat. Samma för en passant

        let b_new = Board::new();
        let b_fen = Board::from_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".chars().collect());

        assert_eq!(b_fen, b_new);
    }
    
    #[test]
    fn e_to_e4() {
        let mut game = Game::new();
        assert_eq!(game.legal_moves.contains_key("e2e4"), true)

    }

    #[test]
    fn where_squares() {
        
        assert_eq!(str_to_square("a1"), 1 << 0);
        assert_eq!(str_to_square("a2"), 1 << 8);
        assert_eq!(str_to_square("h1"), 1 << 7);
        assert_eq!(str_to_square("h8"), 1 << 63);
    }

    #[test]
    fn what_squares() {
        let game = Game::new();
        assert_eq!(game.board.square_to_piece(1 << 0), PieceType::Rook);
        assert_eq!(game.board.square_to_piece(1 << 63), PieceType::Rook);
        assert_eq!(game.board.square_to_piece(1 << 4), PieceType::King);
        for index in 8..16 {
            assert_eq!(game.board.square_to_piece(1 << index), PieceType::Pawn);
        }
    }




    #[test]
    fn push_e4() {
        let mut game = Game::new();
        game.make_move(&String::from("e2e4"));

        assert_eq!(game.board.square_to_piece(str_to_square("e4")), PieceType::Pawn)
    }

    #[test]
    fn fools_mate() {
        let mut game = Game::new();
        game.make_move(&String::from("f2f3"));
        game.make_move(&String::from("e7e5"));
        game.make_move(&String::from("g2g4"));
        game.make_move(&String::from("d8h4"));

        assert_eq!(game.game_state, GameState::GameOver(false))
    }

    #[test]
    fn check_castle() {
        let mut game = Game::import_fen("r2qkbnr/ppp1pppp/2n1b3/3p4/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 4 4");

        println!("{:?}{}", game, game.board.is_check());

        assert_eq!(game.legal_moves.contains_key("e1g1"), true);
        game.make_move(&String::from("e1g1"));
        assert_eq!(game.board.kings & str_to_square("g1") != 0 , true); //kollar om det finns en kung på g2

        println!("{:?}{}", game, game.board.is_check());

        game.make_move(&String::from("d8d6"));

        println!("{:?}{}", game, game.board.is_check());

        assert_eq!(game.legal_moves.contains_key("f1e1"), true); //Vi kan röra på tornet efteråt

        game.make_move(&String::from("f1e1"));

        assert_eq!(game.legal_moves.contains_key("e8c8"), true); //Om vi kan castla queenside

        game.make_move(&String::from("e8c8")); 

        assert_eq!(game.board.kings & str_to_square("c8") != 0 , true); //Finns kungen där den borde finnas?

        game.make_move(&String::from("a2a4"));

        assert_eq!(game.board.kings.count_ones(), 2) //Det borde finnas 2 kungar
    }
        

}

