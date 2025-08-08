use clap::Parser;
use morse_traducer::parser::MorseArgs;
use morse_traducer::translator::{MorseTranslator, TranslatorBuilder};
use morse_traducer::utils::{get_reader, get_writer};
use std::io::BufRead;

fn main() {
    let args = MorseArgs::parse();
    let input_stream: Vec<String> = get_reader(&args.in_file)
        .lines()
        .map_while(Result::ok)
        .collect();
    let output_stream = get_writer(&args.out_file).unwrap();

    let mut translator = TranslatorBuilder::new()
        .input_stream(input_stream)
        .output_stream(output_stream)
        .traduction_type(args.traduction_type)
        .build_streamed()
        .unwrap();
    translator.translate(args.morse_command).unwrap();
}

#[test]
fn test_main() {
    use morse_traducer::parser::MorseCommand;
    use morse_traducer::translator::TranslatorBuilder;
    use std::io::{Cursor, Seek, SeekFrom};
    use std::str::from_utf8;

    let output: Cursor<Vec<u8>> = Default::default();
    let input = vec!["Hello World".into()];
    let mut translator = TranslatorBuilder::new()
        .input_stream(input)
        .output_stream(output)
        .build_streamed()
        .unwrap();
    translator.translate(MorseCommand::Encode).unwrap();
    //launch with cargo test -- --nocapture
    translator.output_stream.seek(SeekFrom::Start(0)).unwrap();
    let result = from_utf8(translator.output_stream.get_ref());
    print!("{:?}", result);
}
