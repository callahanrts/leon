use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_numeric_char_reference_end_state(&mut self) -> Vec<Token> {

        // Check the character reference code.
        // If that number is one of the numbers in the first column of the following table,
        let new_char = match error_table(self.char_reference_code) {
            Some(c) => c,
            None => {
                let ref_code = self.char_reference_code;
                let mut new_char = '\u{0000}';

                // If the number is in the range 0xD800 to 0xDFFF or is greater than 0x10FFFF,
                if ref_code > 0x10FFF || (ref_code > 0xD800 && ref_code < 0xDFFF) {
                    // Parse error.
                    // Set the character reference code to 0xFFFD.
                    new_char = '\u{FFFD}';
                } else if parse_error_range(ref_code) {
                    //then this is a parse error.
                    // panic!("Parse Error");
                    new_char = '\u{0000}';
                }
                new_char
            }
        };

        // Set the temporary buffer to the empty string.
        // Append the Unicode character with code point equal to the character
        // reference code to the temporary buffer.
        self.tmp_buffer = String::from(format!("{}", new_char));

        // Switch to the character reference end state.
        self.state = State::CharReferenceEndState;
        Vec::new()
    }

}

fn parse_error_range(c: i64) -> bool {
    match c {
        // If the number is in the range
        0x0001 ... 0x0008 |
        0x000D ... 0x001F |
        0x007F ... 0x009F |
        0xFDD0 ... 0xFDEF |
        // or is one of
        0x000B | 0xFFFE | 0xFFFF | 0x1FFFE | 0x1FFFF | 0x2FFFE | 0x2FFFF | 0x3FFFE |
        0x3FFFF | 0x4FFFE | 0x4FFFF | 0x5FFFE | 0x5FFFF | 0x6FFFE | 0x6FFFF | 0x7FFFE | 0x7FFFF |
        0x8FFFE | 0x8FFFF | 0x9FFFE | 0x9FFFF | 0xAFFFE | 0xAFFFF | 0xBFFFE | 0xBFFFF | 0xCFFFE |
        0xCFFFF | 0xDFFFE | 0xDFFFF | 0xEFFFE | 0xEFFFF | 0xFFFFE | 0xFFFFF | 0x10FFFE | 0x10FFF => {
            true
        }
        _ => false
    }
}

// List of characters that constitute an error
fn error_table(c: i64) -> Option<char> {
    match c {
        0x00 => Some('\u{FFFD}'), // REPLACEMENT CHARACTER
        0x80 => Some('\u{20AC}'), // EURO SIGN (€)
        0x82 => Some('\u{201A}'), // SINGLE LOW-9 QUOTATION MARK (‚)
        0x83 => Some('\u{0192}'), // LATIN SMALL LETTER F WITH HOOK (ƒ)
        0x84 => Some('\u{201E}'), // DOUBLE LOW-9 QUOTATION MARK („)
        0x85 => Some('\u{2026}'), // HORIZONTAL ELLIPSIS (…)
        0x86 => Some('\u{2020}'), // DAGGER (†)
        0x87 => Some('\u{2021}'), // DOUBLE DAGGER (‡)
        0x88 => Some('\u{02C6}'), // MODIFIER LETTER CIRCUMFLEX ACCENT (ˆ)
        0x89 => Some('\u{2030}'), // PER MILLE SIGN (‰)
        0x8A => Some('\u{0160}'), // LATIN CAPITAL LETTER S WITH CARON (Š)
        0x8B => Some('\u{2039}'), // SINGLE LEFT-POINTING ANGLE QUOTATION MARK (‹)
        0x8C => Some('\u{0152}'), // LATIN CAPITAL LIGATURE OE (Œ)
        0x8E => Some('\u{017D}'), // LATIN CAPITAL LETTER Z WITH CARON (Ž)
        0x91 => Some('\u{2018}'), // LEFT SINGLE QUOTATION MARK (‘)
        0x92 => Some('\u{2019}'), // RIGHT SINGLE QUOTATION MARK (’)
        0x93 => Some('\u{201C}'), // LEFT DOUBLE QUOTATION MARK (“)
        0x94 => Some('\u{201D}'), // RIGHT DOUBLE QUOTATION MARK (”)
        0x95 => Some('\u{2022}'), // BULLET (•)
        0x96 => Some('\u{2013}'), // EN DASH (–)
        0x97 => Some('\u{2014}'), // EM DASH (—)
        0x98 => Some('\u{02DC}'), // SMALL TILDE (˜)
        0x99 => Some('\u{2122}'), // TRADE MARK SIGN (™)
        0x9A => Some('\u{0161}'), // LATIN SMALL LETTER S WITH CARON (š)
        0x9B => Some('\u{203A}'), // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK (›)
        0x9C => Some('\u{0153}'), // LATIN SMALL LIGATURE OE (œ)
        0x9E => Some('\u{017E}'), // LATIN SMALL LETTER Z WITH CARON (ž)
        0x9F => Some('\u{0178}'), // LATIN CAPITAL LETTER Y WITH DIAERESIS (Ÿ)
        _ => None
    }
}
