import "dotenv/config";

import {
  AppBskyEmbedExternal,
  AppBskyEmbedImages,
  AtpAgent,
} from "@atproto/api";

const { BSKY_IDENTIFIER, BSKY_PASSWORD } = process.env;

const agent = new AtpAgent({ service: "https://bsky.social" });

console.log("Logging in…");

await agent.login({
  identifier: BSKY_IDENTIFIER ?? "",
  password: BSKY_PASSWORD ?? "",
});

let cursor: string | undefined;

do {
  const { data } = await agent.getAuthorFeed({
    actor: agent.did!,
    cursor,
    filter: "posts_no_replies",
    includePins: false,
    limit: 100,
  });

  for (const { post, reason } of data.feed) {
    if (
      post.embed?.$type.startsWith("app.bsky.embed.record") ||
      reason?.$type === "app.bsky.feed.defs#reasonRepost"
    ) {
      continue;
    }

    const rkey = post.uri.slice(post.uri.lastIndexOf("/") + 1);
    const createdAt = post.record.createdAt as string;

    const meta: Record<string, unknown> = {
      text: post.record.text ?? "",
    };

    if (post.embed) {
      if (post.embed.$type === "app.bsky.embed.images#view") {
        meta.images = (post.embed as AppBskyEmbedImages.View).images.length;
      } else if (post.embed.$type === "app.bsky.embed.video#view") {
        meta.video = true;
      } else if (post.embed.$type === "app.bsky.embed.external#view") {
        meta.link = (post.embed as AppBskyEmbedExternal.View).external.uri;
      }
    }

    console.dir({
      event_type: "bluesky-post",
      external_id: rkey,
      created_at: createdAt,
      meta,
    });
  }

  cursor = data.cursor;
} while (cursor != null);
