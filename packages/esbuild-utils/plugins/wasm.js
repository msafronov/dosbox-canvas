const path = require('path');
const fs = require('fs');

// https://esbuild.github.io/plugins/#webassembly-plugin

module.exports = {
    name: 'wasm',
    setup(build) {
        build.onResolve({ filter: /\.wasm$/ }, (args) => {
            if (args.namespace === 'wasm-stub') {
                return {
                    path: args.path,
                    namespace: 'wasm-binary',
                };
            }

            if (args.resolveDir === '') {
                return;
            }
    
            return {
                path: path.isAbsolute(args.path)
                    ? args.path
                    : path.join(args.resolveDir, args.path),
                namespace: 'wasm-binary',
            };
        });

        build.onLoad({ filter: /.*/, namespace: 'wasm-binary' }, (args) => {
            const binary = fs.readFileSync(args.path);

            return {
                contents: binary,
                loader: 'binary',
            };
        });
    },
};