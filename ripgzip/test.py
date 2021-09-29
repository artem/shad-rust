#!/usr/bin/env python3

import gzip
import pathlib
import subprocess
import sys
import random

DIR = pathlib.Path(__file__).parent.absolute()
DEBUG_BINARY_PATH = DIR / ".."/ "target" / "debug" / "ripgzip"
RELEASE_BINARY_PATH = DIR / ".." / "target" / "release" / "ripgzip"
OK_TESTS_PATH = DIR / "data" / "ok"
CORRUPTED_TESTS_PATH = DIR / "data" / "corrupted"
DUMP_PATH = DIR / "dump.gz"


def decompress_file_ripgzip(data, debug=True):
    path = DEBUG_BINARY_PATH if debug else RELEASE_BINARY_PATH
    proc = subprocess.run([path, "-d"], input=data, capture_output=True, check=True)
    return proc.stdout


def test_static_cases():
    for file_path in sorted(OK_TESTS_PATH.iterdir()):
        print(f"checking file '{file_path}'")

        with open(file_path, "rb") as f:
            data = f.read()

        expected = gzip.decompress(data)

        got = decompress_file_ripgzip(data)
        assert expected == got, f"incorrect output"


def test_small_random_cases():
    random.seed(23546262367)

    for i in range(100):
        print(f"testing small random data, case #{i}")

        data = bytes(random.randrange(256) for _ in range(10000))
        compressed = gzip.compress(data)

        try:
            decompressed = decompress_file_ripgzip(compressed)
            assert decompressed == data
        except Exception:
            with open(DUMP_PATH, "wb") as f:
                f.write(compressed)
            print(f"check failed, wrote problematic data to {DUMP_PATH}")
            raise


def test_big_random_cases():
    random.seed(326623662)

    for i in range(10):
        print(f"testing big random data, case #{i}")

        data = bytes(random.randrange(256) for _ in range(10000)) * 1000
        compressed = gzip.compress(data)

        try:
            decompressed = decompress_file_ripgzip(compressed, debug=False)
            assert decompressed == data
        except Exception:
            with open(DUMP_PATH, "wb") as f:
                f.write(compressed)
            print(f"check failed, wrote problematic data to {DUMP_PATH}")
            raise


def main():
    bundles = [
        test_static_cases,
        test_small_random_cases,
        test_big_random_cases,
    ]

    if len(sys.argv) > 1:
        n = int(sys.argv[1])
        bundles[n-1]()
    else:
        for i, bundle in enumerate(bundles, start=1):
            print(f"Running tests bundle #{i}")
            bundle()


if __name__ == "__main__":
    main()
