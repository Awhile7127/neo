# neo


## Description

A simple command-line utility to upload a single file to Neocities.

Designed to be used in conjunction with bash / another shell language if you
need to upload multiple pages.

Written in Rust, so should be platform-independent.


## Compiling

### Dependencies

You need to have installed:

- `libssl-dev`
- `pkg-config`

For Ubuntu / Debian-based systems, these are easy to install:

```
sudo apt install libssl-dev pkg-config
```

Tested using cargo:

### Debian / Ubuntu

```
sudo apt install cargo
cargo build -r
./target/release/neo
```


## Usage

```
neo --help        See help and available options
neo <api-key> <file> 
```

- api-key **REQUIRED**: Your neocities API key, see generation below
- file **REQUIRED**: The file to upload to your site


## Generating a Neocities API key

Easily done by `cURL`ing a Neocities web endpoint with your
username and password:

```
curl "https://USER:PASS@neocities.org/api/key"
```

You only need to generate your key once.

See more [here](https://neocities.org/api).


## Anderson

`Anderson` is a small shell script designed to automate the use of `neo`.
It can be run in the root directory of a site and will recursively traverse folders and upload
every file to Neocities.

Simply run:

```
./anderson $username $password
```
