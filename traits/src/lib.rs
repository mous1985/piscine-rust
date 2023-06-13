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
        write!(f, "Name: {}\n", self.name)?;
        write!(
            f,
            "Strength: {}, Score: {}, Money: {}\n",
            self.strength, self.score, self.money
        )?;
        write!(f, "Weapons: ")?;
        for weapon in &self.weapons {
            write!(f, "{} ", weapon)?;
        }
        write!(f, "\n")
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
		force_protein+(self.fat_content*self.weight_in_kg)
	}
}