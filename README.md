# Hogwarts School of Witchcraft & Programming

A satirical Harry Potter–themed web museum where programming languages are Hogwarts houses, compiler errors are forbidden spells, and code tells magical stories.

| House | Language | Founder | House Color |
|---|---|---|---|
| Gryffindor | Rust | Ferris the Crab | 🟥 #740001 |
| Slytherin | Python | Guido van Rossum | 🟩 #1A472A |
| Ravenclaw | C++ | Bjarne Stroustrup | 🟦 #2A4B8C |
| Hufflepuff | JavaScript | The Old Ones | 🟨 #FFDB00 |

## Features

- **Grand Hall (Home)** — Explore the four houses through an enchanted rotunda with stained-glass alcoves, colonnades, and a vaulted oculus.
- **Heresy Detector** — Submit Rust code to the Heresy Analysis Chamber. The Sorting Hat examines it for violations of magical law, classifies heresies, assigns penance, and generates AI-narrated judgment stories.
- **The Sorting Hat (Quiz)** — A personality quiz that sorts you into a programming house based on your coding preferences.
- **Council of Hogwarts** — Ask any programming question and get debate-style answers from Hermione Granger (Rust), Severus Snape (Python), Luna Lovegood (C++), and Cedric Diggory (JavaScript).
- **The Marauder's Map (Guide)** — A floating widget on every page with location-specific wisdom quotes and tips.
- **Pilgrimages** — Guided tours through the history, doctrines, and miracles of each house.
- **Magical Wars** — The great theological conflicts of the software world: The Great Memory War, The Readability Schism, The Static vs Dynamic Inquisition, and more.
- **Magical Relics** — Browse sacred artifacts from each house (Ferris Statue, PEP 8 Scroll, Coercion Stone, etc.).

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
│   ├── religions/           # House/religion data modules
│   │   ├── mod.rs
│   │   ├── rust.rs          # Church of Rust / Gryffindor
│   │   ├── javascript.rs    # Cult of JavaScript / Hufflepuff
│   │   ├── python.rs        # Order of Python / Slytherin
│   │   └── cpp.rs           # Ancient Empire of C++ / Ravenclaw
│   ├── heresy/              # Heresy Detector engine
│   │   ├── mod.rs
│   │   ├── detector.rs      # Compiles Rust code, parses errors
│   │   ├── translator.rs    # Maps compiler errors to heresy reports
│   │   └── ai.rs            # Generates narrative stories
│   ├── quiz.rs              # Sorting Hat quiz logic
│   ├── council.rs           # Council of Hogwarts debate system
│   └── guide.rs             # Marauder's Map page wisdom
├── templates/               # Tera HTML templates
├── static/                  # CSS
└── Cargo.toml
```

## Heresy Detector

The Heresy Detector compiles submitted Rust code with `rustc`, parses the compiler output, and translates errors into religious heresy reports. Known error codes are mapped to specific violations (ownership, borrowing, lifetimes, etc.). A randomized narrative generator frames each judgment in the voice of the Hogwarts Sorting Hat.
