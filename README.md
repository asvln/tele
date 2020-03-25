# tele
`tele` is an ergonomic way to quickly define and `cd` into commonly used directories; written in Rust.

## Installing
Compile and install the binary with cargo.

```bash
cargo install --git https://github.com/asvln/tele.git
```

Import [`tele.sh`](tele.sh) or add the following lines to your shell config file...

```sh
function t() {
  OUTPUT=`tele $@`
  if [ $? -eq 2 ]
    then cd "$OUTPUT"
    else echo "$OUTPUT"
  fi
}
```

Reload your shell and simply type `t` in your terminal to get started.

## License
This project is licensed under GNU GPL-3.0.
