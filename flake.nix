{
  description = "Educationalsp";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      defaultPackage.${system} =
        pkgs.mkShell { buildInputs = with pkgs; [ just watchexec ]; };
    };

}
