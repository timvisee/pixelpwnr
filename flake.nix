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
        # Weitere Argumente hier hinzufügen...
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
          alias fire="./result/bin/pixelpwnr"
          python3 ./scripts/client.py
        '';
      };
    };
    };
}
