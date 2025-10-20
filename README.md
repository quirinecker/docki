# Docki

> [!NOTE]
> This project is under renovations. While installing it right now might work, it is not guaranteed


## Preview

![screencast](.gitlab/screencast.gif)

Docki is cli for converting asciidoctor files into html files. You can build your documentation with `docki build` and write documenation using the live server with `docki serve`.

## Installation

### Nix

This installation method is recommended, becuase it is the most maintained due to me using nix in development.

Note: This is the most basic installation. If you are already more experienced, you might want to add it to your shell or home manager configuration.

```nix
nix profile install github:quirinecker/docki
```

### Homebrew

```
brew tap quirinecker/docki-homebrew https://gitlab.com/quirinecker/docki-homebrew
```

```
brew install docki
```

### Cargo

```shell
cargo install docki
```



