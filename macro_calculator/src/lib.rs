pub extern crate json;
pub use json::object;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_cals: f64 = 0.0;
    let mut total_fats: f64 = 0.0;
    let mut total_carbs: f64 = 0.0;
    let mut total_proteins: f64 = 0.0;

    for food in foods {
        let kcal_str: &str = food.calories[1].trim_end_matches("kcal");
        let kcal: f64 = kcal_str.parse().unwrap();
        total_cals += kcal * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
    }

    total_cals = (total_cals * 100.0).round() / 100.0;
    total_fats = (total_fats * 100.0).round() / 100.0;
    total_carbs = (total_carbs * 100.0).round() / 100.0;
    total_proteins = (total_proteins * 100.0).round() / 100.0;

    let total_cals: String = format!("{:.2}", total_cals);
    let total_fats: String = format!("{:.2}", total_fats);
    let total_carbs: String = format!("{:.2}", total_carbs);
    let total_proteins: String = format!("{:.2}", total_proteins);

    let total_cals: f64 = total_cals.parse().unwrap();
    let total_fats: f64 = total_fats.parse().unwrap();
    let total_carbs: f64 = total_carbs.parse().unwrap();
    let total_proteins: f64 = total_proteins.parse().unwrap();

    json::object! {
        "cals" => total_cals,
        "carbs" => total_carbs,
        "proteins" => total_proteins,
        "fats" => total_fats,
    }
}
