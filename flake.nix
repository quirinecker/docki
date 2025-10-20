{
  description = "Asist Development Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    { nixpkgs, systems, ... }:
    let
      eachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
    in
    {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          name = "asist";
          buildInputs = with pkgs; [
            gcc
            openssl.dev
            pkg-config
            libiconv
            rustc
            cargo
            (ruby.withPackages (
              p: with p; [
                asciidoctor
                bundler
              ]
            ))
          ];
        };
      });
    };
}
