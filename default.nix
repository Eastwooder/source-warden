# Generated with 'nix flake init --template templates#compat' to satisfy VS Codium's Nix environment selector plugin
(import (fetchTarball https://github.com/edolstra/flake-compat/archive/master.tar.gz) {
  src = builtins.fetchGit ./.;
}).defaultNix
