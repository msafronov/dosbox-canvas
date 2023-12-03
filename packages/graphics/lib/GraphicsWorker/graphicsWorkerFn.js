export function graphicsWorkerFn() {
    let instance = null;
    let canvas = null;
    let ctx = null;
    let imageData = null;

    let screenOffset = 131072;
    let screenLen = 0;
    let screenWidth = 0;
    let screenHeight = 0;

    const logWarningHandler = (...args) => {
        console.warn('wasm warning:', ...args);
    };

    const logErrorHandler = (errorCode) => {
        throw new Error(`wasm error, errorCode = ${errorCode}`);
    };

    const renderHandler = () => {
        const canvasData = new Uint8Array(
            instance.exports.memory.buffer,
            screenOffset,
            screenLen,
        );

        imageData.data.set(canvasData);

        ctx.putImageData(imageData, 0, 0);
    };

    // worker initialization
    self.onmessage = (event) => {
        try {
            const module = new WebAssembly.Module(event.data.wasm);
            const wasmInstance = new WebAssembly.Instance(module, {
                env: {
                    log_warning: logWarningHandler,
                    log_error: logErrorHandler,
                    render: renderHandler,
                },
            });

            instance = wasmInstance;
            canvas = event.data.offscreenCanvas;
            ctx = canvas.getContext('2d');

            instance.exports.init();

            screenWidth = instance.exports.screen_width();
            screenHeight = instance.exports.screen_height();
            screenLen = screenWidth * screenHeight * 4;

            canvas.width = screenWidth;
            canvas.height = screenHeight;

            imageData = ctx.createImageData(
                screenWidth,
                screenHeight,
            );

            console.warn('graphicsWorker instance:', instance);

            instance.exports.run();
        } catch (error) {
            console.warn('WASM initialization error:', error);
        }
    };
}
