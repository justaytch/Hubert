import * as toml from "js-toml";
import { z } from "zod";
import { readFileSync } from "fs";

const BotConfigSchema = z.object({
  enabled: z.boolean(),
  token: z.string(),
  channelid: z.bigint(),
  ownerids: z.array(z.bigint()),
  webhook: z.string(),
  defaultavatar: z.string(),
});

const StoatConfigSchema = z.object({
  enabled: z.boolean(),
  token: z.string(),
  channelid: z.string(),
  ownerids: z.array(z.string()),
  webhook: z.string(),
  defaultavatar: z.string(),
});

const ConfigSchema = z.object({
  name: z.string(),
  discord: BotConfigSchema,
  fluxer: BotConfigSchema,
  stoat: StoatConfigSchema,
});

export type WebhookPost = {
  username: string;
  content: string;
  avatar_url: string;
};

export type Config = z.infer<typeof ConfigSchema>;

export function getconfig(): Config {
  const text: string = readFileSync("./config.toml", "utf-8");
  const result = ConfigSchema.safeParse(toml.load(text));

  if (!result.success) {
    throw new Error(result.error.issues.toString());
  } else {
    return result.data;
  }
}
