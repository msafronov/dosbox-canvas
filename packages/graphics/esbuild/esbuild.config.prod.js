const esbuild = require('esbuild');

const configCommon = require('./esbuild.config.common');

esbuild
    .build({
        ...configCommon,
        minify: true,
        sourcemap: true,
    })
    .then(() => {
        console.log('esbuild prod success');
    });