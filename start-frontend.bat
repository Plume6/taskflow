@echo off
cd /d %~dp0
echo 🚀 Starting RustyExpress Frontend...
echo 📍 访问地址: http://localhost:5500/index.html
echo.
echo 请确保后端服务已启动在 http://localhost:8080
echo.
npx http-server . -p 5500 -o
pause