#!/bin/bash
set -e

echo "========================================"
echo "   RustyExpress Docker 一键部署脚本"
echo "========================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check Docker
if ! command -v docker &> /dev/null; then
    echo -e "${RED}[错误] 未检测到 Docker${NC}"
    echo "请先安装 Docker: https://docs.docker.com/get-docker/"
    exit 1
fi

# Check Docker Compose
if ! docker compose version &> /dev/null; then
    echo -e "${RED}[错误] Docker Compose 不可用${NC}"
    exit 1
fi

# Check .env file
if [ ! -f .env ]; then
    echo -e "${YELLOW}[提示] 未发现 .env 文件，正在从模板创建...${NC}"
    cp env.docker.example .env
    echo -e "${GREEN}[提示] 已创建 .env 文件${NC}"
    echo -e "${YELLOW}如果无法连接 Docker Hub，请编辑 .env 设置 REGISTRY_MIRROR${NC}"
    echo -e "${YELLOW}  例如：REGISTRY_MIRROR=docker.m.daocloud.io/${NC}"
    echo -e "${YELLOW}按 Enter 以默认配置继续，或 Ctrl+C 取消后先编辑 .env${NC}"
    read -r
fi

echo ""
echo -e "${GREEN}[1/3] 构建 Docker 镜像...${NC}"
docker compose build
echo -e "${GREEN}[完成] 镜像构建成功${NC}"
echo ""

echo -e "${GREEN}[2/3] 启动所有服务...${NC}"
docker compose up -d
echo -e "${GREEN}[完成] 服务启动成功${NC}"
echo ""

echo -e "${GREEN}[3/3] 验证服务状态...${NC}"
docker compose ps
echo ""

echo "========================================"
echo -e "${GREEN}   部署完成！${NC}"
echo "========================================"
echo ""
echo "  前端页面:   http://localhost"
echo "  健康检查:   http://localhost/health"
echo "  后端 API:   http://localhost/api/v1/"
echo ""
echo "  查看日志:   docker compose logs -f"
echo "  停止服务:   docker compose down"
echo ""
