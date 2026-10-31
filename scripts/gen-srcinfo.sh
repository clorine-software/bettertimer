#!/bin/sh
nix shell nixpkgs#pacman --command sh -c "
makepkg --config ./makepkg.conf --printsrcinfo > .SRCINFO
exit
"
