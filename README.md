# 🦀 RustyExpress - 高性能任务管理系统

基于 **Rust + Actix Web** 构建的高性能任务管理后端，搭配 **PostgreSQL** 数据库和 **Nginx** 反向代理，通过 **Docker** 一键部署。

## 📦 技术栈

| 层级 | 技术 |
|------|------|
| **后端** | Rust, Actix Web, SQLx ORM |
| **数据库** | PostgreSQL 16 (Alpine) |
| **前端** | 原生 HTML/CSS/JavaScript（SPA） |
| **反向代理** | Nginx (Alpine) |
| **容器化** | Docker, Docker Compose |
| **认证** | JWT (jsonwebtoken) + Argon2 密码哈希 |

---

## 📋 前提条件

| 软件 | 说明 |
|------|------|
| [Docker Desktop](https://www.docker.com/products/docker-desktop/) | Windows/Mac 安装即可同时获得 Docker 和 Docker Compose |
| Docker Engine + Compose | Linux 需分别安装 |
| Git（可选） | 用于克隆代码 |

---

## 🚀 一键部署

### Windows

```powershell
# 双击 deploy.bat 或命令行执行
.\deploy.bat
```

### Linux / macOS

```bash
chmod +x deploy.sh && ./deploy.sh
```

### 手动部署

```bash
# 1. 克隆项目
git clone https://github.com/Plume6/rustyexpress rustyexpress
cd rustyexpress

# 2. 复制环境配置模板
cp env.docker.example .env

# 3. ⚠️ 编辑 .env，将 JWT_SECRET 改为随机字符串（生产环境必须）
#    Windows: notepad .env
#    Linux:   vim .env

# 4. 构建并启动
docker compose up -d --build

# 5. 查看启动状态
docker compose ps

# 6. 查看日志确认一切正常
docker compose logs -f
```

### Makefile 快捷命令

```bash
make build          # 构建镜像
make up             # 启动服务
make down           # 停止并删除容器
make logs           # 查看日志
make ps             # 查看状态
make rebuild        # 重新构建并启动（代码更新后使用）
make clean          # 完全清除（包括数据卷）
make db-shell       # 进入数据库
```

---

## ✅ 验证部署

### 检查容器状态

```bash
docker compose ps
```

期望输出（3 个容器都在运行）：
```
NAME                   IMAGE                    STATUS
rustyexpress-db        postgres:16-alpine       Up (healthy)
rustyexpress-backend   taskflow-main-backend    Up
rustyexpress-frontend  taskflow-main-frontend   Up
```

### 访问服务

| 服务 | 地址 | 说明 |
|------|------|------|
| **前端页面** | http://服务器IP | 浏览器直接打开使用 |
| **健康检查** | http://服务器IP/health | 返回服务状态 |
| **后端 API** | http://服务器IP/api/v1/ | RESTful 接口 |

---

## 📖 使用指南

### 1. 注册账号

打开 http://服务器IP ，进入**注册**页面：

- 填写**用户名、邮箱、密码**（至少 6 位）
- 点击「注册」
- 成功后自动切换到登录页

> **演示账号**（数据库初始化时自动创建）：
> - 邮箱：`test@test.com`
> - 密码：`123456`

### 2. 登录

使用邮箱和密码登录。

### 3. 管理任务

| 操作 | 说明 |
|------|------|
| **创建任务** | 填写标题和描述，点击「创建任务」 |
| **开始任务** | 点击「开始」，状态变为"进行中" |
| **完成任务** | 点击「完成」，状态变为"已完成" |
| **删除任务** | 点击「删除」（不可恢复） |

顶部统计栏实时显示各状态的任务数量。

---

## ⚙️ 配置说明（.env）

```ini
# Docker image registry mirror
# 国内用户如果无法连接 Docker Hub，可设置镜像加速器
# 常用镜像：docker.m.daocloud.io/（DaoCloud 推荐）
REGISTRY_MIRROR=

# Server
SERVER_HOST=0.0.0.0                     # 监听地址（Docker 内用 0.0.0.0）
SERVER_PORT=8080                        # 后端端口

# Database
DATABASE_URL=postgres://用户名:密码@postgres:5432/数据库名  # 主机名必须是 postgres
DB_MAX_CONNECTIONS=10                   # 数据库连接池大小

# JWT
JWT_SECRET=修改为随机字符串              # ⚠️ 生产环境必须修改！
JWT_EXPIRATION_HOURS=168                # Token 过期时间（7 天）

# Docker Compose 配置
POSTGRES_USER=rustyexpress              # 数据库用户名（需与 DATABASE_URL 一致）
POSTGRES_PASSWORD=rustyexpress_secret   # ⚠️ 生产环境请修改
POSTGRES_DB=rustyexpress                # 数据库名
POSTGRES_PORT=5432                      # 数据库端口
RUST_LOG=info                           # 日志级别: error/warn/info/debug
FRONTEND_PORT=80                        # 前端页面端口（被占用时修改）
```

---

## 🛠️ 日常管理

### 查看日志

```bash
docker compose logs -f          # 所有服务
docker compose logs -f backend  # 仅后端
```

### 重启 / 更新

```bash
# 重启服务
docker compose restart backend

# 代码更新后重新构建
docker compose up -d --build

# 完全重新构建（不使用缓存）
docker compose build --no-cache
docker compose up -d
```

### 数据库操作

```bash
# 进入数据库
docker compose exec postgres psql -U rustyexpress -d rustyexpress

# 查看用户
docker compose exec postgres psql -U rustyexpress -d rustyexpress \
  -c "SELECT id, email, username FROM users;"

# 查看任务
docker compose exec postgres psql -U rustyexpress -d rustyexpress \
  -c "SELECT id, title, status FROM tasks;"
```

### 备份与恢复

```bash
# 备份
docker compose exec -T postgres pg_dump -U rustyexpress rustyexpress > backup_$(date +%Y%m%d).sql

# 恢复
cat backup.sql | docker compose exec -T postgres psql -U rustyexpress -d rustyexpress
```

---

## 🚨 故障排查

### 端口被占用

```powershell
netstat -ano | findstr :80       # 查看谁占了 80 端口
# 修改 .env 中的 FRONTEND_PORT=8081，然后 docker compose down && docker compose up -d
```

### 后端不断重启

```bash
docker compose logs backend
```

### 完全重置

```bash
# ⚠️ 会删除数据库所有数据！
docker compose down -v
docker compose up -d
```

---

## 📂 项目结构

```
rustyexpress/
├── Dockerfile              # 多阶段构建
├── docker-compose.yml      # 服务编排
├── nginx.conf              # Nginx 反向代理配置
├── init.sql                # 数据库初始化
├── deploy.bat              # Windows 部署脚本
├── deploy.sh               # Linux/Mac 部署脚本
├── Makefile                # 快捷命令
├── .env                    # 环境配置（不提交 Git）
├── env.docker.example      # 环境配置模板
├── src/                    # Rust 后端源码
│   ├── main.rs             # 入口，路由注册
│   ├── config.rs           # 配置加载
│   ├── db.rs               # 数据库连接池
│   ├── handlers/           # API 处理（auth、task、health）
│   ├── middleware/          # JWT 认证中间件
│   ├── services/           # 业务逻辑
│   ├── models.rs           # 数据模型
│   └── error.rs            # 错误处理
├── frontend/               # 前端静态文件
│   ├── index.html          # 主页面
│   ├── css/style.css       # 样式
│   └── js/app.js           # 前端逻辑
└── .sqlx/                  # SQLx 编译时检查
```

---

## 📄 API 参考

### 公开接口

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/health` | 健康检查 |
| POST | `/api/v1/auth/register` | 注册 |
| POST | `/api/v1/auth/login` | 登录 |

### 需认证（Header: `Authorization: Bearer <token>`）

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/tasks` | 任务列表 |
| POST | `/api/v1/tasks` | 创建任务 |
| GET | `/api/v1/tasks/stats` | 任务统计 |
| GET | `/api/v1/tasks/{id}` | 任务详情 |
| PUT | `/api/v1/tasks/{id}` | 更新任务 |
| DELETE | `/api/v1/tasks/{id}` | 删除任务 |

---

## 🔧 本地开发（非 Docker）

```bash
# 1. 安装 Rust，本地安装 PostgreSQL
# 2. cp env.docker.example .env
# 3. 修改 .env 中 DATABASE_URL 为 localhost 地址
# 4. cargo run
```

开发环境下如使用 VS Code Live Server，在 `.env` 中添加：

```ini
CORS_ALLOWED_ORIGINS=http://localhost:5500,http://127.0.0.1:5500
```
