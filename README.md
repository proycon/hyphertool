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

Then just run it on a text file, output will be to standard output, syllable delimiter can be configured via ``--delimiter``.

```
hyphertool --language nl test.txt
```

