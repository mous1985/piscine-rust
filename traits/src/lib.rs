use std::fmt;
#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub strength: f64,
	pub score: i32,
	pub money: i32,
	pub weapons: Vec<String>,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Strength: {}, Score: {}, Money: {}", self.strength, self.score, self.money)?;
        write!(f, "Weapons: [")?;
        if let Some((last, weapons)) = self.weapons.split_last() {
            for weapon in weapons {
                write!(f, "{}, ", weapon)?;
            }
            write!(f, "{}", last)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player {
	pub fn eat<T:Food>(&mut self, food: T) {
		self.strength += food.gives();
		}
		
}


pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
	fn gives(&self) -> f64{
		self.weight_in_kg * 4.0
	}
	
}

impl Food for Meat {
	fn gives(&self) -> f64{
		let protein=self.weight_in_kg-(self.fat_content*self.weight_in_kg);
		let force_protein=protein * 4.0;
		force_protein+(self.fat_content * 9.0 * self.weight_in_kg)
	}
}
// this apple gives 4 units of strength
// Before eating Player { name: "player1", strength: 1.0, score: 0, money: 0, weapons: ["knife"] }
// After eating an apple
// player1
// Strength: 5, Score: 0, Money: 0
// Weapons: ["knife"]
// After eating a steak
// player1
// Strength: 14, Score: 0, Money: 0
// Weapons: ["knife"]

