#!/bin/bash

#Require root
if [ $EUID != 0 ]; then
    sudo "$0" "$@"
    exit $?
fi

NAME="jetson-fan-ctl-rust"
BIN_PATH="/usr/local/bin/$NAME"
CONF_PATH="/etc/$NAME"

echo "Disabling service..."
systemctl stop $NAME
systemctl disable $NAME
echo "Done."

echo "Uninstalling binary..."
rm -r $BIN_PATH 2>/dev/null
echo "Done."

echo "Purging config..."
rm -r $CONF_PATH 2>/dev/null
echo "Done."

echo "Uninstalling service in /lib/systemd/system/..."
rm "/lib/systemd/system/$NAME.service"
echo "Done."

echo "Reloading daemon..."
systemctl daemon-reload
echo "Done."

echo "$NAME uninstalled sucessfully."
echo "To restore NVIDIA-default fan behavior, please reboot now."