fn increment_string(s: &str) -> String 
{
    if s.is_empty() { return "1".to_string(); }
    let mut alphabetic_part = String::new();
    let mut numeric_part    = String::new();
    let mut no_of_digits    = 0; 
    
    let alphanumeric_seperation_ind = s.char_indices()
                                        .rfind(|&(_, c)| !c.is_ascii_digit())
                                        .map(|(i, c)| i + c.len_utf8())
                                        .unwrap_or(0);

    let (alphabetic_part, numeric_part) = s.split_at(alphanumeric_seperation_ind);
    
    let mut digits_at_end: Vec<char> = numeric_part.chars().collect();
    let mut carry = true;
    
    for i in (0..digits_at_end.len()).rev() 
    {
        if carry 
        {
            if digits_at_end[i] == '9' 
            {
                digits_at_end[i] = '0';
                carry = true;
            } 
            else 
            {
                let next_digit   = digits_at_end[i].to_digit(10).unwrap() + 1;
                digits_at_end[i] = std::char::from_digit(next_digit, 10).unwrap();
                carry = false;
            }
        }
    }
    
    let mut numeric_update: String = digits_at_end.into_iter().collect();
    if carry 
    {
        numeric_update.insert(0, '1');
    }
    
    format!("{}{}", alphabetic_part, numeric_update)
}
