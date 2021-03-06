UID=$(shell id -u)
GID=$(shell id -g)
PORT ?= 8888

run-jupyter:
	sudo docker run --user root -e GEN_CERT=yes -e NB_UID=${UID} --rm -p ${PORT}:8888 \
		-v "${PWD}/reports/":/home/jovyan/reports/ \
		-v "${PWD}/../workspace/reports/":/home/jovyan/data/ \
		jupyter/scipy-notebook

build-query-docker-container:
	sudo docker build \
		--build-arg UID=${UID} \
		--build-arg GID=${GID} \
		-t rust-corpus \
		-f Dockerfile \
		.

run-query-docker-container:
	mkdir -p ${PWD}/../rustup/rustup
	mkdir -p ${PWD}/../rustup/cargo
	sudo docker run -ti \
		-v ${PWD}/../workspace:/home/developer/workspace \
		-v ${PWD}/:/home/developer/qrates \
		-v ${PWD}/../rustup/rustup:/home/developer/.rustup \
		-v ${PWD}/../rustup/cargo:/home/developer/.cargo \
		-u ${UID}:${GID} \
		rust-corpus /bin/bash
