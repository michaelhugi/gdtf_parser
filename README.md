A fast and well tested GDTF-parser

🚧️ **gdtf-parser is in pre-release state. Any breaking changes may be implemented without further notice!** 🚧️

⚠️ **Implementation of Matrix and Rotation may be wrong** ⚠️

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
use gdtf_parser::utils::units::attribute_name::AttributeName;
use gdtf_parser::utils::units::physical_unit::PhysicalUnit;

fn main() -> Result<(),GdtfError>{
    let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");
    let gdtf: GDTF = GDTF::try_from(path)?;
    assert_eq!(gdtf.data_version, DataVersion::Version1_0);
    assert_eq!(gdtf.fixture_type.name, Name::new("ACME AE-610 BEAM")?);
    assert_eq!(gdtf.fixture_type.attribute_definitions.attributes.get(&AttributeName::Gobo_n_WheelSpin(1)).unwrap().physical_unit, PhysicalUnit::AngularSpeed);
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
use gdtf_parser::utils::units::name::Name;
use gdtf_parser::utils::units::attribute_name::AttributeName;
use gdtf_parser::utils::units::physical_unit::PhysicalUnit;

fn main() -> Result<(),GdtfError> {
    let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");
    let gdtf: GDTF = path.try_into()?;
    assert_eq!(gdtf.data_version, DataVersion::Version1_0);
    assert_eq!(gdtf.fixture_type.name, Name::new("ACME AE-610 BEAM")?);
    assert_eq!(gdtf.fixture_type.attribute_definitions.attributes.get(&AttributeName::Gobo_n_WheelSpin(1)).unwrap().physical_unit, PhysicalUnit::AngularSpeed);
    Ok(())
}
```

