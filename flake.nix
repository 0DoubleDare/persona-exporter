{
  description = "Metrics Exporter for Influx DB / Victoria Metrics";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          pname = "persona-exporter";
          version = "0.1.0";
          src = ./.;
          cargoHash = "sha256-000000000";

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ openssl ];

          meta = with pkgs.lib; {
            descriptions = "Metrics exporter for InfluxDB / Victoria Metrics written on Rust";
            homepage = "https://github.com/Persona-Team/persona-exporter.git";
            licence = licenses.mit;
          };
        };

      }
    );

}
