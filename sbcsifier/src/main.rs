enum Diacritic {
    None,
    Dakuten,
    Handakuten
}

enum OutputType {
    Binary,
    Decimal,
    Hexadecimal
}

fn map(c: char) -> (u8, Diacritic) {
    use Diacritic::*;
    let id = c as u8;
    let code_point = c as u32;
    match c as u32 {
        // ASCII (except the yen sign)
        0..=0x5B => (id, None),
        0x5D..=0x7F => (id, None),
        // And now the yen sign
        0xA5 => (0x5C, None),
        // Fullwidth characters (except the yen sign)
        0xFF00..=0xFF3B => ((code_point - 0xFEE0) as u8, None),
        0xFF3D..=0xFF5E => ((code_point - 0xFEE0) as u8, None),
        // And now the yen sign, again
        0xFFE5 => (0x5C, None),
        // And the ideographic space
        0x3000 => (0x20, None),
        // Everything else
        _ => match c {
            // Hiragana
            'を' => (0x86, None),
            'ぁ' => (0x87, None),
            'ぃ' => (0x88, None),
            'ぅ' => (0x89, None),
            'ぇ' => (0x8A, None),
            'ぉ' => (0x8B, None),
            'ゃ' => (0x8C, None),
            'ゅ' => (0x8D, None),
            'ょ' => (0x8E, None),
            'っ' => (0x8F, None),
            'あ' => (0x91, None),
            'い' => (0x92, None),
            'う' => (0x93, None),
            'え' => (0x94, None),
            'お' => (0x95, None),
            'か' => (0x96, None),
            'き' => (0x97, None),
            'く' => (0x98, None),
            'け' => (0x99, None),
            'こ' => (0x9A, None),
            'が' => (0x96, Dakuten),
            'ぎ' => (0x97, Dakuten),
            'ぐ' => (0x98, Dakuten),
            'げ' => (0x99, Dakuten),
            'ご' => (0x9A, Dakuten),
            'さ' => (0x9B, None),
            'し' => (0x9C, None),
            'す' => (0x9D, None),
            'せ' => (0x9E, None),
            'そ' => (0x9F, None),
            'ざ' => (0x9B, Dakuten),
            'じ' => (0x9C, Dakuten),
            'ず' => (0x9D, Dakuten),
            'ぜ' => (0x9E, Dakuten),
            'ぞ' => (0x9F, Dakuten),
            'た' => (0xE0, None),
            'ち' => (0xE1, None),
            'つ' => (0xE2, None),
            'て' => (0xE3, None),
            'と' => (0xE4, None),
            'だ' => (0xE0, Dakuten),
            'ぢ' => (0xE1, Dakuten),
            'づ' => (0xE2, Dakuten),
            'で' => (0xE3, Dakuten),
            'ど' => (0xE4, Dakuten),
            'な' => (0xE5, None),
            'に' => (0xE6, None),
            'ぬ' => (0xE7, None),
            'ね' => (0xE8, None),
            'の' => (0xE9, None),
            'は' => (0xEA, None),
            'ひ' => (0xEB, None),
            'ふ' => (0xEC, None),
            'へ' => (0xED, None),
            'ほ' => (0xEE, None),
            'ば' => (0xEA, Dakuten),
            'び' => (0xEB, Dakuten),
            'ぶ' => (0xEC, Dakuten),
            'べ' => (0xED, Dakuten),
            'ぼ' => (0xEE, Dakuten),
            'ぱ' => (0xEA, Handakuten),
            'ぴ' => (0xEB, Handakuten),
            'ぷ' => (0xEC, Handakuten),
            'ぺ' => (0xED, Handakuten),
            'ぽ' => (0xEE, Handakuten),
            'ま' => (0xEF, None),
            'み' => (0xF0, None),
            'む' => (0xF1, None),
            'め' => (0xF2, None),
            'も' => (0xF3, None),
            'や' => (0xF4, None),
            'ゆ' => (0xF5, None),
            'よ' => (0xF6, None),
            'ら' => (0xF7, None),
            'り' => (0xF8, None),
            'る' => (0xF9, None),
            'れ' => (0xFA, None),
            'ろ' => (0xFB, None),
            'わ' => (0xFC, None),
            'ん' => (0xFD, None),
            // Katakana
            'ヲ' => (0xA6, None),
            'ァ' => (0xA7, None),
            'ィ' => (0xA8, None),
            'ゥ' => (0xA9, None),
            'ェ' => (0xAA, None),
            'ォ' => (0xAB, None),
            'ャ' => (0xAC, None),
            'ュ' => (0xAD, None),
            'ョ' => (0xAE, None),
            'ッ' => (0xAF, None),
            'ー' => (0xB0, None),
            'ア' => (0xB1, None),
            'イ' => (0xB2, None),
            'ウ' => (0xB3, None),
            'エ' => (0xB4, None),
            'オ' => (0xB5, None),
            'カ' => (0xB6, None),
            'キ' => (0xB7, None),
            'ク' => (0xB8, None),
            'ケ' => (0xB9, None),
            'コ' => (0xBA, None),
            'ガ' => (0xB6, Dakuten),
            'ギ' => (0xB7, Dakuten),
            'グ' => (0xB8, Dakuten),
            'ゲ' => (0xB9, Dakuten),
            'ゴ' => (0xBA, Dakuten),
            'サ' => (0xBB, None),
            'シ' => (0xBC, None),
            'ス' => (0xBD, None),
            'セ' => (0xBE, None),
            'ソ' => (0xBF, None),
            'ザ' => (0xBB, Dakuten),
            'ジ' => (0xBC, Dakuten),
            'ズ' => (0xBD, Dakuten),
            'ゼ' => (0xBE, Dakuten),
            'ゾ' => (0xBF, Dakuten),
            'タ' => (0xC0, None),
            'チ' => (0xC1, None),
            'ツ' => (0xC2, None),
            'テ' => (0xC3, None),
            'ト' => (0xC4, None),
            'ダ' => (0xC0, Dakuten),
            'ヂ' => (0xC1, Dakuten),
            'ヅ' => (0xC2, Dakuten),
            'デ' => (0xC3, Dakuten),
            'ド' => (0xC4, Dakuten),
            'ナ' => (0xC5, None),
            'ニ' => (0xC6, None),
            'ヌ' => (0xC7, None),
            'ネ' => (0xC8, None),
            'ノ' => (0xC9, None),
            'ハ' => (0xCA, None),
            'ヒ' => (0xCB, None),
            'フ' => (0xCC, None),
            'ヘ' => (0xCD, None),
            'ホ' => (0xCE, None),
            'バ' => (0xCA, Dakuten),
            'ビ' => (0xCB, Dakuten),
            'ブ' => (0xCC, Dakuten),
            'ベ' => (0xCD, Dakuten),
            'ボ' => (0xCE, Dakuten),
            'パ' => (0xCA, Handakuten),
            'ピ' => (0xCB, Handakuten),
            'プ' => (0xCC, Handakuten),
            'ペ' => (0xCD, Handakuten),
            'ポ' => (0xCE, Handakuten),
            'マ' => (0xCF, None),
            'ミ' => (0xD0, None),
            'ム' => (0xD1, None),
            'メ' => (0xD2, None),
            'モ' => (0xD3, None),
            'ヤ' => (0xD4, None),
            'ユ' => (0xD5, None),
            'ヨ' => (0xD6, None),
            'ラ' => (0xD7, None),
            'リ' => (0xD8, None),
            'ル' => (0xD9, None),
            'レ' => (0xDA, None),
            'ロ' => (0xDB, None),
            'ワ' => (0xDC, None),
            'ン' => (0xDD, None),
            // Other
            '゛' => (0xDE, None),
            '゜' => (0xDF, None),
            '。' => (0xA1, None),
            '「' => (0xA2, None),
            '」' => (0xA3, None),
            '、' => (0xA4, None),
            '・' => (0xA5, None),
            _ => {
                use std::process::exit;
                eprintln!("Unknown character: {}", c);
                exit(5)
            }
        }
    }
}

fn map_str(s: &str) -> Vec<u8> {
    // The conversion always reduces the number of bytes,
    // so this is a good way to guess at the needed capacity.
    let mut buffer = Vec::<u8>::with_capacity(s.len() * 2);
    for c in s.chars() {
        let (base, diacritic) = map(c);
        buffer.push(base);
        match diacritic {
            Diacritic::None => (),
            Diacritic::Dakuten => buffer.push(0xDE),
            Diacritic::Handakuten => buffer.push(0xDF)
        }
    }
    buffer
}

fn main() {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::process::exit;
    use std::ffi::{OsStr, OsString};
    let args: Vec<OsString> = std::env::args_os().collect();
    if args.len() != 4 {
        eprintln!("Syntax: sbcsifier [command] [input file] [output file]");
        exit(1);
    }
    let command = &args[1];
    let output_type = if command == OsStr::new("bin") {
        OutputType::Binary
    } else if command == OsStr::new("dec") {
        OutputType::Decimal
    } else if command == OsStr::new("hex") {
        OutputType::Hexadecimal
    } else {
        eprintln!("Unknown command: {}", command.to_string_lossy());
        exit(2)
    };
    let input_filename = &args[2];
    let sbcs_text = if let Ok(mut file) = File::open(input_filename) {
        let mut text = String::new();
        if let Ok(_) = file.read_to_string(&mut text) {
            map_str(&text)
        } else {
            eprintln!("The text in the input file was not valid UTF-8.");
            exit(4);
        }
    } else {
        eprintln!("There was an error opening the input file.");
        exit(3);
    };
    let output_filename = &args[3];
    match output_type {
        OutputType::Binary => {
            if let Ok(mut file) = File::create(output_filename) {
                if let Err(_) = file.write_all(&sbcs_text) {
                    eprintln!("There was an error writing to the output file.");
                    exit(7)
                }
            } else {
                eprintln!("There was an error opening the output file.");
                exit(6)
            }
        },
        _ => ()
    }
}

