# GW-Basic and QBasic in Docker

Based on my original blog post [here](https://ngeor.com/2020/02/22/gwbasic-in-docker.html)

You need to have `GWBASIC.EXE` or `QBASIC.EXE`, it is _not_ baked in the images.

## Tested on

- Ubuntu via Vagrant
- Windows

## Prerequisites

- GWBASIC located at `~/DOSBox/PROGS/GWBASIC/GWBASIC.EXE` and in PATH.
  Vagrant copies it under `/usr/local/bin`.
- QBASIC located at `~/DOSBox/PROGS/QBASIC/QBASIC.EXE` and in PATH.
  Vagrant copies it under `/usr/local/bin`.
- DOSBox
- Rust
- Bazel

Build with `bazel //...`.

## dosbox_wrapper

A Rust binary that can launch DOSBox in headless mode.

```
dosbox_wrapper DIR CMD
```

Parameters:

- `DIR`: The directory in which to mount the C:\ drive
- `CMD`: The command to run

The output of `CMD` will be printed to stdout.

DOSBox needs to be in PATH or in the default Windows installation location.

The directory needs to be writable in order to capture the output
in a temporary file inside DOSBox.

## gwbasic_dosbox_wrapper

A Rust binary, that builds on top of `dosbox_wrapper`,
running a GW-Basic program with DOSBox.

`GWBASIC.EXE` needs to be in PATH.

```
gwbasic_dosbox_wrapper PROGRAM.BAS
```

Parameters:

- `PROGRAM.BAS`: The program to run.

The folder where `PROGRAM.BAS` lives will be mounted as the `C:\` drive
in DOSBox. The folder needs to be writable. `GWBASIC.EXE` is copied
into the folder before execution and cleaned up afterwards.

## qbasic_dosbox_wrapper

Same as `gwbasic_dosbox_wrapper` but for QBasic.

## Bazel rule gwbasic_binary

Defines an executable Bazel target that can run a GW-Basic program.

Under the hood, it uses the `gwbasic_dosbox_wrapper` tool and generates
a Batch file (Windows) / Shell script (*nix) to run the program.

### TODO

implement a `rust_binary` with `include_str!` to have a self-contained
executable without runfiles / batch files.

## Bazel rule qbasic_binary

Defines an executable Bazel target that can run a QBasic program.
See `gwbasic_binary` for details.

## Apache configuration in Vagrant

### Defaults

- Logs at `/var/log/apache2/`, e.g. `/var/log/apache2.error.log`
- `mod_cgi` disabled, but once enabled the default configuration
  activates `/cgi-bin/` pointing to `/usr/lib/cgi-bin/`
- Home folder at `/var/www/html/`

### Customizations

- Extra modules such as `mod_cgi` enabled through Vagrant
- Assigned ownership to `www-data` group for www folders

## basic-launcher-rust

It creates a batch file located at the common ancestor of the input program
and the interpreter.

### GWBasic

### QBasic



## Apache

> Runs Apache HTTPD, supporting BAS files via CGI.

Usage:

```
docker run --rm \
  --name basic-httpd \
  -v /folder/with/basic:/basic/bin:ro \
  -v /folder/with/program:/basic/src \
  -p 8080:80 \
  ngeor/basic-httpd
```

And then visit http://localhost:8080/cgi-bin/PROGRAM.BAS
