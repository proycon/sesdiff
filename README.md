[![Crate](https://img.shields.io/crates/v/sesdiff.svg)](https://crates.io/crates/sesdiff)
[![GitHub build](https://github.com/proycon/sesdiff/actions/workflows/sesdiff.yml/badge.svg?branch=master)](https://github.com/proycon/sesdiff/actions/)
[![GitHub release](https://img.shields.io/github/release/proycon/sesdiff.svg)](https://GitHub.com/proycon/sesdiff/releases/)
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)

# sesdiff: Shortest Edit Script Diff

## Description

This is a small and fast command line tool and Rust library that reads a two-column tab separated input from standard input and computes the shortest edit script (Myers' diff algorithm) to go from the string in column A to the string in column B. It also computes the edit distance (aka levenshtein distance).

There is also a [python binding](python/) available if you want to use sesdiff
from Python. The documentation here covers the command-line version.

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

Note that the edit scripts in suffix mode are formulated differently than in normal mode (they start from the right
too). There is also a ``--prefix`` option that strips common suffixes.

Use the ``--abstract`` parameter to get a slightly more abstract edit script that refers to the length of unchanged parts
rather than their contents. You would then get:

```
pidieron        pedir           -[eron]+[r]=[#2]-[i]+[e]   6
```

Sesdiff can also apply edit scripts to our input, use the ``--apply`` flag and feed the tool tab separated input with
a string in the first column and an edit script in the second, as in the the following example ``input2.tsv``:

```
$ cat input2.tsv
pidieron        -[eron]+[r]=[di]-[i]+[e]
```

Run sesdiff as follows and a third column will be added with the solution:

```
$ sesdiff --suffix --apply < input2.tsv
pidieron        -[eron]+[r]=[di]-[i]+[e]                pedir
```

When using ``--apply``, you can also make use of an extra ``--infix`` parameter to indicate that an edit script must be
attempted to be matched with any infix in the string, including multiple. Consider the following example that replaces
all letters *a* with *o*:

```
$ cat input3.tsv
hahaha       -[a]+[o]

$ sesdiff --infix --apply < input3.tsv
hahaha       -[a]+[o]	hohoho
```

In ``--apply`` mode, you can also make edit scripts applicable to multiple patterns by using the ``|`` operator. This is
only allowed for deletions (``-[]``) and equality checks (``=[]``):

```
$ cat input4.tsv
hihaho       -[a|i|o]+[e]

$ sesdiff --infix --apply < input4.tsv
hihaho       -[a|i|o]+[e]	hehehe
```

# License

GNU General Public Licence v3
