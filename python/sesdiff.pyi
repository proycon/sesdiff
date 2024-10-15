
class EditScript:
    """This class holds an edit scripts, consisting of one or more instructions on how to go from string A to string B"""
 
    def distance(self) -> int:
        """Returns the levenshtein distance"""

    def __len__(self) -> int:
        """Returns the number of edit instructions"""

    def __getitem__(self, item: int) -> tuple[str,str]:
        """Returns the edit instruction at the specified index. Returns a tuple where the first string is just a character (-,+,=) representing the instruction type, and the second string the actual data."""

    def mode(self) -> Mode:
        """Returns the edit mode"""

    def __str__(self) -> str:
        """Returns a string representation of the script"""

class Mode:
    """An enumeration of possible edit modes"""

    NORMAL: Mode 
    SUFFIX: Mode 
    PREFIX: Mode 
    INFIX: Mode 

def shortest_edit_script(source: str, target: str, mode: Mode = Mode.NORMAL, allow_substitutions: bool = True) -> EditScript:
    """Compute the shortest edit script (Myers' diff) between source and target where we look at suffixes and strip common prefixes. Returns an edit script

    Parameters
    -----------

    source: str
        The source string
    target: str
        The target string
    mode: Mode
        The mode in which to run, set to `Mode.PREFIX` to strip common prefix, `Mode.SUFFIX` to formulate edit scripts from the end, or `Mode.NORMAL` for the default behaviour.
    allow_substitutions: bool
        Count substitutions as distance 1 (rather than 2, deletion+insertion)
    """
