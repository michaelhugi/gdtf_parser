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
 use gdtf_parser::GDTF;
 use gdtf_parser::utils::errors::GdtfError;
 use gdtf_parser::utils::units::data_version::DataVersion;
 use gdtf_parser::utils::units::name::Name;
 use gdtf_parser::utils::units::color_cie::ColorCIE;

 fn main() -> Result<(),GdtfError>{
     let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");
     let gdtf: GDTF = GDTF::try_from(path)?;
     assert_eq!(gdtf.data_version, DataVersion::Version1_0);
     assert_eq!(gdtf.fixture_type.name, Name::new("ACME AE-610 BEAM")?);
     assert_eq!(gdtf.fixture_type.attribute_definitions.attributes.get(18).unwrap().color.unwrap(), ColorCIE{x:0.3127, y:0.329, Y:100.0});
     Ok(())
 }
 ```

 ### Example try into

 ```rust
use std::convert::TryInto;
use std::path::Path;
use gdtf_parser::GDTF;
use gdtf_parser::utils::errors::GdtfError;
use gdtf_parser::utils::units::data_version::DataVersion;
use gdtf_parser::utils::units::color_cie::ColorCIE;
use gdtf_parser::utils::units::name::Name;

#[test]
fn test_try_from() -> Result<(),GdtfError> {
     let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");
     let gdtf: GDTF = path.try_into()?;
     assert_eq!(gdtf.data_version, DataVersion::Version1_0);
     assert_eq!(gdtf.fixture_type.name, Name::new("ACME AE-610 BEAM")?);
     assert_eq!(gdtf.fixture_type.attribute_definitions.attributes.get(18).unwrap().color.unwrap(), ColorCIE{x:0.3127, y:0.329, Y:100.0});
     Ok(())
}
 ```

