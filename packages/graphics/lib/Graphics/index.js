import wasm from '../../dist/main.wasm';
import { worker } from '../GraphicsWorker';

export class Graphics {
    create = (canvas) => {
        const offscreenCanvas = canvas.transferControlToOffscreen();

        // init
        worker.postMessage({
            wasm: wasm,
            offscreenCanvas: offscreenCanvas,
        }, [wasm.buffer, offscreenCanvas]);
    };
}