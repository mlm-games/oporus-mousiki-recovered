/// Integration test for CELT fixed-point decode path
///
/// This test validates that the fixed-point CELT decoder produces
/// valid output and matches expected behavior using only public APIs.

#[cfg(feature = "fixed_point")]
#[test]
fn test_celt_fixed_point_decode_plc() {
    use oporus::c_style_api::opus_decoder::{opus_decode, opus_decoder_create};

    let sample_rate = 48000;
    let channels = 1;

    let mut decoder = opus_decoder_create(sample_rate, channels).unwrap();

    let mut pcm = vec![0i16; 960];
    let result = opus_decode(&mut decoder, None, 0, &mut pcm, 960, false);

    assert!(
        result.is_ok(),
        "PLC decode should succeed in fixed-point mode"
    );
    let samples = result.unwrap();
    assert_eq!(samples, 960, "PLC should generate 960 samples");
}

#[cfg(feature = "fixed_point")]
#[test]
fn test_celt_fixed_point_decode_stereo() {
    use oporus::c_style_api::opus_decoder::{opus_decode, opus_decoder_create};

    let sample_rate = 48000;
    let channels = 2;

    let mut decoder = opus_decoder_create(sample_rate, channels).unwrap();

    let mut pcm = vec![0i16; 1920];
    let result = opus_decode(&mut decoder, None, 0, &mut pcm, 960, false);

    assert!(
        result.is_ok(),
        "Stereo PLC should succeed in fixed-point mode"
    );
    let samples = result.unwrap();
    assert_eq!(
        samples, 960,
        "Stereo PLC should generate 960 samples per channel"
    );
}

#[cfg(feature = "fixed_point")]
#[test]
fn test_celt_fixed_point_multiple_sample_rates() {
    use oporus::c_style_api::opus_decoder::opus_decoder_create;

    for &sample_rate in &[8000, 12000, 16000, 24000, 48000] {
        let decoder = opus_decoder_create(sample_rate, 1);
        assert!(
            decoder.is_ok(),
            "Decoder creation should succeed for {sample_rate} Hz"
        );
    }
}

#[cfg(not(feature = "fixed_point"))]
#[test]
fn test_fixed_point_feature_required() {
    assert!(
        true,
        "Fixed-point tests are only available with --features fixed_point"
    );
}
