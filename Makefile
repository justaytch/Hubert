.PHONY: all clean ./versions

all: ./run ./output/fluxerbot ./output/discordbot ./src/typescript/index.js

DISCORD_COMMANDS := $(wildcard ./src/rust/commands/*.rs)
S := "----------"

./run: ./src/c/main.c ./src/c/tomlc17.c
	mkdir -p ./output
	gcc -std=c99 -I ./src/c ./src/c/main.c ./src/c/tomlc17.c -o run

./output/fluxerbot: ./src/go/go.mod ./src/go/go.sum ./src/go/main.go ./src/go/structs.go ./src/go/workarounds.go
	mkdir -p ./output
	go build -C ./src/go -o ../../output/fluxerbot .

./output/discordbot: ./src/rust/main.rs ./src/rust/structs.rs ./src/rust/Cargo.toml $(DISCORD_COMMANDS)
	mkdir -p ./output
	cargo build --release --manifest-path ./src/rust/Cargo.toml
	cp ./src/rust/target/release/discordbot ./output/discordbot

./src/typescript/index.js: ./src/typescript/index.ts ./src/typescript/structs.ts ./src/typescript/package.json ./src/typescript/tsconfig.json
	@(cd ./src/typescript/ && npm install)
	@(cd ./src/typescript/ && npx tsc)

./versions:
	echo "$S GCC VERSION $S" > $@
	gcc -v 2>> $@
	echo "$S RUST TOOLCHAIN VERSIONS $S" >> $@
	rustup --version >> $@ 2> /dev/null
	rustc --version >> $@
	cargo --version >> $@
	echo "$S GO VERSION $S" >> $@
	go version >> $@
	echo "$S NODE VERSION $S" >> $@
	@{ printf "node.js "; node -v; } >> $@
	echo "$S TYPESCRIPT VERSION $S" >> $@
	@(cd ./src/typescript && { printf "tsc "; npx tsc -v; }) >> $@ 2> /dev/null
	echo "$S ZIG VERSION $S" >> $@
	@{ printf "Zig "; zig version; } >> $@


clean:
	rm -rf ./output
	rm -rf ./src/rust/target
	rm -f ./run
	rm -rf ./src/typescript/node_modules ./src/typescript/index.js ./src/typescript/structs.js
