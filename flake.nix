{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      system = "aarch64-darwin";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      # Rust toolchain with all cross-compilation targets
      toolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        targets = [
          # mac
          "aarch64-apple-darwin"
          "x86_64-apple-darwin"
          # linux
          "x86_64-unknown-linux-gnu"
          "aarch64-unknown-linux-gnu"
          # windows
          "aarch64-pc-windows-gnullvm"
          "x86_64-pc-windows-gnullvm"
        ];
      };

      # Pre-built LLVM/Clang toolchain for Windows cross-compilation.
      # Provides {x86_64,aarch64}-w64-mingw32-clang.
      # https://github.com/mstorsjo/llvm-mingw
      llvm-mingw = pkgs.stdenv.mkDerivation {
        pname = "llvm-mingw";
        version = "20260407";
        src = pkgs.fetchurl {
          url = "https://github.com/mstorsjo/llvm-mingw/releases/download/20260407/llvm-mingw-20260407-ucrt-macos-universal.tar.xz";
          hash = "sha256:801b49549ae39043d7195062eede67916b5ab46318a89e3b8209dc8f49441abb";
        };
        sourceRoot = ".";
        phases = [ "unpackPhase" "installPhase" ];
        installPhase = ''
          mkdir -p $out
          cp -r llvm-mingw-*/* $out/
        '';
      };

    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = [
          toolchain
          pkgs.pkg-config
          llvm-mingw
          # GCC cross-compilers for Linux targets
          pkgs.pkgsCross.gnu32.stdenv.cc
          pkgs.pkgsCross.gnu64.stdenv.cc
          pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc
        ];

        buildInputs = [ pkgs.openssl ];

        # Cargo linkers for cross targets
        CARGO_TARGET_AARCH64_PC_WINDOWS_GNULLVM_LINKER = "aarch64-w64-mingw32-clang";
        CARGO_TARGET_X86_64_PC_WINDOWS_GNULLVM_LINKER = "x86_64-w64-mingw32-clang";
        CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_LINKER = "i686-unknown-linux-gnu-gcc";
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "x86_64-unknown-linux-gnu-gcc";
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "aarch64-unknown-linux-gnu-gcc";

        # cc-rs per-target compilers. Build scripts (zstd-sys, bzip2-sys, ...)
        # use cc-rs which respects CC_<target>. Set these explicitly because
        # the shellHook below clears the global CC the cross stdenvs would
        # otherwise leak.
        CC_i686_unknown_linux_gnu = "i686-unknown-linux-gnu-gcc";
        CXX_i686_unknown_linux_gnu = "i686-unknown-linux-gnu-g++";
        CC_x86_64_unknown_linux_gnu = "x86_64-unknown-linux-gnu-gcc";
        CXX_x86_64_unknown_linux_gnu = "x86_64-unknown-linux-gnu-g++";
        CC_aarch64_unknown_linux_gnu = "aarch64-unknown-linux-gnu-gcc";
        CXX_aarch64_unknown_linux_gnu = "aarch64-unknown-linux-gnu-g++";
        CC_x86_64_pc_windows_gnullvm = "x86_64-w64-mingw32-clang";
        CXX_x86_64_pc_windows_gnullvm = "x86_64-w64-mingw32-clang++";
        CC_aarch64_pc_windows_gnullvm = "aarch64-w64-mingw32-clang";
        CXX_aarch64_pc_windows_gnullvm = "aarch64-w64-mingw32-clang++";

        # Both pkgsCross.*.stdenv.cc derivations export CC/CXX via their nix
        # setup hooks; whichever runs last wins, leaking a cross gcc into host
        # builds (cc-rs respects CC unconditionally). Clear them so host
        # builds fall back to clang and per-target builds use CC_<target>.
        shellHook = ''
          unset CC CXX
        '';
      };
    };
}