{
  description = "walrs CLI tool";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          pname = "walrs";
          version = "1.1.2";
          src = ./.;
          
          cargoHash = "sha256-ItjbG/fPAW1mDJI5JXZ73L4c3UeDZzjBdADsyWhqzm8=";         

          nativeBuildInputs = with pkgs; [ bash ];
          
          # Allow unstable Rust features
          preBuild = ''
            export RUSTC_BOOTSTRAP=1
          '';
          
          # Let rustPlatform handle the binary, we just add the extra files
          postInstall = ''
            mkdir -p $out/etc/walrs/{templates,scripts,colorschemes}
            
            if [ -d "templates" ]; then
              cp -r templates/* $out/etc/walrs/templates/ || true
            fi
            if [ -d "scripts" ]; then
              cp -r scripts/* $out/etc/walrs/scripts/ || true
            fi
            if [ -d "colorschemes" ]; then
              cp -r colorschemes/* $out/etc/walrs/colorschemes/ || true
            fi
            
            if [ -f "walrs.1" ]; then
              install -Dm644 walrs.1 $out/share/man/man1/walrs.1
            fi
          '';
          
          meta = with pkgs.lib; {
            description = "Generate colorscheme from image";
            license = licenses.gpl3;
            homepage = "https://github.com/pixel2175/walrs";
          };
        };
      });
}
