{
  projectRootFile = "flake.nix";
  programs.nixpkgs-fmt.enable = true;

  programs.prettier = {
    enable = true;
    includes = [
      "*.md"
      "*.yaml"
      "*.yml"
    ];
  };
  programs.ruff.format = true;
}
