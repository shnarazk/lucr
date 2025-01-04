 {
  description = "Latex math commands to Unicode Converter in Rust";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";
  outputs = { self, nixpkgs }:
  {
    packages = builtins.listToAttrs
      (map
        (system:
          with import nixpkgs { system = "${system}"; };
          {
            name = system;
            value = {
                default = rustPlatform.buildRustPackage rec {
                  name = "lucr-${version}";
                  pname = "lucr";
                  version = "0.1.0-20250104-1";
                  src = fetchFromGitHub {
                    name = "lucr";
                    owner = "shnarazk";
                    repo = "lucr";
                    rev = "3ed22a30a439d4dfa0bc6470cf1de106ebadf263";
                    hash = "sha256-ldmHS9LZ+iRy06QV1JtcUb50SusjdyXTpbS7Aj21Jto=";
                  };
                  cargoHash = "sha256-05Pv4mRtd7bBgxPjr0pz3wmxnBGh8mpAmqFScaQyS9A=";
                  buildInputs = rustc.buildInputs ++ [
                    cargo
                    rustc
                    libiconv
                    pkg-config
                  ];
                  buildPhase = "cargo build --release";
                  installPhase = ''
                    mkdir -p $out/bin;
                    install -t $out/bin target/release/lucr
                  '';
                };
            };
          }
        )
      [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ]
    );
    devShell = builtins.listToAttrs
      (map
        (system:
          with import nixpkgs { system = "${system}"; };
          {
            name = system;
            value = mkShell {
                packages = [
                  bashInteractive
                  libiconv
                  cargo
                  rustc
                  pkg-config
                ];
            };
          }
        )
      [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ]
    );
  };
}
