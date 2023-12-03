const esbuild = require('esbuild');

const configCommon = require('./esbuild.config.common');

esbuild
    .build({
        ...configCommon,
    })
    .then(() => {
        console.log('esbuild dev success');
    });