{
  description = "comicthumb-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.11";
  };

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" ];
      forEachSystem = nixpkgs.lib.genAttrs systems;
      overlayList = [ self.overlays.default ];
      pkgsBySystem = forEachSystem ( system:
        import nixpkgs {
           inherit system;
           overlays = overlayList;
        }
      );
    in {
      overlays.default = final: prev: { comicthumb = final.callPackage ./package.nix { }; };
      packages = forEachSystem ( system: {
        comicthumb = pkgsBySystem.${system}.comicthumb;
        default = pkgsBySystem.${system}.comicthumb;
      });
    };
}
