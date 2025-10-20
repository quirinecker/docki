{
  description = "Asist Development Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    {
      nixpkgs,
      systems,
      naersk,
      ...
    }:
    let
      eachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
      build_asciidoctor_revealjs = pkgs: pkgs.buildRubyGem {
        gemName = "asciidoctor-revealjs";
        version = "5.2.0";
        source = {
          type = "git";
          url = "https://rubygems.org/downloads/asciidoctor-revealjs-5.2.0.gem";
          sha256 = "sha256-NQSl5+ryyR3jf5YYkxT/sea/lNrZ1kbVyaJMZpG/auI=";
        };
      };

      build_asciidoctor = pkgs: pkgs.buildRubyGem {
        gemName = "asciidoctor";
        version = "2.0.25";
        source = {
          type = "git";
          url = "https://rubygems.org/downloads/asciidoctor-2.0.25.gem";
          sha256 = "sha256-sG/oIrCRDU4l6g9WL+z8eUjpRuIi79lJSEnSfaQmfRk=";
        };
      };
    in
    {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          name = "docki";
          buildInputs = with pkgs; [
            gcc
            openssl.dev
            pkg-config
            libiconv
            rustc
            cargo
            (ruby.withPackages (
              p: with p; [
                (build_asciidoctor_revealjs pkgs)
                (build_asciidoctor pkgs)
                bundler
              ]
            ))
          ];
        };
      });

      packages = eachSystem (
        pkgs:
        let
          naerskLib = pkgs.callPackage naersk { };
          runtimeDeps = with pkgs; [
            (ruby.withPackages (
              p: with p; [
                (build_asciidoctor_revealjs pkgs)
                (build_asciidoctor pkgs)
                bundler
              ]
            ))
          ];
        in
        {
          default = naerskLib.buildPackage {
            src = ./.;
            buildInputs =[
            ];
            nativeBuildInputs = with pkgs; [
              pkg-config
              openssl.dev
              libiconv
              makeWrapper
            ];
            postInstall = ''
              wrapProgram $out/bin/docki --prefix PATH : ${pkgs.lib.makeBinPath runtimeDeps}
            '';
          };
        }
      );
    };
}
