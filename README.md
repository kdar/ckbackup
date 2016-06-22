ckbackup
========

Simple wrapping over Borg that does a backup, retention, and consistency checks.
Made to be used with the window's task scheduler. Supports Windows shadow volumes (which borg
by default doesn't).

Only tested in Windows 7.

Notes
=====

This was made more for personal use, but feel free to submit an issue or PR.

Installation
============

Make sure you're compiling with MSYS2 and don't have the gcc package installed. You only
need "mingw-w64-x86_64-toolchain".
