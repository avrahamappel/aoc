#!/usr/bin/env nix-shell
#!nix-shell -p bash npins -i bash

# Update flake inputs
nix flake update

# Update npins sources
npins update
