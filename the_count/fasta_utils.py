from logging import getLogger
from typing import Dict, Iterator, NamedTuple, Optional, TextIO, Union

from .exceptions import FASTAException

_logger = getLogger(__name__)


class Chunk(NamedTuple):
    name: str
    sequence: str


# TODO: Rename to as_sequence_iterator
def as_smart_iterator(file: TextIO) -> Iterator[Chunk]:
    """
    Load a multiple FASTA file and provide its sequences as an
    iterator that contains the sequence and the sequence name (header).

    Sequences are converted to uppercase.

    >>> from io import StringIO
    >>> f0 = StringIO(">foo\\nabcd\\n>bar\\nefgh")
    >>> list(as_smart_iterator(f0))
    [Chunk(name='foo', sequence='ABCD'), Chunk(name='bar', sequence='EFGH')]
    >>> f1 = StringIO(">foo\\nabcd\\ndcba\\n>bar\\nefgh\\n")
    >>> list(as_smart_iterator(f1))
    [Chunk(name='foo', sequence='ABCDDCBA'), Chunk(name='bar', sequence='EFGH')]
    >>> f2 = StringIO(">foo\\n>bar")
    >>> as_sequence_to_name(f2)
    Traceback (most recent call last):
    the_count.exceptions.FASTAException: no sequence for name 'foo' on line 1
    >>> f3 = StringIO("abcd\\n>bar")
    >>> as_sequence_to_name(f3)
    Traceback (most recent call last):
    the_count.exceptions.FASTAException: unnamed sequence 'ABCD' near line 1
    """
    _logger.debug(f"loading smart iterator for {file}")
    name: Optional[str] = None
    seq: Optional[str] = None

    lineno: int = 0
    while True:
        line = next(file, None)
        lineno += 1

        if line is None or line.startswith(">"):
            if name is not None and seq is not None:
                # Finished the last pair, ready for a new pair
                yield Chunk(name, seq)
                name = None
                seq = None

            if name is None and seq is None:
                # Happy path, we're ready for a new seq:name pair or
                # we're finished with the entire file successfully
                if line is None:
                    break
                name = line[1:].strip()
            elif name is not None and seq is None:
                raise FASTAException(
                    f"no sequence for name '{name}' on line {lineno - 1}"
                )
            elif name is None and seq is not None:
                raise FASTAException(
                    f"unnamed sequence '{seq}' near line {lineno - 1}"
                )
            else:
                # This can never happen because we already checked for
                # both being not None above and set them both to None,
                # so if we hit this, there's a bug
                raise RuntimeError()
        else:
            assert line is not None
            fragment = str(line).strip().upper()
            if seq is None:
                seq = fragment
            else:
                seq += fragment


def as_sequence_to_name(file: TextIO) -> Dict[str, str]:
    """
    Load a multiple FASTA file into a dictionary for which the keys are
    the sequences (as strings, without newlines) and the values are
    the sequence names, as specified in the sequence headers.

    If there happen to be multiple sequences with the same name, which
    generally shouldn't happen anyway, the last one found will be used.

    Sequences are converted to uppercase.

    >>> from io import StringIO
    >>> f0 = StringIO(">foo\\nabcd\\n>bar\\nefgh")
    >>> as_sequence_to_name(f0)
    {'ABCD': 'foo', 'EFGH': 'bar'}
    >>> f1 = StringIO(">foo\\nabcd\\ndcba\\n>bar\\nefgh\\n")
    >>> as_sequence_to_name(f1)
    {'ABCDDCBA': 'foo', 'EFGH': 'bar'}
    >>> f2 = StringIO(">foo\\n>bar")
    >>> as_sequence_to_name(f2)
    Traceback (most recent call last):
    the_count.exceptions.FASTAException: no sequence for name 'foo' on line 1
    >>> f3 = StringIO("abcd\\n>bar")
    >>> as_sequence_to_name(f3)
    Traceback (most recent call last):
    the_count.exceptions.FASTAException: unnamed sequence 'ABCD' near line 1
    """
    _logger.debug(f"loading sequence to name dict for {file}")

    s2n: Dict[str, str] = {}
    name: Optional[str] = None
    seq: Optional[str] = None

    lineno: int = 0
    while True:
        line = next(file, None)
        lineno += 1

        if line is None or line.startswith(">"):
            if name is not None and seq is not None:
                # Finished the last pair, ready for a new pair
                s2n[seq] = name
                name = None
                seq = None

            if name is None and seq is None:
                # Happy path, we're ready for a new seq:name pair or
                # we're finished with the entire file successfully
                if line is None:
                    break
                name = line[1:].strip()
            elif name is not None and seq is None:
                raise FASTAException(
                    f"no sequence for name '{name}' on line {lineno - 1}"
                )
            elif name is None and seq is not None:
                raise FASTAException(
                    f"unnamed sequence '{seq}' near line {lineno - 1}"
                )
            else:
                # This can never happen because we already checked for
                # both being not None above and set them both to None,
                # so if we hit this, there's a bug
                raise RuntimeError()
        else:
            assert line is not None
            fragment = str(line).strip().upper()
            if seq is None:
                seq = fragment
            else:
                seq += fragment

    return s2n


def sequence_count(file: TextIO) -> int:
    """
    Determine the number of sequences in a multiple sequence FASTA file,
    assuming that each sequence begins with a header denoted by ">".

    This is not as fast as it could be, shelling out to `wc` is faster,
    for example, but this version is cross-platform and good enough for
    some use-cases.

    >>> from io import StringIO
    >>> f0 = StringIO(">foo\\nabcd\\n>bar\\nefgh")
    >>> sequence_count(f0)
    2
    >>> f1 = StringIO(">foo\\nabcd\\ndcba\\n>bar\\nefgh")
    >>> sequence_count(f1)
    2
    >>> f2 = StringIO("")
    >>> sequence_count(f2)
    0
    """
    seq_count = 0
    for line in file:
        if line.startswith(">"):
            seq_count += 1
    return seq_count
