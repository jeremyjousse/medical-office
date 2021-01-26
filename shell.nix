let
in with import nix/default.nix { };
let
  overlay = self: super: {
    # Default JDK is 15 for everything, including Gradle
    jdk = super.adoptopenjdk-hotspot-bin-15;
  };
in
with import (builtins.fetchTarball {
       name = "nixpkgs-unstable";
       url = https://github.com/nixos/nixpkgs/archive/555641c50462d5ab87afe9ceab7df9d8c7755a37.tar.gz;
       # Hash obtained using `nix-prefetch-url --unpack <url>`
       # This hash is used to guarantee 100% reproducibility
       sha256 = "1lnbqv17i4f71xdi7hxvvd27r75sjqcppxi8rrmz62cm0x977m8b";
     }) { overlays = [overlay]; };

mkShell {
  # This is where all packages needed/wanted to develop must be specified
  buildInputs = [
    ruby
    sqlite
  ];

  # Application environment variables
  MY_ENV = "yop";
}
