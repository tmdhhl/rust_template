use heck::ToSnakeCase;

fn main() {
    let mut snake = String::new();
    for (i, ch) in "MINUTELY".char_indices() {
        if i > 0 && ch.is_uppercase() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    println!("{snake:?}");

    let x= "MINUTELY".to_snake_case();
    println!("{x}")
}
