#!/bin/bash

DLLS=$(ldd dist/ckbackup.exe |grep -iv ConEmu |grep -iv system32 |cut -d= -f2 |cut -d' ' -f 2)
for i in ${DLLS}; do
  cp -un "$i" dist/$(basename "$i" | tr '[:upper:]' '[:lower:]') 2>/dev/null;
done

exit 0
