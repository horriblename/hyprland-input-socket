{
  description = "Hyprland plugin to make borders more useful with a mouse.";

  inputs.hyprland.url = "github:hyprwm/Hyprland";

  outputs = {
    self,
    hyprland,
  }: let
    inherit (hyprland.inputs) nixpkgs;
    withPkgsFor = fn: nixpkgs.lib.genAttrs (builtins.attrNames hyprland.packages) (system: fn system nixpkgs.legacyPackages.${system});
  in {
    packages = withPkgsFor (system: pkgs: {
      default = pkgs.callPackage ./default.nix {
        inherit (hyprland.packages.${system}) hyprland;
        stdenv = pkgs.gcc13Stdenv;
      };
    });

    devShells = withPkgsFor (system: pkgs: {
      default = pkgs.mkShell {
        name = "hyprland-plugins";
        nativeBuildInputs = [pkgs.gcc13];
        buildInputs = with pkgs; [
          meson
          ninja
          pkg-config
          gtk4-layer-shell
          gtk4
          hyprland.packages.${system}.hyprland
          cargo
        ];
        inputsFrom = with pkgs; [
          hyprland.packages.${system}.hyprland
          gtk4
          cairo
        ];
      };
    });

    formatter = withPkgsFor (system: pkgs: pkgs.alejandra);
  };
}
