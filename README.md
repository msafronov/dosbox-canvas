<div align="center">

<h1 style="margin: 0;">DOSBOX-CANVAS</h1>

<br />
<br />

<img src="./.readme/promo.svg" alt="promo" width="300" />

<br />
<br />
<a href="https://msafronov.github.io/dosbox-canvas/" target="_blank">DEMO</a>
<br />
<br />

_work in progress.._

<br />
<br />
</div>

# Local build

First of all, you need to install rust compiler (rustc 1.74+) and wasm utilities (wasm-strip and wasm-opt) to compile .wasm binary file:

- [rust-lang](https://www.rust-lang.org/)
- [wabt 1.0.34](https://github.com/WebAssembly/wabt/releases/tag/1.0.34)
- [binaryen 116](https://github.com/WebAssembly/binaryen/releases/tag/version_116)

<details>
    <summary>Example of installing wasm utilities for Linux</summary>

    wget \
      -P /tmp \
      https://github.com/WebAssembly/wabt/releases/download/1.0.34/wabt-1.0.34-ubuntu.tar.gz
    tar -xf /tmp/wabt-1.0.34-ubuntu.tar.gz -C /tmp
    cp /tmp/wabt-1.0.34/bin/wasm-strip /usr/bin
    rm -rf /tmp/wabt-1.0.34

    wget \
      -P /tmp \
      https://github.com/WebAssembly/binaryen/releases/download/version_116/binaryen-version_116-x86_64-linux.tar.gz
    tar -xf /tmp/binaryen-version_116-x86_64-linux.tar.gz -C /tmp
    cp /tmp/binaryen-version_116/bin/wasm-opt /usr/bin
    rm -rf /tmp/binaryen-version_116
</details>
<br />
<br />

Then install npm packages from the root folder and run "build" command after:

```sh
npm install
npm run build
```

Then run the project:

```sh
npm run dev
```
