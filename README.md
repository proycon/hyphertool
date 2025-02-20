# Introduction

This is a command-line tool for syllabification and hyphenisation, it supports multiple languages.
It is just a thin wrapper on top of the [hypher](https://crates.io/crates/hypher) for Rust, the hyphenation rules it uses for
the various languages are derived from TeX.

## Install

Ensure ``cargo`` and ``rustc`` are installed on your system, then:

```
$ cargo install hyphertool
```

## Usage

### Syllabification

Given a text file, output the text with all syllables explicitly marked. The delimiter can be set via ``--delimiter``:

```
$ hyphertool --language nl test.txt
Dit is een test-be-stand. Kan je dit be-stand mooi voor mij ver-wer-ken?
Ik hoop op een po-si-tief re-sul-taat.
```

### Hyphenation

Wrap a text on a certain width. Note that the width is in unicode points (not bytes), and has no regard for double-spaced characters:

```
$ hyphertool --language nl --width 15 test.txt
Dit is een test-
bestand. Kan je
dit bestand mooi
voor mij verwer-
ken?
Ik hoop op
een positief re-
sultaat.
```

### Stand-off syllabification

Output all syllables with stand-off offsets to the text. Offsets are 0-indexed
unicode character points, the end is non-inclusive. Output will be in TSV (Tab
Separated Values) which can be easily imported in other software like [stam
import](https://github.com/annotation/stam-tools) for further analysis.

```
$ hyphertool --language nl --standoff test.txt
Text	BeginOffset	EndOffset
Dit	0	3
is	4	6
een	7	10
test	11	15
be	15	17
stand	17	22
Kan	24	27
je	28	30
dit	31	34
be	35	37
stand	37	42
mooi	43	47
voor	48	52
mij	53	56
ver	57	60
wer	60	63
ken	63	66
Ik	68	70
hoop	71	75
op	76	78
een	79	82
po	83	85
si	85	87
tief	87	91
re	92	94
sul	94	97
taat	97	101
```

### Dehyphenation

This tool can also be used for simple dehyphenation. The `--language` parameter is not actually implemented for this so this is not a lexical-informed dehyphenation. The mode is triggered by specifying one or more characters to dehyphenate on via `--dehyphenate`. We take the output of our earlier hyphenation test as input again:

```
$ hyphertool --language nl --dehyphenate - test2.txt                                                       ❌2 
Dit is een testbestand. Kan je
dit bestand mooi
voor mij verwerken?
Ik hoop op
een positief resultaat.
```
