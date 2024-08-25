#!/bin/sh

WORK_DIR=`mktemp -d`

plugged="$WORK_DIR/plugged.txt"
unplugged="$WORK_DIR/plugged.txt"

echo "Detect device changes";
echo "Please plug-in your Arduino board";
read -p "Press ENTER once your board is plugged...";
ls -lha /dev/tty* > "$plugged"
echo "\n\n"

echo "Please unplug your Arduino board..."
read -p "Press ENTER once your board is unplugged...";
ls -lha /dev/tty* > "$unplugged"
echo "\n\n"

if cmp -s "$plugged" "$unplugged"; then
  echo "No changes detected"
else
  diff "$plugged" "$unplugged"
fi
