

use std::collections::HashMap;


use nalgebra::Point2;
use nalgebra::Vector2;


pub fn get_card_to_id(alphabet: &Vec<String>) -> HashMap<String, usize>{

    let mut toreturn = HashMap::new();

    let mut cardid = 0;

    for card in alphabet{
        toreturn.insert(  card.clone(), cardid );
        cardid += 1;
    }

    toreturn
}




#[derive(Clone, PartialEq)]
//count cards
pub struct Deck{
    
    pub cards: Vec<String>,
}


impl Deck{

    pub fn new_from( x: Vec<String>) -> Deck{

        Deck{
            cards: x
        }
    }
    

    pub fn get_non_lands(&self, lands: &Vec<String>) -> Vec<String>{

        let mut toreturn = Vec::new();

        for card in &self.cards{

            if !lands.contains(&card){

                toreturn.push(card.clone());
            }
        }

        return toreturn;
    }

    pub fn remove_x_random_cards(&mut self, number: u32 ){

        use rand::seq::SliceRandom; 

        for _ in 0..number{

            //get a random card in the list of cards
            let element = self.cards.choose(&mut rand::thread_rng()).unwrap().clone(); 

            self.cards.remove( self.cards.iter().position(|x| *x == element).expect("needle not found") );
        }

    }


    pub fn get_lands(&self, lands: &Vec<String>) -> Vec<String>{

        let mut toreturn = Vec::new();

        for card in &self.cards{

            if lands.contains(&card){

                toreturn.push(card.clone());
            }
        }

        return toreturn;
    }


    pub fn get_one_hot_vector(&self, alphabet: &Vec<String>) -> Vec<f32>{

        let mut vector = vec![0.0 ; alphabet.len()  ];

        let cardtoid = get_card_to_id(alphabet);

        for card in &self.cards{

            let cardid = cardtoid.get(card).unwrap();

            *vector.get_mut(*cardid).unwrap() += 0.25;
        }
        
        return vector;
    }

    
    
    pub fn get_dissimilarity_matrix( mut decks: Vec<Deck> , lands: &Vec<String>)  -> Vec<Vec<f32>> {
        
    
        for deck in decks.iter_mut(){
            deck.remove_cards(&lands);
        }
        
        
        let mut dissimilaritiesmatrix: Vec<Vec<f32>> = Vec::new();
        
    
        for deck1 in decks.iter(){
            
            let mut repulsionarray = Vec::new();
            
            for deck2 in decks.iter(){
                
                let repulsionscore = deck1.get_difference_score(deck2);
                
                repulsionarray.push(repulsionscore);
            }
            
            dissimilaritiesmatrix.push(repulsionarray);
        }
        
        
        return dissimilaritiesmatrix;    
    }
    


    pub fn remove_cards(&mut self, cards: &Vec<String>){


        //Retains what returns true (if the card is not the one requested)
        self.cards.retain(|x| !cards.contains( x )  );
    }


    
    
    pub fn get_as_name_and_amount(&self) -> HashMap<String, u32>{
        
        let mut toreturn: HashMap<String, u32> = HashMap::new();
        
        for card in &self.cards{
            
            if let Some(amount) = toreturn.get_mut(card){
                *amount +=1;
            }
            else{
                
                toreturn.insert( card.to_string() , 1 );
            }
        }
        
        toreturn
    }
    
    
    pub fn add_card(&mut self, name: &String){
        
        self.cards.push(name.clone());
    }
    
    
    //get the cards in order of most to least common 
    pub fn get_most_common(&self) -> Vec<(String, u32)>{
        
        
        let mut newvec = Vec::new();
        
        for (card, amount) in self.get_as_name_and_amount().iter(){
            
            newvec.push( (card.to_string(), *amount) );
        }
        
        newvec.sort_by(|a, b| b.1.cmp(&a.1) );
        
        
        newvec.truncate(15);
        
        if newvec.len() < 15{
            panic!("less than X other associated cards here when doing this for the combo thing");
        }
        
        return newvec;
        
    }
    
    
    //get the amount of cards in the deck
    fn card_amount(&self) -> u32{
        return self.cards.len() as u32;
    }
    
    fn name_and_amount_count(x : &HashMap<String, u32>) -> u32{
        let mut total = 0;
        
        for (card, amount) in x.iter(){
            total += amount;
        }
        
        return total;
    }

    
    //0 if exactly the same 1 is completely different
    pub fn get_difference_score(&self, otherdeck: &Deck) -> f32{
        
        //the total amount of cards in both deck
        //minus the amount in common
        //divided by the total amount of cards in both decks
        
        let bothtotal = self.card_amount() + otherdeck.card_amount();

        let mut amountincommon = 0;
        
        let mut othercopy = otherdeck.clone();
        
        for card in self.cards.iter(){

            if let Some(pos) = othercopy.cards.iter().position(|x| *x == *card) {

                othercopy.cards.remove(pos);
                amountincommon += 1;
            }
        }

        let bothtotal = bothtotal as f32;
        let amountincommon = amountincommon as f32;
        let toreturn = (bothtotal - amountincommon ) / bothtotal;

        //println!("toreturn {:?}", toreturn);
        
        //at least a score of 0.1
        return toreturn.powf(1.5);
    }
    
        
    pub fn remove_lands(&mut self, lands: Vec<String>){


        self.cards.retain( |x|

            {
                let strx: &str = &x;

                !lands.contains(&strx.to_string()) 
            });

    }



    pub fn deck_to_array(&self, alphabet: Vec<String>) -> Vec<u32>{

        let mut toreturn = Vec::new();

        for name in &self.cards{

            let id = alphabet.iter().position(|r| &r == &name).unwrap() as u32;

            toreturn.push(id);
        } 

        toreturn
    }



    
    pub fn get_color(&self, colourmapping: &HashMap<String, (u32, u32, u32, u32, u32)> ) -> (f32, f32, f32 ){     
        

        let mut colours = (0,0,0,0,0);

        for card in &self.cards{

            let x = colourmapping.get(card).unwrap();

            colours.0 += x.0;
            colours.1 += x.1;
            colours.2 += x.2;
            colours.3 += x.3;
            colours.4 += x.4;
        }


        let rgb = colours_to_rgb(colours.0,colours.1,colours.2,colours.3,colours.4 );
        
        (rgb.0, rgb.1, rgb.2)
    }


    //the list of 
    pub fn as_weighted_array(&self, standardalphabet: &Vec<String>) -> Vec<(u32, f32)>{
            

        let mut toreturn = HashMap::new();

        for name in &self.cards{

            let id = standardalphabet.iter().position(|r| &r == &name).unwrap() as u32;

            if let Some(amount) = toreturn.get_mut(&id){
                *amount += 0.25;
            }
            else{
                toreturn.insert(id, 0.25 );
            }
        }

        let mut tovector = Vec::new();

        for (id, mut amount) in toreturn{

            if amount > 1.0{
                amount = 1.0;            
            }

            tovector.push((id, amount));
        }

        tovector
    }











}




fn colours_to_rgb(w:u32, u:u32, b:u32, r: u32, g: u32) -> (f32, f32, f32 ){
    
    let total = w + u + b + r + g;
    
    let w = w as f32 / total as f32;
    let u = u as f32 / total as f32;
    let b = b as f32 / total as f32;
    let r = r as f32 / total as f32;
    let g = g as f32 / total as f32;
    
    /*
    let mut rgb = (0. , 0., 0.);
    let w_rgb = (w*1.,w*1.,w*1.);
    let u_rgb = (  0.,  0.,u*1.);
    let b_rgb = (0.,0.,0.);
    */
    
    let r = w+r;
    let g = w+g;
    let b = w+u;
    
    
    //let rgb = (w+r, w+g, w+u);
    
    //println!("the coloru {:?}", (r,g,b));
    let rgb = ( r.min(1.), g.min(1.), b.min(1.) );
    //println!("the coloru {:?}", rgb);
    
    
    return (rgb.0, rgb.1, rgb.2);
}


fn rgb_to_hex(rgb: (f32,f32,f32)) -> String{


    let r = ((rgb.0 * 16. *16. ) as u64 ).saturating_sub(1);
    let g = ((rgb.1 * 16. *16. ) as u64 ).saturating_sub(1);
    let b = ((rgb.2 * 16. *16. ) as u64 ).saturating_sub(1);
    
    let hex = format!( "#{:02X}{:02X}{:02X}", r,g,b );
    
    return hex
    
}





