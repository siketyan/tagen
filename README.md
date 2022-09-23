# 🔖 tagen
Generates lots of tags by version and variants.

## 🚀 Motivation
When tagging Docker images or GitHub Actions, we often use lots of tag variants something like:

```
rust:1.64.0
rust:1.64.0-bullseye-slim
rust:1.64-bullseye
rust:1.64
rust:1
```

This style is very convenient for image users, but it is difficult to list them correctly.
We tried to generate these automatically from the given version and variants.

## 📦 Getting Started
### CLI
If you do NOT want to use tagen on your code, use this installation.

#### Using Cargo
```shell
cargo install tagen --features=cli 
```

### Rust API
tagen is a hybrid crate, this mean you can use it on your own code.

```toml
[dependencies]
tagen = "0.1"
```

Well done!

## 💚 Examples
Let's start with the simplest style:

```shell
tagen 1.64.0
```

It generates:

```
1
1.64
1.64.0
```

Easy!

Now add variants to them:

```shell
tagpr 1.64.0 bullseye slim
```

It generates:

```
1-bullseye-slim
1-slim
1.64-bullseye-slim
1.64-slim
1.64.0-bullseye-slim
1.64.0-slim
```

Note that variants are shorten to right variant: `a b c` variants to be `a-b-c`, `b-c`, and `c`.
`a` and `a-b` are not generated in this case to avoid conflicting with `a` or `a b` variants.
