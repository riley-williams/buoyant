# Building Views

This section is an introduction to building views with Buoyant.
It covers the process of using Buoyant, and is not intended to be an exhaustive
reference of available features. For that, refer to the Buoyant documentation on
[docs.rs](https://docs.rs/buoyant/latest/buoyant/).

## Prerequisites

For all the examples in this section, it is assumed that you have installed the
[embedded-graphics-simulator](https://github.com/embedded-graphics/simulator)
requirements and have added the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
buoyant = "0.5"
embedded-graphics = "0.8"
embedded-graphics-simulator = "0.7.0"
```

The boilerplate from the [quickstart](./quickstart.md) is used to drive all
the examples in this section. I've hidden it to keep the examples concise, but
know you can still see it by clicking the eye icon in case you want to run
the example locally.
