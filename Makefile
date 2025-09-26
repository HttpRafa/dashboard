# Detect OS (Windows_NT is set on Windows)
ifeq ($(OS),Windows_NT)
    SHELL := cmd
    COPY := xcopy /E /I /Y
    REMOVE := rmdir /S /Q
else
    COPY := cp -r
    REMOVE := rm -rf
endif

.PHONY: all frontend backend clean

all: frontend backend

frontend:
	cd frontend && bun run build

backend: 
	$(REMOVE) backend/static
	$(COPY) frontend/dist backend/static
	cd backend && cargo run

clean:
	$(REMOVE) backend/static
