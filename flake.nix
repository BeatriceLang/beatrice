{
  description = "Development shell for beatrice";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          llvm = pkgs.llvmPackages_22.llvm;
        in
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.cargo
              pkgs.rustc
              llvm
              pkgs.libffi
              pkgs.libxml2
              pkgs.zlib
            ];

            LLVM_SYS_221_PREFIX = llvm.dev;
            LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.libffi
              pkgs.libxml2
              pkgs.zlib
              pkgs.stdenv.cc.cc.lib
            ];
          };
        }
      );
    };
}
