# Fastly-inspect

*This is alpha-level software, some measurements are currently imprecise or
incorrect. Comments, suggestions, feedback are encouraged, however since the
code is in very early phase, please discuss any contribution with me
beforehand. Thanks!*

Fastly-inspect collects information about your internet connection and 
connectivity to Fastly's Network to help troubleshoot networking problems.

It is a non-official, modern, free-software clone of
[Fastly-debug.com](https://www.fastly-debug.com) that provides both:
 * A [web application](https://fastly-inspect.edgecompute.app), as a WASM binary designed to run on
   [Compute@Edge](https://docs.fastly.com/products/compute-at-edge).
 * A [command-line tool](https://github.com/yannh/fastly-inspect/releases), as a statically linked binary to run on your server.

### Demo
#### Web Application

Visit the [official Fastly-Inspect page](https://fastly-inspect.edgecompute.app/). Note that it makes some requests to fastly-analytics.com
which can be blocked by adblockers, so disable your adblocker on this page if
some information does not load.

#### Command-line interface
```bash
$ ./fastly-inspect 
{
  "geoip": {
    "ci": "berlin",
    "co": "EU",
    "ct": "germany",
    "st": "BE",
    "c_ip": "91.64.44.52",
    "c_asn": "31334",
    "c_asn_name": "vodafone kabel deutschland gmbh",
    [...]
```

The URL of the supporting Fastly-Service API can be set with the `FASTLY_INSPECT_URL`
environment variable (defaults to https://fastly-inspect.edgecompute.app).

## How this works

The fastly-inspect application ([src/main,rs](src/main.rs)) and the Fastly-inspect
library ([src/lib.rs](src/lib.rs)) compile to the Fastly-inspect CLI (eg. for x86_64.*).

The library can also be compiled to WASM, which is then used by the Fastly-Inspect
Web application, itself a VueJS 3.* App.

To be deployed on Fastly's Edge platform, a Rust compute@edge application, forked from
[Fastly pages](https://github.com/yannh/fastly-pages) (in [fastly-pages/](fastly-pages/))
and that also compiles to WASM, embeds all the static assets directly in the WASM
binary and will:
 * serve those static assets
 * provide additional API endpoints

## Build
### Web Application

Requirements:
 * Rust
 * Wasm-Pack
 * The Fastly CLI

Building is a 2-step process:
```bash
  # Compile the Webassembly asset used by the webapp
  # Copy the results to a target folder
  fastly-inspect$ make build-wasm site
  # Compile the compute@edge Webassembly binary
  fastly-inspect$ cd fastly-pages
  fastly-inspect/fastly-pages$ fastly compute build
```

### Command-line tool
```bash
  fastly-inspect$ make build-binary
```
