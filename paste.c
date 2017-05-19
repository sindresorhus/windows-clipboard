#include <stdio.h>
#include <windows.h>
#include <fcntl.h>

int wmain() {
	OpenClipboard(NULL);

	HANDLE clip = GetClipboardData(CF_UNICODETEXT);
	if (clip) {
		_setmode(_fileno(stdout), _O_U8TEXT);
		fputws(clip, stdout);
	}

	CloseClipboard();
	return 0;
}
