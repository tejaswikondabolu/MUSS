# Museum of Programming Religions

A satirical web museum that presents programming languages as religions, complete with deities, doctrines, relics, miracles, and commandments.

## Languages / Religions

| Religion | Language | Deity |
|---|---|---|
| Church of Rust | Rust | The Supreme Compiler |
| Cult of JavaScript | JavaScript | The Old Ones |
| Order of Python | Python | Guido van Rossum |
| Ancient Empire of C++ | C++ | Bjarne Stroustrup |

## Features

- **Exhibits** — Explore each religion's beliefs, history, and sacred texts
- **Heresy Detector** — Submit Rust code and receive a judgment from the Supreme Compiler, with heresy reports, penance, and AI-generated narratives
- **Holy Wars** — Overview of the great theological conflicts between languages
- **Relics** — Browse sacred artifacts from each language
- **Pilgrimage** — A guided tour through all the religions

## Tech Stack

- **Backend:** Rust with Axum web framework
- **Templates:** Tera
- **Frontend:** Vanilla HTML/CSS/JS

## Getting Started

```bash
cd museum
cargo run
```

Open [http://localhost:3000](http://localhost:3000).

## Project Structure

```
museum/
├── src/
│   ├── main.rs              # Server, routes, handlers
│   ├── religions/           # Religion data modules
│   │   ├── mod.rs
│   │   ├── rust.rs
│   │   ├── javascript.rs
│   │   ├── python.rs
│   │   └── cpp.rs
│   └── heresy/              # Heresy Detector engine
│       ├── mod.rs
│       ├── detector.rs      # Compiles Rust code, parses errors
│       ├── translator.rs    # Maps compiler errors to heresy reports
│       └── ai.rs            # Generates narrative stories
├── templates/               # Tera HTML templates
├── static/                  # Static assets (CSS)
└── Cargo.toml
```

## Heresy Detector

The Heresy Detector compiles submitted Rust code with `rustc`, parses the compiler output, and translates errors into religious heresy reports. Known error codes are mapped to specific violations (ownership, borrowing, lifetimes, etc.). A "Level 3 AI High Priest" generates randomized narrative stories around each judgment.
