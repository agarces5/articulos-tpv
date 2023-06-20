# Frontend para TPV

La idea es crear un frontend con Yew que posteriormente conectaremos con una base de datos para gestionar los articulos de un TPV.

### Instalacion

Necesitaremos tener una instalacion tipica de `rustup`: <https://www.rust-lang.org/tools/install>

Para compilar a WASM, necesitamos el target `wasm32-unknown-unknown`.

```bash
rustup target add wasm32-unknown-unknown
```

Instalamos trunk:

```bash
cargo install trunk wasm-bindgen-cli
```

### Run

```bash
trunk serve
```

Se ejecuta un servidor local que se reconstruye cuando realizamos cambios.

Con `trunk watch` hace lo mismo pero sin hacer el hosting.

### Release

```bash
trunk build --release
```

Igual que se hace con `cargo build --release`, podemos pasar `--release` a `trunk serve`.

A no ser que se cambie, el output estara en el directorio `dist`.

