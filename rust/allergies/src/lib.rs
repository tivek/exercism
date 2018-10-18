pub struct Allergies {
    score: u8,
}

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score: score as u8 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        0 != (self.score & match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        })
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8)
            .map(|n| 1 << n)
            .filter_map(|code| match self.score & code {
                1 => Some(Allergen::Eggs),
                2 => Some(Allergen::Peanuts),
                4 => Some(Allergen::Shellfish),
                8 => Some(Allergen::Strawberries),
                16 => Some(Allergen::Tomatoes),
                32 => Some(Allergen::Chocolate),
                64 => Some(Allergen::Pollen),
                128 => Some(Allergen::Cats),
                _ => None,
            }).collect()
    }
}
