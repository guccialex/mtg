

use std::fs;
use std::env;
use std::path::Path;

mod plot;



fn main() {

    println!("Hello, world!");

    //get the list of decks and their positions

    let path = env::current_dir().unwrap();
    println!("{:?}", path);


    let root = Path::new("..");
    assert!(env::set_current_dir(&root).is_ok());
    println!("Successfully changed working directory to {}!", root.display());


    let path = env::current_dir().unwrap();
    println!("{:?}", path);


    //let result: serde_json::Value = serde_json::from_str("mldecks_no_mutations.json").expect("JSON was not well-formatted");


    let contents = fs::read_to_string("mldecks_no_mutations.json").expect("Something went wrong reading the file");

    let v: Vec<Vec<Vec<(u32, f32)>>> = serde_json::from_str(&contents).unwrap();
    //println!( "{:?}", v);


    let mut listofdecks = Vec::new();

    use rand::Rng;
    let mut rng = rand::thread_rng();

    for deck in v{

        let position = (rng.gen::<f32>(), rng.gen::<f32>());

        let mut cards = Vec::new();

        for (cardid, amount) in &deck[0]{

            let amount = (amount * 4.) as u32;

            for amount in 0..amount{
                cards.push(*cardid);
            }
        }


        let toreturn = Deck{

            position: position,
            cards: cards
        };

        listofdecks.push(toreturn);
    }


    let repvec = get_repulsion_vector( &listofdecks[0], &listofdecks[1]);

    println!("the vec {:?}", repvec);

    plot::plot();

}


struct Deck{

    position: (f32,f32),

    cards: Vec<u32>,

}


//get the gradient in the X and Y direction for this deck, with the list of decks
fn get_gradient(deck: Deck,  otherdecks: Vec<Deck>){

}



fn get_repulsion_vector(deck: &Deck, otherdeck: &Deck) -> (f32,f32){


    //get the direction of other deck to this deck
    //this deck minus other deck

    let distancevector = (deck.position.0 - otherdeck.position.0, deck.position.1 - otherdeck.position.1);

    //square inverse the distance vector
    //I want the force of the repulsion vector to have an inverse relationship to the distance of the two objects

    //new length = 1/old length
    let newlength = 1. / ((distancevector.0.powf(2.) + distancevector.1.powf(2.)).sqrt() + 0.1);

    let distancevector = (distancevector.0 * newlength * newlength, distancevector.1 * newlength * newlength);

    //and multiply it by the repulsion score

    distancevector
}



fn get_repulsion_score(deck1: &Vec<String>, deck2: &Vec<String>) -> f32{
    
    //get the cards in deck 1 not in deck 2
    
    //get the cards in deck 2 not in deck 1
    
    //remove the items that are in both of them
    
    //then sum the items left and divide by two
    
    //thats the average amount of items in one and not the other
    
    //then divide by 60
    
    let mut deck1copy = deck1.clone();
    let mut deck2copy = deck2.clone();
    
    for card in deck1{
        
        //remove it from both decks if its in both decks ("well its in the first deck at least of course")
        if let Some(deck1index) = deck1copy.iter().position(|x| x == card){
            
            if let Some(deck2index) = deck2copy.iter().position(|x| x == card){
                
                //println!("the common cards {:?}", card);
                
                deck1copy.remove(deck1index);
                
                deck2copy.remove(deck2index);
            }
        }
        
    }
    
    let difference = (deck1copy.len() as f32 + deck2copy.len() as f32) / 2.0;
    let difference = difference / 60.0;
    
    
    return difference;
}

