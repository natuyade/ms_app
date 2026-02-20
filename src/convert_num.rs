pub fn num_convert(number: usize) -> String {
    match number {
        0 => "0⃣".to_string(),
        1 => "1⃣".to_string(),
        2 => "2⃣".to_string(),
        3 => "3⃣".to_string(),
        4 => "4⃣".to_string(),
        5 => "5⃣".to_string(),
        6 => "6⃣".to_string(),
        7 => "7⃣".to_string(),
        8 => "8⃣".to_string(),
        9 => "9⃣".to_string(),
        _ => "".to_string()
    }
}