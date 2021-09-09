use crate::Deck;


pub fn get_standard_decks_from_files() -> Vec<Deck>{
    
    let alphabet = get_standard_alphabet();

    let mut toreturn = Vec::new();
    
    let dir = "./decks";
    
    for entry in std::fs::read_dir(dir).unwrap(){
        
        let entry = entry.unwrap();
        
        let mut cardsindeck = Vec::new();
        
        let contents = std::fs::read_to_string( entry.path() ).expect("Something went wrong reading the file");
        let cards: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");

        
        for card in cards.as_array().unwrap(){
            
            let cardname = card.as_array().unwrap()[0].as_str().unwrap().to_uppercase();
            
            let amountofcard = card.as_array().unwrap()[1].as_u64().unwrap();
            
            for x in 0..amountofcard{
                cardsindeck.push(cardname.clone() );
            }
        }
        
        
        if let Some(deck) = verify_and_capitalize_deck( &cardsindeck, &alphabet ){
            toreturn.push( Deck::new_from( deck ) );
        }

        
    }

    println!("got decks");
        
    toreturn
}








pub fn get_standard_alphabet() -> Vec<String>{
    
    let contents = std::fs::read_to_string( "oracle-cards.json" ).expect("Something went wrong reading the file");
    
    let standardcards: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    let mut thecards = Vec::new();
    
    for card in standardcards.as_array().unwrap(){

        let standardlegal = card.get("legalities").unwrap().get("standard").unwrap().as_str().unwrap();
        let standardlegal = standardlegal == "legal";
        
        let name = card.get("name").unwrap().as_str().unwrap();
        
        if standardlegal{
            
            thecards.push( name.to_string() );
        }   
    }
    
    thecards.sort();
    
    return thecards;
}









pub fn get_lands() -> Vec<String>{

    let contents = std::fs::read_to_string( "oracle-cards.json" ).expect("Something went wrong reading the file");
    let standardcards: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    
    let mut toreturn = Vec::new();
    
    for card in standardcards.as_array().unwrap(){
        
        let standardlegal = card.get("legalities").unwrap().get("standard").unwrap().as_str().unwrap();
        let standardlegal = standardlegal == "legal";
        

        if standardlegal{

            let name = card.get("name").unwrap().as_str().unwrap().to_string();

            let typeline = card.get("type_line").unwrap().as_str().unwrap();

            if typeline.contains("Land"){
                toreturn.push(name);
            }
            else{
            }
            
            
        }
    }
    
    
    return toreturn;
}











pub fn verify_and_capitalize_deck(deck: &Vec<String>, alphabet: &Vec<String>) -> Option<Vec<String>>{


    let mut capitalizedmapping = HashMap::new();
    for card in alphabet{
        capitalizedmapping.insert( card.to_uppercase(), card );
    }


    let mut toreturn = Vec::new();

    for card in deck{
        let card = card.to_uppercase();

        if let Some(goodname) = capitalizedmapping.get(&card){
            toreturn.push(goodname.to_string());
        }
        else{
            return None;
        }
    }

    if toreturn.len() < 60{

        //println!("INVALID DECK {:?}", toreturn.len());
        return None;
    }

    return Some(toreturn);
}



use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;








//the colours of each card
pub fn get_colour_mapping() -> HashMap<String, (u32,u32,u32,u32,u32)>{
    //mana_cost

    let contents = std::fs::read_to_string( "oracle-cards.json" ).expect("Something went wrong reading the file");
    let standardcards: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    
    let mut toreturn = HashMap::new();
    
    
    for card in standardcards.as_array().unwrap(){
        
        let standardlegal = card.get("legalities").unwrap().get("standard").unwrap().as_str().unwrap();
        let standardlegal = standardlegal == "legal";
        

        if standardlegal{

            let name = card.get("name").unwrap().as_str().unwrap().to_string();

            //println!("my name? {:?}", name);

            let manacost = card.get("color_identity").unwrap().as_array().unwrap();

            //println!("cost: {:?}", manacost);

            let mut temp = Vec::new();
            for x in manacost{
                temp.push( x.as_str().unwrap() );
            }

            let w = temp.contains(&"W") as u32;
            let u = temp.contains(&"U") as u32;
            let b = temp.contains(&"B") as u32;
            let r = temp.contains(&"R") as u32;
            let g = temp.contains(&"G") as u32;

            
            toreturn.insert( name, (w,u,b,r,g)  );
        }
    }
    
    
    return toreturn;
}






