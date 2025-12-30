# Zed support

Duper is available as [an extension for Zed](https://zed.dev/extensions/duper), with syntax highlighting, full LSP support, validation, and auto-formatting.

In order to enable formatting via the LSP, add the following entry to `settings.json`:

```json
{
  "languages": {
    "Duper": {
      "formatter": "language_server"
    }
  }
}
```
