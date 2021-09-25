#![feature(hash_drain_filter)]
#![feature(map_first_last)]
#![feature(drain_filter)]


mod structs;
mod majorization;

mod actions;


pub use structs::Deck;
pub use structs::DeckPosition;


use std::process::Command;

use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use serde_json::json;


fn get_password() -> String{


    let contents = std::fs::read_to_string( "password.txt" ).expect("Something went wrong reading the file");

    return contents;

}


async fn fetch_files(){

    let client: reqwest::Client = reqwest::ClientBuilder::new().cookie_store(true).build().unwrap();

    let password = get_password();
    
    std::fs::create_dir("decks");

    client.get("https://mtgmeta.io/api/login?username=pucci&password=".to_string() + &password).send().await.unwrap();

    let tids = actions::fetchdata::get_standard_tids(&client).await;

    actions::fetchdata::write_decks_to_file(&client, tids).await;

    actions::fetchdata::write_oracle_cards_to_file().await;
}



//card to associated is used for linear predictions
//for each card
//count each other card that is in the same deck as it
//so for each card 
//and then average out to 1, the cards associated
//
//but they should be averaged out

use std::collections::HashMap;

fn get_cards_to_shared_cards(decks: &Vec<Deck>, alphabet: &Vec<String>) -> HashMap<String, HashMap<String, f32>>{

    let mut endvalue = HashMap::new();


    for maincard in alphabet{

        //the othercard, and how many times its alongside this card
        let mut associated: HashMap<String, u32> = HashMap::new();

        let mut totalassociated = 0.0;

        use std::collections::hash_map::Entry;

        for deck in decks{

            //the numbers of this card in the deck
            let numberofmaincard = deck.cards.iter().filter(|&n| n == maincard).count();

            if numberofmaincard > 0{

                for othercard in &deck.cards{

                    let counter = associated.entry( othercard.to_string() ).or_insert(0);
    
                    *counter += numberofmaincard as u32;
    
                    totalassociated += numberofmaincard as f32;
                }

            }

        }


        
        //now divide each by the total amount of cards in it
        let mut toreturn: HashMap<String, f32> = HashMap::new();

        for (card, number) in associated{


            toreturn.insert( card.clone() , number as f32 / totalassociated as f32 );
        }

        println!("toreturn {:?}", toreturn);

        endvalue.insert( maincard.clone(), toreturn );
    }



    endvalue



}



fn write_bbbbeeee_dependencies(){

    
    let mut decks = actions::getdata::get_standard_decks_from_files();
    let alphabet = actions::getdata::get_standard_alphabet();
    let lands = actions::getdata::get_lands();
    let colormapping = actions::getdata::get_colour_mapping();   
    
    

    std::fs::create_dir("mtgdata");



    
    let cardtoassociated = get_cards_to_shared_cards( &decks, &alphabet ) ;
    let mut file = File::create( "mtgdata/cardtoassociated.json" ).unwrap();
    let jsontowrite = json!(cardtoassociated).to_string();
    file.write_all( jsontowrite.as_bytes() ).unwrap();
    

    let cardmetaorder = structs::cards_by_meta_order(&decks, &alphabet);

    let mut file = File::create( "mtgdata/cardmetaorder.json" ).unwrap();
    let jsontowrite = json!(cardmetaorder).to_string();
    file.write_all( jsontowrite.as_bytes() ).unwrap();
    

    std::fs::create_dir("mtgdata/majorization");


    let mut deckcolours: Vec<(f32,f32,f32)> = Vec::new();
    let mut deckcards: Vec<Vec<String>> = Vec::new();

    for deck in &decks{
        deckcolours.push( deck.get_color(&colormapping) );
        deckcards.push( deck.cards.clone() );
    }


    let mut file = File::create( "mtgdata/majorization/deckcolours.json" ).unwrap();
    let jsontowrite = json!(deckcolours).to_string();
    file.write_all( jsontowrite.as_bytes() ).unwrap();


    let mut file = File::create( "mtgdata/majorization/deckcards.json" ).unwrap();
    let jsontowrite = json!(deckcards).to_string();
    file.write_all( jsontowrite.as_bytes() ).unwrap();


    decks.truncate(1500);

    
    let x = structs::Deck::get_dissimilarity_matrix( decks.clone(), &lands );

    let towrite = json!( x ).to_string();
    let mut file = File::create("dissimilaritiesmatrix.json").unwrap();
    file.write_all( towrite.as_bytes() ).unwrap();
    


    let output = Command::new("python3")
    .current_dir(".")
    .args(&["smacofmajorization.py"])
    .output()
    .expect("failed to execute process");


    thread::sleep(time::Duration::from_millis(5000));
    



    //std::fs::copy("smacofdeckpositions.json", "./mtgdata/majorization").unwrap(); 

    let output = Command::new("mv")
    .current_dir(".")
    .args(&["smacofdeckpositions.json", "./mtgdata/majorization"])
    .output()
    .expect("failed to execute process");




}



fn write_deck_assistant_server_dependencies(){


    let decks = actions::getdata::get_standard_decks_from_files();



    let mut towrite: Vec<Vec<String>> = Vec::new();

    for deck in decks.clone(){
        towrite.push( deck.cards );
    }

    let mut file = File::create( "deckassistant/decks.json" ).unwrap();
    let jsontowrite = json!(towrite ).to_string();
    file.write_all( jsontowrite.as_bytes() ).unwrap();





    let lands = actions::getdata::get_lands();

    let mut file = File::create( "deckassistant/lands.json" ).unwrap();
    let jsontowrite = json!( lands ).to_string();
    file.write_all( jsontowrite.as_bytes() ).unwrap();

    
    let mut towrite: (Vec<Vec<String>>, Vec<Vec<String>>) = (Vec::new(), Vec::new());
    

    //get x random cards from the deck
    for mut deck in decks.clone(){


        for x in 0..5{
            towrite.1.push( deck.get_lands(&lands) );
        }

        for x in 0..5{
            towrite.0.push( deck.get_non_lands(&lands) );

            deck.remove_x_random_cards(10);
        }
    
    }


    let mut file = File::create( "deckassistant/cardstolands.json" ).unwrap();
    let jsontowrite = json!(towrite ).to_string();
    file.write_all( jsontowrite.as_bytes() ).unwrap();




    //take a list of cards by mana cost
    //output the list of lands

    //input is the total mana symbols





    
    let alphabet = actions::getdata::get_standard_alphabet();

    let mut file = File::create( "deckassistant/standardalphabet.json" ).unwrap();
    let jsontowrite = json!(alphabet).to_string();
    file.write_all( jsontowrite.as_bytes() ).unwrap();


}


#[tokio::main]
async fn main() {



    //fetch_files().await;

    //write_bbbbeeee_dependencies();
    //write_deck_assistant_server_dependencies();

    //decks.truncate(500);
    //majorization::manual_majorization(decks);





    
    let decks = actions::getdata::get_standard_decks_from_files();
    let alphabet = actions::getdata::get_standard_alphabet();
    

    std::fs::create_dir("mtgdata");


    
    let cardtoassociated = get_cards_to_shared_cards( &decks, &alphabet ) ;
    let mut file = File::create( "mtgdata/cardtoassociated.json" ).unwrap();
    let jsontowrite = json!(cardtoassociated).to_string();
    file.write_all( jsontowrite.as_bytes() ).unwrap();



}


