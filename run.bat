@echo off
setlocal enabledelayedexpansion

:: Verifica se o Rust está instalado
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo Erro: Rust não está instalado. Instale primeiro:
    echo https://win.rustup.rs/
    exit /b 1
)

:: Configurações
set TARGET=%1
if "%TARGET%"=="" set TARGET=release
set EXAMPLE=%2
if "%EXAMPLE%"=="" set EXAMPLE=hello
set OUTPUT_DIR=bin
set STDLIB_DIR=stdlib

:: Função para compilar
:compile
echo 🛠  Compilando %EXAMPLE%.ml...

:: Compila o compilador se necessário
if not exist ".\target\%TARGET%\minilang.exe" (
    echo 🔨 Construindo o compilador MiniLang...
    cargo build --%TARGET%
)

:: Cria diretório de saída
if not exist "%OUTPUT_DIR%" mkdir "%OUTPUT_DIR%"

:: Compila o exemplo
".\target\%TARGET%\minilang.exe" build ".\examples\%EXAMPLE%.ml" -o "%OUTPUT_DIR%\%EXAMPLE%.exe"

:: Copia a stdlib
xcopy /E /I "%STDLIB_DIR%" "%OUTPUT_DIR%\%STDLIB_DIR%\" >nul

echo ✅ Compilação concluída! Executável em: %OUTPUT_DIR%\%EXAMPLE%.exe
goto :eof

:: Função para executar
:run
echo 🚀 Executando %EXAMPLE%...
".\%OUTPUT_DIR%\%EXAMPLE%.exe"
goto :eof

:: Menu principal
if "%1"=="build" (
    call :compile
) else if "%1"=="run" (
    call :compile
    call :run
) else if "%1"=="clean" (
    echo 🧹 Limpando...
    cargo clean
    rmdir /S /Q "%OUTPUT_DIR%"
) else (
    echo Uso: %0 [build^|run^|clean] [exemplo]
    echo Exemplos disponíveis:
    dir /B examples\*.ml | findstr /V /R "\\"
    exit /b 1
)