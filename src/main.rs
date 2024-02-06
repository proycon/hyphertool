use clap::Parser;
use hypher::{hyphenate, Lang};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Language (ISO 639-1 language code; two letters)
    #[arg(short, long)]
    language: String,

    #[arg(short, long, default_value = "-")]
    delimiter: String,

    /// Plain text files (UTF-8 Encoded) to use as input
    input: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let language = match args.language.as_str() {
        "af" => Lang::Afrikaans,
        "be" => Lang::Belarusian,
        "bg" => Lang::Bulgarian,
        "cs" => Lang::Czech,
        "da" => Lang::Danish,
        "de" => Lang::German,
        "en" => Lang::English,
        "es" => Lang::Spanish,
        "et" => Lang::Estonian,
        "fi" => Lang::Finnish,
        "fr" => Lang::French,
        "hu" => Lang::Hungarian,
        "is" => Lang::Icelandic,
        "ka" => Lang::Georgian,
        "ku" => Lang::Kurmanji,
        "la" => Lang::Latin,
        "lt" => Lang::Lithuanian,
        "mn" => Lang::Mongolian,
        "nl" => Lang::Dutch,
        "no" => Lang::Norwegian,
        "pl" => Lang::Polish,
        "pt" => Lang::Portuguese,
        "ru" => Lang::Russian,
        "sk" => Lang::Slovak,
        "sl" => Lang::Slovenian,
        "sq" => Lang::Albanian,
        "sr" => Lang::Serbian,
        "sv" => Lang::Swedish,
        "tk" => Lang::Turkmen,
        "tr" => Lang::Turkish,
        "uk" => Lang::Ukrainian,
        _ => {
            eprintln!(
                "Invalid or unsupported language code (must be iso-639-1, two letters, lower case)"
            );
            std::process::exit(1);
        }
    };

    for filename in args.input.iter() {
        let text = std::fs::read_to_string(filename)
            .map_err(|err| {
                eprintln!("Error reading file {}: {}", filename, err);
                std::process::exit(1);
            })
            .unwrap();
        let mut beginbyte = 0;
        let mut bytepos = 0;
        let mut alphabetic_mode = true;
        for c in text.chars() {
            if !c.is_alphabetic() && alphabetic_mode {
                if beginbyte < bytepos {
                    let word = &text[beginbyte..bytepos];
                    let syllables = hyphenate(word, language);
                    print!("{}", syllables.join("-"));
                }
                alphabetic_mode = false;
                beginbyte = bytepos;
            } else if c.is_alphabetic() && !alphabetic_mode {
                alphabetic_mode = true;
                if beginbyte < bytepos {
                    print!("{}", &text[beginbyte..bytepos]);
                }
                beginbyte = bytepos;
            }
            bytepos += c.len_utf8();
        }
        if beginbyte < bytepos {
            let word = &text[beginbyte..];
            if alphabetic_mode {
                let syllables = hyphenate(word, language);
                print!("{}", syllables.join(args.delimiter.as_str()));
            } else {
                print!("{}", word);
            }
        }
    }
}
