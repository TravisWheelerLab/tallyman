from typing import Iterable, List, Optional

from the_count.bit_compressor import BitCompressor
from the_count.count_occurrences import count_occurrences
from the_count.fasta_utils import Chunk

_compressor: Optional[BitCompressor] = None
_needles: Optional[Iterable[str]] = None


def init_processor(compressor: BitCompressor, needles: Iterable[str]):
    global _compressor
    global _needles
    if _compressor is None:
        _compressor = compressor
    if _needles is None:
        _needles = needles


def process_chunks(chunks: List[Chunk]):
    return list(count_occurrences(_needles, chunks, _compressor))
