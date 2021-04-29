# Extension Fetcher
Extension fetcher is a rust program for automatically fetching paths to firefox addons from a toml file. It is intended to work with nix and will output a ./sources.nix that can be imported into nixExtensions in the unwrapped-firefox package.

## Installation

``` sh
nix-build default.nix
```

Or add to systemPackages or your home.packages variable.

## Usage

``` sh
extfetch
```

## Contributing
If you have something to add to this project feel free to make a PR. I'm not sure why you're using this, though.

Please make sure not to break anything.

## License 
MIT
