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

        runtimeLibs = with pkgs; [
          alsa-lib
          alsa-plugins
          pulseaudio
          libpulseaudio
          udev
          openssl
          wayland
          wayland-protocols
          libxkbcommon
          libx11
          libxi
          libxcursor
          libxrandr
          vulkan-loader
          mesa
          libdrm
        ];

        devPackages = with pkgs; [
          rustc
          cargo
          rust-analyzer
          trunk
          wasm-bindgen-cli
          binaryen
          pkg-config
          nodejs
          mesa-demos
          vulkan-tools
        ] ++ runtimeLibs;
      in {
        devShells.default = pkgs.mkShell {
          packages = devPackages;

          shellHook = ''
            export RUST_BACKTRACE=1
            export PULSE_SERVER=unix:/mnt/wslg/PulseServer
            export ALSA_PLUGIN_DIR="${pkgs.alsa-plugins}/lib/alsa-lib"
            export LD_LIBRARY_PATH="/run/opengl-driver/lib:/usr/lib/wsl/lib:${pkgs.lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"
            export MESA_D3D12_DEFAULT_ADAPTER_NAME=Nvidia
            export VK_DRIVER_FILES=/run/opengl-driver/share/vulkan/icd.d/dzn_icd.x86_64.json
            export WGPU_BACKEND=vulkan
            export WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER=1
            export WGPU_POWER_PREF=high
          '';
        };
      }
    );
}