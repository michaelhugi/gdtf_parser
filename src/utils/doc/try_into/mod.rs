#![cfg(test)]
//! This module runs just tests to ensure that the code examples in the documentation work as expected
//! If code is changed here you must also edit the example documented in lib.rs and README.md in the root directory
//! This example shows how to use try_into() on path

mod tests {
    use std::convert::TryInto;
    use std::path::Path;

    use crate::GDTF;
    use crate::utils::errors::GdtfError;

    #[test]
    fn test_try_from() {
        let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");

        let gdtf: Result<GDTF, GdtfError> = path.try_into();
        match gdtf {
            Ok(gdtf) => {
                println!("The fixture's name is {:?} from the manufacturer {:?}.\n GDTF version is {:?}", gdtf.fixture_type.name, gdtf.fixture_type.manufacturer, gdtf.data_version)
            }
            Err(_) => panic!("Some error occured during parsing gdtf"),
        }
    }
}