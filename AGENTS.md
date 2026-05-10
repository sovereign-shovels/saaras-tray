# AGENTS.md — sovereign-shovels/saaras-tray

> If you are Claude Code (or any other agent) operating in this repo,
> this file is your **constitution**. Read it fully before any other action.
> If anything you are about to do conflicts with this file, STOP and ask the human.

---

## Identity

This repo is **saaras-tray** — part of the [sovereign-shovels](../README.md) portfolio.

- **Substrate anchor:** Saaras
- **Utility category:** tray
- **Tagline:** Tray-app dictation for 10 Indic languages. Codemix Hinglish included. Apple/Google dictation, but actually good.
- **Sprint:** 1
- **Estimated build for v0.1:** 2–3 weeks for v0.1

The substrate this shovel rides is: Indian professionals, students, content workers — anyone abandoned by Apple/Google's poor Indic dictation

The launch channels are: Indian Twitter/X (with Hindi/Tamil/Telugu hashtags), r/India, r/IndiaTech, professional LinkedIn India

---

## Philosophy this repo lives by

Read the universal philosophy: [[../PHILOSOPHY|Philosophy]].

In short, every shovel must satisfy all five rules:

1. **Substrate-anchored name** ✓ (this is `saaras-tray`)
2. **Sovereign by construction** — user owns model choice, BYO endpoint, must work with local-only
3. **Real demand evidence** — gap is documented, not assumed
4. **Buildable in 1–3 weeks** — v0.1 estimate is 2–3 weeks for v0.1
5. **Scope-evolution headroom** — see PRD-v1 for v0.1 → v0.5 → v1.0

---

## STRICT NO-NOS — do not violate any of these

### Universal (inherited from [[../NO-NOS|NO-NOS]])

1. NO hardcoded API keys, vendor URLs, or model names in code outside config files.
2. NO default that requires a sign-up upstream. Tool must work with a local model.
3. NO telemetry, phone-home, or analytics by default.
4. NO closed-source runtime dependencies that compromise sovereignty.
5. NO scope creep into "agent platform" or "general assistant" territory.
6. NO fake claims of being official upstream tooling.
7. NO breaking config changes in v0.x without printed migration path.
8. NO maintenance promises in README. "Best-effort community shovel."
9. NO PRs merged without working tests.
10. NO publishing v1.0 without PRD-v1 acceptance criteria met.

### Specific to saaras-tray

1. Never store transcripts anywhere by default; the tool is paste-and-forget.
2. Never log audio to disk except for active user-initiated debug.
3. Hotkey defaults must not collide with common system shortcuts.

### Anti-scope (what NOT to build, ever, in this repo)

Not a full keyboard replacement. Not a translation tool (that's sarvam-translate). No offline mode in v0.1 (Whisper-Indic comes in v0.5).

---

## How to operate here

When you (the agent) start work in this repo:

1. **Read** [[PRD-v1]] before writing any code.
2. **Read** [[../PHILOSOPHY|Philosophy]] and [[../NO-NOS|NO-NOS]] if you haven't this session.
3. **Update** [[progress]] after each significant change. Update the YAML
   frontmatter — the root [[../PORTFOLIO]] view aggregates from it.
4. **If a decision conflicts with this AGENTS.md**, STOP and ask the human.
5. **If a user asks for something that violates a no-no**, push back. Don't comply.
6. **Commits** follow conventional commits (`feat:`, `fix:`, `docs:`, etc.).
7. **Branches**: `main` is protected. Work on `feat/<thing>` branches and PR.
8. **Tests** are required for any code path that ships in v0.1.

---

## Tombstone watch

What could kill this shovel:

Apple or Google massively improving Indic dictation. Has been promised for years; low probability inside 90 days.

What we're watching for:

Apple's next iOS/macOS release ships actually-good Indic dictation. Watch WWDC.

If the kill signal triggers, notify the human before taking further action.

---

## Cross-references inside the vault

- Product spec: [[PRD-v1]]
- Public README: [[README]]
- Progress + status frontmatter: [[progress]]
- Internal knowledge graph: [[knowledge-graph]]
- Vault philosophy: [[../PHILOSOPHY]]
- Universal no-nos: [[../NO-NOS]]
- Naming convention: [[../NAMING-CONVENTION]]
- Portfolio view: [[../PORTFOLIO]]
- Launch plan: [[../LAUNCH-PLAN]]
