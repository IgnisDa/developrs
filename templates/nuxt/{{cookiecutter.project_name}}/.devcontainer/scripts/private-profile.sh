#!/usr/bin/env bash

# load all my system configs
sudo rm -rf $HOME/.config
git clone https://github.com/IgnisDa/linux-configs.git $HOME/.config
sudo chown -R $USERNAME:$USERNAME $HOME/.config

# set the default interactive shell as fish
rm -rf "$HOME/.bashrc"
ln -s "$HOME/.config/.bashrc" "$HOME/.bashrc"
