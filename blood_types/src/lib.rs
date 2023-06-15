#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};
use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(RhFactor::Negative),
            "+" => Ok(RhFactor::Positive),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            cmp => cmp,
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen_end_index = if s.contains("AB") { 2 } else { 1 };
        let antigen = Antigen::from_str(&s[..antigen_end_index])?;
        let rh_factor = RhFactor::from_str(&s[antigen_end_index..])?;

        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rh_sign = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{:?}{}", self.antigen, rh_sign)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        self.donors().contains(other)
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();

        let compatible_rh_factor = match self.rh_factor {
            RhFactor::Positive => vec![RhFactor::Positive, RhFactor::Negative],
            RhFactor::Negative => vec![RhFactor::Negative],
        };

        let compatible_antigens = match self.antigen {
            Antigen::O => vec![Antigen::O],
            Antigen::A => vec![Antigen::A, Antigen::O],
            Antigen::B => vec![Antigen::B, Antigen::O],
            Antigen::AB => vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O],
        };

        for antigen in &compatible_antigens {
            for rh_factor in &compatible_rh_factor {
                donors.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                });
            }
        }

        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();

        
        let compatible_rh_factor = match self.rh_factor {
            RhFactor::Positive => vec![RhFactor::Positive],
            RhFactor::Negative => vec![RhFactor::Positive, RhFactor::Negative],
        };

        let compatible_antigens = match self.antigen {
            Antigen::O => vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O],
            Antigen::A => vec![Antigen::A, Antigen::AB],
            Antigen::B => vec![Antigen::B, Antigen::AB],
            Antigen::AB => vec![Antigen::AB],
        };

        for antigen in &compatible_antigens {
            for rh_factor in &compatible_rh_factor {
                recipients.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                });
            }
        }

        recipients
    }
}