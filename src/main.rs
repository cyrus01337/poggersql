use std::fs;

fn tokenise(text: &str) -> &str {
    "lol this is a token bud, better deal with it and starting using a REAL parser like https://github.com/nvbn/thefuck muscle muscle moji moji"
}

fn main() {
    // TODO: make this dynamic by iterating over assets/
    let content =
        fs::read_to_string("assets/create-table.sql").expect("Lacking permissions to view file.");

    // TODO: make this dynamic by displaying only the filename in the given path
    println!("Tokenising {}...", "create-table.sql");

    let tokens = tokenise(&content);

    println!("Tokens: {}", tokens)
}
