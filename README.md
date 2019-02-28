# Find Files (ff)

Find Files (ff) utility recursively searches the files whose names match the
specified RegExp pattern in the provided directory (defaults to the current
directory if not provided).

Dual-licensed under [MIT](LICENSE-MIT) or the [UNLICENSE](UNLICENSE).

## Screenshot

![Screenshot](screenshot.png)

## Installation

Download the latest executable `ff` binary for your platform from the [releases](https://github.com/vishaltelangre/ff/releases) page.

If you're a Rust programmer, download and install `ff` command using `cargo install find-files`.

## Usage

```
USAGE:
    ff [FLAGS] <PATTERN> [ROOT_PATH]

FLAGS:
    -s, --case-sensitive    Search case sensitively. By default, files are
                            searched case insensitively.
    -h, --help              Prints help information
    -H, --ignore-hidden     Ignore searching hidden files and directories. By
                            default, hidden files and directories are included
                            in the search results.
    -V, --version           Prints version information

ARGS:
    <PATTERN>      Find files whose name (path) matches this substring or
                   the regular expression.
    <ROOT_PATH>    Path to the directory to search files inside. [default:
                   `$PWD`]
```
## Examples

There are a tons of possibilities to search files using `ff`.
Following examples demonstrate just a tip of an iceberg.

- List paths of files recursively in the current working directory matching `main` string.

```
ff main
```

- List files having `.png`, or `.PNG` extension.

```
ff \.png$
```

- List files having strict `.PNG` extension.

```
ff -s \.PNG$
```

- Search various image files.

```
ff "\.(png|jpg|jpeg|gif|svg)"
```

- List files whose path matches `controllers` string.

```
ff controllers
```

- Search `.js` files in `./spec` directory.

```
ff \.js ./spec
```

- Search a file which is expected to be inside hidden `.git` directory whose name contains `commit` or something similar.

```bash
$ ff git.*commit -H

./.git/COMMIT_EDITMSG
# omitted other results
```
