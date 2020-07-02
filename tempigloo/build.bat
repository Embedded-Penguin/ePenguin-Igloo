@echo off
setlocal

REM Script assumes tools directory is in your path.
REM For now, this is done by running shell.bat inside of tools directory

set CommonCompilerFlags=-I../libcommon/src -I../tests/src -DBUILD_WIN32=1 -D_CRT_SECURE_NO_WARNINGS -fp:fast -fp:except- -nologo -Gm- -GR- -EHsc- -EHa- -Od -Oi -WX -W3 -FC -Zi -GS- -Gs9999999
set CommonLinkerFlags= -nologo -STACK:0x100000,0x100000 -incremental:no -opt:ref -nodefaultlib libcmt.lib libvcruntime.lib libucrt.lib user32.lib gdi32.lib winmm.lib kernel32.lib shlwapi.lib

set BuildDir=%~dp0build

REM set CompileCommand=clang-cl
set CompileCommand=cl

IF NOT EXIST tools\btime.exe (
    pushd tools\build-timer
    call build.bat
    popd
)

IF NOT EXIST %BuildDir% mkdir %BuildDir%
pushd %BuildDir%
del *.pdb > NUL 2> NUL
del *.map > NUL 2> NUL

..\tools\btime.exe --begin ePenguin-Software-Framework.aet

%CompileCommand% %CommonCompilerFlags% -LD ../libcommon/src/libcommon.cpp -Folibcommon.obj -Fmlibcommon.map /link %CommonLinkerFlags% -noentry
lib -nologo libcommon.obj

%CompileCommand% %CommonCompilerFlags% ../tests/src/tests.cpp -Fotests.obj -Fmtests.map /link %CommonLinkerFlags% libcommon.lib

robocopy . ..\ libcommon.lib > NUL 2> NUL
del ..\test > NUL 2> NUL
copy tests.exe ..\test > NUL 2> NUL

..\tools\btime.exe --end ePenguin-Software-Framework.aet
popd