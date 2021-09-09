

//a list of pairs and how many pairs there are in the decks
fn get_2_card_groups(decks: Vec<Vec<String>>) -> HashMap<BTreeSet<String>, u32>{
    
    let mut toreturn: HashMap<BTreeSet<String>, u32> = HashMap::new();
    
    //the 2 card groups
    for deck in decks{
        
        use std::iter::FromIterator;
        let deck = HashSet::<String>::from_iter( deck.iter().cloned() );
        let mut othercardsindeck = deck.clone();
        
        for card in deck{
            
            othercardsindeck.remove(&card);
            
            for othercard in othercardsindeck.iter(){
                
                let othercard = othercard.to_string();
                
                let mut cardset = BTreeSet::new();
                cardset.insert(card.clone());
                cardset.insert(othercard);
                
                
                if let Some(amount) = toreturn.get_mut(&cardset){
                    *amount +=1;
                }
                else{
                    toreturn.insert( cardset, 1);
                }
            }
        }
    }
    
    
    //if true, element is removed
    toreturn.drain_filter(|_, number| *number < 20);
    
    toreturn
}


