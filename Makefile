# RustyExpress Docker Management Commands

.PHONY: help build up down restart logs ps clean rebuild rebuild-no-cache

help: ## Show this help message
	@echo "RustyExpress Docker Management"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}'

build: ## Build all Docker images
	docker compose build

up: ## Start all services in detached mode
	docker compose up -d

down: ## Stop and remove all containers
	docker compose down

restart: ## Restart all services
	docker compose restart

logs: ## View logs of all services (follow mode)
	docker compose logs -f

ps: ## List running containers
	docker compose ps

clean: ## Stop and remove containers, volumes, and images
	docker compose down -v --remove-orphans

rebuild: ## Rebuild and restart services
	docker compose up -d --build

rebuild-no-cache: ## Rebuild without cache and restart
	docker compose build --no-cache
	docker compose up -d

db-shell: ## Open PostgreSQL interactive shell
	docker compose exec postgres psql -U rustyexpress -d rustyexpress

backend-shell: ## Open a shell in the backend container
	docker compose exec backend /bin/sh

frontend-shell: ## Open a shell in the frontend container
	docker compose exec frontend /bin/sh

logs-backend: ## View backend logs only
	docker compose logs -f backend

logs-db: ## View database logs only
	docker compose logs -f postgres

logs-frontend: ## View frontend logs only
	docker compose logs -f frontend

prune: ## Remove all unused Docker resources (caution)
	docker system prune -af --volumes
