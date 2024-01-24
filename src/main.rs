use std::io::{self, Write};

struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: text.to_string(),
        }
    }

    fn count_words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

fn main() {
    print!("Enter text: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        println!("Error: The entered text is empty.");
        return;
    }

    let word_counter = WordCounter::new(trimmed_input);

    let word_count = word_counter.count_words();

    println!("Word count: {}", word_count);
}
