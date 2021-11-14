# Fastly-inspect

*This is alpha-level software. Comments, suggestions, feedback are
encouraged, however since the code is in very early phase, please
discuss any contribution with me beforehand. Thanks!*

Fastly-inspect collects information about your internet connection and 
connectivity to Fastly's Network to help troubleshoot networking problems.

It is a non-official, modern, free-software clone of
[Fastly-debug.com](https://www.fastly-debug.com) that provides both:
 * A web application, as a WASM binary designed to run on
   [Compute@Edge](https://docs.fastly.com/products/compute-at-edge).
 * A command-line tool, as a statically linked binary to run on your server.

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

## Build
### Web Application

Building the web-application is a 2-step process:
```bash
  # Compile the Webassembly asset used by the webapp
  fastly-inspect$ make build-wasm
  # Copy the resulting files to another folder
  fastly-inspect$ make site
  # Compile the compute@edge Webassembly binary
  fastly-inspect$ cd fastly-pages
  fastly-inspect/fastly-pages$ fastly compute build
```

### Web Application
```bash
  fastly-inspect$ make build-binary
```
