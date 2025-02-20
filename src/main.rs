use clap::Parser;
use hypher::Lang;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Language (ISO 639-1 language code; two letters)
    #[arg(short, long)]
    language: String,

    /// Delimiter to use to separate syllables or in hyphenation, if set without --width, all syllables will be marked with this separator
    #[arg(short, long)]
    delimiter: Option<String>,

    /// Width to hyphenate the text on (if set), the width should be in unicode character points, not bytes nor taking double-spaced characters into account)
    #[arg(short, long)]
    width: Option<u8>,

    /// Output syllable information as stand-off annotations (offsets are unicode points, 0-indexed, end non-inclusive).
    #[arg(short, long)]
    standoff: bool,

    /// Characters that are considered 'hyphen' and will be dehyphenatated at the end of a line. Multiple characters may be specified, they will be considered individually. Note that dehyhenation currently does not use the -l parameter, even though it must be set.
    #[arg(short = 'D', long)]
    dehyphenation: Option<String>,

    /// Characters that are considered 'hyphen' and will be dehyphenatated at the beginning of a line. Multiple characters may be specified, they will be considered individually.
    #[arg(short = 'P', long)]
    prefix_dehyphenation: Option<String>,

    /// Plain text files (UTF-8 Encoded) to use as input
    input: Vec<String>,
}

fn syllabify(text: &str, language: Lang, delimiter: &str) {
    let mut beginbyte = 0;
    let mut bytepos = 0;
    let mut alphabetic_mode = true;
    for c in text.chars() {
        if !c.is_alphabetic() && alphabetic_mode {
            if beginbyte < bytepos {
                let word = &text[beginbyte..bytepos];
                let syllables = hypher::hyphenate(word, language);
                print!("{}", syllables.join(delimiter));
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
            let syllables = hypher::hyphenate(word, language);
            print!("{}", syllables.join(delimiter));
        } else {
            print!("{}", word);
        }
    }
}

fn standoff(text: &str, language: Lang) {
    println!("Text\tBeginOffset\tEndOffset");
    let mut beginbyte = 0;
    let mut bytepos = 0;
    let mut alphabetic_mode = true;
    let mut begincharpos = 0;
    for (i, c) in text.chars().enumerate() {
        if !c.is_alphabetic() && alphabetic_mode {
            if beginbyte < bytepos {
                let word = &text[beginbyte..bytepos];
                for syllable in hypher::hyphenate(word, language) {
                    println!(
                        "{}\t{}\t{}",
                        syllable,
                        begincharpos,
                        begincharpos + syllable.len()
                    );
                    begincharpos += syllable.len();
                }
            }
            alphabetic_mode = false;
            beginbyte = bytepos;
            begincharpos = i;
        } else if c.is_alphabetic() && !alphabetic_mode {
            alphabetic_mode = true;
            beginbyte = bytepos;
            begincharpos = i;
        }
        bytepos += c.len_utf8();
    }
    if beginbyte < bytepos {
        let word = &text[beginbyte..];
        if alphabetic_mode {
            for syllable in hypher::hyphenate(word, language) {
                println!(
                    "{}\t{}\t{}",
                    syllable,
                    begincharpos,
                    begincharpos + syllable.len()
                );
                begincharpos += syllable.len();
            }
        }
    }
}

fn hyphenate(text: &str, language: Lang, width: u8, delimiter: &str) {
    let mut beginbyte = 0;
    let mut bytepos = 0;
    let mut alphabetic_mode = true;
    let mut charpos: usize = 0;
    for c in text.chars() {
        if !c.is_alphabetic() && alphabetic_mode {
            if beginbyte < bytepos {
                let word = &text[beginbyte..bytepos];
                for (i, syllable) in hypher::hyphenate(word, language).enumerate() {
                    if charpos >= width as usize {
                        if i == 0 {
                            println!();
                        } else {
                            println!("{}", delimiter);
                        }
                        charpos = 0;
                    }
                    print!("{}", syllable);
                    charpos += syllable.len();
                }
            }
            alphabetic_mode = false;
            beginbyte = bytepos;
        } else if c.is_alphabetic() && !alphabetic_mode {
            alphabetic_mode = true;
            if beginbyte < bytepos {
                let subtext = &text[beginbyte..bytepos];
                if charpos + subtext.len() > width as usize {
                    println!();
                    charpos = 0;
                    print!("{}", subtext.trim_start());
                } else {
                    print!("{}", subtext);
                }
                charpos += subtext.len();
            }
            beginbyte = bytepos;
        }
        bytepos += c.len_utf8();
    }
    if beginbyte < bytepos {
        let word = &text[beginbyte..];
        for (i, syllable) in hypher::hyphenate(word, language).enumerate() {
            if charpos >= width as usize {
                if i == 0 {
                    println!();
                } else {
                    println!("{}", delimiter);
                }
                charpos = 0;
            }
            print!("{}", syllable);
            charpos += syllable.len();
        }
    }
}

fn dehyphenate(text: &str, hyphens: &str, prefix_hyphens: Option<&str>) {
    let mut prevtoken: Option<&str> = None;
    let mut dehyphenate = false;
    let mut stdout = io::stdout().lock();
    let hyphens: Vec<char> = hyphens.chars().collect();
    let prefix_hyphens: Vec<char> = if let Some(prefix_hyphens) = prefix_hyphens {
        prefix_hyphens.chars().collect()
    } else {
        Vec::new()
    };
    for mut token in text.split_inclusive(|c: char| c.is_whitespace()) {
        if prevtoken.is_some() {
            if dehyphenate {
                token = token.trim_start_matches(&prefix_hyphens[..]);
            }
            stdout
                .write(prevtoken.unwrap().as_bytes())
                .expect("failure writing to stdout");
        }
        if token.ends_with('\n') && token.trim_end_matches('\n').ends_with(&hyphens[..]) {
            prevtoken = Some(token.trim_end_matches('\n').trim_end_matches(&hyphens[..]));
            dehyphenate = true;
        } else {
            prevtoken = Some(token);
            dehyphenate = false;
        }
    }
    if let Some(prevtoken) = prevtoken {
        stdout
            .write(prevtoken.as_bytes())
            .expect("failure writing to stdout");
    }
}

fn main() {
    let mut args = Args::parse();

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

    if args.delimiter.is_none() && args.width.is_none() && !args.standoff {
        args.delimiter = Some("-".to_string());
    }

    for filename in args.input.iter() {
        let text = std::fs::read_to_string(filename)
            .map_err(|err| {
                eprintln!("Error reading file {}: {}", filename, err);
                std::process::exit(1);
            })
            .unwrap();
        if args.standoff {
            standoff(text.as_str(), language);
        } else if let Some(width) = args.width.as_ref() {
            hyphenate(
                text.as_str(),
                language,
                *width,
                args.delimiter.as_ref().map(|x| x.as_str()).unwrap_or("-"),
            );
        } else if let Some(dehyphens) = args.dehyphenation.as_ref() {
            dehyphenate(
                text.as_str(),
                &dehyphens,
                args.prefix_dehyphenation.as_ref().map(|x| x.as_str()),
            )
        } else if let Some(delimiter) = args.delimiter.as_ref() {
            syllabify(text.as_str(), language, delimiter);
        }
    }
}
