ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
TARGET_DIR := $(ROOT_DIR)/target/wasm32-unknown-unknown/release
BUILD_DIR := $(ROOT_DIR)/target/build
DIST_DIR := $(ROOT_DIR)/target/dist

.PHONY: build
build: contract web_client

# ------------
# Docker
# ------------

DOCKER_COMPOSE := docker-compose -f docker/docker-compose.yml
CLEOS := $(DOCKER_COMPOSE) exec keosd cleos --url http://nodeosd:8888 --wallet-url http://127.0.0.1:8900
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3

.PHONY: docker-down
docker-down:
	$(DOCKER_COMPOSE) down
	docker volume rm -f nodeos-data-volume
	docker volume rm -f keosd-data-volume

.PHONY: docker
docker-up: docker-down $(CONTRACT_DIST_DIR)
	docker volume create --name=nodeos-data-volume
	docker volume create --name=keosd-data-volume
	$(DOCKER_COMPOSE) up

.PHONY: docker-init
docker-init:
	$(CLEOS) wallet create --to-console
	$(CLEOS) wallet import --private-key $(PRIVKEY)
	$(CLEOS) create account eosio urlshortener $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio alice $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio bob $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio carol $(PUBKEY) $(PUBKEY)

# ------------
# Rust/WASM
# ------------

$(TARGET_DIR)/%.wasm: %/**/* %/*
	RUSTFLAGS="-C link-args=-zstack-size=48000" \
	cargo +nightly-2018-11-26 build -vv --release --target=wasm32-unknown-unknown --package $*

%_gc.wasm: %.wasm
	wasm-gc $*.wasm $*_gc.wasm

%_opt.wasm: %.wasm
	wasm-opt --fuzz-exec --output $*_opt.wasm -Oz $*.wasm

%_bg.wasm: %.wasm
	wasm-bindgen $*.wasm --out-dir $(dir $*)

%.wat: %.wasm
	wasm2wat $*.wasm -o $*.wat --generate-names


# ------------
# Contract
# ------------

CONTRACT_DIST_DIR := $(DIST_DIR)/contract
CONTRACT_SRC_DIR := $(ROOT_DIR)/contract

$(CONTRACT_DIST_DIR):
	mkdir -p $(CONTRACT_DIST_DIR)

$(CONTRACT_DIST_DIR)/contract.wasm: \
	$(TARGET_DIR)/contract.wasm \
	$(TARGET_DIR)/contract_gc.wasm \
	$(TARGET_DIR)/contract_gc_opt.wasm \
	$(TARGET_DIR)/contract_gc_opt.wat \
	$(CONTRACT_DIST_DIR)
	cp $(TARGET_DIR)/contract_gc_opt.wasm $(CONTRACT_DIST_DIR)/contract.wasm

$(CONTRACT_DIST_DIR)/contract.json: $(CONTRACT_DIST_DIR)
	cp $(CONTRACT_SRC_DIR)/contract.json $(CONTRACT_DIST_DIR)

.PHONY: contract
contract: $(CONTRACT_DIST_DIR)/contract.wasm $(CONTRACT_DIST_DIR)/contract.json

.PHONY: deploy-contract
deploy-contract: contract
	$(CLEOS) set abi urlshortener mnt/dev/contract/contract.json
	$(CLEOS) set code urlshortener mnt/dev/contract/contract.wasm

.PHONY: shorten
shorten:
	$(CLEOS) push action urlshortener shorten '["test1", "https://www.google.com", "alice"]' -p 'alice@active'
	$(CLEOS) push action urlshortener shorten '["test2", "https://www.reddit.com", "alice"]' -p 'alice@active'
	$(CLEOS) push action urlshortener shorten '["test3", "https://www.eos.io", "alice"]' -p 'alice@active'
	$(CLEOS) push action urlshortener shorten '["test4", "https://www.facebook.com", "alice"]' -p 'alice@active'

.PHONY: unshorten
unshorten:
	$(CLEOS) push action urlshortener unshorten '["test1", "alice"]' -p 'alice@active'
	$(CLEOS) push action urlshortener unshorten '["test2", "alice"]' -p 'alice@active'
	$(CLEOS) push action urlshortener unshorten '["test3", "alice"]' -p 'alice@active'
	$(CLEOS) push action urlshortener unshorten '["test4", "alice"]' -p 'alice@active'

.PHONY: urls
urls:
	$(CLEOS) get table urlshortener urlshortener urls

# ------------
# Web Client
# ------------

export NODE_ENV = production
WEB_CLIENT_DIR := $(ROOT_DIR)/web_client
export WEB_CLIENT_BUILD_DIR = $(WEB_CLIENT_DIR)/build
export WEB_CLIENT_DIST_DIR = $(WEB_CLIENT_DIR)/dist

.PHONY: web_client_wasm
web_client_wasm: \
	$(TARGET_DIR)/web_client.wasm \
	$(TARGET_DIR)/web_client_bg.wasm \
	$(TARGET_DIR)/web_client_bg_gc.wasm \
	$(TARGET_DIR)/web_client_bg_gc_opt.wasm \
	$(TARGET_DIR)/web_client_bg_gc_opt.wat
	mkdir -p $(WEB_CLIENT_BUILD_DIR)
	cp $(TARGET_DIR)/web_client_bg_gc_opt.wasm $(WEB_CLIENT_BUILD_DIR)/web_client_bg.wasm
	cp $(TARGET_DIR)/web_client.js $(WEB_CLIENT_BUILD_DIR)

.PHONY: clean-web_client
clean-web_client:
	rm -Rf $(WEB_CLIENT_DIST_DIR) $(WEB_CLIENT_BUILD_DIR)

.PHONY: web_client
web_client: \
	clean-web_client \
	web_client_wasm
	mkdir -p $(WEB_CLIENT_DIST_DIR)
	cd $(WEB_CLIENT_DIR) && yarn build

.PHONY: start-web_client
start-web_client: \
	clean-web_client \
	web_client_wasm
	cd $(WEB_CLIENT_DIR) && NODE_ENV=development yarn start
