#[cfg(test)]
mod tests {
    use coordinate_converter::angles;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_angle_constructor() {
        let ang = angles::Angle::new(0.);
        assert_eq!(format!("{}", ang), "0.000000");
    }

    #[test]
    fn test_angle_conversion() {
        let ang = angles::Angle::new(42.15188);
        assert_eq!(format!("{}", ang.as_dms()), "+42:09:06.768");
        assert_eq!(format!("{}", ang.as_hms()), "02:48:36.451");

    }

    #[test]
    fn test_from_functions() {
        let ang_dec = angles::Angle::new(42.15188);
        let ang_hms = angles::Angle::from_hms("02:48:36.451");
        let ang_dms = angles::Angle::from_dms("+42:09:06.768");

        assert_approx_eq!(ang_dec.value(), ang_hms.unwrap().value());
        assert_approx_eq!(ang_dec.value(), ang_dms.unwrap().value());
    }

    #[test]
    fn test_from_error_states() {
        let ang_hms_parts = angles::Angle::from_hms("02:36.451");
        let ang_hms_numbers = angles::Angle::from_hms("02:48:ABCD");
        let ang_dms_parts = angles::Angle::from_dms("+42:06.768");
        let ang_dms_numbers = angles::Angle::from_dms("+42:EF:06.768");


        assert!(ang_hms_parts.is_err());
        assert!(ang_hms_numbers.is_err());
        assert!(ang_dms_parts.is_err());
        assert!(ang_dms_numbers.is_err());

        let e = ang_hms_parts.unwrap_err();
        assert_eq!(e, angles::AngleConstructionError::NotEnoughParts);
        let e = ang_hms_numbers.unwrap_err();
        assert_eq!(e, angles::AngleConstructionError::MapToNumber("second".to_owned()));
        let e = ang_dms_parts.unwrap_err();
        assert_eq!(e, angles::AngleConstructionError::NotEnoughParts);
        let e = ang_dms_numbers.unwrap_err();
        assert_eq!(e, angles::AngleConstructionError::MapToNumber("minute".to_owned()));
    }



}
