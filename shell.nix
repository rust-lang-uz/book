{
  pkgs ? let
    lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
    nixpkgs = fetchTarball {
      url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
      sha256 = lock.narHash;
    };
  in
    import nixpkgs {overlays = [];},
  ...
}: let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ./book.toml).book;
in
  pkgs.stdenv.mkDerivation {
    name = "${manifest.title}-shell";

    nativeBuildInputs = with pkgs; [
      # Utilities
      git
      mdbook

      # Toml
      taplo
      taplo-cli
      taplo-lsp

      # Nix
      nixd
      statix
      deadnix
      alejandra
    ];
  }
