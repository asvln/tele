# tele
> Rust cli app to quickly define and `cd` to commonly used directories.

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

## How to use
All waypoints are saved in `~/.config/tele/waypoints.json`

```bash
# teleport to waypoint
t NAME

# add current directory to waypoints
t add
t add -n NAME  # define a custom name (default name is current directory)
t add -g GROUP # add to group

# remove waypoint
t rm -n NAME
t rm -g GROUP  # remove all group entries

# prints waypoints
t list          # prints ungrouped entries
t list -a       # prints all entries
t list -g GROUP # prints group entries
```

## License
This project is licensed under GNU GPL-3.0.
