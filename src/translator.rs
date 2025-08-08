use crate::parser::{MorseCommand, MorseTraductionType};
use crate::polyphonia::SAMPLE_RATE;
use crate::wav::wav_writer::{WavBuilder, WavOutBuffer};
use crate::Letter;
use std::error::Error;
use std::str::{self, FromStr};

pub trait MorseTranslator<T, W, R> {
    fn translate(&mut self, command: MorseCommand) -> Result<R, Box<dyn Error>>;

    fn translate_to_text(&mut self, command: MorseCommand) -> Result<R, Box<dyn Error>>;

    fn translate_to_audio(&mut self, command: MorseCommand) -> Result<R, Box<dyn Error>>;

    fn encode(raw_data: T) -> W;

    fn decode(raw_data: T) -> W;
}

pub struct StreamedMorseTranslator<T: WavOutBuffer> {
    // idea, create struct AudioMorseTranslation for audio implementation
    // create struct MorseTranslation with functions:
    // - in_file(&str)  -> using get_reader
    // - out_file(&str) -> using get_writer
    // - traduction_type(MorseTraductionType)
    // - traduction_options(MorseCommand)
    // this patter will create and use a StreamedMorseTranslator
    // or an AudioMorseTranslation trasparently
    input_stream: Vec<String>,
    pub output_stream: T,
    pub traduction_type: MorseTraductionType,
}

impl<T: WavOutBuffer> StreamedMorseTranslator<T> {
    pub fn builder() -> TranslatorBuilder<T> {
        TranslatorBuilder::default()
    }
}

impl<'l, T: WavOutBuffer> MorseTranslator<&str, Vec<Letter<'l>>, ()>
    for StreamedMorseTranslator<T>
{
    fn translate(&mut self, command: MorseCommand) -> Result<(), Box<dyn Error>> {
        match self.traduction_type {
            MorseTraductionType::Text => self.translate_to_text(command),
            MorseTraductionType::Audio => self.translate_to_audio(command),
        }
    }

    fn translate_to_audio(&mut self, command: MorseCommand) -> Result<(), Box<dyn Error>> {
        let read_cmd = match command {
            MorseCommand::Encode => Self::encode,
            MorseCommand::Decode => Self::decode,
        };

        let translated_lines = self.input_stream.iter().flat_map(|line| read_cmd(line));
        let wav = WavBuilder::new()
            .sample_rate(SAMPLE_RATE)
            .set_output(&mut self.output_stream);
        let mut wav = wav.init()?;
        wav.write_half_words(Letter::concat_audio(translated_lines))?;
        wav.close()?;
        Ok(())
    }

    fn translate_to_text(&mut self, command: MorseCommand) -> Result<(), Box<dyn Error>> {
        let read_cmd = match command {
            MorseCommand::Encode => Self::encode,
            MorseCommand::Decode => Self::decode,
        };

        let translated_lines = self.input_stream.iter().map(|line| read_cmd(line));

        match command {
            MorseCommand::Encode => {
                let mut buff = Vec::with_capacity(30);
                for line in translated_lines.map(Letter::concat_morse) {
                    unsafe {
                        buff.set_len(0);
                    }
                    buff.extend(line);
                    buff.extend(b"\n");
                    self.output_stream.write_all(&buff)?;
                }
            }
            MorseCommand::Decode => {
                let mut buff = Vec::with_capacity(30);
                for line in translated_lines.map(Letter::concat_text) {
                    unsafe {
                        buff.set_len(0);
                    }
                    buff.extend(line);
                    buff.extend(b"\n");
                    self.output_stream.write_all(&buff)?;
                }
            }
        };
        self.output_stream.flush()?;
        Ok(())
    }

    fn encode(line: &str) -> Vec<Letter<'l>> {
        line.bytes()
            .map(
                |byte| match Letter::from_str(str::from_utf8(&[byte]).unwrap()) {
                    Ok(letter) => letter,
                    Err(err) => panic!("Character not supported {:?}", err),
                },
            )
            .collect::<Vec<Letter<'_>>>()
    }

    fn decode(line: &str) -> Vec<Letter<'l>> {
        line.split_whitespace()
            .map(|morse_letter| match Letter::from_str(morse_letter) {
                Ok(letter) => letter,
                Err(err) => panic!("Character not supported {:?}", err),
            })
            .collect::<Vec<Letter<'_>>>()
    }
}

pub struct TranslatorBuilder<T: WavOutBuffer> {
    traduction_type: MorseTraductionType,
    input_stream: Option<Vec<String>>,
    output_stream: Option<T>,
}

impl<T: WavOutBuffer> TranslatorBuilder<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn input_stream(self, input_stream: Vec<String>) -> Self {
        Self {
            input_stream: Some(input_stream),
            ..self
        }
    }

    pub fn output_stream(self, out_stream: T) -> Self {
        Self {
            output_stream: Some(out_stream),
            ..self
        }
    }

    pub fn traduction_type(self, traduction_type: MorseTraductionType) -> Self {
        Self {
            traduction_type,
            ..self
        }
    }

    pub fn build_streamed(self) -> Result<StreamedMorseTranslator<T>, String> {
        Ok(StreamedMorseTranslator {
            input_stream: self
                .input_stream
                .as_ref()
                .expect("input_stream not set")
                .clone(),
            output_stream: self.output_stream.expect("output_stream not set"),
            traduction_type: self.traduction_type.clone(),
        })
    }
}

impl<T: WavOutBuffer> Default for TranslatorBuilder<T> {
    fn default() -> Self {
        TranslatorBuilder {
            input_stream: None,
            output_stream: None,
            traduction_type: MorseTraductionType::Text,
        }
    }
}
