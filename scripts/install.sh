#!/bin/bash

set -e

cd "$(dirname "$0")"/.. || exit 1


echo "-> Installing files"

cp ./slasyz_ru.service ~/.config/systemd/user/
cp ./caddy/Caddyfile.prod ~/caddy/slasyz_ru.caddy


echo "-> Replacing variables"

FILES_WITH_VARS="/home/sl/.config/systemd/user/slasyz_ru.service /home/sl/caddy/slasyz_ru.caddy"

PROJECT_DIR="$(pwd)"
export PROJECT_DIR
export PROJECT_LOG_DIR="/home/sl/logs/slasyz_ru"

for file in $FILES_WITH_VARS; do
  tmp=$(mktemp)
  envsubst < "$file" > "$tmp";
  mv "$tmp" "$file";
done;

chmod a+r ~/caddy/slasyz_ru.caddy


echo "-> Reloading everything"

systemctl --user daemon-reload
systemctl --user restart slasyz_ru
# Add this line to sudoers file to reload Caddy without password:
#  %sudo ALL=NOPASSWD: /bin/systemctl reload caddy
sudo systemctl reload caddy


echo "-> Installation is done"
