#![feature(alloc_system)]
extern crate alloc_system;

extern crate rucdn;
use std::u32;

fn parse_with_base_detection(inp: &str) -> u32 {
    if inp.starts_with("0x") {
        // hex
        u32::from_str_radix(&inp[2..], 16).expect("invalid number")
    } else {
        // assume decimal
        inp.parse().expect("invalid number")
    }
}

fn main() {
    let mut args = std::env::args();
    let code: u32;
    
    match args.nth(1) {
        Some(v) => code = parse_with_base_detection(&v),
        None => {
            println!("usage: {:?} [codepoint]", std::env::current_exe().unwrap());
            return;
        }
    }

    println!("Unicode version: {}", rucdn::get_unicode_version());

    println!("Analyzing character U+{:04X}", code);
    println!();

    println!("Combining class: {:?}", rucdn::get_combining_class(code));
    println!("General category: {:?}", rucdn::get_general_category(code));
    match rucdn::mirror(code) {
        Ok(v) => println!("Mirrored: U+{:04X}", v),
        Err(_) => {}
    }
    println!("Script: {:?}", rucdn::get_script(code));
    println!("Paired bracket type: {:?}", rucdn::paired_bracket_type(code));
    println!("East-Asian width: {:?}", rucdn::get_east_asian_width(code));
    println!("Linebreak class: {:?}", rucdn::get_linebreak_class(code));
    println!("BiDi class: {:?}", rucdn::get_bidi_class(code));
    
    match rucdn::compat_decompose(code) {
        Ok((len, data)) => {
            print!("Compatibility decomposition: ");
            for i in 0..len {
                print!("U+{:04X} ", data[i]);
            }
            println!();
        }
        Err(_) => {}
    }

    match rucdn::decompose(code) {
        Ok((a, b)) => {
            println!("Decomposition: U+{:04X} U+{:04X}", a, b);
            match rucdn::compose(a, b) {
                Ok(v) => println!("Recomposition: U+{:04X}", v),
                Err(_) => {}
            }
        },
        Err(_) => {}
    }
}
