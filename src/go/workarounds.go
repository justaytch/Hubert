package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"net/http"
)

type User struct {
	Id            string
	Username      string
	Discriminator string
	GlobalName    string `json:"global_name"`
	Avatar        string
	AvatarColor   int32 `json:"avatar_color"`
	System        bool
	Bot           bool
}

type Member struct {
	User                       User
	Nick                       string
	Avatar                     string
	Banner                     string
	AccentColor                int32 `json:"accent_color"`
	Roles                      []string
	JoinedAt                   string `json:"joined_at"`
	Mute                       bool
	Deaf                       bool
	CommunicationDisabledUntil string `json:"communication_disabled_until"`
}

type Channel struct {
	Id                   string
	GuildId              string `json:"guild_id"`
	Name                 *string
	Topic                *string
	Url                  *string
	Icon                 *string
	OwnerId              *string `json:"owner_id"`
	Type                 int32
	Position             int32
	ParentId             *string `json:"parent_id"`
	Bitrate              *int32
	UserLimit            *int32  `json:"user_limit"`
	VoiceConnectionLimit *int32  `json:"voice_connection_limit"`
	RtcRegion            *string `json:"rtc_region"`
	LastMessageId        *string `json:"last_message_id"`
	LastPinTimestamp     *string `json:"last_pin_timestamp"`
	Nsfw                 bool
	NsfwOverride         *bool `json:"nsfw_override"`
}

type Role struct {
	Id          string
	Name        string
	Color       int32
	Position    int32
	Permissions string
	Mentionable bool
}

func getuser(id string) User {
	req, err := http.NewRequest("GET", fmt.Sprintf("https://web.fluxer.app/api/v1/users/%s", id), nil)
	if err != nil {
		log.Fatal("Could not create REST api request.")
	}
	req.Header.Set("Authorization", fmt.Sprintf("Bot %s", config.Fluxer.Token))
	req.Header.Set("Accept", "application/json")
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Fatal("Could not get user from REST api.")
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatal("Could not get user from REST api.")
	}
	var user User
	err = json.Unmarshal([]byte(body), &user)
	if err != nil {
		log.Fatal("Could not read REST response.")
	}
	return user
}

func getmember(id string, guild string) (Member, error) {
	req, err := http.NewRequest("GET", fmt.Sprintf("https://web.fluxer.app/api/v1/guilds/%s/members/%s", guild, id), nil)
	if err != nil {
		log.Fatal("Could not create REST api request.")
	}
	req.Header.Set("Authorization", fmt.Sprintf("Bot %s", config.Fluxer.Token))
	req.Header.Set("Accept", "application/json")
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Fatal("Could not get user from REST api.")
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatal("Could not get user from REST api.")
	}
	e := []byte(`{"code":"UNKNOWN_MEMBER","message":"Member wasn't found in this community."}`)
	if bytes.Equal(body, e) {
		return Member{}, errors.New("Member not in guild.")
	}
	var member Member
	err = json.Unmarshal([]byte(body), &member)
	if err != nil {
		log.Fatal("Could not read REST response.")
	}
	return member, nil
}

func getusername(id string, guild string) string {
	var res string
	member, err := getmember(id, guild)
	if err != nil {
		user := getuser(id)
		if user.GlobalName == "" {
			res = fmt.Sprintf("%s#%s", user.Username, user.Discriminator)
		} else {
			res = user.GlobalName
		}
	} else {
		if member.Nick == "" {
			if member.User.GlobalName == "" {
				res = fmt.Sprintf("%s#%s", member.User.Username, member.User.Discriminator)
			} else {
				res = member.User.GlobalName
			}
		} else {
			res = member.Nick
		}
	}
	return res
}

func getchannel(id string) Channel {
	req, err := http.NewRequest("GET", fmt.Sprintf("https://web.fluxer.app/api/v1/channels/%s", id), nil)
	if err != nil {
		log.Fatal("Could not create REST api request.")
	}
	req.Header.Set("Authorization", fmt.Sprintf("Bot %s", config.Fluxer.Token))
	req.Header.Set("Accept", "application/json")
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Fatal("Could not get channel from REST api.")
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatal("Could not get channel from REST api.")
	}
	var channel Channel
	err = json.Unmarshal([]byte(body), &channel)
	if err != nil {
		fmt.Printf("%s\n%s\n", body, err)
		log.Fatal("Could not read REST response.")
	}
	return channel
}
func getroles(guild string) []Role {
	req, err := http.NewRequest("GET", fmt.Sprintf("https://web.fluxer.app/api/v1/guilds/%s/roles", guild), nil)
	if err != nil {
		log.Fatal("Could not create REST api request.")
	}
	req.Header.Set("Authorization", fmt.Sprintf("Bot %s", config.Fluxer.Token))
	req.Header.Set("Accept", "application/json")
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Fatal("Could not get roles from REST api.")
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatal("Could not get roles from REST api.")
	}
	var roles []Role
	err = json.Unmarshal([]byte(body), &roles)
	if err != nil {
		fmt.Printf("%s\n%s\n", body, err)
		log.Fatal("Could not read REST response.")
	}
	return roles
}

func getrole(id string, roles []Role) Role {
	for _, role := range roles {
		if role.Id == id {
			return role
		}
	}
	return Role{"", "Unknown", 0, 0, "", false}
}
