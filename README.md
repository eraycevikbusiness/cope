<img width="1440" height="500" alt="image" src="https://github.com/user-attachments/assets/efba160b-ddac-4fc5-9da4-46e4942a13b4" />

<p align="center">
<img width="1440" height="500" alt="image" src="https://github.com/user-attachments/assets/efba160b-ddac-4fc5-9da4-46e4942a13b4" />

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
