# MCD

Simple, fast, and fun **generator of memorable codes** (e.g. `7751`).

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![Rust](https://img.shields.io/badge/rust-1.80+-orange.svg)
![Development Status](https://img.shields.io/badge/status-in%20development-yellow.svg)

---

## 🛠️ Development Status

**Current phase:** active development.
MCD is not yet released as a stable version. Basic features (numeric code generation) already work, but more improvements are planned.

---

## 📝 Concept

The idea behind **MCD** is simple:

* You type in the terminal:

```bash
mcd 4
```

👉 and get a **4-digit memorable code**, e.g. `5829`.

* If you just run `mcd` without arguments → default is `4 digits`.

* If you run `mcd gg` → program returns an **error** and exits with code `1`.

This makes it quick and predictable: short codes, same behavior every time.

---

## 🎯 Features

* ✅ Custom RNG implementation (`Xorshift8`)
* ✅ CLI length argument
* ✅ Default length = 4 digits
* ⏳ Planned: alphanumeric codes (e.g. `X7A9`)
* ⏳ Planned: word-based memorable codes (e.g. `sun-dog-42`)
* ⏳ Planned: config file for defaults
* ⏳ Planned: export options (clipboard, file)

---

## 📋 TODO

* [x] Add numeric generator
* [x] Implement CLI argument parsing
* [ ] Add error handling codes
* [ ] Add alphanumeric/word modes
* [ ] Publish first release (v1.0.0)

---

## 🐞 Error Codes (codeerr)

| Code | Meaning                               |
| ---- | ------------------------------------- |
| 1    | Invalid input (e.g. `mcd gg`)         |
| 2    | Out of range (too large length)       |
| 3    | Internal generator error              |

If you see an error, check this table first — quick debugging guide right inside README.

---

## 🛠️ Installation & Usage

1. Clone the repo:

```bash
git clone https://github.com/vladimir2090/MCD
cd MCD
```

2. Build binary:

```bash
cargo build --release
```

3. Run generator:

```bash
mcd 6
```

Example output:

```
length: 6
Next digit: 34
Next digit: 17
Next digit: 89
Next digit: 21
Next digit: 42
Next digit: 56
```

---

## 🤝 Contributing

Suggestions, feature ideas, and PRs are welcome!
Even small contributions (README fixes, new RNG ideas) are valuable.

---

## 📄 License

This project is licensed under the [GNU GPL v3](LICENSE).
Any modifications must remain open-source.

---

## 🧑💻 Contact & Links

**Author & Maintainer:**
[GitHub: vladimir2090](https://github.com/vladimir2090)

**Repository:**
[MCD GitHub repo](https://github.com/vladimir2090/MCD)

**Feedback / Collaboration:**

* GitHub Issues
* Direct message: [@vladimir2090 at GitHub](https://github.com/vladimir2090)

---

⭐ Star this project to follow updates and support development!
