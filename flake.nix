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
				buildInputs = [( pkgs.dyalog.override { acceptLicense = true; } )];
			in {
				packages.default = pkgs.rustPlatform.buildRustPackage {
					pname = "apl-readline";
					version = "0.0.1";

					src = ./.;
					cargoLock.lockFile = ./Cargo.lock;

					inherit buildInputs;
				};
				devShells.default = pkgs.mkShell {
					inherit buildInputs;
					nativeBuildInputs = rust-env;
				};
			}
		);
}
