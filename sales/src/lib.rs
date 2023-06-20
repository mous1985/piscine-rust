#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items:Vec<(String, f32)>,
    pub receipt:Vec<f32>
}
impl Cart {
    pub fn new() -> Cart {
        Cart { 
             items:Vec::new(),
             receipt:Vec::new()
            }

    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for i in s.products.iter() {
            if ele == i.0 {
                 self.items.push(i.clone());
            }
        }
        
    }

    pub fn generate_receipt(&mut self)->Vec<f32>  {


   
    //  let mut result :Vec<f32>= Vec :: new();
     let mut nouv :Vec<f32>= Vec :: new();
    let _ = &self.items.sort_by(|a,b|a.partial_cmp(b).unwrap());
    
for i in self.items.iter(){
    nouv.push(i.1);
}

      let  temp = nouv.len()/3;
    
     let total:f32 = nouv.iter().sum();
    
    
    let  tab_prix_offert : Vec<_>= nouv[0..temp].iter().cloned().collect();
    let prix:f32= tab_prix_offert.iter().sum();
     let prix_reduc = total-prix;
     
    let prix_reduc_per = ((prix_reduc * 100.0)/total) / 100.0;
    
 for i in nouv.iter(){
        self.receipt.push(((i * prix_reduc_per * 100.0).round()) / 100.0)
     }
    
    
    // dbg!(prix_reduc_per);
    self.receipt.sort_by(|a,b|a.partial_cmp(b).unwrap());
     self.receipt.clone()
    
    }
}