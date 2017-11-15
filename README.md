# json_config
json_config is a JSON based configuration management solution for Rust applications. It allows you to do the following,

1. Maintain application settings in JSON form. i.e. in a file, as a string based variable, string literal and in pure JSON form
2. Span and maintain your application settings across multiple different sources, i.e. multiple
3. Maintain a base configuration and override it with JSON partials
4. Define bundles which encapsulate logically related configuration sections and later override the base configuration with them as necessary
5. Do all of the above either at runtime or compile time via `build.rs`

The library also exposes macros which help you do all of the above in a very convenient manner.

Documentation is still in progress...

# License
Dual licensed under MIT/Apache-2.0.

<!-- 1. In build.rs (build script) do,

```

```

2. In your module do,

```

``` -->