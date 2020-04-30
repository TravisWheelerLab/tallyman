from logging import getLogger
from math import ceil, log2
from typing import Dict, Iterable, Tuple

_logger = getLogger("BitCompressor")

DEFAULT_ORDERING = (
    "A",
    "T",
    "C",
    "G",
)


class BitCompressor:
    _mapping: Dict[str, int]
    _pack_width: int

    def __init__(
        self, ordering: Iterable[str] = DEFAULT_ORDERING, pack_width: int = 0
    ):
        self._mapping: Dict[str, int] = {}
        for b, c in enumerate(ordering):
            self._mapping[c] = b
        self._pack_width = (
            pack_width if pack_width > 0 else ceil(log2(len(self._mapping)))
        )

    def compress(self, seq: str) -> Tuple[int, int]:
        """
        Bit compress a string sequence up to 32 base pairs long, assumed to
        be uppercase.

        The first element of the returned tuple is the bit compressed
        sequence represented as an integer.

        The second element of the returned tuple indicates how many of the
        provided nucleotides were actually compressed. For example, if `seq`
        is 10 characters long, then the value will be 10. On the other hand,
        if `seq` is 36 characters long, the value will be 32 since that is
        the maximum number of base pairs that will be encoded at once.

        TODO: See if we can make Python be smart about not using bigints
        TODO: The 32bp limit no longer makes sense with variable packing

        >>> compressor = BitCompressor()
        >>> b, c = compressor.compress("ATCG")
        >>> bin(b) # '0b00011011'
        '0b11011'
        >>> c
        4
        >>> b, c = compressor.compress("A" * 33)
        >>> c
        32
        >>> compressor = BitCompressor(ordering="GCTA")
        >>> b, c = compressor.compress("ATCG")
        >>> bin(b)
        '0b11100100'
        >>> compressor = BitCompressor(ordering="AB")
        >>> b, c = compressor.compress("ABAB")
        >>> bin(b) # '0b0101'
        '0b101'
        >>> compressor = BitCompressor(ordering="AB", pack_width=2)
        >>> b, c = compressor.compress("ABAB")
        >>> bin(b) # '0b00010001'
        '0b10001'
        """
        compressed = 0
        consumed = 0

        for nuc in seq:
            compressed = (compressed << self._pack_width) | self._mapping[nuc]
            if consumed >= 32:
                break
            consumed += 1

        return compressed, consumed

    def find_subsequence(self, needle: str, haystack: str) -> bool:
        """
        Locate the given bit-compressed needle in the haystack, returning
        True if it was found, False if not.

        TODO: Improve the unrecognized nucleotide handling

        >>> c = BitCompressor()
        >>> c.find_subsequence("AT", "GCATGC")
        True
        >>> c.find_subsequence("AT", "ATATAT")
        True
        >>> c.find_subsequence("GG", "AATTCC")
        False
        >>> c.find_subsequence("GG", "")
        False
        >>> c.find_subsequence("GG", "G")
        False
        >>> c.find_subsequence("GG", "AXGGTC")
        True
        """
        _logger.debug(f"searching for '{needle}' in '{haystack}'")
        compressed_needle, needle_length = self.compress(needle)

        compressed_range, range_length, start_index = self._safe_compress(
            haystack, needle_length
        )
        if range_length < needle_length:
            # The available haystack was too small so it can't possibly contain the needle
            return False

        next_index = start_index + needle_length
        while next_index < len(haystack):
            if compressed_range == compressed_needle:
                return True
            try:
                compressed_range = self.move_right(
                    compressed_range, haystack[next_index], range_length
                )
                next_index += 1
            except KeyError:
                _logger.debug(
                    f"skipping unrecognized nucleotide '{haystack[next_index]}'"
                )
                (
                    compressed_range,
                    range_length,
                    start_index,
                ) = self._safe_compress(
                    haystack, needle_length, start=next_index + 1
                )
                if range_length < needle_length:
                    return False
                next_index = start_index + needle_length

        return compressed_range == compressed_needle

    def move_right(self, seq: int, char: str, length: int) -> int:
        """
        Slide the sequence window to the right, dropping the
        left-most nucleotide and adding the nucleotide represented
        by `char` in the right-most position.

        >>> c = BitCompressor()
        >>> b = c.move_right(0b1010, "A", 2)
        >>> bin(b)
        '0b1000'

        # >>> c = BitCompressor(pack_width=4)
        # >>> b = c.move_right(0b10101111, "A", 2)
        # >>> bin(b)
        # '0b11110000'

        TODO: Is an addition really faster than a multiplication? Seems weird
        TODO: This breaks for pack widths != 2 because of 0b11, maybe just cache clear_high
        """
        # This allows us to clear the two highest bits
        clear_high = ~(0b11 << (length + length - self._pack_width))
        return ((seq & clear_high) << self._pack_width) | self._mapping[char]

    def _safe_compress(
        self, full_seq: str, length: int, start: int = 0
    ) -> Tuple[int, int, int]:
        """
        A wrapper around the `compress` method that accounts for
        unrecognized nucleotides in the sequence and skips them.

        TODO: This should probably be what compress does by default

        >>> c = BitCompressor()
        >>> b, l, s = c._safe_compress("ATXCG", 2, 1)
        >>> bin(b)
        '0b1011'
        >>> l
        2
        >>> s
        3
        """
        next_start = start
        while True:
            try:
                compressed_range, range_length = self.compress(
                    full_seq[next_start : next_start + length]
                )
                return compressed_range, range_length, next_start
            except KeyError:
                _logger.debug(
                    f"failed to compress '{full_seq[next_start:next_start+length]}', unrecognized nucleotide"
                )
                next_start += 1
