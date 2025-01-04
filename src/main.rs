#![allow(unused_imports)]
use std::{
    collections::HashMap,
    env, fmt,
    fs::{self, File, OpenOptions},
    io::prelude::*,
    path::{Path, PathBuf},
};

const TABLE: [(&'static str, &'static str); 58] = [
    ("alpha", "α"),
    ("beta", "β"),
    ("gamma", "γ"),
    ("Gamma", "Γ"),
    ("delta", "δ"),
    ("Delta", "Δ"),
    ("epsilon", "ε"),
    ("zeta", "ζ"),
    ("eta", "η"),
    ("theta", "θ"),
    ("Theta", "Θ"),
    ("iota", "ι"),
    ("kappa", "κ"),
    ("lambda", "λ"),
    ("Lambda", "Λ"),
    ("mu", "μ"),
    ("nu", "ν"),
    ("xi", "ξ"),
    ("Xi", "Ξ"),
    ("omicron", "ο"),
    ("pi", "π"),
    ("Pi", "Π"),
    ("rho", "ρ"),
    ("sigma", "σ"),
    ("Sigma", "Σ"),
    ("tau", "τ"),
    ("upsilon", "υ"),
    ("Upsilon", "Υ"),
    ("phi", "φ"),
    ("Phi", "Φ"),
    ("chi", "χ"),
    ("psi", "ψ"),
    ("Psi", "Ψ"),
    ("omega", "ω"),
    ("Omega", "Ω"),
    ("^0", "⁰"),
    ("^1", "¹"),
    ("^2", "²"),
    ("^3", "³"),
    ("^4", "⁴"),
    ("^5", "⁵"),
    ("^6", "⁶"),
    ("^7", "⁷"),
    ("^8", "⁸"),
    ("^9", "⁹"),
    ("^x", "ˣ"),
    ("^y", "ʸ"),
    ("_0", "₀"),
    ("_1", "₁"),
    ("_2", "₂"),
    ("_3", "₃"),
    ("_4", "₄"),
    ("_5", "₅"),
    ("_6", "₆"),
    ("_7", "₇"),
    ("_8", "₈"),
    ("_9", "₉"),
    ("_x", "ₓ"),
    // parens
    // arrows
    // unary operators
    // binary operators
];

fn main() {
    let table: HashMap<String, String> = TABLE
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>();
    let Some(filename) = env::args().nth(1) else {
        return;
    };
    let Ok(contents) = fs::read_to_string(&filename) else {
        panic!();
    };
    let unecaped = contents.split("\\\\").collect::<Vec<_>>();
    let gathered = unecaped
        .iter()
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                s.to_string()
            } else {
                format!("\\\\:star::{s}")
            }
        })
        .map(|s| unlatex(&table, s))
        .collect::<String>();
    dump_to(&filename, gathered);
}

fn dump_to<S: AsRef<str>>(path: &str, contents: S) {
    let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open(path) else {
        panic!();
    };
    let Ok(()) = file.write_all(contents.as_ref().as_bytes()) else {
        panic!();
    };
}

fn unlatex(table: &HashMap<String, String>, s: String) -> String {
    s.split('\\')
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                s.to_string()
            } else {
                latex_to_unicode(table, s)
            }
        })
        .collect::<String>()
}

fn latex_to_unicode(table: &HashMap<String, String>, s: &str) -> String {
    if let Some(i) = s
        .chars()
        .position(|c| [' ', '\t', '(', '{', '[', '\n'].contains(&c))
    {
        let keyword = &s[0..i];
        if let Some(get) = table.get(keyword) {
            return format!("{}{}", get, &s[i..]);
        }
    }
    s.to_string()
}
