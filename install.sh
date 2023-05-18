#!/bin/bash

#Require root
if [ $EUID != 0 ]; then
    sudo "$0" "$@"
    exit $?
fi

NAME="jetson-fan-ctl-rust"
BIN_PATH="/usr/local/bin/$NAME"
CONF_PATH="/etc/$NAME"

echo "Installing binary..."
rm -r $BIN_PATH 2>/dev/null
mkdir -p $BIN_PATH
cp $NAME $BIN_PATH
echo "Done."

echo "Installing config..."
rm -r $CONF_PATH 2>/dev/null
mkdir -p $CONF_PATH
cp config.ini $CONF_PATH
echo "Done."

echo "Adding service to /lib/systemd/system/..."
cp "$NAME.service" /lib/systemd/system/
chmod 644 "/lib/systemd/system/$NAME.service"
echo "Done."

echo "Starting and enabling service..."
systemctl daemon-reload
systemctl start $NAME
systemctl enable $NAME
echo "Done."

echo "$NAME installed sucessfully!"
echo ""
echo "To configure, edit /etc/$NAME/config.ini (needs sudo)"