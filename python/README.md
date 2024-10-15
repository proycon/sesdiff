## Sesdiff python binding

## Description

This is a python library that reads relates two strings and computes the
shortest edit script (Myers' diff algorithm) to go from the string in column A
to the string in column B. It also computes the edit distance (aka levenshtein
distance).

There is also a [command line version](../) available.

## Installation

```
pip install sesdiff
```

## Usage

The `shortest_edit_script` function returns an `EditScript` instance containing
the instructions needed to get from string A to string B. Instructions are
represented as two-tuples with the first string being a character representing
the edit instruction type (`+` for insertion, `-` for deletion, `=` for
identity) and the second the actual textual content.

```python
from sesdiff import shortest_edit_script, Mode

#normal mode
result = shortest_edit_script("hablaron","hablar")
print(result)
assert result.distance() == 2
assert len(result) == 2 
assert result[0] == ('=',"hablar")
assert result[1] == ('-',"on")

#print all instructions manually
for instruction in result:
    print(instruction)

#suffix mode
result = shortest_edit_script("hablaron","hablar", Mode.SUFFIX)
assert result.distance() == 2
assert len(result) == 1
assert result[0] == ('-',"on")

#works fine with unicode:
result = shortest_edit_script("говорим","говорить")
assert result.distance() == 3
assert len(result) == 3
assert result[0] == ('=',"говори")
assert result[1] == ('-',"м")
assert result[2] == ('+',"ть")
```

## Limitations

The apply functionality from the main library/CLI tool is not implemented yet.

Do not use this library if you're merely interested in computing levenshtein
distance, it comes with performance overhead to return the actual edit scripts.
