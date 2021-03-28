# csv-payment-engine

## Requirements

This project is based on cargo as a build system so you should install it before starting:

``` sh
# install rust and cargo alongside rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

## How to use it ?

``` sh
cargo run --release -- <file.csv>

```

### Generate **fat** CSV file

``` sh
cargo run --release --bin generator -- <number of CSV entries>
```

## Usage

``` sh
csv-payment-engine 0.1.0

USAGE:
    csv-payment-engine [OPTIONS] <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --buffer-capacity <buffer-capacity>     [default: 4096]

ARGS:
    <path>
```


