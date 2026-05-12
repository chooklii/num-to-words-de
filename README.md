# num-to-words-de

🇬🇧 **English** | [🇩🇪 Deutsch](#deutsch)

---

Convert a `u32` integer to its written-out equivalent in the German language.

## Examples

```
1       → eins
10      → zehn
13      → dreizehn
1000    → eintausend
12345   → zwölftausenddreihundertvierundfünfzig
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
num-to-words-de = "0.1.2"
```

## Usage

```rust
use num_to_words_de::num_to_word_de;

let result = num_to_word_de(101);
println!("{result}"); // einhunderteins
```

## Supported Range

Values from `0` to `4_294_967_295` (`u32::MAX`).

Supports ones, tens, hundreds, thousands, millions (*Million/Millionen*), and billions (*Milliarde/Milliarden*).

## License

MIT

## Contributing

If you find an issue, feel free to open one on [GitHub](https://github.com/chooklii/num-to-words-de). Please include the input value that caused the problem.

---

<a id="deutsch"></a>

# 🇩🇪 Deutsch

Konvertiert eine `u32`-Ganzzahl in das entsprechende ausgeschriebene Zahlwort der deutschen Sprache.

## Beispiele

```
1       → eins
10      → zehn
13      → dreizehn
1000    → eintausend
12345   → zwölftausenddreihundertvierundfünfzig
```

## Installation

In der `Cargo.toml` hinzufügen:

```toml
[dependencies]
num-to-words-de = "0.1.2"
```

## Verwendung

```rust
use num_to_words_de::num_to_word_de;

let ergebnis = num_to_word_de(101);
println!("{ergebnis}"); // einhunderteins
```

## Unterstützter Bereich

Werte von `0` bis `4.294.967.295` (`u32::MAX`).

Unterstützt Einer, Zehner, Hunderter, Tausender, Millionen und Milliarden.

## Lizenz

MIT

## Mitwirken

Falls dir ein Fehler auffällt, erstelle gerne ein Issue auf [GitHub](https://github.com/chooklii/num-to-words-de). Bitte gib den Eingabewert an, der das Problem verursacht hat.
