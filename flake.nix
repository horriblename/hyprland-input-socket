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
      default = pkgs.callPackage ./default.nix {inherit (hyprland.packages.${system}) hyprland; stdenv = pkgs.gcc13Stdenv;};
    });

    devShells = withPkgsFor (system: pkgs: {
      default = pkgs.mkShell {
        name = "hyprland-plugins";
        nativeBuildInputs = [pkgs.gcc13];
        buildInputs = [hyprland.packages.${system}.hyprland];
        inputsFrom = [hyprland.packages.${system}.hyprland];
      };
    });

    formatter = withPkgsFor (system: pkgs: pkgs.alejandra);
  };
}
