#!/usr/bin/env python3
import argparse
import subprocess
from pathlib import Path


def test_params(echo: Path, binary: Path, params: list[str]) -> None:
    assert (  # noqa: S101
        subprocess.check_output(
            [echo, *params]) == subprocess.check_output([binary, *params])  # noqa: S603
    )


def test_newline(echo: Path, binary: Path) -> None:
    test_params(echo, binary, ["-n", r"test"])
    test_params(echo, binary, [r"test"])


def test_no_escapes(echo: Path, binary: Path) -> None:
    test_params(echo, binary, [r"\\\a\b\c\e\f\n\r\t\v\00\x0"])


def test_escapes(echo: Path, binary: Path) -> None:
    test_params(echo, binary, ["-e", r"test\\test"])
    test_params(echo, binary, ["-e", r"test\atest"])
    test_params(echo, binary, ["-e", r"test\btest"])
    test_params(echo, binary, ["-e", r"test\ctest"])
    test_params(echo, binary, ["-e", r"test\etest"])
    test_params(echo, binary, ["-e", r"test\ftest"])
    test_params(echo, binary, ["-e", r"test\ntest"])
    test_params(echo, binary, ["-e", r"test\rtest"])
    test_params(echo, binary, ["-e", r"test\ttest"])
    test_params(echo, binary, ["-e", r"test\vtest"])


def test_octal(echo: Path, binary: Path) -> None:
    test_params(echo, binary, [
                "-e", "-n", r"\0\00\0000\777\0777", r"\377\0377\376\0376", r"\00000\1000\01000"])


def test_hex(echo: Path, binary: Path) -> None:
    test_params(echo, binary, ["-e", "-n",
                r"\x\x0\x00\xFF", r"\xFE\x000\x100"])


def test_octal_hex(echo: Path, binary: Path) -> None:
    test_params(echo, binary, ["-e", "-n", r"\0\x\00\x0\0000\x00",
                r"\777\0777\377\0377\xFF", r"\376\0376\xFE\00000\x000", r"\1000\01000\x100"])


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("-e", "--echo", help="gnu echo binary", type=Path)
    parser.add_argument("-b", "--binary", help="custom echo binary", type=Path)
    args = parser.parse_args()
    echo = args.echo
    binary = args.binary

    test_newline(echo, binary)
    test_no_escapes(echo, binary)
    test_escapes(echo, binary)
    test_octal(echo, binary)
    test_hex(echo, binary)
    test_octal_hex(echo, binary)


if __name__ == "__main__":
    main()
