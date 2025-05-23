# Tnixc's API

An overly-complicated API built with Rust for stuff I want. This project runs as a Vercel serverless function. See [https://api.tnixc.space](https://api.tnixc.space) for docs.

## Features

- Fetch current or most recently played song from Last.fm
- Send messages to Discord via a bot
- OpenAPI documentation using Scalar UI

## Setup

### Prerequisites

- Rust
- Vercel CLI

### Environment Variables

Put these in your `.env.local` in the root directory:

```
LAST_FM_API_KEY=your_lastfm_api_key
DISCORD_TOKEN=your_discord_bot_token
```

```
cargo build && vercel dev
```

# Notes

You need to include the every endpoint in `Cargo.toml` under a `[[bin]]` section. The docs for the OpenAPI json are generated from macros in `api/doc.rs`. Function "stubs" are included there.

If you want to use this yourself you might need to change some CORS settings.
