#!/bin/sh
function t() {
  OUTPUT=`tele $@`
  if [ $? -eq 2 ]
    then cd "$OUTPUT"
    else echo "$OUTPUT"
  fi
}
