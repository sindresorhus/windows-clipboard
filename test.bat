@echo off

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

copy.exe < big.txt
paste.exe > temp.txt
set /p VAR=<temp.txt

fc /b big.txt temp.txt > nul
if errorlevel 1 (
	echo "big.txt content is not equal after paste"
	EXIT /B 1
)

:: Test multiline string
echo Hello>multiline.txt
echo Lines>>multiline.txt

copy.exe < multiline.txt
paste.exe > temp.txt
set /p VAR=<temp.txt

fc /b multiline.txt temp.txt > nul
if errorlevel 1 (
	echo "multiline.txt content is not equal after paste"
	EXIT /B 1
)
