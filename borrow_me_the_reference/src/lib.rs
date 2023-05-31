pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut next = 0;
    
    for c in s.chars() {
        match c {
            
            '-' => {
                
                result.pop(); 
            }
    
            '+' => {
                
                next += 1;
            }
            
            _ => {
                
                if next == 0 {
                    result.push(c);
                } else {
                    
                    next -= 1
                }
            }
        }
    }
    
    *s = result
}


pub fn do_operations(vec: &mut Vec<String>) {
    for equation in vec.iter_mut() {
        let parts: Vec<&str> = equation.split(|c| c == '+' || c == '-').collect();
        let operator = equation.chars().find(|c| *c == '+' || *c == '-');

        if let Some(op) = operator {
            let num1 = parts[0].parse::<i32>().unwrap();
            let num2 = parts[1].parse::<i32>().unwrap();
            let result = match op {
                '+' => num1 + num2,
                '-' => num1 - num2,
                _ => unreachable!(),
            };
            *equation = result.to_string();
        }
    }
}
