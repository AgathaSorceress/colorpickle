{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in {
        packages.default = naersk-lib.buildPackage ./.;
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              rust-analyzer
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }) // {
        nixosModules.default = { config, lib, pkgs, ... }:
          with lib; {
            options.environment.graphical = with types; {
              colorschemes = mkOption {
                default = { };
                type = attrsOf (submodule {
                  options = {
                    image = mkOption {
                      type = either str path;
                      description =
                        "Path to image that will be used as input for colorscheme generation";
                    };
                    params = mkOption {
                      type = listOf str;
                      default = [ ];
                      description =
                        mdDoc "list of parameters to pass to `colorpickle`";
                      example = [
                        "--colors"
                        "8"
                        "--lighten"
                        "0.05"
                        "--bold-delta"
                        "0.1"
                      ];
                    };
                  };
                });
              };
              colors = mkOption {
                default = { };
                type = attrsOf (attrsOf str);
              };
            };

            config = let
              generate = name: image: args:
                pkgs.runCommand "colors-${name}" {
                  nativeBuildInputs = [ self.packages.${pkgs.system}.default ];
                } ''
                  colorpickle ${image} ${
                    lib.strings.concatStringsSep " " args
                  } > $out
                '';
              formattedColors = name: image: args:
                builtins.listToAttrs (lib.lists.imap0 (i: v: {
                  name = builtins.toString i;
                  value = v;
                }) (lib.strings.splitString "\n"
                  (builtins.readFile (generate name image args))));
            in {
              environment.graphical.colors = lib.attrsets.mapAttrs
                (name: value: formattedColors name value.image value.params)
                config.environment.graphical.colorschemes;
            };
          };
      };
}
