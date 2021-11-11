# Fastly pages

Fastly pages enables hosting of small static websites on Fastly's
compute@edge cloud platform.

Since the website with all its assets is embedded in and served from the
WASM binary, this only works for small static websites
(megabytes-large in size).

# Build

To build the project:
```
 fastly compute build
```

Local testing can be done using Viceroy:
```
 fastly compute serve
```
