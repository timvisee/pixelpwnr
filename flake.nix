{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    
    let
        system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${system};
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        pixelpwnr = pkgs.rustPlatform.buildRustPackage {
                pname = manifest.name;
                version = manifest.version;
                cargoLock.lockFile = ./Cargo.lock;
                src = pkgs.lib.cleanSource ./.;
        };

        python-script = pkgs.python39.pkgs.buildPythonPackage rec {
          pname = "client";
          version = "1.0";

          src = ./scripts;
      };
 
    in{


    devShells.${system} = {
        
        default = pkgs.mkShell {

        packages = [
              pkgs.python3
        ];
      
        buildInputs = [
          pkgs.python3
        ];

        shellHook = ''
          python3 ./scripts/client.py
        '';
      };
    };

  packages.${system} = {
        default = pkgs.writeShellScriptBin "script" ''
          ${python-script}/bin/client.py "''${@:1}"
        '';
        pixelpwnr = pixelpwnr;
    };
    };
}
