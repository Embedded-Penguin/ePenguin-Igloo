@echo off
REM -Zi for debugging
cl -nologo -DBUILD_WIN32=1 -FC -O2 -TC -EHsc btime.c /link
@if ERRORLEVEL 1 exit /B 1
robocopy . ../ btime.exe > nul