#![allow(unused_imports)]
use std::{
    collections::HashMap,
    env, fmt,
    fs::{self, File, OpenOptions},
    io::prelude::*,
    path::{Path, PathBuf},
};

const TABLE: [(&'static str, &'static str); 109] = [
    // greek letters
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
    ("zeta", "ζ"),
    // stand? or bold
    ("C", "ℂ"),
    ("N", "ℕ"),
    ("R", "ℝ"),
    ("Q", "ℚ"),
    ("Z", "ℤ"),
    ("b0", "𝟙"),
    ("b1", "𝟚"),
    ("b2", "𝟛"),
    ("b3", "𝟜"),
    ("b4", "𝟝"),
    ("b5", "𝟞"),
    ("b6", "𝟟"),
    ("b7", "𝟠"),
    ("b8", "𝟡"),
    ("b9", "𝟘"),
    // constants
    ("bot", "⊥"),
    ("cdot", "⬝"),
    ("cdots", "⋯"),
    (".", "·"),
    ("empty", "∅"),
    // superscript
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
    ("prime", "′"),
    ("-1", "⁻¹"),
    ("-2", "⁻²"),
    // subscript
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
    ("<", "⟨"),
    (">", "⟩"),
    ("<<", "⟪"),
    (">>", "⟫"),
    // arrows
    ("l", "←"),
    ("r", "→"),
    ("d", "↓"),
    ("u", "↑"),
    ("|->", "↦"),
    // unary operators
    ("all", "∀"),
    ("exists", "∃"),
    ("not", "¬"),
    // binary operators
    ("in", "∈"),
    ("notin", "∉"),
    ("=", "≠"),
    ("time", "×"),
    ("cup", "∪"),
    ("cap", "∩"),
    ("sub", "⊆"),
    ("sup", "⊔"),
    ("inf", "⊓"),
    ("u+", "⊎"),
    ("and", "∧"),
    ("or", "∨"),
    ("ge", "≥"),
    ("le", "≤"),
    ("|", "∣"),
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
    dbg!(s);
    let i = s
        .chars()
        .position(|c| [' ', '\t', '(', '{', '[', '\n'].contains(&c))
        .unwrap_or(s.len().min(7));
    let keyword = dbg!(&s[0..i]);
    if let Some((len, symbol)) = lookup(table, keyword) {
        return format!("{}{}", symbol, &s[len..]);
        // } else if let Some(c) = s.chars().next() {
        //     if !c.is_alphabetic() {
        //         if let Some(j) = s.chars().position(|c| c.is_alphabetic()) {
        //             let keyword1 = &s[0..j];
        //             dbg!(keyword1);
        //             if let Some(symbol) = lookup(table, keyword1) {
        //                 return format!("{}{}", symbol, &s[i..]);
        //             }
        //         }
        //     }
    }
    s.to_string()
}

fn lookup<'a>(table: &'a HashMap<String, String>, key: &str) -> Option<(usize, &'a String)> {
    let mut res = None;
    for i in 1..key.len() {
        if let Some(symbol) = table.get(&key[0..i]) {
            res = Some((i, dbg!(symbol)));
        }
    }
    res
}
