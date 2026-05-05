#![allow(unused_imports)]
use clap::Parser;
use std::{
    collections::HashMap,
    fmt,
    fs::{self, File, OpenOptions, exists},
    io::{self, prelude::*},
    path::{Path, PathBuf},
};

const TABLE: [(&str, &str); 238] = [
    // italic alphabet
    ("MiA", "𝐴"),
    ("MiB", "𝐵"),
    ("MiC", "𝐶"),
    ("MiD", "𝐷"),
    ("MiE", "𝐸"),
    ("MiF", "𝐹"),
    ("MiG", "𝐺"),
    ("MiH", "𝐻"),
    ("MiI", "𝐼"),
    ("MiJ", "𝐽"),
    ("MiK", "𝐾"),
    ("MiL", "𝐿"),
    ("MiM", "𝑀"),
    ("MiN", "𝑁"),
    ("MiO", "𝑂"),
    ("MiP", "𝑃"),
    ("MiQ", "𝑄"),
    ("MiR", "𝑅"),
    ("MiS", "𝑆"),
    ("MiT", "𝑇"),
    ("MiU", "𝑈"),
    ("MiV", "𝑉"),
    ("MiW", "𝑊"),
    ("MiX", "𝑋"),
    ("MiY", "𝑌"),
    ("MiZ", "𝑍"),
    ("Mia", "𝑎"),
    ("Mib", "𝑏"),
    ("Mic", "𝑐"),
    ("Mid", "𝑑"),
    ("Mie", "𝑒"),
    ("Mif", "𝑓"),
    ("Mig", "𝑔"),
    ("Mih", "ℎ"),
    ("Mii", "𝑖"),
    ("Mij", "𝑗"),
    ("Mik", "𝑘"),
    ("Mil", "𝑙"),
    ("Mim", "𝑚"),
    ("Min", "𝑛"),
    ("Mio", "𝑜"),
    ("Mip", "𝑝"),
    ("Miq", "𝑞"),
    ("Mir", "𝑟"),
    ("Mis", "𝑠"),
    ("Mit", "𝑡"),
    ("Miu", "𝑢"),
    ("Miv", "𝑣"),
    ("Miw", "𝑤"),
    ("Mix", "𝑥"),
    ("Miy", "𝑦"),
    ("Miz", "𝑧"),
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
    ("nbhds", "𝓝 "),
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
    ("infty", "∞"),
    ("top", "⊤"),
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
    // ˣ ʸ
    ("^A", "ᴬ"),
    ("^B", "ᴮ"),
    ("^D", "ᴰ"),
    ("^E", "ᴱ"),
    ("^G", "ᴳ"),
    ("^H", "ᴴ"),
    ("^I", "ᴵ"),
    ("^J", "ᴶ"),
    ("^K", "ᴷ"),
    ("^L", "ᴸ"),
    ("^M", "ᴹ"),
    ("^N", "ᴺ"),
    ("^O", "ᴼ"),
    ("^P", "ᴾ"),
    ("^R", "ᴿ"),
    ("^T", "ᵀ"),
    ("^U", "ᵁ"),
    ("^V", "ⱽ"),
    ("^W", "ᵂ"),
    ("^a", "ᵃ"),
    ("^b", "ᵇ"),
    ("^c", "ᶜ"),
    ("^d", "ᵈ"),
    ("^e", "ᵉ"),
    ("^f", "ᶠ"),
    ("^g", "ᵍ"),
    ("^h", "ʰ"),
    ("^i", "ⁱ"),
    ("^j", "ʲ"),
    ("^k", "ᵏ"),
    ("^l", "ˡ"),
    ("^m", "ᵐ"),
    ("^n", "ⁿ"),
    ("^o", "ᵒ"),
    ("^p", "ᵖ"),
    ("^r", "ʳ"),
    ("^s", "ˢ"),
    ("^t", "ᵗ"),
    ("^u", "ᵘ"),
    ("^v", "ᵛ"),
    ("^w", "ʷ"),
    ("^x", "ˣ"),
    ("^y", "ʸ"),
    ("^z", "ᶻ"),
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
    ("_a", "ₐ"),
    ("_e", "ₑ"),
    ("_h", "ₕ"),
    ("_i", "ᵢ"),
    ("_j", "ⱼ"),
    ("_k", "ₖ"),
    ("_l", "ₗ"),
    ("_m", "ₘ"),
    ("_n", "ₙ"),
    ("_o", "ₒ"),
    ("_p", "ₚ"),
    ("_r", "ᵣ"),
    ("_s", "ₛ"),
    ("_t", "ₜ"),
    ("_u", "ᵤ"),
    ("_v", "ᵥ"),
    ("_x", "ₓ"),
    // parens
    ("<", "⟨"),
    (">", "⟩"),
    ("<<", "⟪"),
    (">>", "⟫"),
    ("f<<", "«"),
    ("f>>", "»"),
    // arrows
    ("l", "←"),
    ("r", "→"),
    ("d", "↓"),
    ("u", "↑"),
    ("|->", "↦"),
    // unary operators
    ("all", "∀"),
    ("allf", "∀ᶠ"),
    ("exists", "∃"),
    ("not", "¬"),
    ("sum", "∑"),
    ("norm", "‖‖"),
    // binary operators
    ("and", "∧"),
    ("cap", "∩"),
    ("comp", "∘"),
    ("cup", "∪"),
    ("dia", "⋄"),
    ("equiv", "≃"),
    ("ge", "≥"),
    ("in", "∈"),
    ("inf", "⊓"),
    ("le", "≤"),
    ("notin", "∉"),
    ("or", "∨"),
    ("pm", "±"),
    ("prec", "≺"),
    ("preceq", "≼"),
    ("quot", "⧸"),
    ("smul", "•"),
    ("subseq", "⊆"),
    ("sup", "⊔"),
    ("supset", "⊇"),
    ("t", "▸"),
    ("time", "×"),
    ("u+", "⊎"),
    ("=", "≠"),
    ("|", "∣"),
    ("cong", "≅"),
    ("div", "÷"),
];

/// Convert LaTeX math commands to Unicode symbols
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Display the conversion table
    #[arg(long, conflicts_with = "file")]
    dump: bool,

    /// Output filename (requires write_back feature to be enabled)
    #[arg(value_name = "FILE", conflicts_with = "dump")]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // If --dump flag is provided, display the conversion table
    if cli.dump {
        println!("LaTeX to Unicode Conversion Table:");
        println!("{:-<60}", "");
        for (latex, unicode) in &TABLE {
            println!("{:<20} -> {}", latex, unicode);
        }
        return;
    }

    let table: HashMap<String, String> = TABLE
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect::<HashMap<_, _>>();
    // dbg!(&out_filename);
    let mut contents = String::new();
    io::stdin()
        .read_to_string(&mut contents)
        .expect("failed to read");
    let unecaped = contents.split("\\\\").collect::<Vec<_>>();
    let gathered = unecaped
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let cnv = unlatex(&table, s);
            if i == 0 {
                cnv.to_string()
            } else {
                format!("\\\\{cnv}")
            }
        })
        // .map(|s| unlatex(&table, s))
        .collect::<String>();
    print!("{gathered}");
    #[cfg(feature = "write_back")]
    if let Some(out_filename) = cli.file {
        dump_to(&out_filename, gathered);
    };
}

#[allow(dead_code)]
fn dump_to<S: AsRef<str>>(path: &str, contents: S) {
    let mut file = if Path::new(path).exists() {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap_or_else(|_| panic!("fail to open {path}"))
    } else {
        File::create(path).unwrap_or_else(|_| panic!("fail to open {path}"))
    };
    let Ok(()) = file.write_all(contents.as_ref().as_bytes()) else {
        panic!();
    };
}

fn unlatex(table: &HashMap<String, String>, s: &str) -> String {
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
    let i = s
        .chars()
        .position(|c| [' ', '\t', '(', '{', '[', '\n'].contains(&c))
        .unwrap_or(s.len().min(7));
    let keyword = &s[0..i];
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
    format!("\\{s}")
}

fn lookup<'a>(table: &'a HashMap<String, String>, key: &str) -> Option<(usize, &'a String)> {
    let mut s = key.to_string();
    let mut was_alpha = false;
    while !s.is_empty() {
        if let Some(symbol) = table.get(&s) {
            return Some((s.len(), symbol));
        }
        if let Some(c) = s.pop()
            && c.is_ascii_alphabetic()
        {
            if was_alpha {
                return None;
            }
            was_alpha = true;
        }
    }
    None
}
