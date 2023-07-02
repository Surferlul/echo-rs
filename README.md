# gnu-echo-rs

My rewrite of the echo GNU core utility in rust

version: 0.1.0
author: [Lu Baumann](https://blog.surferlul.me/)

## usage

refer to `gnu-echo-rs --help`:

```
Usage: gnu-echo-rs [SHORT-OPTION]... [STRING]...
  or:  gnu-echo-rs LONG-OPTION
Echo the STRING(s) to standard output. Rust rewrite of GNU echo util.

  -n             do not output the trailing newline
  -e             enable interpretation of backslash escapes
  -E             disable interpretation of backslash escapes (default)
      --help     display this help and exit
      --version  output version information and exit

If -e is in effect, the following sequences are recognized:

  \\      backslash
  \a      alert (BEL)
  \b      backspace
  \c      produce no further output
  \e      escape
  \f      form feed
  \n      new line
  \r      carriage return
  \t      horizontal tab
  \v      vertical tab
  \0NNN   byte with octal value NNN (1 to 3 digits)
  \xHH    byte with hexadecimal value HH (1 to 2 digits)
```

## testing

test script requires python >= 3.10

`gnu_echo_tests/test.py -e /bin/echo -b <path/to/gnu-echo-rs>`

| parameter        | value                                   |
| ---------------- | --------------------------------------- |
| `-e`, `--echo`   | path to echo executable to test against |
| `-b`, `--binary` | path to gnu-echo-rs binary to test      |
