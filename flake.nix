{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";

    # nixpkgs is a repo that contains all the packages we use.
    # Check out https://search.nixos.org/packages to search avalible packages.
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # Nix Flakes are a new feature in nix, flake-compat allows pre-flake nix to read this repo.
    # The default.nix and shell.nix are there to allow old nix to work with this repo.
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, flake-utils, naersk, flake-compat, nixpkgs }:
    # Loops over the supported OS/Arcitectures, like x86-linux, mac-arm etc
    # use `nix flake show` to see more.
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        # OpenSSL expects all its stuff to be in one location, so merge its packages into one dir.
        # This may only be needed if statically linking?
        openssl-toolchain = (pkgs.symlinkJoin {
          name = "openssl-toolchain";
          paths = with pkgs; [ openssl.out openssl.dev openssl.bin ];
        });

        # This is used so IDEs can find the rust compiler.
        rust-toolchain = (pkgs.symlinkJoin {
          name = "rust-toolchain";
          paths = with pkgs; [ rustc cargo rustPlatform.rustcSrc rust-analyzer ];
        });

        # libaries that both the main package and devShell need.
        # If on a Mac, also include the Security package. (Only seems to be needed on Apple Silicon, TODO why?)
        commonBuildInputs = with pkgs; [
          openssl
          libiconv
          makeWrapper
        ] ++ lib.optionals stdenv.isDarwin [ pkgs.darwin.apple_sdk.frameworks.Security ];

        # naersk is a tool that handle building Rust projects in Nix.
        naerskPkg = pkgs.callPackage naersk { };

        runtimeInputs = with pkgs; [
          awscli2 # aws command.
          dig # need to do DNS lookups
          postgresql # needed for psql
        ];

        # See https://github.com/nix-community/naersk for more options.
        main = naerskPkg.buildPackage {
          src = ./.;
          # nativeBuildInputs is for packages that the compiling machine needs to execute.
          nativeBuildInputs = with pkgs; [ pkg-config ];

          # buildInputs are packages that the final binary needs to link against.
          buildInputs = commonBuildInputs;

          # Openssl needs this env var.
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          OPENSSL_DIR = "${openssl-toolchain}";

          # Create a wrapper around the binary that sets the PATH to include the runtime apps.
          # see https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/setup-hooks/make-wrapper.sh
          postInstall = ''
            wrapProgram $out/bin/ailo-tools \
            --prefix PATH : ${pkgs.lib.makeBinPath runtimeInputs} \
            --set-default RUST_BACKTRACE 1 
          '';
        };

      in
      rec {
        # For `nix build` & `nix run`:
        packages.default = main;

        # For `nix develop` (or nix-shell) gives you a bash prompt with these packages avalible
        devShell = pkgs.mkShell {
          buildInputs = commonBuildInputs;

          # Packages to be installed into the dev shell.
          nativeBuildInputs = with pkgs; [
            awscli2 # Used to get RDS auth token.
            imagemagick # Used to convert heic images to jpg
            rust-toolchain
            dig # Used for DNS lookups to RDS.
            postgresql
            cargo-watch
            pkg-config
          ];
          # Environment variables
          RUST_BACKTRACE = 1;
          OPENSSL_DIR = "${openssl-toolchain}";
          # On NixOS IntelliJ passes its own LD_LIBRARY_PATH into its terminal, which causes glibc issues, so clear it.
          LD_LIBRARY_PATH = "";
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      }
    );
}

