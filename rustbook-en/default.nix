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
  # Helpful nix function
  lib = pkgs.lib;

  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ./book.toml).book;
in
  pkgs.stdenv.mkDerivation {
    pname = manifest.title;
    version = "0.0.1";

    # Website contents
    src = pkgs.lib.cleanSource ./.;

    # Compile time dependencies
    nativeBuildInputs = with pkgs; [
      mdbook
    ];

    buildPhase = ''
      mdbook build
    '';

    installPhase = ''
      mkdir -p $out
      mv ./book/{.,}* $out/
    '';

    meta = with lib; {
      homepage = manifest.website;
      description = "${manifest.title} documentation website generated with mdBook";
      # https://github.com/NixOS/nixpkgs/blob/master/lib/licenses.nix
      license = with lib.licenses; [mit];
      platforms = with platforms; linux ++ darwin;
      maintainers = [lib.maintainers.orzklv];
    };
  }
