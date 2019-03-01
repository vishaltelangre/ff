# Find Files (ff)

[![Build Status](https://travis-ci.org/vishaltelangre/ff.svg?branch=master)](https://travis-ci.org/vishaltelangre/ff)
[![Version info](https://img.shields.io/crates/v/find-files.svg)](https://crates.io/crates/find-files)

Find Files (ff) utility recursively searches the files whose names match the
specified RegExp pattern in the provided directory (defaults to the current
directory if not provided).

Dual-licensed under [MIT](LICENSE-MIT) or the [UNLICENSE](UNLICENSE).

## Screenshot

![Screenshot](screenshot.png)

## Installation

Download the latest precompiled executable `ff` binary for your platform from the [releases](https://github.com/vishaltelangre/ff/releases) page.

If you're a Rust programmer, download and install `ff` command using `cargo install find-files`. To update to a newer version, use the `--force` flag.

## Usage

```
USAGE:
    ff [FLAGS] [OPTIONS] <PATTERN> [ROOT_PATH]

FLAGS:
    -s, --case-sensitive      Search case sensitively. By default, files are
                              searched case insensitively.
    -h, --help                Prints help information
    -G, --ignore-gitignore    Ignore searching files and directories specified
                              in .gitignore. By default, the files and
                              directories specified in .gitignore are included
                              in the search results.
    -H, --ignore-hidden       Ignore searching hidden files and directories. By
                              default, hidden files and directories are included
                              in the search results.
    -V, --version             Prints version information

OPTIONS:
    -x, --exclude <exclude>    Exclude files and directories matching this
                               regular expression from the search results.
    -j, --threads <threads>    The approximate number of threads to use. A value
                               of 0 (which is the default) results in thread
                               count set to available CPU cores.

ARGS:
    <PATTERN>      Find files whose name (path) matches this substring or
                   the regular expression.
    <ROOT_PATH>    Path to the directory to search files inside.[default:
                   `$PWD`]
```
## Examples

There are a tons of possibilities to search files using `ff`.
Following examples demonstrate just a tip of an iceberg.

- List paths of files recursively in the current working directory matching `article_` string.

```
ff article_
```

- List files having `.png`, or `.PNG` extension.

```
ff png$
```

- List files having strict `.PNG` extension.

```
ff -s PNG$
```

- Search various image files.

```
ff "\.(png|jpg|jpeg|gif|svg)$"
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
$ ff git.*commit

./.git/COMMIT_EDITMSG
# omitted other results
```

- Do not show hidden files and directories in the search results.

```
ff something -H
```

- Do not show those files and directories in the search results which are enlisted in `.gitignore`.

```
ff src/.*js$ -G
```

Without `-G (--ignore-gitignore)` flag in the above command, it also includes the results in the directories such as `node_modules` by default.

- Exclude (omit) files and directories which match the provided optional exclude RegExp pattern.

```
ff rb$ app/controllers -x /(audit|admin|sso|api)/
```

Above command will show paths of all files whose name ends with `rb` inside the relative `app/controllers` directory excluding the paths which match `/(audit|admin|sso|api)/` pattern.
