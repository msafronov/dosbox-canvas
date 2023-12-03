use wasm_bindgen_test::*;

use crate::geometry::axis::Axis;
use crate::geometry::coordinate_system::ICoordinateSystem;
use crate::geometry::coordinate_system::CoordinateSystem;
use crate::geometry::coordinate_system::PositiveCartesianCoordinateSystem;

#[wasm_bindgen_test]
fn it_should_be_correct_coordinate_system() {
    let coordinate_system = PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(0, 640),
            Axis::new(0, 480),
            Axis::new(0, 10),
        ),
    );

    let result = coordinate_system;

    assert_eq!(result.x_max(), 640);
}

#[wasm_bindgen_test]
fn it_should_be_invalid_origin_at_abscissa_axis() {
    PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(10, 640),
            Axis::new(0, 480),
            Axis::new(0, 10),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::CoordinateSystemInvalidOrigin));
}

#[wasm_bindgen_test]
fn it_should_be_invalid_origin_at_ordinate_axis() {
    PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(0, 640),
            Axis::new(10, 480),
            Axis::new(0, 10),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::CoordinateSystemInvalidOrigin));
}

#[wasm_bindgen_test]
fn it_should_be_invalid_origin_at_applicate_axis() {
    PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(0, 640),
            Axis::new(0, 480),
            Axis::new(5, 10),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::CoordinateSystemInvalidOrigin));
}

#[wasm_bindgen_test]
fn it_should_be_invalid_negative_abscissa_axis() {
    PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(0, -640),
            Axis::new(0, 480),
            Axis::new(0, 10),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::CoordinateSystemInvalidAbscissa));
}

#[wasm_bindgen_test]
fn it_should_be_invalid_zero_abscissa_axis() {
    PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(0, 0),
            Axis::new(0, 480),
            Axis::new(0, 10),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::CoordinateSystemInvalidAbscissa));
}

#[wasm_bindgen_test]
fn it_should_be_invalid_negative_ordinate_axis() {
    PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(0, 640),
            Axis::new(0, -480),
            Axis::new(0, 10),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::CoordinateSystemInvalidOrdinate));
}

#[wasm_bindgen_test]
fn it_should_be_invalid_zero_ordinate_axis() {
    PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(0, 640),
            Axis::new(0, 0),
            Axis::new(0, 10),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::CoordinateSystemInvalidOrdinate));
}

#[wasm_bindgen_test]
fn it_should_be_invalid_negative_applicate_axis() {
    PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(0, 640),
            Axis::new(0, 480),
            Axis::new(0, -10),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::CoordinateSystemInvalidApplicate));
}

#[wasm_bindgen_test]
fn it_should_be_invalid_zero_applicate_axis() {
    PositiveCartesianCoordinateSystem::new(
        CoordinateSystem::new(
            Axis::new(0, 640),
            Axis::new(0, 480),
            Axis::new(0, 0),
        ),
    );

    let error = errors::get_first_error();

    assert!(matches!(error, errors::Error::CoordinateSystemInvalidApplicate));
}