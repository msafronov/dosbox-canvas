use wasm_bindgen_test::*;

use crate::geometry::point::Point;
use crate::geometry::rect::IRect;
use crate::geometry::rect::Rect;

#[wasm_bindgen_test]
fn it_should_be_correct_rect_without_errors() {
    let rect = Rect::new(
        Point::new(200, 100, 0),
        Point::new(400, 200, 0),
    );

    assert_eq!(rect.x1(), 200);
}

#[wasm_bindgen_test]
fn it_should_be_incorrect_rect_with_negative_width() {
    Rect::new(
        Point::new(200, 100, 0),
        Point::new(100, 200, 0),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::RectInvalidNegativeWidth));
}

#[wasm_bindgen_test]
fn it_should_be_incorrect_rect_with_negative_height() {
    Rect::new(
        Point::new(200, 100, 0),
        Point::new(400, 50, 0),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::RectInvalidNegativeHeight));
}