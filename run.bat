@echo off
REM ==================================================
REM  AeroFlow Runner - Easy Launch Script
REM ==================================================

echo.
echo ====================================
echo   AEROFLOW PROGRAM RUNNER
echo ====================================
echo.

REM Check if we're in the right folder
if not exist "target\debug\aeroflow-cli.exe" (
    echo ERROR: aeroflow-cli.exe not found!
    echo.
    echo Please run this script from the Aeroflow folder:
    echo C:\Users\suman\Downloads\Aeroflow
    echo.
    pause
    exit /b 1
)

REM Check if a file was provided
if "%~1"=="" (
    echo Usage: run.bat YOUR-FILE.aefl
    echo.
    echo Example: run.bat hello.aefl
    echo.
    pause
    exit /b 1
)

REM Run the program
echo Running: %~1
echo.
.\target\debug\aeroflow-cli.exe run --source %~1

echo.
echo ====================================
echo   PROGRAM FINISHED
echo ====================================
pause
