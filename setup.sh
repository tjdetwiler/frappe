#!/bin/bash
set -e

IO_DIR="io/hcf/frappe"

if [[ ! -d "$IO_DIR" ]]; then
    echo "creating $IO_DIR"
    mkdir -p "$IO_DIR"
fi

CLASSNAME="HelloWorld.class"
HELLO_WORLD="test-classes/$CLASSNAME"
if [[ ! -e "$IO_DIR/$CLASSNAME" ]]; then
    echo "copying $HELLO_WORLD to $IO_DIR"
    cp "$HELLO_WORLD" "$IO_DIR"
fi

echo "setup complete."
