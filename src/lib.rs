pub mod angles{
    use std::fmt;
    use thiserror::Error;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Angle(f64);
    impl Angle {
        pub fn new(angle: f64) -> Angle {
            Angle(angle)
        }

        pub fn value(&self) -> f64 {
            self.0
        }

        pub fn from_hms(input: &str) -> Result<Angle, AngleConstructionError>  {
            let (hour_str, arcminute_str, arcsecond_str) = splitn_and_next(input)?;

            let hourangle: i8 = hour_str
                .parse()
                .map_err(|_| AngleConstructionError::MapToNumber("hour".to_owned()))?;
            let arcminutes: u8 = arcminute_str
                .parse()
                .map_err(|_| AngleConstructionError::MapToNumber("minute".to_owned()))?;
            let arcseconds: f64 = arcsecond_str
                .parse()
                .map_err(|_| AngleConstructionError::MapToNumber("second".to_owned()))?;
            
            dbg!(hourangle);
            dbg!(arcminutes);
            dbg!(arcseconds);
            let decimaldegree = (hourangle as f64  + arcminutes as f64 / 60. + arcseconds / 3600.) * 15.;

            Ok(Angle::new(decimaldegree))
        }

        pub fn from_dms(input: &str) -> Result<Angle, AngleConstructionError> {

            let (degree_str, arcminute_str, arcsecond_str) = splitn_and_next(input)?;

            let degree: i16 = degree_str
                .parse()
                .map_err(|_| AngleConstructionError::MapToNumber("degree".to_owned()))?;
            let arcminutes: u8 = arcminute_str
                .parse()
                .map_err(|_| AngleConstructionError::MapToNumber("minute".to_owned()))?;
            let arcseconds: f64 = arcsecond_str
                .parse()
                .map_err(|_| AngleConstructionError::MapToNumber("second".to_owned()))?;
            
            let decimaldegree =degree.signum() as f64 *(degree.abs() as f64 + arcminutes as f64 / 60. + arcseconds / 3600.);

            Ok(Angle::new(decimaldegree))
        }

        pub fn as_hms(&self) -> String {
            let sign = self.0.signum() as i8;
            let abs_angle = self.0.abs();

            let h = abs_angle / 15.;
            let hours = h.floor();
            let m = h * 60.0 - hours as f64 * 60.0;
            let minutes = m.floor() as i32;
            let seconds = (m - minutes as f64) * 60.0;

            format!("{:02}:{:02}:{:06.3}", sign * hours as i8, minutes as u8, seconds)
        }

        pub fn as_dms(&self) -> String {
            let sign = self.0.signum() as i16;
            let abs_angle = self.0.abs();

            let degree = abs_angle;
            let minutes = abs_angle % 1. * 60.;
            let seconds = minutes % 1. * 60.;

            format!("{:+03}:{:02}:{:06.3}", sign * degree as i16, minutes as u8, seconds)
        }
    }

    impl fmt::Display for Angle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:.6}", self.0)
        }
    }


    /// Function that splits a stringslice with the `:` (colon) character.
    /// If there are not 3 components to the resulting vec, it returns the Err state.
    /// Otherwise the components are returned in a three tuple.
    fn splitn_and_next(text: &str) -> Result<(&str, &str, &str), AngleConstructionError> {
        let parts = text.splitn(3, ':').collect::<Vec<&str>>();
        if parts.len() != 3 {return Err(AngleConstructionError::NotEnoughParts);}

        Ok((parts[0], parts[1], parts[2]))
    }

    #[derive(Error, Debug, Clone, PartialEq)]
    pub enum AngleConstructionError {
        #[error("Not enough parts to construct Angle.")]
        NotEnoughParts,
        #[error("Mapping to {0} failed")]
        MapToNumber(String),
    }
}
