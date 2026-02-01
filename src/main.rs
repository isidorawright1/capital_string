fn main() {
    println!("Hello, world!");
}

fn capital_string(s: &str) -> String {
    s.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_capital_string(){
        assert_eq!(capital_string("capitalize"), "CAPITALIZE");
    }

    #[test]
    fn test_capital_string_as_owned(){
        let string_cap = String::from("newString"); 
        assert_eq!(capital_string(&string_cap), "NEWSTRING");
    }
}

