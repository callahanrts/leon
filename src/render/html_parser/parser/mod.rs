
struct Parser<'a> {
    pos: usize,
    input: &'a str,

    insertion_mode: &'a str,
    original_insertion_mode: &'a str,
    template_insertion_modes: Vec<&'a str>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser {
        Parser{
            pos: 0,
            input: input,

            insertion_mode: "",
            original_insertion_mode: "",
            template_insertion_modes: Vec::new(),
        }
    }

    // http://w3c.github.io/html/syntax.html#the-insertion-mode
    fn reset_insertion_mode() {
    }

}
