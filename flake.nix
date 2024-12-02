{
	inputs = {
		nixpkgs.url     = "github:nixos/nixpkgs/nixpkgs-unstable";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem(system:
			let
				pkgs = import nixpkgs {
					inherit system;
					config.allowUnfree = true; # APL
				};
				rust-env = with pkgs; [
					cargo
					rustc
					rustfmt
					clippy
					rust-analyzer
					gdb
				];
				dyalog = pkgs.dyalog.override { acceptLicense = true; };
			in {
				packages = rec {
					apl-readline = pkgs.rustPlatform.buildRustPackage {
						pname = "apl-readline";
						version = "0.0.1";

						src = ./.;
						cargoLock.lockFile = ./Cargo.lock;
					};
					apl = pkgs.writeShellScriptBin "apl" ''
						export PATH=${pkgs.lib.strings.makeBinPath [ dyalog pkgs.coreutils ] }
						${apl-readline}/bin/apl-readline
					'';
					default = apl;
				};
				devShells.default = pkgs.mkShell {
					buildInputs = [ dyalog ];
					nativeBuildInputs = rust-env;
				};
			}
		);
}
