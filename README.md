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
| `config` | read/write json ↔ typed structs | ✅ working |
| `log` | logging helpers | ⏳ planned |
| `error` | shared error types | ⏳ planned |
| `fs` | filesystem helpers | ⏳ planned |

---

## usage

cope isn't on crates.io. pull it from github:

```toml
[dependencies]
cope = { git = "https://github.com/eraycevikbusiness/cope" }
```

then use it like any other crate:

```rust
use cope::config::{read_json_from, write_to_path};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct AppConfig {
    connection_string: String,
}

fn main() {
    // read
    let cfg: AppConfig = read_json_from("app.json").unwrap();
    println!("my connection string: {}", cfg.connection_string);

    // write
    write_to_path("app.json", &cfg).unwrap();
}
```

with this `app.json`:

```json
{
  "connection_string": "Server=myServerAddress;Database=myDataBase;User Id=myUsername;Password=myPassword;"
}
```

handle errors properly in real code:

```rust
match read_json_from::<AppConfig>("app.json") {
    Ok(cfg) => println!("loaded: {}", cfg.connection_string),
    Err(e) => eprintln!("config failed: {:?}", e),
}

match write_to_path("app.json", &cfg) {
    Ok(()) => println!("saved"),
    Err(e) => eprintln!("write failed: {:?}", e),
}
```

errors can be:

- `ConfigError::ReadFailed(String)` — file not found or unreadable
- `ConfigError::WriteFailed(String)` — couldn't write to path
- `ConfigError::Deserialize` — file exists but the json is broken
- `ConfigError::Serialize` — the struct couldn't be serialized to json

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
