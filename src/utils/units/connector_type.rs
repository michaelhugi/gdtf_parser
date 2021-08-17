use quick_xml::events::attributes::Attribute;

use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::units::name::Name;

///A list of predefined connectors or Other with Name
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ConnectorType {
    ///BNC connector
    BNC,
    ///Tag block
    TBLK,
    ///Solder tag
    TAG,
    ///Krone block
    KRN,
    ///Stereo jack
    STJ,
    ///Mini stereo jack
    MSTJ,
    ///Phono connector
    RCA,
    ///SCART connector
    SCART,
    ///4-pin mini-DIN
    SVIDEO,
    ///4-pin mini-DIN
    MDIN4,
    ///5-pin mini-DIN
    MDIN5,
    ///6-pin mini-DIN
    MDIN6,
    ///3-pin XLR
    XLR3,
    ///4-pin XLR
    XLR4,
    ///5-pin XLR
    XLR5,
    ///10/100 BaseT ethernet type
    RJ45,
    ///Telephone type
    RJ11,
    ///9-pin D-type
    DB9,
    ///15-pin D-type
    DB15,
    ///25-pin D-type
    DB25,
    ///37-pin D-type
    DB37,
    ///50-pin D-type
    DB50,
    ///15-pin D-type hi-density
    HD15,
    ///25-pin D-type hi-density
    HD25,
    ///3-pin DIN
    DIN3,
    ///5-pin DIN
    DIN5,
    ///EDAC 20-pin
    EDAC20,
    ///EDAC 38-pin
    EDAC38,
    ///EDAC 56-pin
    EDAC56,
    ///EDAC 90-pin
    EDAC90,
    ///EDAC 120-pin
    EDAC120,
    ///DL 96-pin
    DL96,
    ///SCSI connector 68-pin
    SCSI68,
    ///IEE488 connector 36-pin
    IEE488,
    ///Centronics 50-pin
    CENT50,
    ///Centronics 36-pin
    CENT36,
    ///Centronics 24-pin
    CENT24,
    ///DisplayPort connector
    DisplayPort,
    ///DVI connector
    DVI,
    ///HDMI connector
    HDMI,
    ///PS2 connector
    PS2,
    ///TosLink connector
    TL_ST,
    ///Fibre optic LC DUPLEX-type
    LCDUP,
    ///Fibre optic SC DUPLEX-type
    SCDUP,
    ///Fibre optic SC-type
    SC,
    ///Fibre optic ST-type
    ST,
    ///Speakon
    NL4,
    ///8-pin LS conn
    CACOM,
    ///USB connector
    USB,
    ///N connector
    N_CON,
    ///F connector
    F_CON,
    ///Eurostecker
    IEC_60320_C7_C8,
    ///Schutzkontakt
    CEE_7_7,
    ///IEC 60320
    IEC_60320_C13_14,
    ///Edison
    Edison,
    ///Wieland
    Wieland,
    ///16A-Blue
    _16A_CEE_2P,
    ///16A-Yellow
    _16A_CEE_2P_110,
    ///16A-CEE
    _16A_CEE,
    ///32A-CEE
    _32A_CEE,
    ///32A-Blue
    _32A_CEE_2P,
    ///32A-Yellow
    _32A_CEE_2P_110,
    ///63A-CEE
    _63A_CEE,
    ///125A-CEE
    _125A_CEE,
    ///Powerlock
    Powerlock,
    ///Powerlock 120A
    Powerlock_120A,
    ///Powerlock 400A
    Powerlock_400A,
    ///Powerlock 660A
    Powerlock_660A,
    ///Powerlock 800A
    Powerlock_800A,
    ///Camlock
    Camlock,
    ///Powercon Blue
    NAC3FCA,
    ///Powercon Grey
    NAC3FCB,
    ///Powercon TRUE1
    PowerconTRUE1,
    ///powerCON TRUE1 TOP
    PowerconTRUE1TOP,
    ///Socapex-16
    Socapex_16,
    ///Socapex-7
    Socapex_7,
    ///Socapex-9
    Socapex_9,
    ///HAN-16
    HAN_16,
    ///HAN-4
    HAN_4,
    ///L6-20
    L6_20,
    ///L15-30
    L15_30,
    ///Stagepin
    Stagepin,
    ///HUBBELL 6-4
    HUBBELL_6_4,
    ///Eberl
    DIN_56905,
    ///Any other connector type
    Other(Name),
}

///Default is of Type 'Other' with empty name
impl Default for ConnectorType {
    fn default() -> Self {
        Self::Other(Name::new("").unwrap())
    }
}

impl ConnectorType {
    ///Creates a new ConnectorType from a string defined in the xml-description of GDTF
    pub fn new_from_str(inp: &str) -> Result<Self, GdtfError> {
        Ok(match inp {
            "BNC" => ConnectorType::BNC,
            "TBLK" => ConnectorType::TBLK,
            "TAG" => ConnectorType::TAG,
            "KRN" => ConnectorType::KRN,
            "STJ" => ConnectorType::STJ,
            "MSTJ" => ConnectorType::MSTJ,
            "RCA" => ConnectorType::RCA,
            "SCART" => ConnectorType::SCART,
            "SVIDEO" => ConnectorType::SVIDEO,
            "MDIN4" => ConnectorType::MDIN4,
            "MDIN5" => ConnectorType::MDIN5,
            "MDIN6" => ConnectorType::MDIN6,
            "XLR3" => ConnectorType::XLR3,
            "XLR4" => ConnectorType::XLR4,
            "XLR5" => ConnectorType::XLR5,
            "RJ45" => ConnectorType::RJ45,
            "RJ11" => ConnectorType::RJ11,
            "DB9" => ConnectorType::DB9,
            "DB15" => ConnectorType::DB15,
            "DB25" => ConnectorType::DB25,
            "DB37" => ConnectorType::DB37,
            "DB50" => ConnectorType::DB50,
            "HD15" => ConnectorType::HD15,
            "HD25" => ConnectorType::HD25,
            "DIN3" => ConnectorType::DIN3,
            "DIN5" => ConnectorType::DIN5,
            "EDAC20" => ConnectorType::EDAC20,
            "EDAC38" => ConnectorType::EDAC38,
            "EDAC56" => ConnectorType::EDAC56,
            "EDAC90" => ConnectorType::EDAC90,
            "EDAC120" => ConnectorType::EDAC120,
            "DL96" => ConnectorType::DL96,
            "SCSI68" => ConnectorType::SCSI68,
            "IEE488" => ConnectorType::IEE488,
            "CENT50" => ConnectorType::CENT50,
            "CENT36" => ConnectorType::CENT36,
            "CENT24" => ConnectorType::CENT24,
            "DisplayPort" => ConnectorType::DisplayPort,
            "DVI" => ConnectorType::DVI,
            "HDMI" => ConnectorType::HDMI,
            "PS2" => ConnectorType::PS2,
            "TL-ST" => ConnectorType::TL_ST,
            "LCDUP" => ConnectorType::LCDUP,
            "SCDUP" => ConnectorType::SCDUP,
            "SC" => ConnectorType::SC,
            "ST" => ConnectorType::ST,
            "NL4" => ConnectorType::NL4,
            "CACOM" => ConnectorType::CACOM,
            "USB" => ConnectorType::USB,
            "N_CON" => ConnectorType::N_CON,
            "F_CON" => ConnectorType::F_CON,
            "IEC 60320-C7/C8" => ConnectorType::IEC_60320_C7_C8,
            "CEE 7/7" => ConnectorType::CEE_7_7,
            "IEC 60320-C13/14" => ConnectorType::IEC_60320_C13_14,
            "Edison" => ConnectorType::Edison,
            "Wieland" => ConnectorType::Wieland,
            "16A-CEE-2P" => ConnectorType::_16A_CEE_2P,
            "16A-CEE-2P-110" => ConnectorType::_16A_CEE_2P_110,
            "16A-CEE" => ConnectorType::_16A_CEE,
            "32A-CEE" => ConnectorType::_32A_CEE,
            "32A-CEE-2P" => ConnectorType::_32A_CEE_2P,
            "32A-CEE-2P-110" => ConnectorType::_32A_CEE_2P_110,
            "63A-CEE" => ConnectorType::_63A_CEE,
            "125A-CEE" => ConnectorType::_125A_CEE,
            "Powerlock" => ConnectorType::Powerlock,
            "Powerlock 120A" => ConnectorType::Powerlock_120A,
            "Powerlock 400A" => ConnectorType::Powerlock_400A,
            "Powerlock 660A" => ConnectorType::Powerlock_660A,
            "Powerlock 800A" => ConnectorType::Powerlock_800A,
            "Camlock" => ConnectorType::Camlock,
            "NAC3FCA" => ConnectorType::NAC3FCA,
            "NAC3FCB" => ConnectorType::NAC3FCB,
            "PowerconTRUE1" => ConnectorType::PowerconTRUE1,
            "powerCONTRUE1TOP" => ConnectorType::PowerconTRUE1TOP,
            "Socapex-16" => ConnectorType::Socapex_16,
            "Socapex-7" => ConnectorType::Socapex_7,
            "Socapex-9" => ConnectorType::Socapex_9,
            "HAN-16" => ConnectorType::HAN_16,
            "HAN-4" => ConnectorType::HAN_4,
            "L6-20" => ConnectorType::L6_20,
            "L15-30" => ConnectorType::L15_30,
            "Stagepin" => ConnectorType::Stagepin,
            "HUBBELL-6-4" => ConnectorType::HUBBELL_6_4,
            "DIN 56905" => ConnectorType::DIN_56905,
            _ => ConnectorType::Other(Name::new(inp)?)
        })
    }

    ///Creates a new Connector type from an attribute provided by GDTF-XML
    pub fn new_from_attr(inp: Attribute) -> Result<Self, GdtfError> {
        Self::new_from_str(read::attr_to_str(&inp))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use quick_xml::events::attributes::Attribute;

    use crate::utils::units::connector_type::ConnectorType;
    use crate::utils::units::name::Name;

    #[test]
    fn test_new_from_str() {
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"BNC") }).unwrap(), ConnectorType::BNC);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"TBLK") }).unwrap(), ConnectorType::TBLK);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"TAG") }).unwrap(), ConnectorType::TAG);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"KRN") }).unwrap(), ConnectorType::KRN);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"STJ") }).unwrap(), ConnectorType::STJ);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"MSTJ") }).unwrap(), ConnectorType::MSTJ);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"RCA") }).unwrap(), ConnectorType::RCA);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SCART") }).unwrap(), ConnectorType::SCART);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SVIDEO") }).unwrap(), ConnectorType::SVIDEO);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"MDIN4") }).unwrap(), ConnectorType::MDIN4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"MDIN5") }).unwrap(), ConnectorType::MDIN5);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"MDIN6") }).unwrap(), ConnectorType::MDIN6);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"XLR3") }).unwrap(), ConnectorType::XLR3);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"XLR4") }).unwrap(), ConnectorType::XLR4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"XLR5") }).unwrap(), ConnectorType::XLR5);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"RJ45") }).unwrap(), ConnectorType::RJ45);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"RJ11") }).unwrap(), ConnectorType::RJ11);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB9") }).unwrap(), ConnectorType::DB9);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB15") }).unwrap(), ConnectorType::DB15);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB25") }).unwrap(), ConnectorType::DB25);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB37") }).unwrap(), ConnectorType::DB37);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB50") }).unwrap(), ConnectorType::DB50);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HD15") }).unwrap(), ConnectorType::HD15);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HD25") }).unwrap(), ConnectorType::HD25);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DIN3") }).unwrap(), ConnectorType::DIN3);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DIN5") }).unwrap(), ConnectorType::DIN5);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC20") }).unwrap(), ConnectorType::EDAC20);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC38") }).unwrap(), ConnectorType::EDAC38);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC56") }).unwrap(), ConnectorType::EDAC56);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC90") }).unwrap(), ConnectorType::EDAC90);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC120") }).unwrap(), ConnectorType::EDAC120);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DL96") }).unwrap(), ConnectorType::DL96);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SCSI68") }).unwrap(), ConnectorType::SCSI68);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"IEE488") }).unwrap(), ConnectorType::IEE488);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CENT50") }).unwrap(), ConnectorType::CENT50);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CENT36") }).unwrap(), ConnectorType::CENT36);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CENT24") }).unwrap(), ConnectorType::CENT24);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DisplayPort") }).unwrap(), ConnectorType::DisplayPort);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DVI") }).unwrap(), ConnectorType::DVI);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HDMI") }).unwrap(), ConnectorType::HDMI);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"PS2") }).unwrap(), ConnectorType::PS2);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"TL-ST") }).unwrap(), ConnectorType::TL_ST);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"LCDUP") }).unwrap(), ConnectorType::LCDUP);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SCDUP") }).unwrap(), ConnectorType::SCDUP);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SC") }).unwrap(), ConnectorType::SC);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"ST") }).unwrap(), ConnectorType::ST);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"NL4") }).unwrap(), ConnectorType::NL4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CACOM") }).unwrap(), ConnectorType::CACOM);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"USB") }).unwrap(), ConnectorType::USB);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"N_CON") }).unwrap(), ConnectorType::N_CON);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"F_CON") }).unwrap(), ConnectorType::F_CON);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"IEC 60320-C7/C8") }).unwrap(), ConnectorType::IEC_60320_C7_C8);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CEE 7/7") }).unwrap(), ConnectorType::CEE_7_7);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"IEC 60320-C13/14") }).unwrap(), ConnectorType::IEC_60320_C13_14);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Edison") }).unwrap(), ConnectorType::Edison);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Wieland") }).unwrap(), ConnectorType::Wieland);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"16A-CEE-2P") }).unwrap(), ConnectorType::_16A_CEE_2P);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"16A-CEE-2P-110") }).unwrap(), ConnectorType::_16A_CEE_2P_110);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"16A-CEE") }).unwrap(), ConnectorType::_16A_CEE);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"32A-CEE") }).unwrap(), ConnectorType::_32A_CEE);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"32A-CEE-2P") }).unwrap(), ConnectorType::_32A_CEE_2P);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"32A-CEE-2P-110") }).unwrap(), ConnectorType::_32A_CEE_2P_110);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"63A-CEE") }).unwrap(), ConnectorType::_63A_CEE);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"125A-CEE") }).unwrap(), ConnectorType::_125A_CEE);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock") }).unwrap(), ConnectorType::Powerlock);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock 120A") }).unwrap(), ConnectorType::Powerlock_120A);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock 400A") }).unwrap(), ConnectorType::Powerlock_400A);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock 660A") }).unwrap(), ConnectorType::Powerlock_660A);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock 800A") }).unwrap(), ConnectorType::Powerlock_800A);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Camlock") }).unwrap(), ConnectorType::Camlock);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"NAC3FCA") }).unwrap(), ConnectorType::NAC3FCA);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"NAC3FCB") }).unwrap(), ConnectorType::NAC3FCB);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"PowerconTRUE1") }).unwrap(), ConnectorType::PowerconTRUE1);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"powerCONTRUE1TOP") }).unwrap(), ConnectorType::PowerconTRUE1TOP);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Socapex-16") }).unwrap(), ConnectorType::Socapex_16);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Socapex-7") }).unwrap(), ConnectorType::Socapex_7);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Socapex-9") }).unwrap(), ConnectorType::Socapex_9);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HAN-16") }).unwrap(), ConnectorType::HAN_16);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HAN-4") }).unwrap(), ConnectorType::HAN_4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"L6-20") }).unwrap(), ConnectorType::L6_20);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"L15-30") }).unwrap(), ConnectorType::L15_30);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Stagepin") }).unwrap(), ConnectorType::Stagepin);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HUBBELL-6-4") }).unwrap(), ConnectorType::HUBBELL_6_4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DIN 56905") }).unwrap(), ConnectorType::DIN_56905);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"") }).unwrap(), ConnectorType::Other(Name::new("").unwrap()));
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Something weird") }).unwrap(), ConnectorType::Other(Name::new("Something weird").unwrap()));
    }
}