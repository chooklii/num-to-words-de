<h1>num-to-words-de</h1>

Convert a u32 Integer value to its String equivalent in the German Language

_Zahlen in den das entsprechende Zahlenwort der deutschen Sprache konvertieren_


```rust
1 -> eins
10 -> zehn
13 -> dreizehn
1000 -> eintausend
12345 -> zwölftausenddreihundertvierundfünfzig
```

<h2>Usage</h2>

```rust
use num_to_words_de::num_to_word_de;

let input_number: u32 = 101;

let number_as_string: String = num_to_word_de(input_number);

println!("Number: {number_as_string}");
// Number: einhunderteins

```

# Contributing

If you are able to detect an issue feel free to create an issue. Please add the input value when doing so.