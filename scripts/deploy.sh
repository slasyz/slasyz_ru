#!/bin/bash

set -e

cd "$(dirname "$0")"/.. || exit 1

if [ $# -eq 0 ]
then
    echo "Usage: $0 user@hostname.ru"
    exit 1;
fi

SRC="./"
DEST="$1:~/deployments/slasyz_ru/"


rsync -avz --delete \
  --exclude='/.git' --filter="dir-merge,- .gitignore" \
  "$SRC" "$DEST"
scp "$SRC/config.json" "$1:/etc/slasyz_ru/config.json"
scp "$SRC/slasyz_ru.service" "$1:~/.config/systemd/user/"


docker compose create
docker cp slasyz_ru_app_1:/usr/local/bin/slasyz_ru - > /tmp/slasyz_ru.tar

tar xf /tmp/slasyz_ru.tar --directory /tmp
scp /tmp/slasyz_ru "$DEST"
rm /tmp/slasyz_ru.tar /tmp/slasyz_ru


ssh "$1" "systemctl --user daemon-reload && systemctl --user restart slasyz_ru"

echo "-> Done."
