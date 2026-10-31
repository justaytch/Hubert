.PHONY: all c go rust

all: c go rust

c: ./config.toml ./src/c/main.c ./src/c/tomlc17.c
	gcc -std=c99 -I./src/c ./src/c/main.c ./src/c/tomlc17.c -o run

go: ./config.toml ./src/go/go.mod ./src/go/go.sum ./src/go/main.go ./src/go/structs.go ./src/go/workarounds.go
	go build -C ./src/go -o ../../output/fluxerbot .

rust: ./config.toml ./src/rust/main.rs ./src/rust/structs.rs ./src/rust/Cargo.toml ./src/rust/commands/
	cargo build --release --manifest-path ./src/rust/Cargo.toml
	cp ./src/rust/target/release/discordbot ./output/discordbot
