@echo off
cd /d %~dp0

echo ========================================
echo   RustyExpress Docker Deploy Script
echo ========================================
echo.

:: Check Docker
where docker >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Docker not found. Please install Docker Desktop first.
    echo Download: https://www.docker.com/products/docker-desktop/
    pause
    exit /b 1
)

:: Check Docker Compose
docker compose version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Docker Compose not available.
    pause
    exit /b 1
)

:: Check .env file
if not exist .env (
    echo [INFO] Creating .env file from template...
    copy env.docker.example .env >nul
    if exist .env (
        echo [INFO] .env file created.
        echo [INFO] If you are in China and cannot access Docker Hub,
        echo [INFO] edit .env and set REGISTRY_MIRROR=dao.m.daocloud.io/
        echo [INFO] Press any key to continue...
        pause
    )
)

echo.
echo [1/3] Building Docker images...
docker compose build
if %errorlevel% neq 0 (
    echo [ERROR] Build failed
    pause
    exit /b 1
)
echo [OK] Build successful
echo.

echo [2/3] Starting services...
docker compose up -d
if %errorlevel% neq 0 (
    echo [ERROR] Failed to start services
    pause
    exit /b 1
)
echo [OK] Services started
echo.

echo [3/3] Checking service status...
docker compose ps
echo.

echo ========================================
echo   Deployment Complete!
echo ========================================
echo.
echo   Frontend:  http://localhost
echo   Health:    http://localhost/health
echo   API:       http://localhost/api/v1/
echo.
echo   View logs: docker compose logs -f
echo   Stop:      docker compose down
echo.

pause
