#!/bin/bash

set -e

FILES_WITH_VARS="/home/sl/.config/systemd/user/slasyz_ru.service /home/sl/deployments/slasyz_ru/caddy/Caddyfile.prod"
PROJECT_DIR="/home/sl/deployments/slasyz_ru"
PROJECT_LOG_DIR="/home/sl/logs/slasyz_ru"

for file in $FILES_WITH_VARS; do
  echo "replacing in $file"
  sed -i "s:###PROJECT_DIR###:$PROJECT_DIR:" "$file";
  sed -i "s:###PROJECT_LOG_DIR###:$PROJECT_LOG_DIR:" "$file";
done;

systemctl --user daemon-reload
systemctl --user restart slasyz_ru
# Add this line to sudoers file to reload Caddy without password:
#  %sudo ALL=NOPASSWD: /bin/systemctl reload caddy
sudo systemctl reload caddy
