//! Ethiopic script Unicode definitions and utilities.

/// Standard Ethiopic punctuation characters.
pub mod punctuation {
    /// Ethiopic wordspace (፡) U+1361
    pub const WORD_SPACE: char = '\u{1361}';
    /// Ethiopic full stop (።) U+1362
    pub const FULL_STOP: char = '\u{1362}';
    /// Ethiopic comma (፣) U+1363
    pub const COMMA: char = '\u{1363}';
    /// Ethiopic semicolon (፤) U+1364
    pub const SEMICOLON: char = '\u{1364}';
    /// Ethiopic colon (፥) U+1365
    pub const COLON: char = '\u{1365}';
    /// Ethiopic preface colon (፦) U+1366
    pub const PREFACE_COLON: char = '\u{1366}';
    /// Ethiopic question mark (፧) U+1367
    pub const QUESTION_MARK: char = '\u{1367}';
    /// Ethiopic paragraph separator (፨) U+1368
    pub const PARAGRAPH_SEPARATOR: char = '\u{1368}';
}

/// Ethiopic numerals 1–10,000.
pub mod numerals {
    /// Ethiopic digit one (፩) U+1369
    pub const ONE: char = '\u{1369}';
    /// Ethiopic digit two (፪) U+136A
    pub const TWO: char = '\u{136A}';
    /// Ethiopic digit three (፫) U+136B
    pub const THREE: char = '\u{136B}';
    /// Ethiopic digit four (፬) U+136C
    pub const FOUR: char = '\u{136C}';
    /// Ethiopic digit five (፭) U+136D
    pub const FIVE: char = '\u{136D}';
    /// Ethiopic digit six (፮) U+136E
    pub const SIX: char = '\u{136E}';
    /// Ethiopic digit seven (፯) U+136F
    pub const SEVEN: char = '\u{136F}';
    /// Ethiopic digit eight (፰) U+1370
    pub const EIGHT: char = '\u{1370}';
    /// Ethiopic digit nine (፱) U+1371
    pub const NINE: char = '\u{1371}';
    /// Ethiopic number ten (፲) U+1372
    pub const TEN: char = '\u{1372}';
    /// Ethiopic number twenty (፳) U+1373
    pub const TWENTY: char = '\u{1373}';
    /// Ethiopic number one hundred (፻) U+137B
    pub const HUNDRED: char = '\u{137B}';
    /// Ethiopic number ten thousand (፼) U+137C
    pub const TEN_THOUSAND: char = '\u{137C}';
}

/// Vowel order indices for Ethiopic Fidel syllables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VowelOrder {
    /// 1st order: Ge'ez (ግዕዝ) - ä
    First = 1,
    /// 2nd order: Ka'eb (ካዕብ) - u
    Second = 2,
    /// 3rd order: Sales (ሣልስ) - i
    Third = 3,
    /// 4th order: Rabe (ራብዕ) - a
    Fourth = 4,
    /// 5th order: Hames (ኃምስ) - e
    Fifth = 5,
    /// 6th order: Sades (ሳድስ) - ə / standalone consonant
    Sixth = 6,
    /// 7th order: Sabe (ሳብዕ) - o
    Seventh = 7,
}

/// Checks whether a character belongs to any defined Ethiopic Unicode block.
#[inline]
pub fn is_ethiopic_character(c: char) -> bool {
    let u = c as u32;
    // Ethiopic base block
    (0x1200..=0x137F).contains(&u)
    // Ethiopic Supplement
    || (0x1380..=0x139F).contains(&u)
    // Ethiopic Extended
    || (0x2D80..=0x2DDF).contains(&u)
    // Ethiopic Extended-A
    || (0xAB00..=0xAB2F).contains(&u)
}
