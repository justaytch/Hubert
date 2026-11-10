package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"regexp"
	"slices"
	"strconv"
	"strings"

	"github.com/go-fluxer/gofluxer"
)

var config Config = getconfig()
var chanid string = strconv.FormatUint(config.Fluxer.ChannelId, 10)

func main() {
	bot := gofluxer.NewBot(config.Fluxer.Token, "!")
	// Replace FLUXERBOTTOKEN with your actual fluxer.app bot token.
	// bot.NewBotInstance("https://example.com/api/v1", "wss://example.com/gateway/?v=1")
	// Use a 3rd party Fluxer instance with NewBotInstance

	bot.OnMessage(func(m *gofluxer.Message) {
		if !m.Author.Bot {
			userid, err := strconv.ParseUint(m.Author.ID, 10, 64)
			if err != nil {
				log.Fatal("Invalid ID")
			}
			if strings.ToLower(m.Content) == "!ping" {
				bot.SendMessage(m.ChannelID, "Pong!")
			}
			if strings.ToLower(m.Content) == "!h" {
				bot.SendMessage(m.ChannelID, "h")
			}
			if strings.ToLower(m.Content) == "!logout" && slices.Contains(config.Fluxer.OwnerIds, userid) {
				bot.SendMessage(m.ChannelID, "Logging Out...")
				os.Exit(0)
			}
			if m.ChannelID == chanid {
				if config.Discord.Enabled || config.Stoat.Enabled {
					re := regexp.MustCompile(`[<][@]!?(\d{17,19})[>]`)
					msgnew := m.Content
					matches := re.FindAllStringSubmatch(m.Content, -1)
					for _, match := range matches {
						msgnew = strings.Replace(msgnew, match[0], fmt.Sprintf("@%s", getusername(match[1], m.GuildID)), 1)
					}
					re = regexp.MustCompile(`[<][#](\d{17,19})[>]`)
					matches = re.FindAllStringSubmatch(m.Content, -1)
					for _, match := range matches {
						msgnew = strings.Replace(msgnew, match[0], fmt.Sprintf("#%s", *getchannel(match[1]).Name), 1)
					}
					re = regexp.MustCompile(`[<][@][&](\d{17,19})[>]`)
					matches = re.FindAllStringSubmatch(m.Content, -1)
					roles := getroles(m.GuildID)
					for _, match := range matches {
						msgnew = strings.Replace(msgnew, match[0], fmt.Sprintf("@%s", getrole(match[1], roles).Name), 1)
					}
					if config.Discord.Enabled {
						var avatar string
						if m.Author.Avatar == "" {
							avatar = config.Fluxer.DefaultAvatar
						} else {
							avatar = fmt.Sprintf("https://fluxerusercontent.com/avatars/%s/%s.webp", m.Author.ID, m.Author.Avatar)
						}
						payload := WebhookPost{
							Username:  fmt.Sprintf("%s (Fluxer User)", getusername(m.Author.ID, m.GuildID)),
							Content:   msgnew,
							AvatarUrl: avatar,
						}
						marshalled, err := json.Marshal(payload)
						if err != nil {
							log.Fatal("Error marshalling webhook payload")
						}
						req, err := http.NewRequest("POST", config.Discord.Webhook, bytes.NewReader(marshalled))
						if err != nil {
							log.Fatal("Could not create webhook post.")
						}
						req.Header.Set("Content-Type", "application/json")
						client := &http.Client{}
						resp, err := client.Do(req)
						if err != nil {
							log.Fatal("Could not post to webhook.")
						}
						defer resp.Body.Close()
					}
					if config.Stoat.Enabled {
						payload := StoatWebhookPost{
							Masquerade: Masquerade{
								Name:   fmt.Sprintf("%s (Fluxer User)", getusername(m.Author.ID, m.GuildID)),
								Avatar: fmt.Sprintf("https://fluxerusercontent.com/avatars/%s/%s.webp", m.Author.ID, m.Author.Avatar),
							},
							Content: msgnew,
						}
						marshalled, err := json.Marshal(payload)
						if err != nil {
							log.Fatal("Error marshalling webhook payload")
						}
						req, err := http.NewRequest("POST", config.Stoat.Webhook, bytes.NewReader(marshalled))
						if err != nil {
							log.Fatal("Could not create webhook post.")
						}
						req.Header.Set("Content-Type", "application/json")
						client := &http.Client{}
						resp, err := client.Do(req)
						if err != nil {
							log.Fatal("Could not post to webhook.")
						}
						defer resp.Body.Close()
					}
				}
			}
		}
	})
	bot.OnReady(func() {
		bot.SetStatus("dnd", &gofluxer.CustomStatus{
			Text: fmt.Sprintf("Watching #%s", *getchannel(strconv.FormatUint(config.Fluxer.ChannelId, 10)).Name),
		})
		fmt.Printf("%s is Connected to Fluxer!\n", config.Name)
	})

	bot.Run()
}
