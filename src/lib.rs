pub mod parser;
pub mod polyphonia;
pub mod translator;
pub mod utils;
pub mod wav;

use polyphonia::{notable_notes, Amplitude};
use std::{str::FromStr, sync::LazyLock};

const DOT_DURATION: f32 = 0.1;
const LETTER_SEPARATION_DURATION: f32 = DOT_DURATION * 3.0;
const LINE_DURATION: f32 = DOT_DURATION * 3.0;
const SLASH_DURATION: f32 = DOT_DURATION * 4.0;

static LETTER_SEPARATION: LazyLock<Vec<i16>> =
    LazyLock::new(|| notable_notes::G0.audio_wave(LETTER_SEPARATION_DURATION, &Amplitude::Silent));

#[derive(Debug)]
pub struct Letter<'a>(&'a str, &'a str);

impl Letter<'_> {
    pub fn concat_morse<'a, T>(args: T) -> impl Iterator<Item = u8> + 'a
    where
        T: IntoIterator<Item = Letter<'a>> + 'a,
    {
        args.into_iter()
            .flat_map(|Letter(_, morse)| morse.as_bytes().iter().chain(b" "))
            .cloned()
    }

    pub fn concat_text<'a, T>(args: T) -> impl Iterator<Item = u8> + 'a
    where
        T: IntoIterator<Item = Letter<'a>> + 'a,
    {
        args.into_iter()
            .flat_map(|Letter(text, _)| text.as_bytes())
            .copied()
    }

    pub fn concat_audio<'a, T: Iterator<Item = Letter<'a>> + 'a>(
        args: T,
    ) -> impl Iterator<Item = i16> + 'a {
        args.map(|Letter(_, y)| y)
            .flat_map(|x| x.chars())
            .flat_map(|ch| {
                let mut chunk = match ch {
                    '.' => notable_notes::A4.audio_wave(DOT_DURATION, &Amplitude::Medium),
                    '-' => notable_notes::A4.audio_wave(LINE_DURATION, &Amplitude::Medium),
                    '/' => notable_notes::A4.audio_wave(SLASH_DURATION, &Amplitude::Silent),
                    _ => Vec::new(),
                };
                chunk.extend_from_slice(&LETTER_SEPARATION);
                chunk
            })
    }
}

pub mod morse_alphabet {
    use crate::Letter;

    pub const A: Letter = Letter("a", ".-");
    pub const B: Letter = Letter("b", "-...");
    pub const C: Letter = Letter("c", "-.-.");
    pub const D: Letter = Letter("d", "-..");
    pub const E: Letter = Letter("e", ".");
    pub const F: Letter = Letter("f", "..-.");
    pub const G: Letter = Letter("g", "--.");
    pub const H: Letter = Letter("h", "....");
    pub const I: Letter = Letter("i", "..");
    pub const J: Letter = Letter("j", ".---");
    pub const K: Letter = Letter("k", "-.-");
    pub const L: Letter = Letter("l", ".-..");
    pub const M: Letter = Letter("m", "--");
    pub const N: Letter = Letter("n", "-.");
    pub const O: Letter = Letter("o", "---");
    pub const P: Letter = Letter("p", ".--.");
    pub const Q: Letter = Letter("q", "--.-");
    pub const R: Letter = Letter("r", ".-.");
    pub const S: Letter = Letter("s", "...");
    pub const T: Letter = Letter("t", "-");
    pub const U: Letter = Letter("u", "..-");
    pub const V: Letter = Letter("v", "...-");
    pub const W: Letter = Letter("w", ".--");
    pub const X: Letter = Letter("x", "-..-");
    pub const Y: Letter = Letter("y", "-.--");
    pub const Z: Letter = Letter("z", "--..");
    pub const ONE: Letter = Letter("1", ".----");
    pub const TWO: Letter = Letter("2", "..---");
    pub const THREE: Letter = Letter("3", "...--");
    pub const FOUR: Letter = Letter("4", "....-");
    pub const FIVE: Letter = Letter("5", ".....");
    pub const SIX: Letter = Letter("6", "-....");
    pub const SEVEN: Letter = Letter("7", "--...");
    pub const EIGHT: Letter = Letter("8", "---..");
    pub const NINE: Letter = Letter("9", "----.");
    pub const ZERO: Letter = Letter("0", "-----");
    pub const SPACE: Letter = Letter(" ", "/");
}

impl FromStr for Letter<'_> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "a" | ".-" => Ok(morse_alphabet::A),
            "b" | "-..." => Ok(morse_alphabet::B),
            "c" | "-.-." => Ok(morse_alphabet::C),
            "d" | "-.." => Ok(morse_alphabet::D),
            "e" | "." => Ok(morse_alphabet::E),
            "f" | "..-." => Ok(morse_alphabet::F),
            "g" | "--." => Ok(morse_alphabet::G),
            "h" | "...." => Ok(morse_alphabet::H),
            "i" | ".." => Ok(morse_alphabet::I),
            "j" | ".---" => Ok(morse_alphabet::J),
            "k" | "-.-" => Ok(morse_alphabet::K),
            "l" | ".-.." => Ok(morse_alphabet::L),
            "m" | "--" => Ok(morse_alphabet::M),
            "n" | "-." => Ok(morse_alphabet::N),
            "o" | "---" => Ok(morse_alphabet::O),
            "p" | ".--." => Ok(morse_alphabet::P),
            "q" | "--.-" => Ok(morse_alphabet::Q),
            "r" | ".-." => Ok(morse_alphabet::R),
            "s" | "..." => Ok(morse_alphabet::S),
            "t" | "-" => Ok(morse_alphabet::T),
            "u" | "..-" => Ok(morse_alphabet::U),
            "v" | "...-" => Ok(morse_alphabet::V),
            "w" | ".--" => Ok(morse_alphabet::W),
            "x" | "-..-" => Ok(morse_alphabet::X),
            "y" | "-.--" => Ok(morse_alphabet::Y),
            "z" | "--.." => Ok(morse_alphabet::Z),
            "1" | ".----" => Ok(morse_alphabet::ONE),
            "2" | "..---" => Ok(morse_alphabet::TWO),
            "3" | "...--" => Ok(morse_alphabet::THREE),
            "4" | "....-" => Ok(morse_alphabet::FOUR),
            "5" | "....." => Ok(morse_alphabet::FIVE),
            "6" | "-...." => Ok(morse_alphabet::SIX),
            "7" | "--..." => Ok(morse_alphabet::SEVEN),
            "8" | "---.." => Ok(morse_alphabet::EIGHT),
            "9" | "----." => Ok(morse_alphabet::NINE),
            "0" | "-----" => Ok(morse_alphabet::ZERO),
            " " | "/" => Ok(morse_alphabet::SPACE),
            _ => Err(format!("No representation found for the string: {}", s)),
        }
    }
}

impl PartialEq for Letter<'_> {
    fn eq(&self, other: &Self) -> bool {
        let Self(human1, morse1) = self;
        let Self(human2, morse2) = other;
        human1 == human2 && morse1 == morse2
    }
}
