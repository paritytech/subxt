{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        isDarwin = pkgs.lib.hasSuffix "darwin" system;
      in
      {
        devShells.default =
          pkgs.mkShell {
            packages = with pkgs; [ protobuf llvm rustup ] ++
              (if isDarwin
              then with pkgs.darwin.apple_sdk; [ frameworks.SystemConfiguration Libsystem libcxx pkgs.darwin.apple_sdk.CLTools_Executables ]
              else [ ]);
            buildInputs = with pkgs; [ libclang ];
            LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
            HOST_CXXFLAGS =
              if isDarwin
              then "-I ${pkgs.darwin.apple_sdk.CLTools_Executables}/usr/include/c++/v1 -I ${pkgs.darwin.apple_sdk.CLTools_Executables}/usr/include"
              else "";
            BINDGEN_EXTRA_CLANG_ARGS = with pkgs;
              if isDarwin
              then "-isystem ${darwin.apple_sdk.Libsystem}/include"
              else "-isystem ${libclang.lib}/lib/clang/${lib.getVersion libclang}/include";

            shellHook = ''
              rustup install 1.70.0
              rustup default 1.70.0

              rustup toolchain install nightly-2023-05-23
              rustup target add wasm32-unknown-unknown --toolchain nightly-2023-05-23
              rustup override set nightly-2023-05-23
            '';
          };
      });
}
