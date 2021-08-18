use quick_xml::events::attributes::Attribute;

use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::units::name::Name;

///A list of predefined connectors or Other with Name
#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum ConnectorType {
    ///BNC connector
    Bnc,
    ///Tag block
    Tblk,
    ///Solder tag
    Tag,
    ///Krone block
    Krn,
    ///Stereo jack
    Stj,
    ///Mini stereo jack
    Mstj,
    ///Phono connector
    Rca,
    ///SCART connector
    Scart,
    ///4-pin mini-DIN
    Svideo,
    ///4-pin mini-DIN
    Mdin4,
    ///5-pin mini-DIN
    Mdin5,
    ///6-pin mini-DIN
    Mdin6,
    ///3-pin XLR
    Xlr3,
    ///4-pin XLR
    Xlr4,
    ///5-pin XLR
    Xlr5,
    ///10/100 BaseT ethernet type
    Rj45,
    ///Telephone type
    Rj11,
    ///9-pin D-type
    Db9,
    ///15-pin D-type
    Db15,
    ///25-pin D-type
    Db25,
    ///37-pin D-type
    Db37,
    ///50-pin D-type
    Db50,
    ///15-pin D-type hi-density
    Hd15,
    ///25-pin D-type hi-density
    Hd25,
    ///3-pin DIN
    Din3,
    ///5-pin DIN
    Din5,
    ///EDAC 20-pin
    Edac20,
    ///EDAC 38-pin
    Edac38,
    ///EDAC 56-pin
    Edac56,
    ///EDAC 90-pin
    Edac90,
    ///EDAC 120-pin
    Edac120,
    ///DL 96-pin
    Dl96,
    ///SCSI connector 68-pin
    Scsi68,
    ///IEE488 connector 36-pin
    Iee488,
    ///Centronics 50-pin
    Cent50,
    ///Centronics 36-pin
    Cent36,
    ///Centronics 24-pin
    Cent24,
    ///DisplayPort connector
    DisplayPort,
    ///DVI connector
    Dvi,
    ///HDMI connector
    Hdmi,
    ///PS2 connector
    Ps2,
    ///TosLink connector
    Tl_St,
    ///Fibre optic LC DUPLEX-type
    Lcdup,
    ///Fibre optic SC DUPLEX-type
    Scdup,
    ///Fibre optic SC-type
    Sc,
    ///Fibre optic ST-type
    St,
    ///Speakon
    Nl4,
    ///8-pin LS conn
    Cacom,
    ///USB connector
    Usb,
    ///N connector
    N_Con,
    ///F connector
    F_Con,
    ///Eurostecker
    Iec_60320_C7_C8,
    ///Schutzkontakt
    Cee_7_7,
    ///IEC 60320
    Iec_60320_C13_14,
    ///Edison
    Edison,
    ///Wieland
    Wieland,
    ///16A-Blue
    _16A_Cee_2P,
    ///16A-Yellow
    _16A_Cee_2P_110,
    ///16A-CEE
    _16A_Cee,
    ///32A-CEE
    _32A_Cee,
    ///32A-Blue
    _32A_Cee_2P,
    ///32A-Yellow
    _32A_Cee_2P_110,
    ///63A-CEE
    _63A_Cee,
    ///125A-CEE
    _125A_Cee,
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
    Nac3Fca,
    ///Powercon Grey
    Nac3Fcb,
    ///Powercon TRUE1
    PowerconTrue1,
    ///powerCON TRUE1 TOP
    PowerconTrue1Top,
    ///Socapex-16
    Socapex_16,
    ///Socapex-7
    Socapex_7,
    ///Socapex-9
    Socapex_9,
    ///HAN-16
    Han_16,
    ///HAN-4
    Han_4,
    ///L6-20
    L6_20,
    ///L15-30
    L15_30,
    ///Stagepin
    Stagepin,
    ///HUBBELL 6-4
    Hubbell_6_4,
    ///Eberl
    Din_56905,
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
            "BNC" => ConnectorType::Bnc,
            "TBLK" => ConnectorType::Tblk,
            "TAG" => ConnectorType::Tag,
            "KRN" => ConnectorType::Krn,
            "STJ" => ConnectorType::Stj,
            "MSTJ" => ConnectorType::Mstj,
            "RCA" => ConnectorType::Rca,
            "SCART" => ConnectorType::Scart,
            "SVIDEO" => ConnectorType::Svideo,
            "MDIN4" => ConnectorType::Mdin4,
            "MDIN5" => ConnectorType::Mdin5,
            "MDIN6" => ConnectorType::Mdin6,
            "XLR3" => ConnectorType::Xlr3,
            "XLR4" => ConnectorType::Xlr4,
            "XLR5" => ConnectorType::Xlr5,
            "RJ45" => ConnectorType::Rj45,
            "RJ11" => ConnectorType::Rj11,
            "DB9" => ConnectorType::Db9,
            "DB15" => ConnectorType::Db15,
            "DB25" => ConnectorType::Db25,
            "DB37" => ConnectorType::Db37,
            "DB50" => ConnectorType::Db50,
            "HD15" => ConnectorType::Hd15,
            "HD25" => ConnectorType::Hd25,
            "DIN3" => ConnectorType::Din3,
            "DIN5" => ConnectorType::Din5,
            "EDAC20" => ConnectorType::Edac20,
            "EDAC38" => ConnectorType::Edac38,
            "EDAC56" => ConnectorType::Edac56,
            "EDAC90" => ConnectorType::Edac90,
            "EDAC120" => ConnectorType::Edac120,
            "DL96" => ConnectorType::Dl96,
            "SCSI68" => ConnectorType::Scsi68,
            "IEE488" => ConnectorType::Iee488,
            "CENT50" => ConnectorType::Cent50,
            "CENT36" => ConnectorType::Cent36,
            "CENT24" => ConnectorType::Cent24,
            "DisplayPort" => ConnectorType::DisplayPort,
            "DVI" => ConnectorType::Dvi,
            "HDMI" => ConnectorType::Hdmi,
            "PS2" => ConnectorType::Ps2,
            "TL-ST" => ConnectorType::Tl_St,
            "LCDUP" => ConnectorType::Lcdup,
            "SCDUP" => ConnectorType::Scdup,
            "SC" => ConnectorType::Sc,
            "ST" => ConnectorType::St,
            "NL4" => ConnectorType::Nl4,
            "CACOM" => ConnectorType::Cacom,
            "USB" => ConnectorType::Usb,
            "N_CON" => ConnectorType::N_Con,
            "F_CON" => ConnectorType::F_Con,
            "IEC 60320-C7/C8" => ConnectorType::Iec_60320_C7_C8,
            "CEE 7/7" => ConnectorType::Cee_7_7,
            "IEC 60320-C13/14" => ConnectorType::Iec_60320_C13_14,
            "Edison" => ConnectorType::Edison,
            "Wieland" => ConnectorType::Wieland,
            "16A-CEE-2P" => ConnectorType::_16A_Cee_2P,
            "16A-CEE-2P-110" => ConnectorType::_16A_Cee_2P_110,
            "16A-CEE" => ConnectorType::_16A_Cee,
            "32A-CEE" => ConnectorType::_32A_Cee,
            "32A-CEE-2P" => ConnectorType::_32A_Cee_2P,
            "32A-CEE-2P-110" => ConnectorType::_32A_Cee_2P_110,
            "63A-CEE" => ConnectorType::_63A_Cee,
            "125A-CEE" => ConnectorType::_125A_Cee,
            "Powerlock" => ConnectorType::Powerlock,
            "Powerlock 120A" => ConnectorType::Powerlock_120A,
            "Powerlock 400A" => ConnectorType::Powerlock_400A,
            "Powerlock 660A" => ConnectorType::Powerlock_660A,
            "Powerlock 800A" => ConnectorType::Powerlock_800A,
            "Camlock" => ConnectorType::Camlock,
            "NAC3FCA" => ConnectorType::Nac3Fca,
            "NAC3FCB" => ConnectorType::Nac3Fcb,
            "PowerconTRUE1" => ConnectorType::PowerconTrue1,
            "powerCONTRUE1TOP" => ConnectorType::PowerconTrue1Top,
            "Socapex-16" => ConnectorType::Socapex_16,
            "Socapex-7" => ConnectorType::Socapex_7,
            "Socapex-9" => ConnectorType::Socapex_9,
            "HAN-16" => ConnectorType::Han_16,
            "HAN-4" => ConnectorType::Han_4,
            "L6-20" => ConnectorType::L6_20,
            "L15-30" => ConnectorType::L15_30,
            "Stagepin" => ConnectorType::Stagepin,
            "HUBBELL-6-4" => ConnectorType::Hubbell_6_4,
            "DIN 56905" => ConnectorType::Din_56905,
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
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"BNC") }).unwrap(), ConnectorType::Bnc);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"TBLK") }).unwrap(), ConnectorType::Tblk);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"TAG") }).unwrap(), ConnectorType::Tag);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"KRN") }).unwrap(), ConnectorType::Krn);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"STJ") }).unwrap(), ConnectorType::Stj);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"MSTJ") }).unwrap(), ConnectorType::Mstj);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"RCA") }).unwrap(), ConnectorType::Rca);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SCART") }).unwrap(), ConnectorType::Scart);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SVIDEO") }).unwrap(), ConnectorType::Svideo);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"MDIN4") }).unwrap(), ConnectorType::Mdin4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"MDIN5") }).unwrap(), ConnectorType::Mdin5);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"MDIN6") }).unwrap(), ConnectorType::Mdin6);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"XLR3") }).unwrap(), ConnectorType::Xlr3);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"XLR4") }).unwrap(), ConnectorType::Xlr4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"XLR5") }).unwrap(), ConnectorType::Xlr5);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"RJ45") }).unwrap(), ConnectorType::Rj45);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"RJ11") }).unwrap(), ConnectorType::Rj11);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB9") }).unwrap(), ConnectorType::Db9);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB15") }).unwrap(), ConnectorType::Db15);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB25") }).unwrap(), ConnectorType::Db25);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB37") }).unwrap(), ConnectorType::Db37);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DB50") }).unwrap(), ConnectorType::Db50);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HD15") }).unwrap(), ConnectorType::Hd15);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HD25") }).unwrap(), ConnectorType::Hd25);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DIN3") }).unwrap(), ConnectorType::Din3);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DIN5") }).unwrap(), ConnectorType::Din5);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC20") }).unwrap(), ConnectorType::Edac20);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC38") }).unwrap(), ConnectorType::Edac38);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC56") }).unwrap(), ConnectorType::Edac56);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC90") }).unwrap(), ConnectorType::Edac90);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"EDAC120") }).unwrap(), ConnectorType::Edac120);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DL96") }).unwrap(), ConnectorType::Dl96);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SCSI68") }).unwrap(), ConnectorType::Scsi68);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"IEE488") }).unwrap(), ConnectorType::Iee488);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CENT50") }).unwrap(), ConnectorType::Cent50);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CENT36") }).unwrap(), ConnectorType::Cent36);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CENT24") }).unwrap(), ConnectorType::Cent24);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DisplayPort") }).unwrap(), ConnectorType::DisplayPort);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DVI") }).unwrap(), ConnectorType::Dvi);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HDMI") }).unwrap(), ConnectorType::Hdmi);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"PS2") }).unwrap(), ConnectorType::Ps2);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"TL-ST") }).unwrap(), ConnectorType::Tl_St);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"LCDUP") }).unwrap(), ConnectorType::Lcdup);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SCDUP") }).unwrap(), ConnectorType::Scdup);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"SC") }).unwrap(), ConnectorType::Sc);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"ST") }).unwrap(), ConnectorType::St);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"NL4") }).unwrap(), ConnectorType::Nl4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CACOM") }).unwrap(), ConnectorType::Cacom);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"USB") }).unwrap(), ConnectorType::Usb);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"N_CON") }).unwrap(), ConnectorType::N_Con);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"F_CON") }).unwrap(), ConnectorType::F_Con);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"IEC 60320-C7/C8") }).unwrap(), ConnectorType::Iec_60320_C7_C8);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"CEE 7/7") }).unwrap(), ConnectorType::Cee_7_7);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"IEC 60320-C13/14") }).unwrap(), ConnectorType::Iec_60320_C13_14);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Edison") }).unwrap(), ConnectorType::Edison);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Wieland") }).unwrap(), ConnectorType::Wieland);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"16A-CEE-2P") }).unwrap(), ConnectorType::_16A_Cee_2P);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"16A-CEE-2P-110") }).unwrap(), ConnectorType::_16A_Cee_2P_110);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"16A-CEE") }).unwrap(), ConnectorType::_16A_Cee);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"32A-CEE") }).unwrap(), ConnectorType::_32A_Cee);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"32A-CEE-2P") }).unwrap(), ConnectorType::_32A_Cee_2P);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"32A-CEE-2P-110") }).unwrap(), ConnectorType::_32A_Cee_2P_110);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"63A-CEE") }).unwrap(), ConnectorType::_63A_Cee);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"125A-CEE") }).unwrap(), ConnectorType::_125A_Cee);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock") }).unwrap(), ConnectorType::Powerlock);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock 120A") }).unwrap(), ConnectorType::Powerlock_120A);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock 400A") }).unwrap(), ConnectorType::Powerlock_400A);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock 660A") }).unwrap(), ConnectorType::Powerlock_660A);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Powerlock 800A") }).unwrap(), ConnectorType::Powerlock_800A);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Camlock") }).unwrap(), ConnectorType::Camlock);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"NAC3FCA") }).unwrap(), ConnectorType::Nac3Fca);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"NAC3FCB") }).unwrap(), ConnectorType::Nac3Fcb);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"PowerconTRUE1") }).unwrap(), ConnectorType::PowerconTrue1);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"powerCONTRUE1TOP") }).unwrap(), ConnectorType::PowerconTrue1Top);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Socapex-16") }).unwrap(), ConnectorType::Socapex_16);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Socapex-7") }).unwrap(), ConnectorType::Socapex_7);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Socapex-9") }).unwrap(), ConnectorType::Socapex_9);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HAN-16") }).unwrap(), ConnectorType::Han_16);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HAN-4") }).unwrap(), ConnectorType::Han_4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"L6-20") }).unwrap(), ConnectorType::L6_20);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"L15-30") }).unwrap(), ConnectorType::L15_30);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Stagepin") }).unwrap(), ConnectorType::Stagepin);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"HUBBELL-6-4") }).unwrap(), ConnectorType::Hubbell_6_4);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"DIN 56905") }).unwrap(), ConnectorType::Din_56905);
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"") }).unwrap(), ConnectorType::Other(Name::new("").unwrap()));
        assert_eq!(ConnectorType::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Something weird") }).unwrap(), ConnectorType::Other(Name::new("Something weird").unwrap()));
    }
}