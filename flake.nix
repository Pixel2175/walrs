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
          
          # Install everything to the Nix store first
          postInstall = ''
            # Install config files to result/
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
            
            # Install man page
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
        
        # Create installer that copies from result/ to system locations
        packages.install-system = pkgs.writeShellScriptBin "walrs-install-system" ''
          set -e
          WALRS_PACKAGE="${self.packages.${system}.default}"
          
          echo "Installing walrs from Nix store to system locations..."
          
          # Install binary
          sudo install -m755 "$WALRS_PACKAGE/bin/walrs" /usr/local/bin/walrs
          
          # Install config files from result/etc/walrs/ to /etc/walrs/
          sudo mkdir -p /etc/walrs
          if [ -d "$WALRS_PACKAGE/etc/walrs" ]; then
            sudo cp -r "$WALRS_PACKAGE/etc/walrs"/* /etc/walrs/
          fi
          
          # Install man page
          if [ -f "$WALRS_PACKAGE/share/man/man1/walrs.1" ]; then
            sudo install -Dm644 "$WALRS_PACKAGE/share/man/man1/walrs.1" /usr/share/man/man1/walrs.1
          fi
          
          echo "✓ walrs installed to /usr/local/bin/walrs"
          echo "✓ Configuration files installed to /etc/walrs/"
          echo "✓ Man page installed to /usr/share/man/man1/walrs.1"
        '';
      });
}
