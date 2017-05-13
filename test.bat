:: Test small string
echo unicorn| copy.exe
paste.exe > temp.txt
set /p VAR=<temp.txt

if "%VAR%" NEQ "unicorn" (
	echo "<%VAR%> is not equal <unicorn>"
	EXIT /B 1
)


:: Test big string
:: Create file with 2 bytes
echo.>big.txt

:: Expand to 1 KB
for /L %%i in (1, 1, 9) do type big.txt>>big.txt

:: Expand to 1 MB
for /L %%i in (1, 1, 10) do type big.txt>>big.txt

set /p BIG=<big.txt
copy.exe < big.txt
paste.exe > temp.txt
set /p VAR=<temp.txt

fc /b big.txt temp.txt > nul
if errorlevel 1 (
	echo "big.txt content is not equal after paste"
	EXIT /B 1
)
