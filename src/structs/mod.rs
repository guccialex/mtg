

mod deck;
mod deckposition;



pub use deckposition::DeckPosition;
pub use deck::Deck;

use std::collections::HashMap;








//the order of cards by their usage in the meta
pub fn cards_by_meta_order( decks: &Vec<Deck>, alphabet: &Vec<String> ) -> Vec<String>{

    //a map of all cards to the total amount in decks
    let mut cardtoamount = HashMap::new();

    for card in alphabet{
        cardtoamount.insert(card, 0);
    }


    for deck in decks{

        for card in &deck.cards{

            *cardtoamount.get_mut(card).unwrap() += 1;
        }
    }

    let mut nameandamount: Vec<(&&String, &u32)> = cardtoamount.iter().collect();

    
    nameandamount.sort_by(|(_,a), (_, b)| b.cmp(a) );

    let mut toreturn = Vec::new();


    for (name, _) in nameandamount{
        toreturn.push(name.clone().clone());
    }
    
    toreturn
}









