

use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

use std::collections::HashSet;
use std::collections::HashMap;

use serde_json::json;



extern crate nalgebra as na;
use crate::structs;

use na::Point2;
use na::Vector2;


use std::collections::BTreeSet;

use structs::Deck;
use structs::DeckPosition;




pub fn smacofmajorization(decks: &Vec<Deck>, lands: &Vec<String>, colormapping: &HashMap<String, (u32,u32,u32,u32,u32)>) {

    
    let x = Deck::get_dissimilarity_matrix( decks.clone(), lands );


    //the tensormajorization needs 
    let towrite = json!( x ).to_string();
    let mut file = File::create("dissimilaritiesmatrix.json").unwrap();
    file.write_all( towrite.as_bytes() ).unwrap();
    

    
    //the colors of the decks
    let mut deckcolours: Vec<(f32,f32,f32)> = Vec::new();

    for deck in decks{
        deckcolours.push ( deck.get_color( colormapping ) );
    }
    
    let towrite = json!( deckcolours ).to_string();
    let mut file = File::create("deckcolours.json").unwrap();
    file.write_all( towrite.as_bytes() ).unwrap();



    //the cards in the decks
    let mut deckcards: Vec<Vec<String>> = Vec::new();

    for deck in decks{
        deckcards.push( deck.cards.clone() );
    }

    let towrite = json!( deckcards ).to_string();
    let mut file = File::create("deckcards.json").unwrap();
    file.write_all( towrite.as_bytes() ).unwrap();


}



pub fn manual_majorization(decks: Vec<Deck>, colormapping: &HashMap<String, (u32,u32,u32,u32,u32)>) {

    let mut posdecks = Vec::new();

    for deck in decks{
        posdecks.push( DeckPosition::from_deck_with_random_initialization(&deck) );
    }
    


    //used to normalize how much the objects should move
    //the objects should move on average 0.05 units per tick
    let mut normalizationamount = 1.;



    for x in 0..1500{
        
        let mut curtotalmovement = 0.;

        for deckid in 0..posdecks.len(){
            curtotalmovement += improve_deck_position(&mut posdecks, deckid, normalizationamount);
        }

        curtotalmovement = curtotalmovement/(posdecks.len() as f32);


        //should
        normalizationamount = 1.0;// ((1.0/(curtotalmovement + 0.1)) * 0.2  ).min(10.0)  *0.2  + normalizationamount*0.8;


        use crate::actions::draw;

        println!("sts{:?}", DeckPosition::get_stress_score(&posdecks));

        draw::draw_decks(&posdecks, "manmaj", &colormapping);

    }


}





fn improve_deck_position(decks: &mut Vec<DeckPosition>, target: usize, multiplier: f32) -> f32{

    let targetdeckcopy = decks[target].clone();


    let deltapos = targetdeckcopy.get_repulsion_vector_batch(&decks) * multiplier;

    decks[target].move_pos(deltapos);

    return deltapos.magnitude();
}

