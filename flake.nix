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
      self,
      ...
    }:
    let
      eachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
      build_asciidoctor_revealjs =
        pkgs:
        pkgs.buildRubyGem {
          gemName = "asciidoctor-revealjs";
          version = "5.2.0";
          source = {
            type = "git";
            url = "https://rubygems.org/downloads/asciidoctor-revealjs-5.2.0.gem";
            sha256 = "sha256-NQSl5+ryyR3jf5YYkxT/sea/lNrZ1kbVyaJMZpG/auI=";
          };
        };

      build_asciidoctor =
        pkgs:
        pkgs.buildRubyGem {
          gemName = "asciidoctor";
          version = "2.0.25";
          source = {
            type = "git";
            url = "https://rubygems.org/downloads/asciidoctor-2.0.25.gem";
            sha256 = "sha256-sG/oIrCRDU4l6g9WL+z8eUjpRuIi79lJSEnSfaQmfRk=";
          };
        };

      build_docki =
        { naerskLib, pkgs }:
        let
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
        naerskLib.buildPackage {
          src = ./.;
          buildInputs = [
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl.dev
            libiconv
            makeWrapper
          ];
          postInstall = ''
            mkdir -p $out/share/bash-completion/completions
            mkdir -p $out/share/zsh/site-functions
            mkdir -p $out/share/fish/vendor_completions.d

            $out/bin/docki completions bash > $out/share/bash-completion/completions/docki
            $out/bin/docki completions zsh > $out/share/zsh/site-functions/_docki
            $out/bin/docki completions fish > $out/share/fish/vendor_completions.d/docki.fish

            wrapProgram $out/bin/docki --prefix PATH : ${pkgs.lib.makeBinPath runtimeDeps}
          '';
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

        preview = pkgs.mkShell {
          name = "docki-preview";
          buildInputs = [
            pkgs.zsh
            pkgs.fish
            pkgs.bash
            self.packages.${pkgs.system}.default
          ];
        };
      });

      packages = eachSystem (
        pkgs:
        let
          naerskLib = pkgs.callPackage naersk { };
        in
        {
          default = build_docki {
            naerskLib = naerskLib;
            pkgs = pkgs;
          };

          docker = pkgs.dockerTools.buildImage {
            name = "docki";
            tag = "latest";
            copyToRoot = pkgs.buildEnv {
              name = "docki-docker";
              paths = [
                pkgs.coreutils
                pkgs.bash
                pkgs.cacert
                (build_docki {
                  naerskLib = naerskLib;
                  pkgs = pkgs;
                })
              ];
            };
          };
        }
      );
    };
}
