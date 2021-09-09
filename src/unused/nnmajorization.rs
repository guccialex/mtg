use crate::mtg;
use crate::structs;
use structs::Deck;
use structs::DeckPosition;

use std::fs::File;
use std::io::prelude::*;

use serde_json::json;

use std::process::Command;

use nalgebra::Point2;

fn write_decks_and_positions_file(deckpos: &Vec<DeckPosition>, alphabet: &Vec<String>){

    //the one hot vector of the cards and the
    let mut towritestruct: Vec<(Vec<f32>, (f32,f32))> = Vec::new();


    for deck in deckpos{

        let ohv = deck.deck.get_one_hot_vector(&alphabet);
        let pos = deck.get_position();

        towritestruct.push( (ohv, pos)  );
    }

    
    let towrite = json!( towritestruct ).to_string();
    let mut file = File::create("nnmajorization/decksandpositions.json").unwrap();
    file.write_all( towrite.as_bytes() ).unwrap();
}



fn train_on_decks_and_posiitions_file(){

    let output = Command::new("python3")
        .current_dir("nnmajorization")
        .args(&["train.py"])
        .output()
        //.spawn()
        .expect("failed to execute process");

}



fn get_predictions(oldpos: &Vec<DeckPosition>) -> Vec<DeckPosition>{


    //get predictions
    let output = Command::new("python3")
        .current_dir("nnmajorization")
        .args(&["predict.py"])
        .output()
        //.spawn()
        .expect("failed to execute process");

    //println!("output of get predictions {:?}", output);

    let contents = std::fs::read_to_string( "nnmajorization/predicted.json" ).expect("Something went wrong reading the file");
    let predictedpositions: Vec<(f32,f32)> = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    let mut toreturn = Vec::new();


    let mut id = 0;

    for newpos in &predictedpositions{

        let newpos = Point2::new(newpos.0, newpos.1);

        toreturn.push(  oldpos[id].clone_with_new_pos( newpos )  );

        id +=1;
    }




    toreturn
}


//given two lists of deck positions, return the one that has a better score
pub fn get_better_positions(oldpos: &Vec<DeckPosition>, newpos: &Vec<DeckPosition>) -> Vec<DeckPosition>{

    
    let mut toreturn = oldpos.clone();


    use rand::Rng;
    let mut rng = rand::thread_rng();


    let mut id = 0;
    for newdeckpos in newpos{

        let mut toreturncopy = toreturn.clone();
        
        //toreturncopy[id] = newdeckpos.deck.to_deck_position_with_random_initialization();
        toreturncopy[id] = newdeckpos.clone();

        let randx = (rng.gen::<f32>() - 0.5) * 0.05;
        let randy = (rng.gen::<f32>() - 0.5) * 0.05;

        toreturncopy[id].position.x += randx;
        toreturncopy[id].position.y += randy;



        //is toreturn improved by this change?
        if DeckPosition::get_stress_score( &toreturncopy ) < DeckPosition::get_stress_score( &toreturn ){

            toreturn = toreturncopy;
        }

        id +=1;
    }


    println!("stress score {:?}", DeckPosition::get_stress_score(&toreturn) );

    return toreturn;
}


fn new_model() {


    //get predictions
    let output = Command::new("python3")
        .current_dir("nnmajorization")
        .args(&["newmodel.py"])
        .output()
        //.spawn()
        .expect("failed to execute process");

    println!("output {:?}", output);

}


pub fn nnmajorization(decks: Vec<Deck>) {


    let mut deckpos = Vec::new();

    for deck in decks{
        deckpos.push(  deck.to_deck_position_with_random_initialization() );
    }


    let standardalphabet = mtg::get_standard_alphabet();
    let colormapping = mtg::get_colour_mapping();
    
    
    new_model();


    //write the deck contents and its position to file (1)
    write_decks_and_positions_file(&deckpos, &standardalphabet);


    for x in 0..1000{

        let predictions = get_predictions(&deckpos);
        println!("got predictions");

        use crate::draw;

        draw::draw_decks(&predictions, "pred", &colormapping);
        draw::draw_decks(&deckpos, "realpos", &colormapping);

        
        //get the list of deckpos that has a better score
        deckpos = get_better_positions(&deckpos, &predictions);
        println!("got the better position");

        //write the decks and the better positions to file (1)
        write_decks_and_positions_file(&deckpos, &standardalphabet);
        println!("wrote decks to file again");

        //train the decks on the file (1), the decks to position
        train_on_decks_and_posiitions_file();
        println!("trained decks on file");
    }








}
