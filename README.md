# Auto Sway

A set of scripts that make some interactions with sway more pleasant.

## Building

Should be as easy as running:

```console ignore
$ cargo build
```

## Running

Make sure you are running [`sway`](https://github.com/swaywm/sway) and
`$SWAYSOCK` points to the sway socket.

You can then run any of the provided scripts.

## Help messages

You can see the provided scripts' help messages here:

### Auto Resize

```console
$ auto-resize --help
Better resize commands

Instead of having to specify grow/shrink it will try to grow the container in the specified
direction and shrink it if it is not possible:

+---------+----------+ 
| <- left | right -> | 
+---------+----------+ 
| focused |          | 
+---------+----------+

`resize right` will grow the container to the right, but `resize left` will shrink the right instead
of trying to grow it. It will do the same with up and down.

Usage: auto-resize [OPTIONS] <COMMAND>

Commands:
  up     
  down   
  left   
  right  
  help   Print this message or the help of the given subcommand(s)

Options:
      --flip
          Flip the order of the rules; if it would have caused the container to grow then shrink it
          instead

  -h, --help
          Print help (see a summary with '-h')

```

## Running tests

If you want to run this crate's tests, then:

```console ignore
$ cargo build --all && cargo test
```

Or if you use [`cargo-nextest`](https://github.com/nextest-rs/nextest):

```console ignore
$ cargo build --all && cargo nextest run
```

If you have modified the help messages and want to regenerate the readme (make
sure check the diff first):

```console ignore
$ TRYCMD=overwrite cargo build --all && cargo nextest run
```
