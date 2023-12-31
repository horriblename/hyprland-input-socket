{
  lib,
  stdenv,
  hyprland,
}:
stdenv.mkDerivation {
  pname = "hyprland-input-socket";
  version = "0.1";
  src = ./.;

  inherit (hyprland) nativeBuildInputs;

  buildInputs = [hyprland] ++ hyprland.buildInputs;

  meta = with lib; {
    homepage = "https://github.com/horriblename/hyprland-input-socket";
    description = "Hyprland plugin to make borders more useful with a mouse.";
    license = licenses.bsd3;
    platforms = platforms.linux;
  };
}
