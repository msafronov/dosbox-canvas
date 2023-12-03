use wasm_bindgen_test::*;

use crate::screen_pencil::GeometryToScreenMapper;

#[wasm_bindgen_test]
fn it_should_be_correct_screen_position() {
    let mapper = GeometryToScreenMapper::new(
        640,
        480,
        4,
    );

    let result = mapper.from_geometry_to_screen_pos(50, 240);

    assert_eq!(result, 614600);
}

#[wasm_bindgen_test]
fn it_should_be_correct_screen_position_with_zeroed_y() {
    let mapper = GeometryToScreenMapper::new(
        640,
        480,
        4,
    );

    let result = mapper.from_geometry_to_screen_pos(50, 0);

    assert_eq!(result, 200);
}

#[wasm_bindgen_test]
fn it_should_be_correct_screen_position_with_zeroed_x() {
    let mapper = GeometryToScreenMapper::new(
        640,
        480,
        4,
    );

    let result = mapper.from_geometry_to_screen_pos(0, 10);

    assert_eq!(result, 25600);
}

#[wasm_bindgen_test]
fn it_should_be_correct_screen_position_with_zeroed_values() {
    let mapper = GeometryToScreenMapper::new(
        640,
        480,
        4,
    );

    let result = mapper.from_geometry_to_screen_pos(0, 0);

    assert_eq!(result, 0);
}

#[wasm_bindgen_test]
fn it_should_be_correct_screen_position_with_max_values() {
    let mapper = GeometryToScreenMapper::new(
        640,
        480,
        4,
    );

    let result = mapper.from_geometry_to_screen_pos(640, 480);

    assert_eq!(result, 1231360);
}