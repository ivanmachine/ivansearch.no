deploy_frontend:
	docker build --platform linux/amd64 -t ivanmachine/ivansearch_frontend:latest -f frontend/Dockerfile ./frontend
	docker push ivanmachine/ivansearch_frontend:latest
	git push && git push origin main:deploy_frontend -f

deploy_backend:
	docker buildx build --build-arg TARGETARCH=x86_64 --platform linux/amd64 -t ivanmachine/ivansearch_backend:latest -f backend/Dockerfile ./backend --push
	git push && git push origin main:deploy_backend -f

build_backend_local:
	DOCKER_DEFAULT_PLATFORM=linux/arm64/v8 docker build -t ivanmachine/ivansearch_backend:latest -f backend/Dockerfile ./backend

run_backend:
	docker rm -f ivansearch_backend || true
	docker run --name ivansearch_backend -p 4200:8000 ivanmachine/ivansearch_backend:latest

inspect_backend:
	docker rm -f ivansearch_backend || true
	docker run --rm -it --entrypoint="/bin/sh" ivanmachine/ivansearch_backend:latest

run_qdrant:
	docker rm -f qdrant || true
	docker run -d \
		--name qdrant \
		-p 6333:6333 \
		-v ~/projects/ivansearch/qdrant/storage:/qdrant/storage \
		qdrant/qdrant

run_pocketbase:
	./db/pocketbase serve --dir ./db/pb_data