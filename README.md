# saaras-tray

> Tray-app dictation for 10 Indic languages. Codemix Hinglish included. Apple/Google dictation, but actually good.

**Status:** v0.1 — planning. Not yet released.

**Sovereignty:** sovereign-by-construction. BYO endpoint, BYO key, BYO model.
A local-only configuration is documented and tested.

This is a community project, **not affiliated with Saaras**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## What this is

Tray-app dictation for 10 Indic languages. Codemix Hinglish included. Apple/Google dictation, but actually good.

## What this isn't

Not a keyboard replacement. Not a translator (that's sarvam-translate). Not (in v0.1) an offline-only mode.

## Install

> Coming with v0.1 release.

## Configure

You bring the model. By default `saaras-tray` tries to use a local provider:

- For LLM endpoints: Ollama at `http://localhost:11434`
- For voice endpoints: configurable, see [docs/configure.md]

To use any other provider (Claude, GPT, Hermes, OpenRouter, Sarvam, etc.):

```toml
# ~/.config/saaras-tray/config.toml
[provider]
endpoint = "https://api.your-provider.com/v1"
api_key_env = "YOUR_PROVIDER_KEY"
model = "your-model-name"
```

Anthropic, OpenAI, and Sarvam endpoints all work. Local Ollama, llama.cpp,
LM Studio, and vLLM all work via their OpenAI-compatible endpoints.

## Why this exists

Indic dictation on macOS, Windows, and Linux is genuinely broken. Apple's dictation for Hindi/Tamil/Telugu has been bad for years. Google's only works in Chrome. Indians who think and write in their first language type slower than they think — and that's a quality-of-life problem at scale. Saaras v3 is best-in-class for Indic STT including Hinglish/Tanglish codemix. saaras-tray puts it on a global hotkey.

## What's next

See [PRD-v1.md](./PRD-v1.md) for the full v0.1 → v0.5 → v1.0 plan.

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels)
portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts,
ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm,
llm-diff, claude-bridge, claude-radio, sarvam-cast.
