# Installing

## Github Releases

Compiled binaries are available at [Github Releases](https://github.com/kyleu/solitaire/releases). Simply download and extract your platform's distribution, unzip, and place on your path.

## Homebrew

`brew install kyleu/kyleu/solitaire`

## App Stores

Mobile apps are coming soon!

## Cargo

`cargo install solitaire`

## Build From Source

Clone this repository, and use the provided [scripts](scripts.md) to build and run it.

## Fresh Ubuntu

```shell
sudo apt-get install -y build-essential git curl nodejs npm libwebkit2gtk-4.0-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
