{
  description = "CHIP-8 dev shell";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
  let
    system = "x86_64-linux";
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs { inherit system overlays; };
    rustToolchain = pkgs.rust-bin.stable.latest.default;

  in {
    devShells.${system}.default = pkgs.mkShell {
      name = "rust-dev-shell";

      packages = [
        rustToolchain
        pkgs.rust-analyzer
        pkgs.pkg-config
        pkgs.openssl
        pkgs.openssl.dev
        pkgs.SDL2
        pkgs.SDL2.dev
      ];

      shellHook = ''
        echo "🦀 Rust environment activated."
        echo "Cargo: $(cargo --version)"
        echo "SDL2 version: $(pkg-config --modversion sdl2 2>/dev/null || echo 'not found')"
      '';
    };
  };
}
