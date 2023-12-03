FROM rust:1.74 as compiler

WORKDIR /app

ADD . .

RUN wget -P /tmp https://github.com/WebAssembly/wabt/releases/download/1.0.34/wabt-1.0.34-ubuntu.tar.gz
RUN tar -xf /tmp/wabt-1.0.34-ubuntu.tar.gz -C /tmp
RUN cp /tmp/wabt-1.0.34/bin/wasm-strip /usr/bin
RUN rm -rf /tmp/wabt-1.0.34

RUN wget -P /tmp https://github.com/WebAssembly/binaryen/releases/download/version_116/binaryen-version_116-x86_64-linux.tar.gz
RUN tar -xf /tmp/binaryen-version_116-x86_64-linux.tar.gz -C /tmp
RUN cp /tmp/binaryen-version_116/bin/wasm-opt /usr/bin
RUN rm -rf /tmp/binaryen-version_116

# TODO
