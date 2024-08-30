use nalgebra::Vector2;

#[test]
fn test_cross_2d() {
    use crate::math::cross_2d;

    let vec1 = Vector2::new(1f64, 0f64);
    let vec2 = Vector2::new(0f64, 1f64);

    assert_eq!(1f64, cross_2d(vec1, vec2));
    assert_eq!(-1f64, cross_2d(vec2, vec1));

    assert_eq!(-1f64, cross_2d(-vec1, vec2));
    assert_eq!(-1f64, cross_2d(vec1, -vec2));

    assert_eq!(0f64, cross_2d(vec1, vec1));
    assert_eq!(0f64, cross_2d(vec2, vec2));

    let vec1 = Vector2::new(5f64, 2f64);
    let vec2 = Vector2::new(3f64, -3f64);

    assert_eq!(-21f64, cross_2d(vec1, vec2));
    assert_eq!(21f64, cross_2d(vec2, vec1));
}
