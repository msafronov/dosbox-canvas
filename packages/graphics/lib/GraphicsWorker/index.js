import { graphicsWorkerFn } from './graphicsWorkerFn';

const graphicsWorkerAsString = graphicsWorkerFn
    .toString()
    .replace(/^function\s*\S+\s*\([^)]*\)\s*\{|\}$/g, '');

const blob = new Blob([graphicsWorkerAsString], { type: 'application/javascript' });
const worker = new Worker(URL.createObjectURL(blob));

export { worker };