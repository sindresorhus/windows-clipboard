#include <stdio.h>
#include <windows.h>
#include <fcntl.h>

int wmain() {
	OpenClipboard(NULL);

	HANDLE hData = GetClipboardData(CF_UNICODETEXT);
	if (hData) {
		wchar_t *pText = (wchar_t *) GlobalLock(hData);
		if (pText) {
			_setmode(_fileno(stdout), _O_U8TEXT);
			fputws(pText, stdout);
			GlobalUnlock(hData);
		}
	}

	CloseClipboard();
	return 0;
}
