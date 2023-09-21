{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
        flake-utils.follows = "flake-utils";
      };
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, pre-commit-hooks }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          # import and bind toolchain to the provided `rust-toolchain.toml` in the root directory
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rustToolchainWasm = pkgs.rust-bin.stable.latest.default.override {
            # Set the build targets supported by the toolchain, wasm32-unknown-unknown is required for trunk.
            targets = [ "wasm32-unknown-unknown" ];
          };
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          craneLibWasm = (crane.mkLib pkgs).overrideToolchain rustToolchainWasm;

          # declare the sources
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              # include everything in the `tests` directory - including test objects
              (pkgs.lib.hasInfix "/tests/" path) ||
              # Default filter from crane (allow .rs files)
              (craneLib.filterCargoSources path type)
            ;
          };
          wasmSrc = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              # include all html and scss files
              (pkgs.lib.hasSuffix "\.html" path) ||
              (pkgs.lib.hasSuffix "\.scss" path) ||
              # Include additional assets we may have
              (pkgs.lib.hasInfix "/assets/" path) ||
              # Default filter from crane (allow .rs files)
              (craneLibWasm.filterCargoSources path type)
            ;
          };

          # declare the build inputs used to build the projects
          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
            act
          ] ++ macosBuildInputs;
          # declare the build inputs used to link and run the projects, will be included in the final artifact container
          buildInputs = with pkgs; [ openssl sqlite ];
          macosBuildInputs = with pkgs.darwin.apple_sdk.frameworks;
            [ ]
            ++ (nixpkgs.lib.optionals (nixpkgs.lib.hasSuffix "-darwin" system) [
              Security
              CoreFoundation
            ]);

          # declare build arguments
          commonArgs = {
            inherit src buildInputs nativeBuildInputs;
          };
          wasmArgs = {
            src = wasmSrc;
            pname = "wasm-client";
            cargoExtraArgs = "-p service-ui";
            CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          };

          # Cargo artifact dependency output
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          cargoArtifactsWasm = craneLibWasm.buildDepsOnly (wasmArgs // {
            doCheck = false;
          });

          # Actual targets to be built and executed
          service-ui = craneLibWasm.buildTrunkPackage (wasmArgs // {
            inherit cargoArtifactsWasm;
            pname = "service-ui";
            cargoExtraArgs = "-p service-ui";
            trunkIndexPath = "service-ui/index.html";
          });

          server = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
            pname = "server";
            cargoExtraArgs = "-p server";
            CLIENT_DIST = service-ui;
          });

          serverOci = {
            name = "source-warden";
            tag = "latest";
            config = {
              Cmd = [ "${server}/bin/server" ];
            };
          };
          serverOciImage = pkgs.dockerTools.buildImage ({
            copyToRoot = [ server ];
          } // serverOci);
          serverOciStream = pkgs.dockerTools.streamLayeredImage ({
            contents = [ server ];
          } // serverOci);
        in
        with pkgs;
        {
          # formatter for the flake.nix
          formatter = nixpkgs-fmt;

          # executes all checks
          checks = {
            inherit server service-ui;
            source-warden-clippy = craneLib.cargoClippy (commonArgs // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets";
              CLIENT_DIST = "";
            });
            source-warden-fmt = craneLib.cargoFmt commonArgs;
            # pre-commit-checks to be installed for the dev environment
            pre-commit-check = pre-commit-hooks.lib.${system}.run {
              src = ./.;

              # git commit hooks
              hooks = {
                nixpkgs-fmt.enable = true;
                # clippy.enable = true;
                rustfmt.enable = true;
                markdownlint.enable = true;
                commitizen.enable = true;
                typos.enable = true;
              };
            };
          };

          # packages to build and provide
          packages = {
            inherit server service-ui;
            server-docker = serverOciImage;
            server-docker-stream = serverOciStream;
            default = server;
            about = pkgs.writeScriptBin "about" ''
              #!/bin/sh
              echo "Welcome to our bot!"
            '';
          };

          # applications which can be started as-is
          apps.server = {
            type = "app";
            program = "${self.packages.${system}.server}/bin/server";
          };

          # development environment provided with all bells and whistles included
          devShells.default = mkShell {
            inherit (self.checks.${system}.pre-commit-check) shellHook;
            inputsFrom = [
              server
              service-ui
            ];
            buildInputs = with pkgs; [
              dive
            ];
          };
        });
}
