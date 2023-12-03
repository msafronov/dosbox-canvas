use wasm_bindgen_test::*;

use crate::geometry::point::Point;
use crate::geometry::rect::IRect;
use crate::geometry::rect::Rect;
use crate::geometry::rect::FlatRect;

#[wasm_bindgen_test]
fn it_should_be_correct_rect_without_errors() {
    let rect = FlatRect::new(
        Rect::new(
            Point::new(200, 100, 0),
            Point::new(400, 200, 0),
        ),
    );

    assert_eq!(rect.x1(), 200);
}

#[wasm_bindgen_test]
fn it_should_be_incorrect_immersed_rect() {
    FlatRect::new(
        Rect::new(
            Point::new(200, 100, 2),
            Point::new(400, 200, 0),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::FlatRectInvalid));
}

#[wasm_bindgen_test]
fn it_should_be_incorrect_sticking_out_rect() {
    FlatRect::new(
        Rect::new(
            Point::new(200, 100, 0),
            Point::new(400, 200, 5),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::FlatRectInvalid));
}