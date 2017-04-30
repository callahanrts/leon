#[cfg(test)]
use super::*;

fn tokenizer(input: String) -> Tokenizer {
    Tokenizer{
        pos: 0,
        input: input,
    }
}

