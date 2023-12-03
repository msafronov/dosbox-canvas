#[derive(Clone, Copy)]
pub struct SizeToWasmBlockMapper {
    wasm_block_size: usize,
}

impl SizeToWasmBlockMapper {
    pub fn new(wasm_block_size: usize) -> Self {
        SizeToWasmBlockMapper {
            wasm_block_size,
        }
    }

    pub fn size_to_wasm_blocks_count(&self, size: usize) -> usize {
        if size < self.wasm_block_size {
            return 1;
        }

        let ceiling = (size % self.wasm_block_size != 0) as usize;

        (size / self.wasm_block_size) + ceiling
    }

    pub fn wasm_blocks_count_to_size(&self, wasm_blocks_count: usize) -> usize {
        wasm_blocks_count * self.wasm_block_size
    }
}
