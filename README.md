<p align="center">
  <img width="1440" height="500" alt="image" src="https://github.com/user-attachments/assets/e690c7be-644c-4e99-ac20-0961900e73c5" />
</p>

# cope

> my personal copium for rust

i'm learning rust and just felt like building my own little toolkit
along the way. cope is where i drop reusable stuff — config loaders,
error helpers, whatever — so i don't have to rewrite the same code
in every new project.

honestly it's mostly an excuse to apply what i learn from the rust book
to something that isn't a throwaway exercise. if it ends up useful,
cool. if not, i still learned something.

---

## what's in it

right now: a config loader that takes a json file and gives you
back a typed struct via serde. that's it. more modules will land
when i actually need them in real projects.

| module | what it does | status |
|--------|--------------|--------|
| `config` | load json into a typed struct | ✅ working |
| `log` | logging helpers | ⏳ planned |
| `error` | shared error types | ⏳ planned |
| `fs` | filesystem helpers | ⏳ planned |

---

## usage

cope isn't on crates.io. pull it from github:

```toml
[dependencies]
cope = { git = "https://github.com/eray/cope" }
```

then use it like any other crate:

```rust
use cope::config::read_json_from;
use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    token: String,
    port: u16,
}

fn main() {
    let cfg: AppConfig = read_json_from("app.json").unwrap();
    println!("running on port {}", cfg.port);
}
```

with this `app.json`:

```json
{
  "token": "secret-abc-123",
  "port": 8080
}
```

handle errors properly in real code:

```rust
match read_json_from::<AppConfig>("app.json") {
    Ok(cfg) => println!("loaded: {}", cfg.token),
    Err(e) => eprintln!("config failed: {:?}", e),
}
```

errors can be:

- `ConfigError::PathDoesNotExist` — file not found or unreadable
- `ConfigError::ParsingError` — file exists but the json is broken

---

## why "cope"?

because learning rust is suffering and writing `use cope::config;` is funny.
that's literally the whole reasoning. don't overthink it.

---

## a few things to know

- this is `0.x`. i'll change anything anytime without warning.
- modules get added when i hit a real use case, not "just in case."
- if i learn a better way to do something, i rewrite without guilt.
- it's shaped around my needs. you're welcome to use it but don't
  expect it to be a polished library.

---

## current status

i'm somewhere in chapter 7 of the rust book (modules and packages),
which is honestly why this project exists in the first place —
to practice building a real library structure instead of just
reading about it.

---

## license

mit. do whatever you want.

---

<p align="center">
  <sub>built while learning rust · 2026</sub>
</p>
