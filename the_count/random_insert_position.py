from random import randint


def random_insert_position(n_seqs: int, seq_len: int, target_len: int) -> (int, int):
    """
    Given a particular number of sequences of particular length, return
    a sequence index, and an index into that sequence, at which another
    sequence of the given target length might be inserted.

    The indices returned are zero-based and, in the case of the inner
    index, include the left endpoint.

    For example, the following code should work:
    >>> seqs = ['abcd', 'efgh']
    >>> target_len = 2
    >>> seq_index, inner_index = random_insert_position(2, 4, target_len)
    >>> seq_index in [0, 1]
    True
    >>> inner_index in [0, 1, 2]
    True

    In this case, the sequence index determines which of "abcd" and "efgh"
    should be selected. The inner index determines where to start an
    insertion. In this case, since the target length is 2, the insertion
    can begin at index 0, 1, or 2. If it began at index 3 the insertion
    would overrun the end of the sequence.
    """
    seq_index: int = randint(0, n_seqs - 1)
    inner_index: int = randint(0, seq_len - target_len)
    return seq_index, inner_index
