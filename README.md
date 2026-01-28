# lucr

**lucr** (LaTeX-to-Unicode Converter for Rust) is a fast formatter that converts LaTeX math commands to Unicode symbols. It's designed primarily for writing Lean4 files with proper mathematical notation in editors like Helix and Zed.

## Features

- **Fast Conversion**: Converts 238+ LaTeX commands to Unicode symbols
- **Comprehensive Symbol Support**:
  - Greek letters (`\alpha` → α, `\beta` → β, `\Gamma` → Γ)
  - Mathematical operators (`\le` → ≤, `\ge` → ≥, `\in` → ∈, `\notin` → ∉)
  - Arrows (`\r` → →, `\l` → ←, `|->` → ↦)
  - Superscripts and subscripts (`^2` → ², `_1` → ₁)
  - Set notation (`\cap` → ∩, `\cup` → ∪, `\empty` → ∅)
  - Number sets (`\N` → ℕ, `\R` → ℝ, `\Z` → ℤ, `\Q` → ℚ, `\C` → ℂ)
  - Logical operators (`\and` → ∧, `\or` → ∨, `\not` → ¬, `\all` → ∀, `\exists` → ∃)
  - And many more mathematical symbols
- **Editor Integration**: Works seamlessly with Helix and Zed editors
- **Stdin/Stdout Filter**: Can be used in pipelines or as a standalone tool

## Installation

### From Cargo

```bash
cargo install lucr
```

### From Source

```bash
git clone https://github.com/shnarazk/lucr
cd lucr
cargo build --release
# The binary will be in target/release/lucr
```

## Usage

### Command Line

lucr reads from standard input and writes to standard output:

```bash
# Basic usage
echo '\alpha + \beta = \gamma' | lucr
# Output: α + β = γ

# Process a file
cat math_file.lean | lucr > output.lean

# Use in a pipeline
cat input.lean | lucr | other_tool
```

### Examples

```bash
# Greek letters
echo '\alpha \beta \gamma \Delta \Omega' | lucr
# → α β γ Δ Ω

# Mathematical operators
echo 'x \in \N, x \le 10 \and x \ge 0' | lucr
# → x ∈ ℕ, x ≤ 10 ∧ x ≥ 0

# Complex expressions
echo '\all x \in \R, \exists y \in \R, y^2 = x' | lucr
# → ∀ x ∈ ℝ, ∃ y ∈ ℝ, y² = x
```

### helix configuration

- .config/helix/languages.toml

```toml
[[language]]
name = "lean"
auto-format = true
formatter = { command = "lucr", args = [] }
```

### zed configuration

- You need Lean4 extension (now in dev mode): https://github.com/shnarazk/zed-lean4
- .config/zed/settings.json

```json
  "languages": {
    "Lean4": {
      "formatter": {
        "external": {
          "command": "lucr"
        }
      }
    }
  },
```

## How It Works

lucr uses a lookup table to match LaTeX commands with their Unicode equivalents. When it encounters a backslash (`\`) in the input, it attempts to match the following characters against known LaTeX commands and replaces them with the corresponding Unicode symbol.

The tool is smart enough to:
- Handle escaped backslashes (`\\`) correctly
- Stop matching at word boundaries (spaces, parentheses, brackets, etc.)
- Preserve unrecognized LaTeX commands as-is

## Supported Symbol Categories

1. **Italic alphabet** (Mi): Mathematical italic letters
2. **Greek letters**: Both lowercase and uppercase
3. **Number sets**: ℕ, ℤ, ℚ, ℝ, ℂ
4. **Constants**: ∞, ∅, ⊤, ⊥
5. **Superscripts**: ⁰¹²³⁴⁵⁶⁷⁸⁹ and letters
6. **Subscripts**: ₀₁₂₃₄₅₆₇₈₉ and letters
7. **Arrows**: ←, →, ↑, ↓, ↦
8. **Parentheses**: ⟨, ⟩, ⟪, ⟫, «, »
9. **Binary operators**: ∧, ∨, ∩, ∪, ×, •, ⊎, ⊓, ⊔
10. **Relations**: ≤, ≥, ∈, ∉, ⊆, ⊇, ≠, ≃, ≅, ≺, ≼

## License

This project is licensed under the Mozilla Public License Version 2.0 (MPL-2.0). See the [LICENSE](LICENSE) file for details.

## Author

Narazaki Shuji <shujinarazaki@protonmail.com>

## Repository

https://github.com/shnarazk/lucr
