from logging import getLogger
from typing import Dict, Iterable, Set

from the_count.bit_compressor import BitCompressor
from the_count.fasta_utils import Chunk

_logger = getLogger(__name__)


def count_occurrences(
    needles: Iterable[str],
    haystacks: Iterable[Chunk],
    compressor: BitCompressor,
) -> Dict[str, Set[str]]:
    """
    >>> compressor = BitCompressor()
    >>> needles = ["AT", "GG"]
    >>> haystacks = [Chunk("A", "ATCG"), Chunk("B", "ATTCGG")]
    >>> counts = count_occurrences(needles, haystacks, compressor=compressor)
    >>> list(counts.keys())
    ['AT', 'GG']
    >>> sorted(counts['AT'])
    ['A', 'B']
    >>> sorted(counts['GG'])
    ['B']
    """
    hits: Dict[str, Set[str]] = {}

    for haystack in haystacks:
        for needle in needles:
            present = compressor.find_subsequence(needle, haystack.sequence)
            if present:
                _logger.info(f"found '{needle}' in {haystack.name}")
                if needle not in hits:
                    hits[needle] = set()
                hits[needle].add(haystack.name)

    return hits
