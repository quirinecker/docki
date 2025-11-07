# Docki

> [!NOTE]
> This project is under renovations. It can be used, but it might not be completely stable yet.


## Preview

![screencast](.github/assets/screencast.gif)

Docki is cli for converting asciidoctor files into html files.

## Usage

> [!NOTE]
> This works in a directory with a directory called `docs`, which contains the asciidoctor documentation.

### Building the documentation

```shell
docki build
```

### Serving the documentation on a live server

```shell
docki serve
```

## Installation

### Homebrew

```shell
brew tap quirinecker/homebrew-docki https://github.com/quirinecker/homebrew-docki
```

```
brew install docki
```

### Nix

If you just want to try it out real quick and the nix package manager is available on your system you can use the following command.

```shell
nix develop github:quirinecker/docki#preview
```

This will open a shell evnironment with docki installed. If you want to install it permanently with nix, i would recommend following the instructions in the [Nix (Advanced, Flake)](#nix-advanced-flake) section.

### Cargo

> [!NOTE]
> This is the most basic installation. It will not include asciidoctor_revealjs and asciidoctor itself. Installing asciidoctor has to be done manually, while installing asciidoctor_revealjs can be done with `docki install-reveal`

```shell
cargo install docki
```

### Docker

There is also a docker image available to use. It is primarily used for the gh actions.

```shell
docker pull ghcr.io/quirinecker/docki:latest
```

You can also build it yourself with nix.

```
nix build .#docker && docker load -i result
```

### Nix (Advanced, Flake)

> [!NOTE]
> There are multiple ways to install docki with nix. This is the way I installed it on my machine.

1. Add it to the flake inputs

```nix
docki = {
	url = "github:quirinecker/docki";
	inputs.nixpkgs.follows = "nixpkgs";
};
```

2. Add `@inputs` at the end of the outputs (if you haven't already)

```nix
outputs = {
	nixpkgs
	...
}@inputs:
...Rest of your flake...
```

3. Add the input to your system packages (system configuration) or home packages (home manager configuration)

```nix
environment.systemPackages = with pkgs; [
	inputs.docki.packages.${system}.default
]
```

or

```nix
home.packages = with pkgs; [
	inputs.docki.packages.${system}.default
]
```

## Development

### Running it

If you just want to run it, you can use the `nix run` command. This will install all the build dependencies, build the binary and run it.

```shell
nix run
```

### Development Shell

You can also use the development shell with the command below. In this shell all dependencies for building and running the project are installed.

```shell
nix develop
```

Afterwards it can be built and run with cargo

```shell
cargo run -- <args>
```

