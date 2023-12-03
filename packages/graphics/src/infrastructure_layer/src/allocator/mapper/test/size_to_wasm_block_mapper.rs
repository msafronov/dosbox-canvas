use wasm_bindgen_test::*;
use super::super::SizeToWasmBlockMapper;

#[wasm_bindgen_test]
fn it_should_return_one_block_for_zero_size() {
    let mapper = SizeToWasmBlockMapper::new(
        65536,
    );

    let blocks_count = mapper.size_to_wasm_blocks_count(0);

    assert_eq!(blocks_count, 1);
}

#[wasm_bindgen_test]
fn it_should_return_one_block() {
    let mapper = SizeToWasmBlockMapper::new(
        65536,
    );

    let blocks_count = mapper.size_to_wasm_blocks_count(100);

    assert_eq!(blocks_count, 1);
}

#[wasm_bindgen_test]
fn it_should_return_one_block_corner_case() {
    let mapper = SizeToWasmBlockMapper::new(
        65536,
    );

    let blocks_count = mapper.size_to_wasm_blocks_count(65536);

    assert_eq!(blocks_count, 1);
}

#[wasm_bindgen_test]
fn it_should_return_two_blocks_corner_case() {
    let mapper = SizeToWasmBlockMapper::new(
        65536,
    );

    let blocks_count = mapper.size_to_wasm_blocks_count(131072);

    assert_eq!(blocks_count, 2);
}

#[wasm_bindgen_test]
fn it_should_return_two_blocks() {
    let mapper = SizeToWasmBlockMapper::new(
        65536,
    );

    let blocks_count = mapper.size_to_wasm_blocks_count(100000);

    assert_eq!(blocks_count, 2);
}

#[wasm_bindgen_test]
fn it_should_return_three_blocks() {
    let mapper = SizeToWasmBlockMapper::new(
        65536,
    );

    let blocks_count = mapper.size_to_wasm_blocks_count(180000);

    assert_eq!(blocks_count, 3);
}

#[wasm_bindgen_test]
fn it_should_return_correct_size_for_one_block() {
    let mapper = SizeToWasmBlockMapper::new(
        65536,
    );

    let size = mapper.wasm_blocks_count_to_size(1);

    assert_eq!(size, 65536);
}

#[wasm_bindgen_test]
fn it_should_return_correct_size_when_zero() {
    let mapper = SizeToWasmBlockMapper::new(
        65536,
    );

    let size = mapper.wasm_blocks_count_to_size(0);

    assert_eq!(size, 0);
}

#[wasm_bindgen_test]
fn it_should_return_correct_size_regular() {
    let mapper = SizeToWasmBlockMapper::new(
        65536,
    );

    let size = mapper.wasm_blocks_count_to_size(16);

    assert_eq!(size, 1048576);
}