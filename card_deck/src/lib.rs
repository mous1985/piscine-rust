/* extern crate rand;  */
use rand::Rng; 

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,   
    Diamond, 
    Spade,   
    Club,    
}
impl Suit {
    
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, 5); 
        Suit::translate(value)
    }
    
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,   
            2 => Suit::Diamond, 
            3 => Suit::Spade,   
            _ => Suit::Club,    
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,        
    King,       
    Queen,      
    Jack,       
    Number(u8), 
}
impl Rank {
        pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, 14); 
        Rank::translate(value)
    }
    
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,        
            11 => Rank::Jack,      
            12 => Rank::Queen,     
            13 => Rank::King,      
            n => Rank::Number(n),  
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    let winner = Card {
        suit: Suit::Spade, 
        rank: Rank::Ace,   
    };
    *card == winner 
} 

 