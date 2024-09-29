
Lycka till om du ska försöka dig på den här koden. Det här är en produkt av dåliga idéer och botchade lösningar. 
De flesta funktionerna förklarar vad de ska göra men hur de gör de har jag typ inte förklarat och det finns nog delar jag inte förstår själv.
Om det finns buggar i koden är man nog lite körd om man inte förstår hur allt hänger ihop så snälla kontakta mig då.
Castling är ett legal move men det gör så att alla kungar försvinner så gör inte det tills jag fixat det
Du behöver förhoppningsvis inte läsa igenom alla metoder. Jag rekommenderar dock att läsa igenom vilka structs och enums som finns och vad de innehåller

Börja ett game:

    Game::new()                 skapar game som innehåller bräde och legal moves i start position.
    Game::import_fen(&str)      skapar ett game från fen string. Väldigt bra för debugging. 
                                Importerar inte castling rights eller en passant rutor

Info om spelet

    Game.get_possible_moves(&self)      Metod i Game. Ger alla lagliga drag som en vector av Strings
    Game.get_board_as_option_vec(&self) Metod i Game. Ger brädet som en vector av options som innehåller 
                                        en pjäs och en bool. Svart är true. Börjar på a1, sen a2 osv...

    Game.game_state                     Field i Game. Ger en enum av GameState
    Game.turn_number                    Field i Game. Ger en u64 som är vems tur det är

    print!("{}", Game) printar ett fint bräde och alla legal moves


