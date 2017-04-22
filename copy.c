#include <stdio.h>
#include <windows.h>

int wmain(int argc, wchar_t **argv) {
	DWORD cbSize = (wcslen(argv[1]) + 1) * 2;
	HANDLE hgMem = GlobalAlloc(GMEM_MOVEABLE, cbSize);
	LPBYTE lpData = GlobalLock(hgMem);
	CopyMemory(lpData, argv[1], cbSize);
	GlobalUnlock(hgMem);
	OpenClipboard(NULL);
	EmptyClipboard();
	SetClipboardData(CF_UNICODETEXT, hgMem);
	CloseClipboard();
	GlobalFree(hgMem);
	return 0;
}
