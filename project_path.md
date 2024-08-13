# explore_rust_plotly_two

## init 

```
cd \
&& mkdir rust_project \
&& cd $_ \
```

## configure project

```bash
rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& cargo add rustfmt \
&& rustup component add rustfmt \
# cold update 
# https://github.com/rust-lang/rustup/issues/2729
# rustup toolchain uninstall stable
# rustup toolchain install stable
&& rustup show \
&& touch FROM_HERE.md \ 
&& cargo build
&& cargo run \
&& cargo run --example example

```

## add crates for these project

```bash
cargo add plotly --default-features
cargo build
```

