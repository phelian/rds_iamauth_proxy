{
  description = "phelian/rds_iamauth_proxy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let

        # Apply the custom overlay to `pkgsBase`
        pkgs = import nixpkgs {
          inherit system;
          overlays = [];
        };

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
          ];

          shellHook = ''
          '';
        };
      }
    );
}
