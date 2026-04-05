{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      targets = {
        "x86_64-linux" = [ "x86_64-unknown-linux-gnu" ];
        "aarch64-darwin" = [ "aarch64-apple-darwin" "x86_64-apple-darwin" ];
      };
      mkShell = system:
        let
          pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
          toolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" "clippy" ];
            targets = targets.${system};
          };
        in
        pkgs.mkShell {
          nativeBuildInputs = [ toolchain pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };
    in {
      devShells.x86_64-linux.default = mkShell "x86_64-linux";
      devShells.aarch64-darwin.default = mkShell "aarch64-darwin";
    };
}
