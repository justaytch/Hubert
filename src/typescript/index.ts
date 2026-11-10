import { Client } from "stoat.js";
import * as structs from "./structs.js";

const client = new Client();
const config: structs.Config = structs.getconfig();

const originaldebug = console.debug;
console.debug = (...args: unknown[]) => {
  const message = args.map(String).join(" ");
  if (
    message.startsWith("Skipping key ") &&
    message.includes("during hydration!")
  ) {
    return;
  }
  originaldebug(...args);
};

client.on("ready", async () => {
  console.log(`${config.name} is connected to Stoat!`);
  if (client.user) {
    await client.user.edit({
      status: {
        presence: "Busy",
        text: `Watching #${client.channels.get(config.stoat.channelid)?.name}`,
      },
    });
  }
});

client.on("messageCreate", async (message) => {
  if (message.author && !message.author.bot && message.authorId) {
    if (message.content === "!h" && message.channel) {
      await message.channel.sendMessage("h");
    }
    if (message.content === "!ping" && message.channel) {
      await message.channel.sendMessage("Pong!");
    }
    if (
      message.content === "!logout" &&
      message.channel &&
      config.stoat.ownerids.includes(message.authorId)
    ) {
      await message.channel.sendMessage("Logging Out...");
      process.exit(0);
    }
    if (
      message.channelId &&
      message.channel &&
      message.channelId === config.stoat.channelid
    ) {
      if (config.discord.enabled || config.fluxer.enabled) {
        var re = /<@([A-Za-z0-9]{26})>/g;
        var msgnew = message.content;
        for (const match of message.content.matchAll(re)) {
          var res: string;
          try {
            res =
              message.server?.getMember(match[1])?.displayName ??
              (await client.users.fetch(match[1]))?.displayName ??
              "Unknown";
          } catch (error) {
            res = "Unknown";
          }
          res = `@${res}`;
          msgnew = msgnew.replace(match[0], res);
        }
        re = /<#([A-Za-z0-9]{26})>/g;
        for (const match of message.content.matchAll(re)) {
          var res: string;
          try {
            res = (await client.channels.fetch(match[1]))?.name ?? "Unknown";
          } catch (error) {
            res = "Unknown";
          }
          res = `#${res}`;
          msgnew = msgnew.replace(match[0], res);
        }
        if (config.discord.enabled) {
          const payload: structs.WebhookPost = {
            username: `${message.author.displayName} (Stoat User)`,
            avatar_url: message.author.avatarURL ?? config.stoat.defaultavatar,
            content: msgnew,
          };
          await fetch(config.discord.webhook, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
          });
        }
        if (config.fluxer.enabled) {
          const payload: structs.WebhookPost = {
            username: `${message.author.displayName} (Stoat User)`,
            avatar_url: message.author.avatarURL ?? config.stoat.defaultavatar,
            content: msgnew,
          };
          await fetch(config.fluxer.webhook, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
          });
        }
      }
    }
  }
});

client.loginBot(config.stoat.token);
