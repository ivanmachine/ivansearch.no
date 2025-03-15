deploy_frontend:
	DOCKER_DEFAULT_PLATFORM=linux/amd64 docker build -t ivanmachine/ivansearch_frontend:latest -f frontend/Dockerfile ./frontend
	docker push ivanmachine/ivansearch_frontend:latest
	git push && git push origin main:deploy_frontend -f

run_qdrant:
	docker rm -f qdrant || true
	docker run -d \
		--name qdrant \
		-p 6333:6333 \
		-v ~/projects/ivansearch/qdrant/storage:/qdrant/storage \
		qdrant/qdrant

run_pocketbase:
	./db/pocketbase serve --dir ./db/pb_data