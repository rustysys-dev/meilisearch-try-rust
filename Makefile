server-up-test:
	podman run --rm --pod new:meili-rust \
		--name meilisearch-rust -p 7701:7700 \
		-v ${PWD}/data.ms:/data.ms:Z \
		getmeili/meilisearch

server-up:
	podman run -d --pod new:meili-rust \
		--name meilisearch-rust -p 7701:7700 \
		-v ${PWD}/data.ms:/data.ms:Z \
		getmeili/meilisearch

server-down:
	podman pod rm meili-rust --force
	rm -rf data.ms/*

index:
	cargo run --bin index
