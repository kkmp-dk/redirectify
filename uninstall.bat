@echo off
del "C:\Program Files\Redirectify\redirectify.exe"
reg delete "HKEY_CURRENT_USER\Software\Classes\http" /f
reg delete "HKEY_CURRENT_USER\Software\Classes\https" /f
reg delete "HKEY_CURRENT_USER\Software\Clients\StartMenuInternet\Redirectify" /f
echo Uninstallation complete.
pause
