# Upchuk

**Upchuk** is a simple CLI tool to manage, store, and check the availability of URLs from a local config file. It helps you track tagged URLs over time and test their accessibility on demand.

---

## 📦 Features

- Add URLs with optional tags and auto-generated date
- Store URLs in a line-delimited JSON file (`~/.config/upchuk/upchuk_urls.json`)
- List all saved URLs with metadata
- Check URL reachability with HTTP GET requests
- CLI output includes timing, status, and errors

---

## 🚀 Getting Started

### Add a URL
```bash
upchuk add https://example.com --tag blog
