package main

import (
	"log"

	"github.com/BurntSushi/toml"
)

type BotConfig struct {
	Enabled       bool
	Token         string
	ChannelId     uint64
	OwnerIds      []uint64
	Webhook       string
	DefaultAvatar string
}

type StoatConfig struct {
	Enabled       bool
	Token         string
	ChannelId     string
	OwnerIds      []string
	Webhook       string
	DefaultAvatar string
}

type Config struct {
	Name    string
	Discord BotConfig
	Fluxer  BotConfig
	Stoat   StoatConfig
}

type WebhookPost struct {
	Username  string `json:"username"`
	Content   string `json:"content"`
	AvatarUrl string `json:"avatar_url"`
}

type Masquerade struct {
	Name   string `json:"name"`
	Avatar string `json:"avatar"`
}

type StoatWebhookPost struct {
	Content    string     `json:"content"`
	Masquerade Masquerade `json:"masquerade"`
}

func getconfig() Config {
	var cfg Config
	if _, err := toml.DecodeFile("config.toml", &cfg); err != nil {
		log.Fatal(err)
	}
	return cfg
}
