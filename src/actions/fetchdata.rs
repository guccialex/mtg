
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::error::Error;

use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use std::io;





pub async fn write_oracle_cards_to_file() {

    let client: reqwest::Client = reqwest::ClientBuilder::new().cookie_store(true).build().unwrap();
    
    let url = format!("https://api.scryfall.com/bulk-data" );
        
    let result = client.get(url).send().await.unwrap();
    
    let json: serde_json::Value = result.json().await.unwrap();


    for x in json.get("data").unwrap().as_array().unwrap(){

        if x.get("type").unwrap().as_str().unwrap() == "oracle_cards"{

            let oracleurl = x.get("download_uri").unwrap().as_str().unwrap();
 
            let result = reqwest::get(oracleurl).await.unwrap().text().await.unwrap();

            let mut out = File::create("oracle-cards.json").unwrap();

            out.write_all( result.as_bytes() ).unwrap();
        }
    }
    
}




//given the list of tourneys
//get the decks from these tourneys
//and return a list of the decks and then the names of the cards in the deck, and how many of them there are
pub async fn write_decks_to_file(client: &reqwest::Client, tids: Vec<u32>) {
    
    
    for x in tids{
        
        let url = format!("https://mtgmeta.io/api/tournaments/{}", x);
        
        let result = client.get(url).send().await.unwrap();
        
        let json: serde_json::Value = result.json().await.unwrap();
        
        let decks = json.get("data").unwrap().get("results").unwrap().as_array().unwrap();
        
        let mut deckintourney = 0;
        
        for deck in decks{
            
            let mut parseddeck = Vec::new();
            
            let decklist = deck.get("decklist").unwrap();
            
            let maindeck = decklist.get("main").unwrap().as_array().unwrap();
            
            for card in maindeck{
                
                let name = card.get("card").unwrap().as_str().unwrap().to_string();
                
                let quantity = card.get("quantity").unwrap().as_str().unwrap().parse::<u32>().unwrap();
                
                parseddeck.push(  (name, quantity) );
            }
            
            
            //deck id is tourney id * 100 + cur deck in tourney
            let deckid = x * 100 + deckintourney;

            println!("fethed deck {:?}", deckid);

            let mut file = File::create( format!("decks/{}.json", deckid) ).unwrap();
            use serde_json::json;
            let tourneyidstring= json!(parseddeck).to_string();
            file.write_all( tourneyidstring.as_bytes() ).unwrap();
            
            
            deckintourney += 1;
        }
        
        
        //1 every second
        let wait = time::Duration::from_millis(1000);
        thread::sleep(wait);
    }
    
}




pub async fn get_standard_tids( client: &reqwest::Client ) -> Vec<u32> {
    
    
    let mut standardtourneys: Vec<u32> = Vec::new();
    
    //the last 10 pages of standard tournaments
    for x in 0..10{
        
        let url = format!("https://mtgmeta.io/api/tournaments/standard/{}", x);
        
        let result = client.get(url).send().await.unwrap();
        
        let json : serde_json::Value = result.json().await.unwrap();
        

        let tourneys = json.get("data").unwrap().get("tournaments").unwrap().as_array().unwrap();
        
        for tournament in tourneys{
            
            let id = tournament.get("tid").unwrap().as_str().unwrap();
            
            println!("result {:?}", id );
            
            standardtourneys.push( id.parse::<u32>().unwrap() );
        }
        
        
        //1 every  second
        let wait = time::Duration::from_millis(1000);
        thread::sleep(wait);
    }

    
    return standardtourneys;
}




