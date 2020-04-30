import argparse
from more_itertools import chunked
from multiprocessing import Pool
import signal
from typing import Dict, Set

import logging
import sys

from the_count._process_chunks import init_processor, process_chunks
from the_count.bit_compressor import BitCompressor
from the_count.count_occurrences import count_occurrences
from the_count.fasta_utils import as_sequence_to_name, as_smart_iterator

_logger = logging.getLogger(__name__)


def main():
    parser = argparse.ArgumentParser(
        prog="the_count", description="Exact subsequence match counter",
    )

    parser.add_argument(
        "--needles",
        type=str,
        required=True,
        help="FASTA file containing sequences to count",
    )
    parser.add_argument(
        "--haystack",
        type=str,
        required=True,
        help="FASTA file containing sequences to search",
    )
    parser.add_argument(
        "--debug",
        action="store_true",
        default=False,
        help="Print debugging information",
    )
    parser.add_argument(
        "--silent",
        action="store_true",
        default=False,
        help="Do not print anything at all",
    )
    parser.add_argument(
        "--workers",
        type=int,
        default=0,
        help="Number of worker processes to spawn",
    )
    args = parser.parse_args(sys.argv[1:])

    if args.debug:
        logging.root.setLevel(logging.DEBUG)
    elif args.silent:
        pass
    else:
        logging.root.setLevel(logging.INFO)

    if not args.silent:
        logging.root.addHandler(logging.StreamHandler())

    compressor = BitCompressor()

    with open(args.needles, "r") as needles_file:
        needles = as_sequence_to_name(needles_file)

    with open(args.haystack, "r") as haystack_file:
        haystacks = as_smart_iterator(haystack_file)

        progress = 0
        if not args.silent:
            print("PROGRESS 0", end="")

        counts: Dict[str, Set[str]] = {}

        if args.workers > 0:
            pool = Pool(
                initializer=init_processor, initargs=(compressor, needles)
            )

            def sigint_handler(_sig, _frame):
                pool.terminate()
                pool.join()
                sys.exit(0)

            signal.signal(signal.SIGINT, sigint_handler)

            for matches in pool.imap(process_chunks, chunked(haystacks, 10)):
                progress += 10
                if not args.silent:
                    print(f"\rPROGRESS {progress}", end="")
                for match_needle in matches:
                    if match_needle not in counts:
                        counts[match_needle] = set()
                    counts[match_needle].update(matches[match_needle])
                for match in matches:
                    if match.needle not in counts:
                        counts[match.needle] = set()
                    counts[match.needle].add(match.haystack)

            pool.close()
            pool.join()
        else:
            for haystacks in chunked(haystacks, 10):
                matches = count_occurrences(needles, haystacks, compressor)
                progress += 10
                if not args.silent:
                    print(f"\rPROGRESS {progress}", end="")
                for match_needle in matches:
                    if match_needle not in counts:
                        counts[match_needle] = set()
                    counts[match_needle].update(matches[match_needle])

        if not args.silent:
            print("\r", end="")

        for seq in counts:
            if len(counts[seq]) == 0:
                continue
            print(f"{seq}:")
            for loc in counts[seq]:
                print(f"    {loc}")


if __name__ == "__main__":
    main()
