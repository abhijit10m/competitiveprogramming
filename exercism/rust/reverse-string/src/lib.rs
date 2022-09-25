pub fn reverse(input: &str) -> String {
    let mut stack: Vec<&str> = Vec::<&str>::new();
    let mut rev: Vec<&str> = Vec::<&str>::new();
    input.split("").for_each(|c| {stack.push(c)});
    while !stack.is_empty() {
        let character = stack.pop();
        if character.is_some() {
            rev.push(character.unwrap());
        }
        
    }
    rev.concat()
}
