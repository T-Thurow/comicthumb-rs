{
  lib,
  fetchFromGitHub,
  rustPlatform
}:
let
  name = "comicthumb-rs";
  pname = "comicthumb";
  version = "0.1.0";

  src = ./.;
in
rustPlatform.buildRustPackage {
  inherit pname version src;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  preInstall = ''
      mkdir -p $out/share/thumbnailers
      cp comicthumb.thumbnailer $out/share/thumbnailers/
    '';

  postInstall = ''
      substituteInPlace $out/share/thumbnailers/comicthumb.thumbnailer \
        --replace-fail '=comicthumb' "=$out/bin/comicthumb"
    '';

  meta = with lib; {
    description = "Comicbook thumbnailer";
    homepage = "https://github.com/T-Thurow/comicthumb-rs";
    mainProgramm = "comicthumb";
  };
}
