[![Crate](https://img.shields.io/crates/v/sesdiff.svg)](https://crates.io/crates/sesdiff)

# sesdiff: Shortest Edit Script Diff

## Description

This is a small and fast command line tool that reads a two-column tab separated input from standard input and computes the shortest edit script (Myers' diff algorithm) to go from the string in column A to the string in column B. It also computed the edit distance (aka levenshtein distance).

It was written to build lemmatisers.

## Installation

Install it using Rust's package manager:

```
cargo install sesdiff
```

No cargo/rust on your system yet? Do ``sudo apt install cargo`` on Debian/ubuntu based systems, ``brew install rust`` on mac, or use [rustup](https://rustup.rs/).

This tool builds upon [Dissimilar](https://crates.io/crates/dissimilar) that provides the actual diff algorithm (will be
downloaded and compiled in automatically).

## Usage

```
$ sesdiff < input.tsv
```

Example input and output (reformatted for legibility, the first two columns correspond to the input). Output is in a four-column tab separated format:

```
hablaron        hablar     =[hablar]-[on]                  2
contaron        contar     =[contar]-[on]                  2
pidieron        pedir      =[p]-[i]+[e]=[di]-[eron]+[r]    6
говорим         говорить   =[говори]-[м]+[ть]              3
```

By default the full edit script will be provided in a simple language:

* ``=[]`` - The text between brackets is identical in strings A and B
    * ``=[#n]`` - If you use the ``--abstract`` parameter, this will be used instead, where ``n`` represents a number
      indicating the length of text between  that is identical in strings A and B
* ``-[]`` - The text between brackets is removed to get to string B
* ``+[]`` - The text between brackets is added to get to string B

For lemmatisation purposes, it makes sense for many languages to look at
suffixes (from right to left) and strip common prefixes. Pass the ``--suffix``
option for that behaviour and output is now:

```
$ sesdiff --suffix < input.tsv
hablaron        hablar          -[on]                      2
contaron        contar          -[on]                      2
pidieron        pedir           -[eron]+[r]=[di]-[i]+[e]   6
говорим         говорить        -[м]+[ть]                  3
```

There is also a ``--prefix`` option that strips common suffixes.

Use the ``--abstract`` parameter to get slightly more abstract edit script that refer to the length of unchanged parts
rather than their contents. You would then get:

```
pidieron        pedir           -[eron]+[r]=[#2]-[i]+[e]   6
```


# License

GNU General Public Licence v3

