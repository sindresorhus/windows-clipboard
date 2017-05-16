copy.exe unicorn
paste.exe > temp.txt
set /p VAR=<temp.txt

if "%VAR%" NEQ "unicorn" (
	echo "<%VAR%> is not equal <unicorn>"
	EXIT /B 1
)
