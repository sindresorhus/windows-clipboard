@echo off

set PATH=C:\MinGW\bin;%PATH%

mkdir Win32

windres --codepage=65001 copy-meta.rc copy-meta.o
gcc copy.c copy-meta.o -municode -O2 -s -o Win32/copy.exe -std=c99
del copy-meta.o

windres --codepage=65001 paste-meta.rc paste-meta.o
gcc paste.c paste-meta.o -municode -O2 -s -o Win32/paste.exe -std=c99
del paste-meta.o

set PATH=C:\mingw-w64\x86_64-6.3.0-posix-seh-rt_v5-rev1\mingw64\bin;%PATH%

mkdir Win64

windres --codepage=65001 copy-meta.rc copy-meta.o
gcc copy.c copy-meta.o -municode -O2 -s -o Win64/copy.exe -std=c99
del copy-meta.o

windres --codepage=65001 paste-meta.rc paste-meta.o
gcc paste.c paste-meta.o -municode -O2 -s -o Win64/paste.exe -std=c99
del paste-meta.o
