DOCKER_COMPOSE := docker-compose -f docker/docker-compose.yml

.PHONY: docker-down
docker-down:
	$(DOCKER_COMPOSE) down
	docker volume rm -f nodeos-data-volume
	docker volume rm -f keosd-data-volume

.PHONY: docker
docker-up: docker-down
	docker volume create --name=nodeos-data-volume
	docker volume create --name=keosd-data-volume
	$(DOCKER_COMPOSE) up

CLEOS := winpty $(DOCKER_COMPOSE) exec keosd cleos --url http://nodeosd:8888 --wallet-url http://127.0.0.1:8900
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3

.PHONY: docker-init
docker-init:
	$(CLEOS) wallet create --to-console
	$(CLEOS) wallet import --private-key $(PRIVKEY)
	$(CLEOS) create account eosio urlshortener $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio alice $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio bob $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio carol $(PUBKEY) $(PUBKEY)

%_gc.wasm: %.wasm
	wasm-gc $*.wasm $*_gc.wasm

%_opt.wasm: %.wasm
	wasm-opt --fuzz-exec --output $*_opt.wasm -Oz $*.wasm

%.wat: %.wasm
	wasm2wat $*.wasm -o $*.wat --generate-names

target/wasm32-unknown-unknown/release/%.wasm: crates/%/**/* crates/%/*
	docker run --rm rustlang/rust:nightly \
		-v .:/mnt/dev/project:ro \
		cd mnt/dev/project && cargo +nightly build -vv --release --target=wasm32-unknown-unknown --package $*

.PHONY: contract
contract: \
	target/wasm32-unknown-unknown/release/contract.wasm \
	target/wasm32-unknown-unknown/release/contract_gc.wasm \
	target/wasm32-unknown-unknown/release/contract_gc_opt.wasm \
	target/wasm32-unknown-unknown/release/contract_gc_opt.wat

.PHONY: deploy-contract
deploy-contract: contract
	$(CLEOS) set abi urlshortener mnt/dev/contract/contract.abi.json
	$(CLEOS) set code urlshortener mnt/dev/release/contract_gc_opt.wasm

.PHONY: shorten
shorten:
	$(CLEOS) push action urlshortener shorten '["test", "https://www.google.com", "alice"]' -p 'alice@active'

.PHONY: unshorten
unshorten:
	$(CLEOS) push action urlshortener unshorten '["test", "alice"]' -p 'alice@active'

.PHONY: urls
urls:
	$(CLEOS) get table urlshortener urlshortener urls
