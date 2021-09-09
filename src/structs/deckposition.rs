


use std::collections::HashMap;

use std::collections::HashSet;


extern crate nalgebra as na;


use na::Point2;
use na::Vector2;


use super::Deck;



#[derive(Clone, PartialEq)]
pub struct DeckPosition{
    
    pub deck: Deck,
    
    pub position: Point2<f32>,

    
}

impl DeckPosition{


    fn new(deck: Deck, position: Point2<f32>) -> DeckPosition{

        DeckPosition{
            deck,
            position
        }
    }


    pub fn from_deck_with_random_initialization( deck: &Deck) -> DeckPosition{

        use rand::Rng;

        let mut rng = rand::thread_rng();

        let randompoint = Point2::new( rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5  );

        DeckPosition::new(deck.clone(), randompoint)
    }



    pub fn get_position(&self) -> (f32,f32){

        ( self.position.x , self.position.y )
    }

    pub fn clone_with_new_pos(&self, newpos: Point2<f32>) -> DeckPosition{

        let mut copy = self.clone();

        copy.position = newpos;

        copy
    }
    
    pub fn move_pos(&mut self, delta: Vector2<f32>){
        self.position = self.position + delta;
    }
    
    pub fn get_inverse_distance_new_length(oldlen: f32) -> f32{

        let toreturn = 1.0 / (oldlen + 0.05);

        toreturn.powf(1.0)
    }


    //the repulsion between this deck and another one
    fn get_repulsion_vector(&self, otherdeck: &DeckPosition) -> Vector2<f32>{

        //the vector from other deck to this deck, as a unit long
        let difference = self.position - otherdeck.position;

        let differencelength = difference.magnitude() + 0.05;
        let differenceunit = difference / differencelength;

        let closenessstress = DeckPosition::get_inverse_distance_new_length( differencelength );

        let newvec = differenceunit * closenessstress;

        let toreturn = newvec * self.deck.get_difference_score(&otherdeck.deck);


        toreturn
    }


    //get the vector of repulsion that is applied to this deck by other decks
    pub fn get_repulsion_vector_batch(&self, otherdecks: &Vec<DeckPosition>) -> Vector2<f32>{
        
        let mut sumvector = Vector2::new(0., 0.);

        for otherdeck in otherdecks.iter(){

            sumvector += self.get_repulsion_vector(otherdeck);
        }

        sumvector = sumvector/ (otherdecks.len() as f32 );

        sumvector += self.get_repulsion_from_edge_vector();

        sumvector = sumvector;

        return sumvector;
    }


    //a vector that points exactly to the center
    fn get_repulsion_from_edge_vector(&self) -> Vector2<f32>{

        let center = Point2::new(0.0, 0.0);

        let toreturnunit = center - self.position;
        let toreturnunit = toreturnunit / (toreturnunit.magnitude() + 0.001);


        let toreturndist = (center - self.position).magnitude();//.min(1.0);


        return toreturnunit * toreturndist * 0.8;
    }


    pub fn get_stress_score(decks: &Vec<DeckPosition>) -> f32{

        let mut score = 0.;

        for deck in decks{

            score += deck.get_repulsion_vector_batch(decks).magnitude();

            //score += deck.get_repulsion_from_edge_vector().magnitude();
        }

        

        return score;
    }
    
}




