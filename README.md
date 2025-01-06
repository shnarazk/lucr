# lucr

A formatter for helix/zed to write Lean4 files

### helix configuration

- .config/helix/languages.toml

```toml
[[language]]
name = "lean"
auto-format = true
formatter = { command = "lucr", args = [] }
```

### zed configuration

- You need Lean4 extension (now in dev mode): https://github.com/shnarazk/zed-lean4
- .config/zed/settings.json

```json
  "languages": {
    "Lean4": {
      "formatter": {
        "external": {
          "command": "lucr"
        }
      }
    }
  },
```
