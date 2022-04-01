# `trrs`

A CLI tool to transform data between different encodings.

## Installation

```bash
cargo install trrs
```

## Usage

See `trrs --help` for all options (as well as shorthand variants).

Some examples:

```bash
# Reading from STDIN and printing to STDOUT
echo -n ALLYOURBASEAREBELONGTOUS                 | trrs --in-type ascii  --out-type base32
# IFGEYWKPKVJEEQKTIVAVERKCIVGE6TSHKRHVKUY=
echo -n IFGEYWKPKVJEEQKTIVAVERKCIVGE6TSHKRHVKUY= | trrs --in-type base32 --out-type base64
# QUxMWU9VUkJBU0VBUkVCRUxPTkdUT1VT
echo -n QUxMWU9VUkJBU0VBUkVCRUxPTkdUT1VT         | trrs --in-type base64 --out-type hex
# 414c4c594f55524241534541524542454c4f4e47544f5553

# Reading and writing files
echo -n 414c4c594f55524241534541524542454c4f4e47544f5553 > hex
trrs --in hex    --in-type hex    --out base64 --out-type base64
trrs --in base64 --in-type base64 --out base32 --out-type base32
trrs --in base32 --in-type base32 --out ascii  --out-type ascii

cat ascii
# ALLYOURBASEAREBELONGTOUS
```

Tips

```bash
# A shorthand exists to convert STDIN between encodings.

# This means that this:
echo -n 'hello world' | trrs ascii base64
# is equivalent to this:
echo -n 'hello world' | trrs        --in-type ascii         --out-type base64
# and even this:
echo -n 'hello world' | trrs --in - --in-type ascii --out - --out-type base64
```
