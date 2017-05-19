#include <fcntl.h>
#include <stdio.h>
#include <windows.h>

typedef struct node_tag {
	char buffer[1024];
	int capacity;
	struct node_tag *next;
} node;

int wmain() {
	_setmode(_fileno(stdin), _O_U8TEXT);

	node *bufferList = malloc(sizeof(node));
	if (bufferList == NULL) {
		wprintf(L"Could not allocate memory for reading stdin.\n");
		return 1;
	}
	bufferList->capacity = 0;
	node *head = bufferList;

	DWORD cbSize = 0;
	DWORD count;
	while ((count = fread(head->buffer, 1, sizeof(head->buffer), stdin)) != 0) {
		cbSize += count;
		head->capacity = count;
		head->next = malloc(sizeof(node));
		if (head->next == NULL) {
			wprintf(L"Could not allocate memory for reading stdin.\n");
			return 1;
		}
		head->next->capacity = 0;
		head = head->next;
	}

	if (ferror(stdin)) {
		fwprintf(stderr, L"Reading stdin failed.\r\n");
		return 1;
	}

	HANDLE hgMem = GlobalAlloc(GMEM_MOVEABLE, cbSize + 1);
	LPBYTE lpData = GlobalLock(hgMem);

	cbSize = 0;
	head = bufferList;
	while (head->capacity != 0) {
		CopyMemory(lpData + cbSize, head->buffer, head->capacity);
		cbSize += head->capacity;
		node *prev = head;
		head = head->next;
		free(prev);
	}

	lpData[cbSize] = 0;

	GlobalUnlock(hgMem);
	OpenClipboard(NULL);
	EmptyClipboard();
	SetClipboardData(CF_UNICODETEXT, hgMem);
	CloseClipboard();
	GlobalFree(hgMem);
	return 0;
}
