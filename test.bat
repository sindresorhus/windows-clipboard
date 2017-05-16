echo Hello World | copy.exe
paste.exe > temp.txt
set /p VAR=<temp.txt

if "%VAR%" NEQ "Hello World" (
	echo "<%VAR%> is not equal <Hello World>"
	EXIT /B 1
)
