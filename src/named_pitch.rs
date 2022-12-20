use std::ops::Add;

use crate::{pitch::{HasPitch, Pitch}, base::HasStaticName};

// Traits.


/// A trait for types that have a named pitch.
pub trait HasNamedPitch {
    /// Returns the named pitch of the type.
    fn named_pitch(&self) -> NamedPitch;
}

/// A trait for types that have a letter.
pub trait HasLetter {
    /// Returns the letter of the type.
    fn letter(&self) -> &'static str;
}

// Enum.

/// An enum representing named pitch.
/// 
/// A [`NamedPitch`] is a pitch that has a name, such as `C` or `F♯`.
/// While a [`Pitch`] is a pitch that has a frequency, a [`NamedPitch`] is a pitch that has an
/// enharmonic name (could share the same pitch with another).
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum NamedPitch {
    FTripleFlat,
    CTripleFlat,
    GTripleFlat,
    DTripleFlat,
    ATripleFlat,
    ETripleFlat,
    BTripleFlat,

    FDoubleFlat,
    CDoubleFlat,
    GDoubleFlat,
    DDoubleFlat,
    ADoubleFlat,
    EDoubleFlat,
    BDoubleFlat,

    FFlat,
    CFlat,
    GFlat,
    DFlat,
    AFlat,
    EFlat,
    BFlat,

    F,
    C,
    G,
    D,
    A,
    E,
    B,

    FSharp,
    CSharp,
    GSharp,
    DSharp,
    ASharp,
    ESharp,
    BSharp,

    FDoubleSharp,
    CDoubleSharp,
    GDoubleSharp,
    DDoubleSharp,
    ADoubleSharp,
    EDoubleSharp,
    BDoubleSharp,

    FTripleSharp,
    CTripleSharp,
    GTripleSharp,
    DTripleSharp,
    ATripleSharp,
    ETripleSharp,
    BTripleSharp,
}

// Impls.

impl HasNamedPitch for NamedPitch {
    fn named_pitch(&self) -> NamedPitch {
        *self
    }
}

impl HasLetter for NamedPitch {
    fn letter(&self) -> &'static str {
        match self {
            NamedPitch::FTripleFlat => "F",
            NamedPitch::CTripleFlat => "C",
            NamedPitch::GTripleFlat => "G",
            NamedPitch::DTripleFlat => "D",
            NamedPitch::ATripleFlat => "A",
            NamedPitch::ETripleFlat => "E",
            NamedPitch::BTripleFlat => "B",

            NamedPitch::FDoubleFlat => "F",
            NamedPitch::CDoubleFlat => "C",
            NamedPitch::GDoubleFlat => "G",
            NamedPitch::DDoubleFlat => "D",
            NamedPitch::ADoubleFlat => "A",
            NamedPitch::EDoubleFlat => "E",
            NamedPitch::BDoubleFlat => "B",

            NamedPitch::FFlat => "F",
            NamedPitch::CFlat => "C",
            NamedPitch::GFlat => "G",
            NamedPitch::DFlat => "D",
            NamedPitch::AFlat => "A",
            NamedPitch::EFlat => "E",
            NamedPitch::BFlat => "B",

            NamedPitch::F => "F",
            NamedPitch::C => "C",
            NamedPitch::G => "G",
            NamedPitch::D => "D",
            NamedPitch::A => "A",
            NamedPitch::E => "E",
            NamedPitch::B => "B",

            NamedPitch::FSharp => "F",
            NamedPitch::CSharp => "C",
            NamedPitch::GSharp => "G",
            NamedPitch::DSharp => "D",
            NamedPitch::ASharp => "A",
            NamedPitch::ESharp => "E",
            NamedPitch::BSharp => "B",

            NamedPitch::FDoubleSharp => "F",
            NamedPitch::CDoubleSharp => "C",
            NamedPitch::GDoubleSharp => "G",
            NamedPitch::DDoubleSharp => "D",
            NamedPitch::ADoubleSharp => "A",
            NamedPitch::EDoubleSharp => "E",
            NamedPitch::BDoubleSharp => "B",

            NamedPitch::FTripleSharp => "F",
            NamedPitch::CTripleSharp => "C",
            NamedPitch::GTripleSharp => "G",
            NamedPitch::DTripleSharp => "D",
            NamedPitch::ATripleSharp => "A",
            NamedPitch::ETripleSharp => "E",
            NamedPitch::BTripleSharp => "B",
        }
    }
}

impl HasStaticName for NamedPitch {
    fn static_name(&self) -> &'static str {
        match self {
            NamedPitch::FTripleFlat => "F♭𝄫",
            NamedPitch::CTripleFlat => "C♭𝄫",
            NamedPitch::GTripleFlat => "G♭𝄫",
            NamedPitch::DTripleFlat => "D♭𝄫",
            NamedPitch::ATripleFlat => "A♭𝄫",
            NamedPitch::ETripleFlat => "E♭𝄫",
            NamedPitch::BTripleFlat => "B♭𝄫",

            NamedPitch::FDoubleFlat => "F𝄫",
            NamedPitch::CDoubleFlat => "C𝄫",
            NamedPitch::GDoubleFlat => "G𝄫",
            NamedPitch::DDoubleFlat => "D𝄫",
            NamedPitch::ADoubleFlat => "A𝄫",
            NamedPitch::EDoubleFlat => "E𝄫",
            NamedPitch::BDoubleFlat => "B𝄫",

            NamedPitch::FFlat => "F♭",
            NamedPitch::CFlat => "C♭",
            NamedPitch::GFlat => "G♭",
            NamedPitch::DFlat => "D♭",
            NamedPitch::AFlat => "A♭",
            NamedPitch::EFlat => "E♭",
            NamedPitch::BFlat => "B♭",

            NamedPitch::F => "F",
            NamedPitch::C => "C",
            NamedPitch::G => "G",
            NamedPitch::D => "D",
            NamedPitch::A => "A",
            NamedPitch::E => "E",
            NamedPitch::B => "B",

            NamedPitch::FSharp => "F♯",
            NamedPitch::CSharp => "C♯",
            NamedPitch::GSharp => "G♯",
            NamedPitch::DSharp => "D♯",
            NamedPitch::ASharp => "A♯",
            NamedPitch::ESharp => "E♯",
            NamedPitch::BSharp => "B♯",

            NamedPitch::FDoubleSharp => "F𝄪",
            NamedPitch::CDoubleSharp => "C𝄪",
            NamedPitch::GDoubleSharp => "G𝄪",
            NamedPitch::DDoubleSharp => "D𝄪",
            NamedPitch::ADoubleSharp => "A𝄪",
            NamedPitch::EDoubleSharp => "E𝄪",
            NamedPitch::BDoubleSharp => "B𝄪",

            NamedPitch::FTripleSharp => "F♯𝄪",
            NamedPitch::CTripleSharp => "C♯𝄪",
            NamedPitch::GTripleSharp => "G♯𝄪",
            NamedPitch::DTripleSharp => "D♯𝄪",
            NamedPitch::ATripleSharp => "A♯𝄪",
            NamedPitch::ETripleSharp => "E♯𝄪",
            NamedPitch::BTripleSharp => "B♯𝄪",
        }
    }
}

impl HasPitch for NamedPitch {
    fn pitch(&self) -> Pitch {
        match self {
            NamedPitch::FTripleFlat => Pitch::D,
            NamedPitch::CTripleFlat => Pitch::A,
            NamedPitch::GTripleFlat => Pitch::E,
            NamedPitch::DTripleFlat => Pitch::B,
            NamedPitch::ATripleFlat => Pitch::FSharp,
            NamedPitch::ETripleFlat => Pitch::CSharp,
            NamedPitch::BTripleFlat => Pitch::GSharp,

            NamedPitch::FDoubleFlat => Pitch::DSharp,
            NamedPitch::CDoubleFlat => Pitch::ASharp,
            NamedPitch::GDoubleFlat => Pitch::F,
            NamedPitch::DDoubleFlat => Pitch::C,
            NamedPitch::ADoubleFlat => Pitch::G,
            NamedPitch::EDoubleFlat => Pitch::D,
            NamedPitch::BDoubleFlat => Pitch::A,

            NamedPitch::FFlat => Pitch::E,
            NamedPitch::CFlat => Pitch::B,
            NamedPitch::GFlat => Pitch::FSharp,
            NamedPitch::DFlat => Pitch::CSharp,
            NamedPitch::AFlat => Pitch::GSharp,
            NamedPitch::EFlat => Pitch::DSharp,
            NamedPitch::BFlat => Pitch::ASharp,

            NamedPitch::F => Pitch::F,
            NamedPitch::C => Pitch::C,
            NamedPitch::G => Pitch::G,
            NamedPitch::D => Pitch::D,
            NamedPitch::A => Pitch::A,
            NamedPitch::E => Pitch::E,
            NamedPitch::B => Pitch::B,
            
            NamedPitch::FSharp => Pitch::FSharp,
            NamedPitch::CSharp => Pitch::CSharp,
            NamedPitch::GSharp => Pitch::GSharp,
            NamedPitch::DSharp => Pitch::DSharp,
            NamedPitch::ASharp => Pitch::ASharp,
            NamedPitch::ESharp => Pitch::F,
            NamedPitch::BSharp => Pitch::C,

            NamedPitch::FDoubleSharp => Pitch::G,
            NamedPitch::CDoubleSharp => Pitch::D,
            NamedPitch::GDoubleSharp => Pitch::A,
            NamedPitch::DDoubleSharp => Pitch::E,
            NamedPitch::ADoubleSharp => Pitch::B,
            NamedPitch::EDoubleSharp => Pitch::FSharp,
            NamedPitch::BDoubleSharp => Pitch::CSharp,

            NamedPitch::FTripleSharp => Pitch::GSharp,
            NamedPitch::CTripleSharp => Pitch::DSharp,
            NamedPitch::GTripleSharp => Pitch::ASharp,
            NamedPitch::DTripleSharp => Pitch::F,
            NamedPitch::ATripleSharp => Pitch::C,
            NamedPitch::ETripleSharp => Pitch::G,
            NamedPitch::BTripleSharp => Pitch::D,
        }
    }
}

// Iterators.

impl Add<i8> for NamedPitch {
    type Output = Self;

    fn add(self, rhs: i8) -> Self {
        let index = ALL_PITCHES.iter().position(|&p| p == self).unwrap();

        let new_index = index as i8 + rhs;

        if !(0..=49).contains(&new_index) {
            panic!("NamedPitch out of range.");
        }

        ALL_PITCHES[new_index as usize]
    }
}

// Statics.

static ALL_PITCHES: [NamedPitch; 49] = [
    NamedPitch::FTripleFlat,
    NamedPitch::CTripleFlat,
    NamedPitch::GTripleFlat,
    NamedPitch::DTripleFlat,
    NamedPitch::ATripleFlat,
    NamedPitch::ETripleFlat,
    NamedPitch::BTripleFlat,

    NamedPitch::FDoubleFlat,
    NamedPitch::CDoubleFlat,
    NamedPitch::GDoubleFlat,
    NamedPitch::DDoubleFlat,
    NamedPitch::ADoubleFlat,
    NamedPitch::EDoubleFlat,
    NamedPitch::BDoubleFlat,

    NamedPitch::FFlat,
    NamedPitch::CFlat,
    NamedPitch::GFlat,
    NamedPitch::DFlat,
    NamedPitch::AFlat,
    NamedPitch::EFlat,
    NamedPitch::BFlat,

    NamedPitch::F,
    NamedPitch::C,
    NamedPitch::G,
    NamedPitch::D,
    NamedPitch::A,
    NamedPitch::E,
    NamedPitch::B,

    NamedPitch::FSharp,
    NamedPitch::CSharp,
    NamedPitch::GSharp,
    NamedPitch::DSharp,
    NamedPitch::ASharp,
    NamedPitch::ESharp,
    NamedPitch::BSharp,

    NamedPitch::FDoubleSharp,
    NamedPitch::CDoubleSharp,
    NamedPitch::GDoubleSharp,
    NamedPitch::DDoubleSharp,
    NamedPitch::ADoubleSharp,
    NamedPitch::EDoubleSharp,
    NamedPitch::BDoubleSharp,

    NamedPitch::FTripleSharp,
    NamedPitch::CTripleSharp,
    NamedPitch::GTripleSharp,
    NamedPitch::DTripleSharp,
    NamedPitch::ATripleSharp,
    NamedPitch::ETripleSharp,
    NamedPitch::BTripleSharp,
];