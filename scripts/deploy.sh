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
BINARY="$(mktemp)"


echo "-> Compiling binary"

podman-compose build
podman-compose up --no-start  # TODO: --force-recreate
podman cp slasyz_ru_app_1:/usr/local/bin/slasyz_ru - > /tmp/slasyz_ru.tar
tar xf /tmp/slasyz_ru.tar --directory /tmp
rm /tmp/slasyz_ru.tar
mv /tmp/slasyz_ru "$BINARY"


echo "-> Copying files"

rsync -avz --delete \
  --exclude='/.git' --filter="dir-merge,- .gitignore" \
  "$SRC" "$DEST"
rsync -avz --delete "$SRC/root/cv/" "$DEST/root/cv/"
scp "$SRC/config.json" "$DEST/config.json"
scp "$BINARY" "$DEST/slasyz_ru_new"
rm "$BINARY"

ssh "$1" "/bin/bash ~/deployments/slasyz_ru/scripts/install.sh"

echo "-> Deployment is done"
