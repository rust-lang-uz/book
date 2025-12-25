{
  description = "A beginning of an awesome project bootstrapped with github:bleur-org/templates";

  inputs = {
    # Unstable Nixpkgs
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Flake parts for eachSystem
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} (top @ {...}: {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem = {pkgs, ...}: {
        # Nix formatter
        formatter = pkgs.alejandra;

        # Development shells
        devShells.default = import ./shell.nix {inherit pkgs;};

        # Output package
        packages.default = pkgs.callPackage ./. {inherit pkgs;};
      };
    });
}
