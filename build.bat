@echo off

windres --codepage=65001 copy-meta.rc copy-meta.o
gcc copy.c copy-meta.o -municode -O2 -s -o copy.exe -std=c99
del copy-meta.o

windres --codepage=65001 paste-meta.rc paste-meta.o
gcc paste.c paste-meta.o -municode -O2 -s -o paste.exe -std=c99
del paste-meta.o
