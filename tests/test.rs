const ABS: f64 = 1e-3;

#[test]
#[cfg(all(feature = "jpeg-decoder", feature = "gd"))]
fn regress_2() {
    let images = vec![
        ("vendor/src/pics/luxmarket_tshirt01.jpg", 0.0),
        ("vendor/src/pics/luxmarket_tshirt01_black.jpg", 0.102186),
        ("vendor/src/pics/luxmarket_tshirt01_sal.jpg", 0.112364),
        ("vendor/src/pics/luxmarket_tshirt01_sheum.jpg", 0.195009),
        ("vendor/src/pics/duck.gif", 0.831804),
        ("vendor/src/pics/pic-a-0.jpg", 0.750249),
    ];

    let mut context = puzzle::Context::new();
    let a = context.cvec_from_file(images[0].0).unwrap();
    for (image, diff) in images {
        let b = context.cvec_from_file(image).unwrap();
        assert!((a.distance(&b) - diff).abs() < ABS);
        if image.ends_with(".jpg") {
            let b_jpeg = context.cvec_from_jpeg_file(image).unwrap();
            assert!(b.distance(&b_jpeg) < ABS);
        }
    }
}
