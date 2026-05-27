# COORDINATE CONVERTER
> Has this ever happened to you? You're preparing an observation run and need
> the coordinates of an object in HMS DMS (or DMS DMS or DD.DDDD DD.DDDD)
> format. Fear not, the `COORDINATE CONVERTER` comes to the rescue.

## Installation
To install you need [Cargo](https://rustup.rs/), the Rust package manager, then just run:
```bash
cargo install --git https://github.com/AstroTeutloff/coordinate_converter.git

```

It can be uninstalled via
```bash
cargo uninstall coordinate_converter
```

## Usage
Just run one of the three included binaries `ddconv` for convertion from
DD.DDDD DD.DDDD format, `hmsconv` for convertion from HMS DMS format, and
`dmsconv` for convertion from DMS DMS format.

### Example
```bash
ddconv 37.954561 89.264109
```
returns `02:31:49.095 +89:15:50.792`.

Running the commands with the `--help` flag is possible.

## Contributions
Arrogant that I am, I _think_ I've eradicated all bugs, but should you
encounter interesting behaviour please let me know via PR or PM.


This project was [brainmade](https://brainmade.org/).
