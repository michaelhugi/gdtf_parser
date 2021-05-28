# gdtf-parser

 A fast and well tested GDTF-parser

 **gdtf-parser is in pre-release state. Any breaking changes may be implemented without further notice!**

 ## Description
 GDTF stands for [`General Device Type Format`] and describes the hierarchical and logical structure and controls of any type of controllable device (e.g. luminaires, fog machines, etc.) in the lighting and entertainment industry.

 With gdtf-parser you can simply parse a .gdtf file in an struct that is similarly organized like the GDTF description.xml but with lighter hierarchy.

 gdtf-parser supports GDTF 1.0 and 1.1 accorting to DIN SPEC 15800:2020-07

 [`General Device Type Format`]: https://www.gdtf-share.com

 ## Usage
 The main struct `GDTF` implements the trait `TryFrom<&Path>` so usage is straight forward:

 ### Example try from

```rust
 use std::convert::TryFrom;
 use std::path::Path;

 use crate::GDTF;
 use crate::utils::errors::GdtfError;

 fn main() {
    let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");

    let gdtf: Result<GDTF, GdtfError> = GDTF::try_from(path);
        match gdtf {
            Ok(gdtf) => {
                println!("The fixture's name is {:?} from the manufacturer {:?}.\n GDTF version is {:?}", gdtf.fixture_type.name, gdtf.fixture_type.manufacturer, gdtf.data_version)
            }
            Err(_) => panic!("Some error occured during parsing gdtf"),
        }
 }
 ```

 ### Example try into

 ```rust
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
 ```

