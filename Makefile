# Detect OS
ifeq ($(OS),Windows_NT)
    COPY := xcopy /E /I /Y
    REMOVE := rmdir /S /Q
else
    COPY := cp -r
    REMOVE := rm -rf
endif

.PHONY: run-dev build-frontend dev-backend clean

dev-backend: build-frontend dev-backend

build-frontend:
	cd frontend && bun run build

dev-frontend:
	cd frontend && bun run dev

dev-backend: 
	$(REMOVE) backend/static
	$(COPY) frontend/dist backend/static
	cd backend && cargo run

clean:
	$(REMOVE) backend/static
