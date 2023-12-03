const path = require('path');
const wasmPlugin = require('esbuild-utils/plugins/wasm');

module.exports = {
    entryPoints: [
        path.join(__dirname, '../lib/index.js'),
    ],
    plugins: [wasmPlugin],
    bundle: true,
    treeShaking: true,
    outdir: path.join(__dirname, '../dist'),
    format: 'esm',
};