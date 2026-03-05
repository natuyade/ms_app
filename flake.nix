{
  description = "Rust + Trunk devShell (WSL NixOS)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Rust toolchain（簡易版：nixpkgsのrust）
            rustc
            cargo
            rust-analyzer

            # Trunk + wasm
            trunk
            wasm-bindgen-cli
            binaryen

            # よくある依存
            pkg-config
            openssl
            nodejs
          ];

          # fish利用でもOK。環境変数の補助
          shellHook = ''
            export RUST_BACKTRACE=1
          '';
        };
      });
}
