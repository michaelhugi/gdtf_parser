//!AttributeName is a preferred Name for Attributes in GDTF Format.
#![allow(non_camel_case_types)]

use std::str::FromStr;

use lazy_static::lazy_static;
use quick_xml::events::attributes::Attribute;
use regex::{Regex, RegexSet, SetMatches};

use crate::utils::deparse;
use crate::utils::units::name::{GdtfNameError, Name};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
///AttributeName is an enum for preferred Names used in GDTF for Attributes. It contains an option UserDefined(Name) which can contain all other Names for Atttribute
pub enum AttributeName {
    ///Fallback if a user-defined Name for an Attribute was used
    UserDefined(Name),
    ///Controls the intensity of a fixture.
    Dimmer,
    ///Controls the fixture’s sideward movement (horizontal axis).
    Pan,
    ///Controls the fixture’s upward and the downward movement (vertical axis).
    Tilt,
    ///Controls the speed of the fixture’s continuous pan movement (horizontal axis).
    PanRotate,
    ///Controls the speed of the fixture’s continuous tilt movement (vertical axis).
    TiltRotate,
    ///Selects the predefined position effects that are built into the fixture.
    PositionEffect,
    ///Controls the speed of the predefined position effects that are built into the fixture.
    PositionEffectRate,
    ///Snaps or smooth fades with timing in running predefined position effects.
    PositionEffectFade,
    ///Defines a fixture’s x-coordinate within an XYZ coordinate system.
    Xyz_X,
    ///Defines a fixture’s y-coordinate within an XYZ coordinate system.
    Xyz_Y,
    ///Defines a fixture‘s z-coordinate within an XYZ coordinate system.
    Xyz_Z,
    ///Defines rotation around X axis.
    Rot_X,
    ///Defines rotation around Y axis.
    Rot_Y,
    ///Defines rotation around Z axis.
    Rot_Z,
    ///Scaling on X axis.
    Scale_X,
    ///Scaling on Y axis.
    Scale_Y,
    ///Scaling on Y axis.
    Scale_Z,
    ///Unified scaling on all axes.
    Scale_Xyz,
    ///The fixture’s gobo wheel _n_. This is the main attribute of gobo wheel’s _n_ wheel control. Selects gobos in gobo wheel _n_. A different channel function sets the angle of the indexed position in the selected gobo or the angular speed of its continuous rotation.
    Gobo_n_(u8),
    ///Selects gobos whose rotation is continuous in gobo wheel _n_ and controls the angular speed of the gobo’s spin within the same channel function.
    Gobo_n_SelectSpin(u8),
    ///Selects gobos which shake in gobo wheel _n_ and controls the frequency of the gobo’s shake within the same channel function.
    Gobo_n_SelectShake(u8),
    ///Selects gobos which run effects in gobo wheel _n_.
    Gobo_n_SelectEffects(u8),
    ///Controls angle of indexed rotation of gobo wheel _n_.
    Gobo_n_WheelIndex(u8),
    ///Controls the speed and direction of continuous rotation of gobo wheel _n_.
    Gobo_n_WheelSpin(u8),
    ///Controls frequency of the shake of gobo wheel _n_.
    Gobo_n_WheelShake(u8),
    ///Controls speed of gobo wheel’s _n_ random gobo slot selection.
    Gobo_n_WheelRandom(u8),
    ///Controls audio-controlled functionality of gobo wheel _n_.
    Gobo_n_WheelAudio(u8),
    ///Controls angle of indexed rotation of gobos in gobo wheel _n_. This is the main attribute of gobo wheel’s _n_ wheel slot control.
    Gobo_n_Pos(u8),
    ///Controls the speed and direction of continuous rotation of gobos in gobo wheel _n_.
    Gobo_n_PosRotate(u8),
    ///Controls frequency of the shake of gobos in gobo wheel _n_.
    Gobo_n_PosShake(u8),
    ///This is the main attribute of the animation wheel’s _n_ wheel control. Selects slots in the animation wheel. A different channel function sets the angle of the indexed position in the selected slot or the angular speed of its continuous rotation. Is used for animation effects with multiple slots.
    AnimationWheel_n_(u8),
    ///Controls audio-controlled functionality of animation wheel _n_.
    AnimationWheel_n_Audio(u8),
    ///Selects predefined effects in animation wheel _n_.
    AnimationWheel_n_Macro(u8),
    ///Controls frequency of animation wheel _n_ random slot selection.
    AnimationWheel_n_Random(u8),
    ///Selects slots which run effects in animation wheel _n_.
    AnimationWheel_n_SelectEffects(u8),
    ///Selects slots which shake in animation wheel and controls the frequency of the slots shake within the same channel function.
    AnimationWheel_n_SelectShake(u8),
    ///Selects slots whose rotation is continuous in animation wheel and controls the angular speed of the slot spin within the same channel function
    AnimationWheel_n_SelectSpin(u8),
    ///Controls angle of indexed rotation of slots in animation wheel. This is the main attribute of animation wheel _n_ wheel slot control.
    AnimationWheel_n_Pos(u8),
    ///Controls the speed and direction of continuous rotation of slots in animation wheel _n_.
    AnimationWheel_n_PosRotate(u8),
    ///Controls frequency of the shake of slots in animation wheel _n_.
    AnimationWheel_n_PosShake(u8),
    ///This is the main attribute of the animation system insertion control. Controls the insertion of the fixture's animation system in the light output. Is used for animation effects where a disk is inserted into the light output.
    AnimationSystem_n_(u8),
    ///Sets frequency of animation system _n_ insertion ramp.
    AnimationSystem_n_Ramp(u8),
    ///Sets frequency of animation system _n_ insertion shake.
    AnimationSystem_n_Shake(u8),
    ///Controls audio-controlled functionality of animation system _n_ insertion.
    AnimationSystem_n_Audio(u8),
    ///Controls frequency of animation system _n_ random insertion.
    AnimationSystem_n_Random(u8),
    ///This is the main attribute of the animation system spinning control. Controls angle of indexed rotation of animation system _n_ disk.
    AnimationSystem_n_Pos(u8),
    ///Controls the speed and direction of continuous rotation of animation system _n_ disk.
    AnimationSystem_n_PosRotate(u8),
    ///Controls frequency of the shake of animation system _n_ disk.
    AnimationSystem_n_PosShake(u8),
    ///Controls random speed of animation system _n_ disk.
    AnimationSystem_n_PosRandom(u8),
    ///Controls audio-controlled functionality of animation system _n_ disk.
    AnimationSystem_n_PosAudio(u8),
    ///Selects predefined effects in animation system _n_.
    AnimationSystem_n_Macro(u8),
    ///Selects folder that contains media content.
    MediaFolder_n_(u8),
    ///Selects file with media content.
    MediaContent_n_(u8),
    ///Selects folder that contains 3D model content. For example 3D meshes for mapping.
    ModelFolder_n_(u8),
    ///Selects file with 3D model content.
    ModelContent_n_(u8),
    ///Defines media playback mode.
    PlayMode,
    ///Defines starting point of media content playback.
    PlayBegin,
    ///Defines end point of media content playback.
    PlayEnd,
    ///Adjusts playback speed of media content.
    PlaySpeed,
    ///Selects predefined color effects built into the fixture.
    ColorEffects_n_(u8),
    ///The fixture’s color wheel _n_. Selects colors in color wheel _n_. This is the main attribute of color wheel’s _n_ wheel control.
    Color_n_(u8),
    ///Controls angle of indexed rotation of color wheel _n_
    Color_n_WheelIndex(u8),
    ///Controls the speed and direction of continuous rotation of color wheel _n_.
    Color_n_WheelSpin(u8),
    ///Controls frequency of color wheel’s _n_ random color slot selection.
    Color_n_WheelRandom(u8),
    ///Controls audio-controlled functionality of color wheel _n_.
    Color_n_WheelAudio(u8),
    ///Controls the intensity of the fixture’s red emitters for direct additive color mixing.
    ColorAdd_R,
    ///Controls the intensity of the fixture’s green emitters for direct additive color mixing
    ColorAdd_G,
    ///Controls the intensity of the fixture’s blue emitters for direct additive color mixing.
    ColorAdd_B,
    ///Controls the intensity of the fixture’s cyan emitters for direct additive color mixing.
    ColorAdd_C,
    ///Controls the intensity of the fixture’s magenta emitters for direct additive color mixing.
    ColorAdd_M,
    ///Controls the intensity of the fixture’s yellow emitters for direct additive color mixing.
    ColorAdd_Y,
    ///Controls the intensity of the fixture’s amber emitters for direct additive color mixing.
    ColorAdd_Ry,
    ///Controls the intensity of the fixture’s lime emitters for direct additive color mixing.
    ColorAdd_Gy,
    ///Controls the intensity of the fixture’s blue-green emitters for direct additive color mixing.
    ColorAdd_Gc,
    ///Controls the intensity of the fixture’s light-blue emitters for direct additive color mixing.
    ColorAdd_Bc,
    ///Controls the intensity of the fixture’s purple emitters for direct additive color mixing.
    ColorAdd_Bm,
    ///Controls the intensity of the fixture’s pink emitters for direct additive color mixing.
    ColorAdd_Rm,
    ///Controls the intensity of the fixture’s white emitters for direct additive color mixing.
    ColorAdd_W,
    ///Controls the intensity of the fixture’s warm white emitters for direct additive color mixing.
    ColorAdd_Ww,
    ///Controls the intensity of the fixture’s cool white emitters for direct additive color mixing.
    ColorAdd_Cw,
    ///Controls the intensity of the fixture’s UV emitters for direct additive color mixing.
    ColorAdd_Uv,
    ///Controls the insertion of the fixture’s red filter flag for direct subtractive color mixing.
    ColorSub_R,
    ///Controls the insertion of the fixture’s green filter flag for direct subtractive color mixing.
    ColorSub_G,
    ///Controls the insertion of the fixture’s blue filter flag for direct subtractive color mixing.
    ColorSub_B,
    ///Controls the insertion of the fixture’s cyan filter flag for direct subtractive color mixing.
    ColorSub_C,
    ///Controls the insertion of the fixture’s magenta filter flag for direct subtractive color mixing.
    ColorSub_M,
    ///Controls the insertion of the fixture’s yellow filter flag for direct subtractive color mixing.
    ColorSub_Y,
    ///Selects predefined colors that are programed in the fixture’s firmware.
    ColorMacro_n_(u8),
    ///Controls the time between Color Macro steps.
    ColorMacro_n_Rate(u8),
    ///Controls the fixture’s “Correct to orange” wheel or mixing system.
    Cto,
    ///Controls the fixture’s “Correct to color” wheel or mixing system.
    Ctc,
    ///Controls the fixture’s “Correct to blue” wheel or mixing system.
    Ctb,
    ///Controls the fixture’s “Correct green to magenta” wheel or mixing system.
    Tint,
    ///Controls the fixture’s color attribute regarding the hue.
    Hsb_Hue,
    ///Controls the fixture’s color attribute regarding the saturation.
    Hsb_Saturation,
    ///Controls the fixture’s color attribute regarding the brightness.
    Hsb_Brightness,
    ///Controls the fixture’s color attribute regarding the quality.
    Hsb_Quality,
    ///Controls the fixture’s CIE 1931 color attribute regarding the chromaticity x.
    Cie_X,
    ///Controls the fixture’s CIE 1931 color attribute regarding the chromaticity y.
    Cie_Y,
    ///Controls the fixture’s CIE 1931 color attribute regarding the brightness (Y).
    Cie_Brightness,
    ///Controls the fixture’s red attribute for indirect RGB color mixing.
    ColorRgb_Red,
    ///Controls the fixture’s green attribute for indirect RGB color mixing.
    ColorRgb_Green,
    ///Controls the fixture’s blue attribute for indirect RGB color mixing.
    ColorRgb_Blue,
    ///Controls the fixture’s cyan attribute for indirect CMY color mixing.
    ColorRgb_Cyan,
    ///Controls the fixture’s magenta attribute for indirect CMY color mixing.
    ColorRgb_Magenta,
    ///Controls the fixture’s yellow attribute for indirect CMY color mixing.
    ColorRgb_Yellow,
    ///Controls the fixture’s quality attribute for indirect color mixing.
    ColorRgb_Quality,
    ///Adjusts color boost red of content.
    VideoBoost_R,
    ///Adjusts color boost green of content.
    VideoBoost_G,
    ///Adjusts color boost blue of content.
    VideoBoost_B,
    ///Adjusts color hue shift of content.
    VideoHueShift,
    ///Adjusts saturation of content.
    VideoSaturation,
    ///Adjusts brightness of content.
    VideoBrightness,
    ///Adjusts contrast of content.
    VideoContrast,
    ///Adjusts red color for color keying.
    VideoKeyColor_R,
    ///Adjusts green color for color keying.
    VideoKeyColor_G,
    ///Adjusts blue color for color keying.
    VideoKeyColor_B,
    ///Adjusts intensity of color keying.
    VideoKeyIntensity,
    ///Adjusts tolerance of color keying.
    VideoKeyTolerance,
    ///Controls the length of a strobe flash.
    StrobeDuration,
    ///Controls the time between strobe flashes.
    StrobeRate,
    ///Controls the fixture’s mechanical or electronical shutter feature.
    Shutter_n_(u8),
    ///Controls the frequency of the fixture’s mechanical or electronical strobe shutter feature.
    Shutter_n_Strobe(u8),
    ///Controls the frequency of the fixture’s mechanical or electronical pulse shutter feature.
    Shutter_n_StrobePulse(u8),
    ///Controls the frequency of the fixture’s mechanical or electronical closing pulse shutter feature.
    Shutter_n_StrobePulseClose(u8),
    ///Controls the frequency of the fixture’s mechanical or electronical opening pulse shutter feature.
    Shutter_n_StrobePulseOpen(u8),
    ///Controls the frequency of the fixture’s mechanical or electronical random strobe shutter feature.
    Shutter_n_StrobeRandom(u8),
    ///Controls the frequency of the fixture’s mechanical or electronical random pulse shutter feature.
    Shutter_n_StrobeRandomPulse(u8),
    ///Controls the frequency of the fixture’s mechanical or electronical random closing pulse shutter feature.
    Shutter_n_StrobeRandomPulseClose(u8),
    ///Controls the frequency of the fixture’s mechanical or electronical random opening pulse shutter feature.
    Shutter_n_StrobeRandomPulseOpen(u8),
    ///Controls the frequency of the fixture’s mechanical or electronical shutter effect feature.
    Shutter_n_StrobeEffect(u8),
    ///Controls the diameter of the fixture’s beam.
    Iris,
    ///Sets frequency of the iris’s strobe feature.
    IrisStrobe,
    ///Sets frequency of the iris’s random movement.
    IrisStrobeRandom,
    ///Sets frequency of iris’s closing pulse.
    IrisPulseClose,
    ///Sets frequency of iris’s opening pulse.
    IrisPulseOpen,
    ///Sets frequency of iris’s random closing pulse.
    IrisRandomPulseClose,
    ///Sets frequency of iris’s random opening pulse.
    IrisRandomPulseOpen,
    ///The ability to soften the fixture’s spot light with a frosted lens.
    Frost_n_(u8),
    ///Sets frequency of frost’s opening pulse
    Frost_n_PulseOpen(u8),
    ///Sets frequency of frost’s closing pulse.
    Frost_n_PulseClose(u8),
    ///Sets frequency of frost’s ramp.
    Frost_n_Ramp(u8),
    ///The fixture’s prism wheel _n_. Selects prisms in prism wheel _n_. A different channel function sets the angle of the indexed position in the selected prism or the angular speed of its continuous rotation. This is the main attribute of prism wheel’s _n_ wheel control.
    Prism_n_(u8),
    ///Selects prisms whose rotation is continuous in prism wheel _n_ and controls the angular speed of the prism’s spin within the same channel function.
    Prism_n_SelectSpin(u8),
    ///Macro functions of prism wheel _n_.
    Prism_n_Macro(u8),
    ///Controls angle of indexed rotation of prisms in prism wheel _n_. This is the main attribute of prism wheel’s 1 wheel slot control.
    Prism_n_Pos(u8),
    ///Controls the speed and direction of continuous rotation of prisms in prism wheel _n_.
    Prism_n_PosRotate(u8),
    ///Generically predefined macros and effects of a fixture.
    Effects_n_(u8),
    ///Frequency of running effects.
    Effects_n_Rate(u8),
    ///Snapping or smooth look of running effects.
    Effects_n_Fade(u8),
    ///Controls parameter (m) of effect _n_
    Effects_n_Adjust_m_(u8, u8),
    ///Controls angle of indexed rotation of slot/effect in effect wheel/macro _n_. This is the main attribute of effect wheel/macro _n_ slot/effect control.
    Effects_n_Pos(u8),
    ///Controls speed and direction of slot/effect in effect wheel _n_.
    Effects_n_PosRotate(u8),
    ///Sets offset between running effects and effects 2.
    EffectsSync,
    ///Activates fixture’s beam shaper.
    BeamShaper,
    ///Predefined presets for fixture’s beam shaper positions.
    BeamShaperMacro,
    ///Indexing of fixture’s beam shaper.
    BeamShaperPos,
    ///Continuous rotation of fixture’s beam shaper.
    BeamShaperPosRotate,
    ///Controls the spread of the fixture’s beam/spot.
    Zoom,
    ///Selects spot mode of zoom.
    ZoomModeSpot,
    ///Selects beam mode of zoom.
    ZoomModeBeam,
    ///Controls the sharpness of the fixture’s spot light. Can blur or sharpen the edge of the spot.
    Focus_n_(u8),
    ///Autofocuses functionality using presets.
    Focus_n_Adjust(u8),
    ///Autofocuses functionality using distance.
    Focus_n_Distance(u8),
    ///Controls the channel of a fixture.
    Control_n_(u8),
    ///Selects different modes of intensity.
    DimmerMode,
    ///Selects different dimmer curves of the fixture.
    DimmerCurve,
    ///Closes the light output under certain conditions (movement correction, gobo movement, etc.).
    BlackoutMode,
    ///Controls LED frequency.
    LedFrequency,
    ///Changes zones of LEDs.
    LedZoneMode,
    ///Controls behavior of LED pixels.
    PixelMode,
    ///Selects fixture’s pan mode. Selects between a limited pan range (e.g. −270 to 270) or a continuous pan range.
    PanMode,
    ///Selects fixture’s pan mode. Selects between a limited tilt range (e.g. −130 to 130) or a continuous tilt range.
    TiltMode,
    ///Selects fixture’s pan/tilt mode. Selects between a limited pan/tilt range or a continuous pan/tilt range.
    PanTiltMode,
    ///Selects the fixture’s position mode.
    PositionModes,
    ///Changes control between selecting, indexing, and rotating the gobos of gobo wheel _n_.
    Gobo_n_WheelMode(u8),
    ///Changes control between selecting, indexing, and rotating the slots of animation wheel _n_.
    AnimationWheel_n_Mode(u8),
    ///Defines whether the animation wheel takes the shortest distance between two positions.
    AnimationWheelShortcutMode,
    ///Changes control between selecting, continuous selection, half selection, random selection, color spinning, etc. in colors of color wheel _n_.
    Color_n_Mode(u8),
    ///Defines whether the color wheel takes the shortest distance between two colors.
    ColorWheelShortcutMode,
    ///Controls how Cyan is used within the fixture’s cyan CMY-mixing feature.
    CyanMode,
    ///Controls how Cyan is used within the fixture’s magenta CMY-mixing.
    MagentaMode,
    ///Controls how Cyan is used within the fixture’s yellow CMY-mixing feature.
    YellowMode,
    ///Changes control between selecting continuous selection, half selection, random selection, color spinning, etc. in color mixing.
    ColorMixMode,
    ///Selects chromatic behavior of the device.
    ChromaticMode,
    ///Sets calibration mode (for example on/off).
    ColorCalibrationMode,
    ///Controls consistent behavior of color.
    ColorConsistency,
    ///Controls special color related functions.
    ColorControl,
    ///Controls color model (CMY/RGB/HSV).
    ColorModelMode,
    ///Resets settings of color control channel.
    ColorSettingsReset,
    ///Controls behavior of color uniformity.
    ColorUniformity,
    ///Controls CRI settings of output.
    CriMode,
    ///Custom color related functions (save, recall).
    CustomColor,
    ///Settings for UV stability color behavior.
    UvStability,
    ///Settings for WaveLength corrections of colors.
    WavelengthCorrection,
    ///Controls if White LED is proportionally added to RGB.
    WhiteCount,
    ///Changes strobe style (strobe, pulse, random strobe, etc.) of the shutter attribute.
    StrobeMode,
    ///Changes modes of the fixture’s zoom.
    ZoomMode,
    ///Changes modes of the fixture’s focus – manual or auto-focus.
    FocusMode,
    ///Changes modes of the fixture’s iris – linear, strobe, pulse.
    IrisMode,
    ///Controls fan _n_ mode.
    Fan_n_Mode(u8),
    ///Selects follow spot control mode.
    FollowSpotMode,
    ///Changes mode to control either index or rotation of the beam effects.
    BeamEffectIndexRotateMode,
    ///Movement speed of the fixture’s intensity.
    IntensityMSpeed,
    ///Movement speed of the fixture’s pan/tilt.
    PositionMSpeed,
    ///Movement speed of the fixture’s ColorMix presets.
    ColorMixMSpeed,
    ///Movement speed of the fixture’s color wheel.
    ColorWheelSelectMSpeed,
    ///Movement speed of the fixture’s gobo wheel _n_.
    GoboWheel_n_MSpeed(u8),
    ///Movement speed of the fixture’s iris.
    IrisMSpeed,
    ///Movement speed of the fixture’s prism wheel _n_.
    Prism_n_MSpeed(u8),
    ///Movement speed of the fixture’s focus.
    FocusMSpeed,
    ///Movement speed of the fixture’s frost _n_.
    Frost_n_MSpeed(u8),
    ///Movement speed of the fixture’s zoom.
    ZoomMSpeed,
    ///Movement speed of the fixture’s shapers.
    FrameMSpeed,
    ///General speed of fixture’s features.
    GlobalMSpeed,
    ///Movement speed of the fixture’s frost.
    ReflectorAdjust,
    ///Generally resets the entire fixture.
    FixtureGlobalReset,
    ///Resets the fixture’s shutter.
    ShutterReset,
    ///Resets the fixture’s beam features.
    BeamReset,
    ///Resets the fixture’s color mixing system.
    ColorMixReset,
    ///Resets the fixture’s color wheel.
    ColorWheelReset,
    ///Resets the fixture’s focus.
    FocusReset,
    ///Resets the fixture’s shapers.
    FrameReset,
    ///Resets the fixture’s gobo wheel.
    GoboWheelReset,
    ///Resets the fixture’s intensity.
    IntensityReset,
    ///Resets the fixture’s iris.
    IrisReset,
    ///Resets the fixture’s pan/tilt.
    PositionReset,
    ///Resets the fixture’s pan.
    PanReset,
    ///Resets the fixture’s tilt.
    TiltReset,
    ///Resets the fixture’s zoom.
    ZoomReset,
    ///Resets the fixture’s CTB.
    CtbReset,
    ///Resets the fixture’s CTO.
    CtoReset,
    ///Resets the fixture’s CTC.
    CtcReset,
    ///Resets the fixture's animation system features.
    AnimationSystemReset,
    ///Resets the fixture’s calibration.
    FixtureCalibrationReset,
    ///Generally controls features of the fixture.
    Function,
    ///Controls the fixture’s lamp on/lamp off feature.
    LampControl,
    ///Adjusts intensity of display
    DisplayIntensity,
    ///Selects DMX Input
    DmxInput,
    ///Ranges without a functionality.
    NoFeature,
    ///Fog or hazer‘s blower feature.
    Blower_n_(u8),
    ///Fog or hazer’s Fan feature.
    Fan_n_(u8),
    ///Fog or hazer’s Fog feature.
    Fog_n_(u8),
    ///Fog or hazer’s haze feature.
    Haze_n_(u8),
    ///Controls the energy consumption of the lamp.
    LampPowerMode,
    ///Fancontrols a fixture or device.
    Fans,
    ///1 of 2 shutters that shape the top/right/bottom/left of the beam.
    Blade_n_A(u8),
    ///2 of 2 shutters that shape the top/right/bottom/left of the beam.
    Blade_n_B(u8),
    ///Rotates position of blade_n_.
    Blade_n_Rot(u8),
    ///Rotates position of blade assembly.
    ShaperRot,
    ///Predefined presets for shaper positions.
    ShaperMacros,
    ///Speed of predefined effects on shapers.
    ShaperMacrosSpeed,
    ///1 of 2 soft edge blades that shape the top/right/bottom/left of the beam.
    BladeSoft_n_A(u8),
    ///2 of 2 soft edge blades that shape the top/right/bottom/left of the beam.
    BladeSoft_n_B(u8),
    ///1 of 2 corners that shape the top/right/bottom/left of the beam.
    KeyStone_n_A(u8),
    ///2 of 2 corners that shape the top/right/bottom/left of the beam.
    KeyStone_n_B(u8),
    ///Controls video features.
    Video,
    ///Selects dedicated effects which are used for media.
    VideoEffect_n_Type(u8),
    ///Controls parameter (m) of VideoEffect_n_Type.
    VideoEffect_n_Parameter_m_(u8, u8),
    ///Selects the video camera_n_.
    VideoCamera_n_(u8),
    ///Adjusts sound volume.
    VideoSoundVolume_n_(u8),
    ///Defines mode of video blending.
    VideoBlendMode,
    ///Defines media input source e.g. a camera input.
    InputSource,
    ///Defines field of view.
    FieldOfView,
}

///```rust
/// use gdtf_parser::utils::units::attribute_name::AttributeName;
/// use gdtf_parser::utils::units::name::Name;
/// assert_eq!(AttributeName::UserDefined(Name("".to_string())), Default::default())
///```
impl Default for AttributeName {
    fn default() -> Self {
        AttributeName::new_from_str("").unwrap()
    }
}


impl AttributeName {
    ///Parses a string from gdtf-xml-description into an AttributeName. Only chars `[32..=122] = (SPACE..='z')` are allowed. If one of these chars is passed to the function, it will return an Error
    ///```rust
    /// use gdtf_parser::utils::units::attribute_name::AttributeName;
    /// use gdtf_parser::utils::units::name::Name;
    /// assert_eq!(AttributeName::new_from_str("Tilt").unwrap(), AttributeName::Tilt);
    /// assert_eq!(AttributeName::new_from_str("PanTiltMode").unwrap(), AttributeName::PanTiltMode);
    /// assert_eq!(AttributeName::new_from_str("Effects1Adjust2").unwrap(), AttributeName::Effects_n_Adjust_m_(1,2));
    /// assert_eq!(AttributeName::new_from_str("Something else").unwrap(), AttributeName::UserDefined(Name("Something else".to_string())));
    /// assert!(AttributeName::new_from_str("Name with invalid char {").is_err());
    /// assert!(AttributeName::new_from_str("Name with invalid char ȸ").is_err());
    ///```
    pub fn new_from_str(value: &str) -> Result<Self, GdtfNameError> {
        use AttributeName::*;
        Ok(match value {
            "Dimmer" => Dimmer,
            "Pan" => Pan,
            "Tilt" => Tilt,
            "PanRotate" => PanRotate,
            "TiltRotate" => TiltRotate,
            "PositionEffect" => PositionEffect,
            "PositionEffectRate" => PositionEffectRate,
            "PositionEffectFade" => PositionEffectFade,
            "XYZ_X" => Xyz_X,
            "XYZ_Y" => Xyz_Y,
            "XYZ_Z" => Xyz_Z,
            "Rot_X" => Rot_X,
            "Rot_Y" => Rot_Y,
            "Rot_Z" => Rot_Z,
            "Scale_X" => Scale_X,
            "Scale_Y" => Scale_Y,
            "Scale_Z" => Scale_Z,
            "Scale_XYZ" => Scale_Xyz,
            "PlayMode" => PlayMode,
            "PlayBegin" => PlayBegin,
            "PlayEnd" => PlayEnd,
            "PlaySpeed" => PlaySpeed,
            "ColorAdd_R" => ColorAdd_R,
            "ColorAdd_G" => ColorAdd_G,
            "ColorAdd_B" => ColorAdd_B,
            "ColorAdd_C" => ColorAdd_C,
            "ColorAdd_M" => ColorAdd_M,
            "ColorAdd_Y" => ColorAdd_Y,
            "ColorAdd_RY" => ColorAdd_Ry,
            "ColorAdd_GY" => ColorAdd_Gy,
            "ColorAdd_GC" => ColorAdd_Gc,
            "ColorAdd_BC" => ColorAdd_Bc,
            "ColorAdd_BM" => ColorAdd_Bm,
            "ColorAdd_RM" => ColorAdd_Rm,
            "ColorAdd_W" => ColorAdd_W,
            "ColorAdd_WW" => ColorAdd_Ww,
            "ColorAdd_CW" => ColorAdd_Cw,
            "ColorAdd_UV" => ColorAdd_Uv,
            "ColorSub_R" => ColorSub_R,
            "ColorSub_G" => ColorSub_G,
            "ColorSub_B" => ColorSub_B,
            "ColorSub_C" => ColorSub_C,
            "ColorSub_M" => ColorSub_M,
            "ColorSub_Y" => ColorSub_Y,
            "CTO" => Cto,
            "CTC" => Ctc,
            "CTB" => Ctb,
            "Tint" => Tint,
            "HSB_Hue" => Hsb_Hue,
            "HSB_Saturation" => Hsb_Saturation,
            "HSB_Brightness" => Hsb_Brightness,
            "HSB_Quality" => Hsb_Quality,
            "CIE_X" => Cie_X,
            "CIE_Y" => Cie_Y,
            "CIE_Brightness" => Cie_Brightness,
            "ColorRGB_Red" => ColorRgb_Red,
            "ColorRGB_Green" => ColorRgb_Green,
            "ColorRGB_Blue" => ColorRgb_Blue,
            "ColorRGB_Cyan" => ColorRgb_Cyan,
            "ColorRGB_Magenta" => ColorRgb_Magenta,
            "ColorRGB_Yellow" => ColorRgb_Yellow,
            "ColorRGB_Quality" => ColorRgb_Quality,
            "VideoBoost_R" => VideoBoost_R,
            "VideoBoost_G" => VideoBoost_G,
            "VideoBoost_B" => VideoBoost_B,
            "VideoHueShift" => VideoHueShift,
            "VideoSaturation" => VideoSaturation,
            "VideoBrightness" => VideoBrightness,
            "VideoContrast" => VideoContrast,
            "VideoKeyColor_R" => VideoKeyColor_R,
            "VideoKeyColor_G" => VideoKeyColor_G,
            "VideoKeyColor_B" => VideoKeyColor_B,
            "VideoKeyIntensity" => VideoKeyIntensity,
            "VideoKeyTolerance" => VideoKeyTolerance,
            "StrobeDuration" => StrobeDuration,
            "StrobeRate" => StrobeRate,
            "Iris" => Iris,
            "IrisStrobe" => IrisStrobe,
            "IrisStrobeRandom" => IrisStrobeRandom,
            "IrisPulseClose" => IrisPulseClose,
            "IrisPulseOpen" => IrisPulseOpen,
            "IrisRandomPulseClose" => IrisRandomPulseClose,
            "IrisRandomPulseOpen" => IrisRandomPulseOpen,
            "EffectsSync" => EffectsSync,
            "BeamShaper" => BeamShaper,
            "BeamShaperMacro" => BeamShaperMacro,
            "BeamShaperPos" => BeamShaperPos,
            "BeamShaperPosRotate" => BeamShaperPosRotate,
            "Zoom" => Zoom,
            "ZoomModeSpot" => ZoomModeSpot,
            "ZoomModeBeam" => ZoomModeBeam,
            "DimmerMode" => DimmerMode,
            "DimmerCurve" => DimmerCurve,
            "BlackoutMode" => BlackoutMode,
            "LEDFrequency" => LedFrequency,
            "LEDZoneMode" => LedZoneMode,
            "PixelMode" => PixelMode,
            "PanMode" => PanMode,
            "TiltMode" => TiltMode,
            "PanTiltMode" => PanTiltMode,
            "PositionModes" => PositionModes,
            "AnimationWheelShortcutMode" => AnimationWheelShortcutMode,
            "ColorWheelShortcutMode" => ColorWheelShortcutMode,
            "CyanMode" => CyanMode,
            "MagentaMode" => MagentaMode,
            "YellowMode" => YellowMode,
            "ColorMixMode" => ColorMixMode,
            "ChromaticMode" => ChromaticMode,
            "ColorCalibrationMode" => ColorCalibrationMode,
            "ColorConsistency" => ColorConsistency,
            "ColorControl" => ColorControl,
            "ColorModelMode" => ColorModelMode,
            "ColorSettingsReset" => ColorSettingsReset,
            "ColorUniformity" => ColorUniformity,
            "CRIMode" => CriMode,
            "CustomColor" => CustomColor,
            "UVStability" => UvStability,
            "WavelengthCorrection" => WavelengthCorrection,
            "WhiteCount" => WhiteCount,
            "StrobeMode" => StrobeMode,
            "ZoomMode" => ZoomMode,
            "FocusMode" => FocusMode,
            "IrisMode" => IrisMode,
            "FollowSpotMode" => FollowSpotMode,
            "BeamEffectIndexRotateMode" => BeamEffectIndexRotateMode,
            "IntensityMSpeed" => IntensityMSpeed,
            "PositionMSpeed" => PositionMSpeed,
            "ColorMixMSpeed" => ColorMixMSpeed,
            "ColorWheelSelectMSpeed" => ColorWheelSelectMSpeed,
            "IrisMSpeed" => IrisMSpeed,
            "FocusMSpeed" => FocusMSpeed,
            "ZoomMSpeed" => ZoomMSpeed,
            "FrameMSpeed" => FrameMSpeed,
            "GlobalMSpeed" => GlobalMSpeed,
            "ReflectorAdjust" => ReflectorAdjust,
            "FixtureGlobalReset" => FixtureGlobalReset,
            "ShutterReset" => ShutterReset,
            "BeamReset" => BeamReset,
            "ColorMixReset" => ColorMixReset,
            "ColorWheelReset" => ColorWheelReset,
            "FocusReset" => FocusReset,
            "FrameReset" => FrameReset,
            "GoboWheelReset" => GoboWheelReset,
            "IntensityReset" => IntensityReset,
            "IrisReset" => IrisReset,
            "PositionReset" => PositionReset,
            "PanReset" => PanReset,
            "TiltReset" => TiltReset,
            "ZoomReset" => ZoomReset,
            "CTBReset" => CtbReset,
            "CTOReset" => CtoReset,
            "CTCReset" => CtcReset,
            "AnimationSystemReset" => AnimationSystemReset,
            "FixtureCalibrationReset" => FixtureCalibrationReset,
            "Function" => Function,
            "LampControl" => LampControl,
            "DisplayIntensity" => DisplayIntensity,
            "DMXInput" => DmxInput,
            "NoFeature" => NoFeature,
            "LampPowerMode" => LampPowerMode,
            "Fans" => Fans,
            "ShaperRot" => ShaperRot,
            "ShaperMacros" => ShaperMacros,
            "ShaperMacrosSpeed" => ShaperMacrosSpeed,
            "Video" => Video,
            "VideoBlendMode" => VideoBlendMode,
            "InputSource" => InputSource,
            "FieldOfView" => FieldOfView,
            "" => UserDefined(Name::new(value)?),
            _ => {
                lazy_static! {
                    static ref REGEX1: RegexSet = RegexSet::new(&[
                        r"^Gobo\d{1,}$",
                        r"^Gobo\d{1,}SelectSpin$",
                        r"^Gobo\d{1,}SelectShake$",
                        r"^Gobo\d{1,}SelectEffects$",
                        r"^Gobo\d{1,}WheelIndex$",
                        r"^Gobo\d{1,}WheelSpin$",
                        r"^Gobo\d{1,}WheelShake$",
                        r"^Gobo\d{1,}WheelRandom$",
                        r"^Gobo\d{1,}WheelAudio$",
                        r"^Gobo\d{1,}Pos$",
                        r"^Gobo\d{1,}PosRotate$",
                        r"^Gobo\d{1,}PosShake$",
                        r"^AnimationWheel\d{1,}$",
                        r"^AnimationWheel\d{1,}Audio$",
                        r"^AnimationWheel\d{1,}Macro$",
                        r"^AnimationWheel\d{1,}Random$",
                        r"^AnimationWheel\d{1,}SelectEffects$",
                        r"^AnimationWheel\d{1,}SelectShake$",
                        r"^AnimationWheel\d{1,}SelectSpin$",
                        r"^AnimationWheel\d{1,}Pos$",
                        r"^AnimationWheel\d{1,}PosRotate$",
                        r"^AnimationWheel\d{1,}PosShake$",
                        r"^AnimationSystem\d{1,}$",
                        r"^AnimationSystem\d{1,}Ramp$",
                        r"^AnimationSystem\d{1,}Shake$",
                        r"^AnimationSystem\d{1,}Audio$",
                        r"^AnimationSystem\d{1,}Random$",
                        r"^AnimationSystem\d{1,}Pos$",
                        r"^AnimationSystem\d{1,}PosRotate$",
                        r"^AnimationSystem\d{1,}PosShake$",
                        r"^AnimationSystem\d{1,}PosRandom$",
                        r"^AnimationSystem\d{1,}PosAudio$",
                        r"^AnimationSystem\d{1,}Macro$",
                        r"^MediaFolder\d{1,}$",
                        r"^MediaContent\d{1,}$",
                        r"^ModelFolder\d{1,}$",
                        r"^ModelContent\d{1,}$",
                        r"^ColorEffects\d{1,}$",
                        r"^Color\d{1,}$",
                        r"^Color\d{1,}WheelIndex$",
                        r"^Color\d{1,}WheelSpin$",
                        r"^Color\d{1,}WheelRandom$",
                        r"^Color\d{1,}WheelAudio$",
                        r"^ColorMacro\d{1,}$",
                        r"^ColorMacro\d{1,}Rate$",
                        r"^Shutter\d{1,}$",
                        r"^Shutter\d{1,}Strobe$",
                        r"^Shutter\d{1,}StrobePulse$",
                        r"^Shutter\d{1,}StrobePulseClose$",
                        r"^Shutter\d{1,}StrobePulseOpen$",
                        r"^Shutter\d{1,}StrobeRandom$",
                        r"^Shutter\d{1,}StrobeRandomPulse$",
                        r"^Shutter\d{1,}StrobeRandomPulseClose$",
                        r"^Shutter\d{1,}StrobeRandomPulseOpen$",
                        r"^Shutter\d{1,}StrobeEffect$",
                        r"^Frost\d{1,}$",
                        r"^Frost\d{1,}PulseOpen$",
                        r"^Frost\d{1,}PulseClose$",
                        r"^Frost\d{1,}Ramp$",
                        r"^Prism\d{1,}$",
                        r"^Prism\d{1,}SelectSpin$",
                        r"^Prism\d{1,}Macro$",
                        r"^Prism\d{1,}Pos$",
                        r"^Prism\d{1,}PosRotate$",
                        r"^Effects\d{1,}$",
                        r"^Effects\d{1,}Rate$",
                        r"^Effects\d{1,}Fade$",
                        r"^Effects\d{1,}Adjust\d{1,}$",
                        r"^Effects\d{1,}Pos$",
                        r"^Effects\d{1,}PosRotate$",
                        r"^Focus\d{1,}$",
                        r"^Focus\d{1,}Adjust$",
                        r"^Focus\d{1,}Distance$",
                        r"^Control\d{1,}$",
                        r"^Gobo\d{1,}WheelMode$",
                        r"^AnimationWheel\d{1,}Mode$",
                        r"^Color\d{1,}Mode$",
                        r"^Fan\d{1,}Mode$",
                        r"^GoboWheel\d{1,}MSpeed$",
                        r"^Prism\d{1,}MSpeed$",
                        r"^Frost\d{1,}MSpeed$",
                        r"^Blower\d{1,}$",
                        r"^Fan\d{1,}$",
                        r"^Fog\d{1,}$",
                        r"^Haze\d{1,}$",
                        r"^Blade\d{1,}A$",
                        r"^Blade\d{1,}B$",
                        r"^Blade\d{1,}Rot$",
                        r"^BladeSoft\d{1,}A$",
                        r"^BladeSoft\d{1,}B$",
                        r"^KeyStone\d{1,}A$",
                        r"^KeyStone\d{1,}B$",
                        r"^VideoEffect\d{1,}Type$",
                        r"^VideoEffect\d{1,}Parameter\d{1,}$",
                        r"^VideoCamera\d{1,}$",
                        r"^VideoSoundVolume\d{1,}$",
                    ]).unwrap();
                }
                let matches: SetMatches = REGEX1.matches(value);
                if !matches.matched_any() {
                    UserDefined(Name::new(value)?)
                } else {
                    lazy_static! {
                        static ref RE2: Regex = Regex::new(r"\d{1,}").unwrap();
                    }

                    let mut caps = RE2.captures_iter(&value);
                    let n = caps.next().map_or(0_u8, |m| u8::from_str(&m[0]).unwrap());
                    let m = caps.next().map_or(0_u8, |m| u8::from_str(&m[0]).unwrap());

                    if matches.matched(0) { return Ok(Gobo_n_(n)); }
                    if matches.matched(1) { return Ok(Gobo_n_SelectSpin(n)); }
                    if matches.matched(2) { return Ok(Gobo_n_SelectShake(n)); }
                    if matches.matched(3) { return Ok(Gobo_n_SelectEffects(n)); }
                    if matches.matched(4) { return Ok(Gobo_n_WheelIndex(n)); }
                    if matches.matched(5) { return Ok(Gobo_n_WheelSpin(n)); }
                    if matches.matched(6) { return Ok(Gobo_n_WheelShake(n)); }
                    if matches.matched(7) { return Ok(Gobo_n_WheelRandom(n)); }
                    if matches.matched(8) { return Ok(Gobo_n_WheelAudio(n)); }
                    if matches.matched(9) { return Ok(Gobo_n_Pos(n)); }
                    if matches.matched(10) { return Ok(Gobo_n_PosRotate(n)); }
                    if matches.matched(11) { return Ok(Gobo_n_PosShake(n)); }
                    if matches.matched(12) { return Ok(AnimationWheel_n_(n)); }
                    if matches.matched(13) { return Ok(AnimationWheel_n_Audio(n)); }
                    if matches.matched(14) { return Ok(AnimationWheel_n_Macro(n)); }
                    if matches.matched(15) { return Ok(AnimationWheel_n_Random(n)); }
                    if matches.matched(16) { return Ok(AnimationWheel_n_SelectEffects(n)); }
                    if matches.matched(17) { return Ok(AnimationWheel_n_SelectShake(n)); }
                    if matches.matched(18) { return Ok(AnimationWheel_n_SelectSpin(n)); }
                    if matches.matched(19) { return Ok(AnimationWheel_n_Pos(n)); }
                    if matches.matched(20) { return Ok(AnimationWheel_n_PosRotate(n)); }
                    if matches.matched(21) { return Ok(AnimationWheel_n_PosShake(n)); }
                    if matches.matched(22) { return Ok(AnimationSystem_n_(n)); }
                    if matches.matched(23) { return Ok(AnimationSystem_n_Ramp(n)); }
                    if matches.matched(24) { return Ok(AnimationSystem_n_Shake(n)); }
                    if matches.matched(25) { return Ok(AnimationSystem_n_Audio(n)); }
                    if matches.matched(26) { return Ok(AnimationSystem_n_Random(n)); }
                    if matches.matched(27) { return Ok(AnimationSystem_n_Pos(n)); }
                    if matches.matched(28) { return Ok(AnimationSystem_n_PosRotate(n)); }
                    if matches.matched(29) { return Ok(AnimationSystem_n_PosShake(n)); }
                    if matches.matched(30) { return Ok(AnimationSystem_n_PosRandom(n)); }
                    if matches.matched(31) { return Ok(AnimationSystem_n_PosAudio(n)); }
                    if matches.matched(32) { return Ok(AnimationSystem_n_Macro(n)); }
                    if matches.matched(33) { return Ok(MediaFolder_n_(n)); }
                    if matches.matched(34) { return Ok(MediaContent_n_(n)); }
                    if matches.matched(35) { return Ok(ModelFolder_n_(n)); }
                    if matches.matched(36) { return Ok(ModelContent_n_(n)); }
                    if matches.matched(37) { return Ok(ColorEffects_n_(n)); }
                    if matches.matched(38) { return Ok(Color_n_(n)); }
                    if matches.matched(39) { return Ok(Color_n_WheelIndex(n)); }
                    if matches.matched(40) { return Ok(Color_n_WheelSpin(n)); }
                    if matches.matched(41) { return Ok(Color_n_WheelRandom(n)); }
                    if matches.matched(42) { return Ok(Color_n_WheelAudio(n)); }
                    if matches.matched(43) { return Ok(ColorMacro_n_(n)); }
                    if matches.matched(44) { return Ok(ColorMacro_n_Rate(n)); }
                    if matches.matched(45) { return Ok(Shutter_n_(n)); }
                    if matches.matched(46) { return Ok(Shutter_n_Strobe(n)); }
                    if matches.matched(47) { return Ok(Shutter_n_StrobePulse(n)); }
                    if matches.matched(48) { return Ok(Shutter_n_StrobePulseClose(n)); }
                    if matches.matched(49) { return Ok(Shutter_n_StrobePulseOpen(n)); }
                    if matches.matched(50) { return Ok(Shutter_n_StrobeRandom(n)); }
                    if matches.matched(51) { return Ok(Shutter_n_StrobeRandomPulse(n)); }
                    if matches.matched(52) { return Ok(Shutter_n_StrobeRandomPulseClose(n)); }
                    if matches.matched(53) { return Ok(Shutter_n_StrobeRandomPulseOpen(n)); }
                    if matches.matched(54) { return Ok(Shutter_n_StrobeEffect(n)); }
                    if matches.matched(55) { return Ok(Frost_n_(n)); }
                    if matches.matched(56) { return Ok(Frost_n_PulseOpen(n)); }
                    if matches.matched(57) { return Ok(Frost_n_PulseClose(n)); }
                    if matches.matched(58) { return Ok(Frost_n_Ramp(n)); }
                    if matches.matched(59) { return Ok(Prism_n_(n)); }
                    if matches.matched(60) { return Ok(Prism_n_SelectSpin(n)); }
                    if matches.matched(61) { return Ok(Prism_n_Macro(n)); }
                    if matches.matched(62) { return Ok(Prism_n_Pos(n)); }
                    if matches.matched(63) { return Ok(Prism_n_PosRotate(n)); }
                    if matches.matched(64) { return Ok(Effects_n_(n)); }
                    if matches.matched(65) { return Ok(Effects_n_Rate(n)); }
                    if matches.matched(66) { return Ok(Effects_n_Fade(n)); }
                    if matches.matched(67) { return Ok(Effects_n_Adjust_m_(n, m)); }
                    if matches.matched(68) { return Ok(Effects_n_Pos(n)); }
                    if matches.matched(69) { return Ok(Effects_n_PosRotate(n)); }
                    if matches.matched(70) { return Ok(Focus_n_(n)); }
                    if matches.matched(71) { return Ok(Focus_n_Adjust(n)); }
                    if matches.matched(72) { return Ok(Focus_n_Distance(n)); }
                    if matches.matched(73) { return Ok(Control_n_(n)); }
                    if matches.matched(74) { return Ok(Gobo_n_WheelMode(n)); }
                    if matches.matched(75) { return Ok(AnimationWheel_n_Mode(n)); }
                    if matches.matched(76) { return Ok(Color_n_Mode(n)); }
                    if matches.matched(77) { return Ok(Fan_n_Mode(n)); }
                    if matches.matched(78) { return Ok(GoboWheel_n_MSpeed(n)); }
                    if matches.matched(79) { return Ok(Prism_n_MSpeed(n)); }
                    if matches.matched(80) { return Ok(Frost_n_MSpeed(n)); }
                    if matches.matched(81) { return Ok(Blower_n_(n)); }
                    if matches.matched(82) { return Ok(Fan_n_(n)); }
                    if matches.matched(83) { return Ok(Fog_n_(n)); }
                    if matches.matched(84) { return Ok(Haze_n_(n)); }
                    if matches.matched(85) { return Ok(Blade_n_A(n)); }
                    if matches.matched(86) { return Ok(Blade_n_B(n)); }
                    if matches.matched(87) { return Ok(Blade_n_Rot(n)); }
                    if matches.matched(88) { return Ok(BladeSoft_n_A(n)); }
                    if matches.matched(89) { return Ok(BladeSoft_n_B(n)); }
                    if matches.matched(90) { return Ok(KeyStone_n_A(n)); }
                    if matches.matched(91) { return Ok(KeyStone_n_B(n)); }
                    if matches.matched(92) { return Ok(VideoEffect_n_Type(n)); }
                    if matches.matched(93) { return Ok(VideoEffect_n_Parameter_m_(n, m)); }
                    if matches.matched(94) { return Ok(VideoCamera_n_(n)); }
                    if matches.matched(95) { return Ok(VideoSoundVolume_n_(n)); }

                    UserDefined(Name::new(value)?)
                }
            }
        })
    }

    ///Parses a quick-xml-attribute from gdtf-xml-description into an AttributeName. Only chars `[32..=122] = (SPACE..='z')` are allowed. If one of these chars is passed to the function, it will return an Error
    ///```rust
    /// use gdtf_parser::utils::units::attribute_name::AttributeName;
    /// use gdtf_parser::utils::units::name::Name;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(AttributeName::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Tilt")}).unwrap(), AttributeName::Tilt);
    /// assert_eq!(AttributeName::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"PanTiltMode")}).unwrap(), AttributeName::PanTiltMode);
    /// assert_eq!(AttributeName::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Effects1Adjust2")}).unwrap(), AttributeName::Effects_n_Adjust_m_(1,2));
    /// assert_eq!(AttributeName::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Something else")}).unwrap(), AttributeName::UserDefined(Name("Something else".to_string())));
    /// assert!(AttributeName::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Name with invalid char {")}).is_err());
    ///```
    pub fn new_from_attr(attr: Attribute<'_>) -> Result<Self, GdtfNameError> {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::attribute_name::AttributeName as T;
    use crate::utils::units::name::Name;

    #[test]
    fn test_default() -> Result<(), GdtfError> {
        use T::*;
        assert_eq!(UserDefined(Name::new("")?), T::default());
        Ok(())
    }

    #[test]
    fn test_new_from_str() -> Result<(), GdtfError> {
        use T::*;
        assert_eq!(UserDefined(Name::new("test")?), T::new_from_str("test")?);
        assert_eq!(UserDefined(Name::new("")?), T::new_from_str("")?);
        assert_eq!(Dimmer, T::new_from_str("Dimmer")?);
        assert_eq!(Pan, T::new_from_str("Pan")?);
        assert_eq!(Tilt, T::new_from_str("Tilt")?);
        assert_eq!(PanRotate, T::new_from_str("PanRotate")?);
        assert_eq!(TiltRotate, T::new_from_str("TiltRotate")?);
        assert_eq!(PositionEffect, T::new_from_str("PositionEffect")?);
        assert_eq!(PositionEffectRate, T::new_from_str("PositionEffectRate")?);
        assert_eq!(PositionEffectFade, T::new_from_str("PositionEffectFade")?);
        assert_eq!(Xyz_X, T::new_from_str("XYZ_X")?);
        assert_eq!(Xyz_Y, T::new_from_str("XYZ_Y")?);
        assert_eq!(Xyz_Z, T::new_from_str("XYZ_Z")?);
        assert_eq!(Rot_X, T::new_from_str("Rot_X")?);
        assert_eq!(Rot_Y, T::new_from_str("Rot_Y")?);
        assert_eq!(Rot_Z, T::new_from_str("Rot_Z")?);
        assert_eq!(Scale_X, T::new_from_str("Scale_X")?);
        assert_eq!(Scale_Y, T::new_from_str("Scale_Y")?);
        assert_eq!(Scale_Z, T::new_from_str("Scale_Z")?);
        assert_eq!(Scale_Xyz, T::new_from_str("Scale_XYZ")?);
        assert_eq!(Gobo_n_(1), T::new_from_str("Gobo1")?);
        assert_eq!(Gobo_n_(2), T::new_from_str("Gobo2")?);
        assert_eq!(Gobo_n_(120), T::new_from_str("Gobo120")?);
        assert_eq!(Gobo_n_SelectSpin(1), T::new_from_str("Gobo1SelectSpin")?);
        assert_eq!(Gobo_n_SelectSpin(2), T::new_from_str("Gobo2SelectSpin")?);
        assert_eq!(Gobo_n_SelectSpin(120), T::new_from_str("Gobo120SelectSpin")?);
        assert_eq!(Gobo_n_SelectShake(1), T::new_from_str("Gobo1SelectShake")?);
        assert_eq!(Gobo_n_SelectShake(2), T::new_from_str("Gobo2SelectShake")?);
        assert_eq!(Gobo_n_SelectShake(120), T::new_from_str("Gobo120SelectShake")?);
        assert_eq!(Gobo_n_SelectEffects(1), T::new_from_str("Gobo1SelectEffects")?);
        assert_eq!(Gobo_n_SelectEffects(2), T::new_from_str("Gobo2SelectEffects")?);
        assert_eq!(Gobo_n_SelectEffects(120), T::new_from_str("Gobo120SelectEffects")?);
        assert_eq!(Gobo_n_WheelIndex(1), T::new_from_str("Gobo1WheelIndex")?);
        assert_eq!(Gobo_n_WheelIndex(2), T::new_from_str("Gobo2WheelIndex")?);
        assert_eq!(Gobo_n_WheelIndex(120), T::new_from_str("Gobo120WheelIndex")?);
        assert_eq!(Gobo_n_WheelSpin(1), T::new_from_str("Gobo1WheelSpin")?);
        assert_eq!(Gobo_n_WheelSpin(2), T::new_from_str("Gobo2WheelSpin")?);
        assert_eq!(Gobo_n_WheelSpin(120), T::new_from_str("Gobo120WheelSpin")?);
        assert_eq!(Gobo_n_WheelShake(1), T::new_from_str("Gobo1WheelShake")?);
        assert_eq!(Gobo_n_WheelShake(2), T::new_from_str("Gobo2WheelShake")?);
        assert_eq!(Gobo_n_WheelShake(120), T::new_from_str("Gobo120WheelShake")?);
        assert_eq!(Gobo_n_WheelRandom(1), T::new_from_str("Gobo1WheelRandom")?);
        assert_eq!(Gobo_n_WheelRandom(2), T::new_from_str("Gobo2WheelRandom")?);
        assert_eq!(Gobo_n_WheelRandom(120), T::new_from_str("Gobo120WheelRandom")?);
        assert_eq!(Gobo_n_WheelAudio(1), T::new_from_str("Gobo1WheelAudio")?);
        assert_eq!(Gobo_n_WheelAudio(2), T::new_from_str("Gobo2WheelAudio")?);
        assert_eq!(Gobo_n_WheelAudio(120), T::new_from_str("Gobo120WheelAudio")?);
        assert_eq!(Gobo_n_Pos(1), T::new_from_str("Gobo1Pos")?);
        assert_eq!(Gobo_n_Pos(2), T::new_from_str("Gobo2Pos")?);
        assert_eq!(Gobo_n_Pos(120), T::new_from_str("Gobo120Pos")?);
        assert_eq!(Gobo_n_PosRotate(1), T::new_from_str("Gobo1PosRotate")?);
        assert_eq!(Gobo_n_PosRotate(2), T::new_from_str("Gobo2PosRotate")?);
        assert_eq!(Gobo_n_PosRotate(120), T::new_from_str("Gobo120PosRotate")?);
        assert_eq!(Gobo_n_PosShake(1), T::new_from_str("Gobo1PosShake")?);
        assert_eq!(Gobo_n_PosShake(2), T::new_from_str("Gobo2PosShake")?);
        assert_eq!(Gobo_n_PosShake(120), T::new_from_str("Gobo120PosShake")?);
        assert_eq!(AnimationWheel_n_(1), T::new_from_str("AnimationWheel1")?);
        assert_eq!(AnimationWheel_n_(2), T::new_from_str("AnimationWheel2")?);
        assert_eq!(AnimationWheel_n_(120), T::new_from_str("AnimationWheel120")?);
        assert_eq!(AnimationWheel_n_Audio(1), T::new_from_str("AnimationWheel1Audio")?);
        assert_eq!(AnimationWheel_n_Audio(2), T::new_from_str("AnimationWheel2Audio")?);
        assert_eq!(AnimationWheel_n_Audio(120), T::new_from_str("AnimationWheel120Audio")?);
        assert_eq!(AnimationWheel_n_Macro(1), T::new_from_str("AnimationWheel1Macro")?);
        assert_eq!(AnimationWheel_n_Macro(2), T::new_from_str("AnimationWheel2Macro")?);
        assert_eq!(AnimationWheel_n_Macro(120), T::new_from_str("AnimationWheel120Macro")?);
        assert_eq!(AnimationWheel_n_Random(1), T::new_from_str("AnimationWheel1Random")?);
        assert_eq!(AnimationWheel_n_Random(2), T::new_from_str("AnimationWheel2Random")?);
        assert_eq!(AnimationWheel_n_Random(120), T::new_from_str("AnimationWheel120Random")?);
        assert_eq!(AnimationWheel_n_SelectEffects(1), T::new_from_str("AnimationWheel1SelectEffects")?);
        assert_eq!(AnimationWheel_n_SelectEffects(2), T::new_from_str("AnimationWheel2SelectEffects")?);
        assert_eq!(AnimationWheel_n_SelectEffects(120), T::new_from_str("AnimationWheel120SelectEffects")?);
        assert_eq!(AnimationWheel_n_SelectShake(1), T::new_from_str("AnimationWheel1SelectShake")?);
        assert_eq!(AnimationWheel_n_SelectShake(2), T::new_from_str("AnimationWheel2SelectShake")?);
        assert_eq!(AnimationWheel_n_SelectShake(120), T::new_from_str("AnimationWheel120SelectShake")?);
        assert_eq!(AnimationWheel_n_SelectSpin(1), T::new_from_str("AnimationWheel1SelectSpin")?);
        assert_eq!(AnimationWheel_n_SelectSpin(2), T::new_from_str("AnimationWheel2SelectSpin")?);
        assert_eq!(AnimationWheel_n_SelectSpin(120), T::new_from_str("AnimationWheel120SelectSpin")?);
        assert_eq!(AnimationWheel_n_Pos(1), T::new_from_str("AnimationWheel1Pos")?);
        assert_eq!(AnimationWheel_n_Pos(2), T::new_from_str("AnimationWheel2Pos")?);
        assert_eq!(AnimationWheel_n_Pos(120), T::new_from_str("AnimationWheel120Pos")?);
        assert_eq!(AnimationWheel_n_PosRotate(1), T::new_from_str("AnimationWheel1PosRotate")?);
        assert_eq!(AnimationWheel_n_PosRotate(2), T::new_from_str("AnimationWheel2PosRotate")?);
        assert_eq!(AnimationWheel_n_PosRotate(120), T::new_from_str("AnimationWheel120PosRotate")?);
        assert_eq!(AnimationWheel_n_PosShake(1), T::new_from_str("AnimationWheel1PosShake")?);
        assert_eq!(AnimationWheel_n_PosShake(2), T::new_from_str("AnimationWheel2PosShake")?);
        assert_eq!(AnimationWheel_n_PosShake(120), T::new_from_str("AnimationWheel120PosShake")?);
        assert_eq!(AnimationSystem_n_(1), T::new_from_str("AnimationSystem1")?);
        assert_eq!(AnimationSystem_n_(2), T::new_from_str("AnimationSystem2")?);
        assert_eq!(AnimationSystem_n_(120), T::new_from_str("AnimationSystem120")?);
        assert_eq!(AnimationSystem_n_Ramp(1), T::new_from_str("AnimationSystem1Ramp")?);
        assert_eq!(AnimationSystem_n_Ramp(2), T::new_from_str("AnimationSystem2Ramp")?);
        assert_eq!(AnimationSystem_n_Ramp(120), T::new_from_str("AnimationSystem120Ramp")?);
        assert_eq!(AnimationSystem_n_Shake(1), T::new_from_str("AnimationSystem1Shake")?);
        assert_eq!(AnimationSystem_n_Shake(2), T::new_from_str("AnimationSystem2Shake")?);
        assert_eq!(AnimationSystem_n_Shake(120), T::new_from_str("AnimationSystem120Shake")?);
        assert_eq!(AnimationSystem_n_Audio(1), T::new_from_str("AnimationSystem1Audio")?);
        assert_eq!(AnimationSystem_n_Audio(2), T::new_from_str("AnimationSystem2Audio")?);
        assert_eq!(AnimationSystem_n_Audio(120), T::new_from_str("AnimationSystem120Audio")?);
        assert_eq!(AnimationSystem_n_Random(1), T::new_from_str("AnimationSystem1Random")?);
        assert_eq!(AnimationSystem_n_Random(2), T::new_from_str("AnimationSystem2Random")?);
        assert_eq!(AnimationSystem_n_Random(120), T::new_from_str("AnimationSystem120Random")?);
        assert_eq!(AnimationSystem_n_Pos(1), T::new_from_str("AnimationSystem1Pos")?);
        assert_eq!(AnimationSystem_n_Pos(2), T::new_from_str("AnimationSystem2Pos")?);
        assert_eq!(AnimationSystem_n_Pos(120), T::new_from_str("AnimationSystem120Pos")?);
        assert_eq!(AnimationSystem_n_PosRotate(1), T::new_from_str("AnimationSystem1PosRotate")?);
        assert_eq!(AnimationSystem_n_PosRotate(2), T::new_from_str("AnimationSystem2PosRotate")?);
        assert_eq!(AnimationSystem_n_PosRotate(120), T::new_from_str("AnimationSystem120PosRotate")?);
        assert_eq!(AnimationSystem_n_PosShake(1), T::new_from_str("AnimationSystem1PosShake")?);
        assert_eq!(AnimationSystem_n_PosShake(2), T::new_from_str("AnimationSystem2PosShake")?);
        assert_eq!(AnimationSystem_n_PosShake(120), T::new_from_str("AnimationSystem120PosShake")?);
        assert_eq!(AnimationSystem_n_PosRandom(1), T::new_from_str("AnimationSystem1PosRandom")?);
        assert_eq!(AnimationSystem_n_PosRandom(2), T::new_from_str("AnimationSystem2PosRandom")?);
        assert_eq!(AnimationSystem_n_PosRandom(120), T::new_from_str("AnimationSystem120PosRandom")?);
        assert_eq!(AnimationSystem_n_PosAudio(1), T::new_from_str("AnimationSystem1PosAudio")?);
        assert_eq!(AnimationSystem_n_PosAudio(2), T::new_from_str("AnimationSystem2PosAudio")?);
        assert_eq!(AnimationSystem_n_PosAudio(120), T::new_from_str("AnimationSystem120PosAudio")?);
        assert_eq!(AnimationSystem_n_Macro(1), T::new_from_str("AnimationSystem1Macro")?);
        assert_eq!(AnimationSystem_n_Macro(2), T::new_from_str("AnimationSystem2Macro")?);
        assert_eq!(AnimationSystem_n_Macro(120), T::new_from_str("AnimationSystem120Macro")?);
        assert_eq!(MediaFolder_n_(1), T::new_from_str("MediaFolder1")?);
        assert_eq!(MediaFolder_n_(2), T::new_from_str("MediaFolder2")?);
        assert_eq!(MediaFolder_n_(120), T::new_from_str("MediaFolder120")?);
        assert_eq!(MediaContent_n_(1), T::new_from_str("MediaContent1")?);
        assert_eq!(MediaContent_n_(2), T::new_from_str("MediaContent2")?);
        assert_eq!(MediaContent_n_(120), T::new_from_str("MediaContent120")?);
        assert_eq!(ModelFolder_n_(1), T::new_from_str("ModelFolder1")?);
        assert_eq!(ModelFolder_n_(2), T::new_from_str("ModelFolder2")?);
        assert_eq!(ModelFolder_n_(120), T::new_from_str("ModelFolder120")?);
        assert_eq!(ModelContent_n_(1), T::new_from_str("ModelContent1")?);
        assert_eq!(ModelContent_n_(2), T::new_from_str("ModelContent2")?);
        assert_eq!(ModelContent_n_(120), T::new_from_str("ModelContent120")?);
        assert_eq!(PlayMode, T::new_from_str("PlayMode")?);
        assert_eq!(PlayBegin, T::new_from_str("PlayBegin")?);
        assert_eq!(PlayEnd, T::new_from_str("PlayEnd")?);
        assert_eq!(PlaySpeed, T::new_from_str("PlaySpeed")?);
        assert_eq!(ColorEffects_n_(1), T::new_from_str("ColorEffects1")?);
        assert_eq!(ColorEffects_n_(2), T::new_from_str("ColorEffects2")?);
        assert_eq!(ColorEffects_n_(120), T::new_from_str("ColorEffects120")?);
        assert_eq!(Color_n_(1), T::new_from_str("Color1")?);
        assert_eq!(Color_n_(2), T::new_from_str("Color2")?);
        assert_eq!(Color_n_(120), T::new_from_str("Color120")?);
        assert_eq!(Color_n_WheelIndex(1), T::new_from_str("Color1WheelIndex")?);
        assert_eq!(Color_n_WheelIndex(2), T::new_from_str("Color2WheelIndex")?);
        assert_eq!(Color_n_WheelIndex(120), T::new_from_str("Color120WheelIndex")?);
        assert_eq!(Color_n_WheelSpin(1), T::new_from_str("Color1WheelSpin")?);
        assert_eq!(Color_n_WheelSpin(2), T::new_from_str("Color2WheelSpin")?);
        assert_eq!(Color_n_WheelSpin(120), T::new_from_str("Color120WheelSpin")?);
        assert_eq!(Color_n_WheelRandom(1), T::new_from_str("Color1WheelRandom")?);
        assert_eq!(Color_n_WheelRandom(2), T::new_from_str("Color2WheelRandom")?);
        assert_eq!(Color_n_WheelRandom(120), T::new_from_str("Color120WheelRandom")?);
        assert_eq!(Color_n_WheelAudio(1), T::new_from_str("Color1WheelAudio")?);
        assert_eq!(Color_n_WheelAudio(2), T::new_from_str("Color2WheelAudio")?);
        assert_eq!(Color_n_WheelAudio(120), T::new_from_str("Color120WheelAudio")?);
        assert_eq!(ColorAdd_R, T::new_from_str("ColorAdd_R")?);
        assert_eq!(ColorAdd_G, T::new_from_str("ColorAdd_G")?);
        assert_eq!(ColorAdd_B, T::new_from_str("ColorAdd_B")?);
        assert_eq!(ColorAdd_C, T::new_from_str("ColorAdd_C")?);
        assert_eq!(ColorAdd_M, T::new_from_str("ColorAdd_M")?);
        assert_eq!(ColorAdd_Y, T::new_from_str("ColorAdd_Y")?);
        assert_eq!(ColorAdd_Ry, T::new_from_str("ColorAdd_RY")?);
        assert_eq!(ColorAdd_Gy, T::new_from_str("ColorAdd_GY")?);
        assert_eq!(ColorAdd_Gc, T::new_from_str("ColorAdd_GC")?);
        assert_eq!(ColorAdd_Bc, T::new_from_str("ColorAdd_BC")?);
        assert_eq!(ColorAdd_Bm, T::new_from_str("ColorAdd_BM")?);
        assert_eq!(ColorAdd_Rm, T::new_from_str("ColorAdd_RM")?);
        assert_eq!(ColorAdd_W, T::new_from_str("ColorAdd_W")?);
        assert_eq!(ColorAdd_Ww, T::new_from_str("ColorAdd_WW")?);
        assert_eq!(ColorAdd_Cw, T::new_from_str("ColorAdd_CW")?);
        assert_eq!(ColorAdd_Uv, T::new_from_str("ColorAdd_UV")?);
        assert_eq!(ColorSub_R, T::new_from_str("ColorSub_R")?);
        assert_eq!(ColorSub_G, T::new_from_str("ColorSub_G")?);
        assert_eq!(ColorSub_B, T::new_from_str("ColorSub_B")?);
        assert_eq!(ColorSub_C, T::new_from_str("ColorSub_C")?);
        assert_eq!(ColorSub_M, T::new_from_str("ColorSub_M")?);
        assert_eq!(ColorSub_Y, T::new_from_str("ColorSub_Y")?);
        assert_eq!(ColorMacro_n_(1), T::new_from_str("ColorMacro1")?);
        assert_eq!(ColorMacro_n_(2), T::new_from_str("ColorMacro2")?);
        assert_eq!(ColorMacro_n_(120), T::new_from_str("ColorMacro120")?);
        assert_eq!(ColorMacro_n_Rate(1), T::new_from_str("ColorMacro1Rate")?);
        assert_eq!(ColorMacro_n_Rate(2), T::new_from_str("ColorMacro2Rate")?);
        assert_eq!(ColorMacro_n_Rate(120), T::new_from_str("ColorMacro120Rate")?);
        assert_eq!(Cto, T::new_from_str("CTO")?);
        assert_eq!(Ctc, T::new_from_str("CTC")?);
        assert_eq!(Ctb, T::new_from_str("CTB")?);
        assert_eq!(Tint, T::new_from_str("Tint")?);
        assert_eq!(Hsb_Hue, T::new_from_str("HSB_Hue")?);
        assert_eq!(Hsb_Saturation, T::new_from_str("HSB_Saturation")?);
        assert_eq!(Hsb_Brightness, T::new_from_str("HSB_Brightness")?);
        assert_eq!(Hsb_Quality, T::new_from_str("HSB_Quality")?);
        assert_eq!(Cie_X, T::new_from_str("CIE_X")?);
        assert_eq!(Cie_Y, T::new_from_str("CIE_Y")?);
        assert_eq!(Cie_Brightness, T::new_from_str("CIE_Brightness")?);
        assert_eq!(ColorRgb_Red, T::new_from_str("ColorRGB_Red")?);
        assert_eq!(ColorRgb_Green, T::new_from_str("ColorRGB_Green")?);
        assert_eq!(ColorRgb_Blue, T::new_from_str("ColorRGB_Blue")?);
        assert_eq!(ColorRgb_Cyan, T::new_from_str("ColorRGB_Cyan")?);
        assert_eq!(ColorRgb_Magenta, T::new_from_str("ColorRGB_Magenta")?);
        assert_eq!(ColorRgb_Yellow, T::new_from_str("ColorRGB_Yellow")?);
        assert_eq!(ColorRgb_Quality, T::new_from_str("ColorRGB_Quality")?);
        assert_eq!(VideoBoost_R, T::new_from_str("VideoBoost_R")?);
        assert_eq!(VideoBoost_G, T::new_from_str("VideoBoost_G")?);
        assert_eq!(VideoBoost_B, T::new_from_str("VideoBoost_B")?);
        assert_eq!(VideoHueShift, T::new_from_str("VideoHueShift")?);
        assert_eq!(VideoSaturation, T::new_from_str("VideoSaturation")?);
        assert_eq!(VideoBrightness, T::new_from_str("VideoBrightness")?);
        assert_eq!(VideoContrast, T::new_from_str("VideoContrast")?);
        assert_eq!(VideoKeyColor_R, T::new_from_str("VideoKeyColor_R")?);
        assert_eq!(VideoKeyColor_G, T::new_from_str("VideoKeyColor_G")?);
        assert_eq!(VideoKeyColor_B, T::new_from_str("VideoKeyColor_B")?);
        assert_eq!(VideoKeyIntensity, T::new_from_str("VideoKeyIntensity")?);
        assert_eq!(VideoKeyTolerance, T::new_from_str("VideoKeyTolerance")?);
        assert_eq!(StrobeDuration, T::new_from_str("StrobeDuration")?);
        assert_eq!(StrobeRate, T::new_from_str("StrobeRate")?);
        assert_eq!(Shutter_n_(1), T::new_from_str("Shutter1")?);
        assert_eq!(Shutter_n_(2), T::new_from_str("Shutter2")?);
        assert_eq!(Shutter_n_(120), T::new_from_str("Shutter120")?);
        assert_eq!(Shutter_n_Strobe(1), T::new_from_str("Shutter1Strobe")?);
        assert_eq!(Shutter_n_Strobe(2), T::new_from_str("Shutter2Strobe")?);
        assert_eq!(Shutter_n_Strobe(120), T::new_from_str("Shutter120Strobe")?);
        assert_eq!(Shutter_n_StrobePulse(1), T::new_from_str("Shutter1StrobePulse")?);
        assert_eq!(Shutter_n_StrobePulse(2), T::new_from_str("Shutter2StrobePulse")?);
        assert_eq!(Shutter_n_StrobePulse(120), T::new_from_str("Shutter120StrobePulse")?);
        assert_eq!(Shutter_n_StrobePulseClose(1), T::new_from_str("Shutter1StrobePulseClose")?);
        assert_eq!(Shutter_n_StrobePulseClose(2), T::new_from_str("Shutter2StrobePulseClose")?);
        assert_eq!(Shutter_n_StrobePulseOpen(1), T::new_from_str("Shutter1StrobePulseOpen")?);
        assert_eq!(Shutter_n_StrobePulseOpen(2), T::new_from_str("Shutter2StrobePulseOpen")?);
        assert_eq!(Shutter_n_StrobePulseOpen(120), T::new_from_str("Shutter120StrobePulseOpen")?);
        assert_eq!(Shutter_n_StrobeRandom(1), T::new_from_str("Shutter1StrobeRandom")?);
        assert_eq!(Shutter_n_StrobeRandom(2), T::new_from_str("Shutter2StrobeRandom")?);
        assert_eq!(Shutter_n_StrobeRandom(120), T::new_from_str("Shutter120StrobeRandom")?);
        assert_eq!(Shutter_n_StrobeRandomPulse(1), T::new_from_str("Shutter1StrobeRandomPulse")?);
        assert_eq!(Shutter_n_StrobeRandomPulse(2), T::new_from_str("Shutter2StrobeRandomPulse")?);
        assert_eq!(Shutter_n_StrobeRandomPulse(120), T::new_from_str("Shutter120StrobeRandomPulse")?);
        assert_eq!(Shutter_n_StrobeRandomPulseClose(1), T::new_from_str("Shutter1StrobeRandomPulseClose")?);
        assert_eq!(Shutter_n_StrobeRandomPulseClose(2), T::new_from_str("Shutter2StrobeRandomPulseClose")?);
        assert_eq!(Shutter_n_StrobeRandomPulseClose(120), T::new_from_str("Shutter120StrobeRandomPulseClose")?);
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(1), T::new_from_str("Shutter1StrobeRandomPulseOpen")?);
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(2), T::new_from_str("Shutter2StrobeRandomPulseOpen")?);
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(120), T::new_from_str("Shutter120StrobeRandomPulseOpen")?);
        assert_eq!(Shutter_n_StrobeEffect(1), T::new_from_str("Shutter1StrobeEffect")?);
        assert_eq!(Shutter_n_StrobeEffect(2), T::new_from_str("Shutter2StrobeEffect")?);
        assert_eq!(Shutter_n_StrobeEffect(120), T::new_from_str("Shutter120StrobeEffect")?);
        assert_eq!(Iris, T::new_from_str("Iris")?);
        assert_eq!(IrisStrobe, T::new_from_str("IrisStrobe")?);
        assert_eq!(IrisStrobeRandom, T::new_from_str("IrisStrobeRandom")?);
        assert_eq!(IrisPulseClose, T::new_from_str("IrisPulseClose")?);
        assert_eq!(IrisPulseOpen, T::new_from_str("IrisPulseOpen")?);
        assert_eq!(IrisRandomPulseClose, T::new_from_str("IrisRandomPulseClose")?);
        assert_eq!(IrisRandomPulseOpen, T::new_from_str("IrisRandomPulseOpen")?);
        assert_eq!(Frost_n_(1), T::new_from_str("Frost1")?);
        assert_eq!(Frost_n_(2), T::new_from_str("Frost2")?);
        assert_eq!(Frost_n_(120), T::new_from_str("Frost120")?);
        assert_eq!(Frost_n_PulseOpen(1), T::new_from_str("Frost1PulseOpen")?);
        assert_eq!(Frost_n_PulseOpen(2), T::new_from_str("Frost2PulseOpen")?);
        assert_eq!(Frost_n_PulseOpen(120), T::new_from_str("Frost120PulseOpen")?);
        assert_eq!(Frost_n_PulseClose(1), T::new_from_str("Frost1PulseClose")?);
        assert_eq!(Frost_n_PulseClose(2), T::new_from_str("Frost2PulseClose")?);
        assert_eq!(Frost_n_PulseClose(120), T::new_from_str("Frost120PulseClose")?);
        assert_eq!(Frost_n_Ramp(1), T::new_from_str("Frost1Ramp")?);
        assert_eq!(Frost_n_Ramp(2), T::new_from_str("Frost2Ramp")?);
        assert_eq!(Frost_n_Ramp(120), T::new_from_str("Frost120Ramp")?);
        assert_eq!(Prism_n_(1), T::new_from_str("Prism1")?);
        assert_eq!(Prism_n_(2), T::new_from_str("Prism2")?);
        assert_eq!(Prism_n_(120), T::new_from_str("Prism120")?);
        assert_eq!(Prism_n_SelectSpin(1), T::new_from_str("Prism1SelectSpin")?);
        assert_eq!(Prism_n_SelectSpin(2), T::new_from_str("Prism2SelectSpin")?);
        assert_eq!(Prism_n_SelectSpin(120), T::new_from_str("Prism120SelectSpin")?);
        assert_eq!(Prism_n_Macro(1), T::new_from_str("Prism1Macro")?);
        assert_eq!(Prism_n_Macro(2), T::new_from_str("Prism2Macro")?);
        assert_eq!(Prism_n_Macro(120), T::new_from_str("Prism120Macro")?);
        assert_eq!(Prism_n_Pos(1), T::new_from_str("Prism1Pos")?);
        assert_eq!(Prism_n_Pos(2), T::new_from_str("Prism2Pos")?);
        assert_eq!(Prism_n_Pos(120), T::new_from_str("Prism120Pos")?);
        assert_eq!(Prism_n_PosRotate(1), T::new_from_str("Prism1PosRotate")?);
        assert_eq!(Prism_n_PosRotate(2), T::new_from_str("Prism2PosRotate")?);
        assert_eq!(Prism_n_PosRotate(120), T::new_from_str("Prism120PosRotate")?);
        assert_eq!(Effects_n_(1), T::new_from_str("Effects1")?);
        assert_eq!(Effects_n_(2), T::new_from_str("Effects2")?);
        assert_eq!(Effects_n_(120), T::new_from_str("Effects120")?);
        assert_eq!(Effects_n_Rate(1), T::new_from_str("Effects1Rate")?);
        assert_eq!(Effects_n_Rate(2), T::new_from_str("Effects2Rate")?);
        assert_eq!(Effects_n_Rate(120), T::new_from_str("Effects120Rate")?);
        assert_eq!(Effects_n_Fade(1), T::new_from_str("Effects1Fade")?);
        assert_eq!(Effects_n_Fade(2), T::new_from_str("Effects2Fade")?);
        assert_eq!(Effects_n_Fade(120), T::new_from_str("Effects120Fade")?);
        assert_eq!(Effects_n_Adjust_m_(1, 1), T::new_from_str("Effects1Adjust1")?);
        assert_eq!(Effects_n_Adjust_m_(1, 2), T::new_from_str("Effects1Adjust2")?);
        assert_eq!(Effects_n_Adjust_m_(2, 1), T::new_from_str("Effects2Adjust1")?);
        assert_eq!(Effects_n_Adjust_m_(2, 2), T::new_from_str("Effects2Adjust2")?);
        assert_eq!(Effects_n_Adjust_m_(2, 120), T::new_from_str("Effects2Adjust120")?);
        assert_eq!(Effects_n_Adjust_m_(120, 2), T::new_from_str("Effects120Adjust2")?);
        assert_eq!(Effects_n_Adjust_m_(120, 120), T::new_from_str("Effects120Adjust120")?);
        assert_eq!(Effects_n_Pos(1), T::new_from_str("Effects1Pos")?);
        assert_eq!(Effects_n_Pos(2), T::new_from_str("Effects2Pos")?);
        assert_eq!(Effects_n_Pos(120), T::new_from_str("Effects120Pos")?);
        assert_eq!(Effects_n_PosRotate(1), T::new_from_str("Effects1PosRotate")?);
        assert_eq!(Effects_n_PosRotate(2), T::new_from_str("Effects2PosRotate")?);
        assert_eq!(Effects_n_PosRotate(120), T::new_from_str("Effects120PosRotate")?);
        assert_eq!(EffectsSync, T::new_from_str("EffectsSync")?);
        assert_eq!(BeamShaper, T::new_from_str("BeamShaper")?);
        assert_eq!(BeamShaperMacro, T::new_from_str("BeamShaperMacro")?);
        assert_eq!(BeamShaperPos, T::new_from_str("BeamShaperPos")?);
        assert_eq!(BeamShaperPosRotate, T::new_from_str("BeamShaperPosRotate")?);
        assert_eq!(Zoom, T::new_from_str("Zoom")?);
        assert_eq!(ZoomModeSpot, T::new_from_str("ZoomModeSpot")?);
        assert_eq!(ZoomModeBeam, T::new_from_str("ZoomModeBeam")?);
        assert_eq!(Focus_n_(1), T::new_from_str("Focus1")?);
        assert_eq!(Focus_n_(2), T::new_from_str("Focus2")?);
        assert_eq!(Focus_n_(120), T::new_from_str("Focus120")?);
        assert_eq!(Focus_n_Adjust(1), T::new_from_str("Focus1Adjust")?);
        assert_eq!(Focus_n_Adjust(2), T::new_from_str("Focus2Adjust")?);
        assert_eq!(Focus_n_Adjust(120), T::new_from_str("Focus120Adjust")?);
        assert_eq!(Focus_n_Distance(1), T::new_from_str("Focus1Distance")?);
        assert_eq!(Focus_n_Distance(2), T::new_from_str("Focus2Distance")?);
        assert_eq!(Focus_n_Distance(120), T::new_from_str("Focus120Distance")?);
        assert_eq!(Control_n_(1), T::new_from_str("Control1")?);
        assert_eq!(Control_n_(2), T::new_from_str("Control2")?);
        assert_eq!(Control_n_(120), T::new_from_str("Control120")?);
        assert_eq!(DimmerMode, T::new_from_str("DimmerMode")?);
        assert_eq!(DimmerCurve, T::new_from_str("DimmerCurve")?);
        assert_eq!(BlackoutMode, T::new_from_str("BlackoutMode")?);
        assert_eq!(LedFrequency, T::new_from_str("LEDFrequency")?);
        assert_eq!(LedZoneMode, T::new_from_str("LEDZoneMode")?);
        assert_eq!(PixelMode, T::new_from_str("PixelMode")?);
        assert_eq!(PanMode, T::new_from_str("PanMode")?);
        assert_eq!(TiltMode, T::new_from_str("TiltMode")?);
        assert_eq!(PanTiltMode, T::new_from_str("PanTiltMode")?);
        assert_eq!(PositionModes, T::new_from_str("PositionModes")?);
        assert_eq!(Gobo_n_WheelMode(1), T::new_from_str("Gobo1WheelMode")?);
        assert_eq!(Gobo_n_WheelMode(2), T::new_from_str("Gobo2WheelMode")?);
        assert_eq!(Gobo_n_WheelMode(120), T::new_from_str("Gobo120WheelMode")?);
        assert_eq!(AnimationWheel_n_Mode(1), T::new_from_str("AnimationWheel1Mode")?);
        assert_eq!(AnimationWheel_n_Mode(2), T::new_from_str("AnimationWheel2Mode")?);
        assert_eq!(AnimationWheel_n_Mode(120), T::new_from_str("AnimationWheel120Mode")?);
        assert_eq!(AnimationWheelShortcutMode, T::new_from_str("AnimationWheelShortcutMode")?);
        assert_eq!(Color_n_Mode(1), T::new_from_str("Color1Mode")?);
        assert_eq!(Color_n_Mode(2), T::new_from_str("Color2Mode")?);
        assert_eq!(Color_n_Mode(120), T::new_from_str("Color120Mode")?);
        assert_eq!(ColorWheelShortcutMode, T::new_from_str("ColorWheelShortcutMode")?);
        assert_eq!(CyanMode, T::new_from_str("CyanMode")?);
        assert_eq!(MagentaMode, T::new_from_str("MagentaMode")?);
        assert_eq!(YellowMode, T::new_from_str("YellowMode")?);
        assert_eq!(ColorMixMode, T::new_from_str("ColorMixMode")?);
        assert_eq!(ChromaticMode, T::new_from_str("ChromaticMode")?);
        assert_eq!(ColorCalibrationMode, T::new_from_str("ColorCalibrationMode")?);
        assert_eq!(ColorConsistency, T::new_from_str("ColorConsistency")?);
        assert_eq!(ColorControl, T::new_from_str("ColorControl")?);
        assert_eq!(ColorModelMode, T::new_from_str("ColorModelMode")?);
        assert_eq!(ColorSettingsReset, T::new_from_str("ColorSettingsReset")?);
        assert_eq!(ColorUniformity, T::new_from_str("ColorUniformity")?);
        assert_eq!(CriMode, T::new_from_str("CRIMode")?);
        assert_eq!(CustomColor, T::new_from_str("CustomColor")?);
        assert_eq!(UvStability, T::new_from_str("UVStability")?);
        assert_eq!(WavelengthCorrection, T::new_from_str("WavelengthCorrection")?);
        assert_eq!(WhiteCount, T::new_from_str("WhiteCount")?);
        assert_eq!(StrobeMode, T::new_from_str("StrobeMode")?);
        assert_eq!(ZoomMode, T::new_from_str("ZoomMode")?);
        assert_eq!(FocusMode, T::new_from_str("FocusMode")?);
        assert_eq!(IrisMode, T::new_from_str("IrisMode")?);
        assert_eq!(Fan_n_Mode(1), T::new_from_str("Fan1Mode")?);
        assert_eq!(Fan_n_Mode(2), T::new_from_str("Fan2Mode")?);
        assert_eq!(Fan_n_Mode(120), T::new_from_str("Fan120Mode")?);
        assert_eq!(FollowSpotMode, T::new_from_str("FollowSpotMode")?);
        assert_eq!(BeamEffectIndexRotateMode, T::new_from_str("BeamEffectIndexRotateMode")?);
        assert_eq!(IntensityMSpeed, T::new_from_str("IntensityMSpeed")?);
        assert_eq!(PositionMSpeed, T::new_from_str("PositionMSpeed")?);
        assert_eq!(ColorMixMSpeed, T::new_from_str("ColorMixMSpeed")?);
        assert_eq!(ColorWheelSelectMSpeed, T::new_from_str("ColorWheelSelectMSpeed")?);
        assert_eq!(GoboWheel_n_MSpeed(1), T::new_from_str("GoboWheel1MSpeed")?);
        assert_eq!(GoboWheel_n_MSpeed(2), T::new_from_str("GoboWheel2MSpeed")?);
        assert_eq!(GoboWheel_n_MSpeed(120), T::new_from_str("GoboWheel120MSpeed")?);
        assert_eq!(IrisMSpeed, T::new_from_str("IrisMSpeed")?);
        assert_eq!(Prism_n_MSpeed(1), T::new_from_str("Prism1MSpeed")?);
        assert_eq!(Prism_n_MSpeed(2), T::new_from_str("Prism2MSpeed")?);
        assert_eq!(Prism_n_MSpeed(120), T::new_from_str("Prism120MSpeed")?);
        assert_eq!(FocusMSpeed, T::new_from_str("FocusMSpeed")?);
        assert_eq!(Frost_n_MSpeed(1), T::new_from_str("Frost1MSpeed")?);
        assert_eq!(Frost_n_MSpeed(2), T::new_from_str("Frost2MSpeed")?);
        assert_eq!(Frost_n_MSpeed(120), T::new_from_str("Frost120MSpeed")?);
        assert_eq!(ZoomMSpeed, T::new_from_str("ZoomMSpeed")?);
        assert_eq!(FrameMSpeed, T::new_from_str("FrameMSpeed")?);
        assert_eq!(GlobalMSpeed, T::new_from_str("GlobalMSpeed")?);
        assert_eq!(ReflectorAdjust, T::new_from_str("ReflectorAdjust")?);
        assert_eq!(FixtureGlobalReset, T::new_from_str("FixtureGlobalReset")?);
        assert_eq!(ShutterReset, T::new_from_str("ShutterReset")?);
        assert_eq!(BeamReset, T::new_from_str("BeamReset")?);
        assert_eq!(ColorMixReset, T::new_from_str("ColorMixReset")?);
        assert_eq!(ColorWheelReset, T::new_from_str("ColorWheelReset")?);
        assert_eq!(FocusReset, T::new_from_str("FocusReset")?);
        assert_eq!(FrameReset, T::new_from_str("FrameReset")?);
        assert_eq!(GoboWheelReset, T::new_from_str("GoboWheelReset")?);
        assert_eq!(IntensityReset, T::new_from_str("IntensityReset")?);
        assert_eq!(IrisReset, T::new_from_str("IrisReset")?);
        assert_eq!(PositionReset, T::new_from_str("PositionReset")?);
        assert_eq!(PanReset, T::new_from_str("PanReset")?);
        assert_eq!(TiltReset, T::new_from_str("TiltReset")?);
        assert_eq!(ZoomReset, T::new_from_str("ZoomReset")?);
        assert_eq!(CtbReset, T::new_from_str("CTBReset")?);
        assert_eq!(CtoReset, T::new_from_str("CTOReset")?);
        assert_eq!(CtcReset, T::new_from_str("CTCReset")?);
        assert_eq!(AnimationSystemReset, T::new_from_str("AnimationSystemReset")?);
        assert_eq!(FixtureCalibrationReset, T::new_from_str("FixtureCalibrationReset")?);
        assert_eq!(Function, T::new_from_str("Function")?);
        assert_eq!(LampControl, T::new_from_str("LampControl")?);
        assert_eq!(DisplayIntensity, T::new_from_str("DisplayIntensity")?);
        assert_eq!(DmxInput, T::new_from_str("DMXInput")?);
        assert_eq!(NoFeature, T::new_from_str("NoFeature")?);
        assert_eq!(Blower_n_(1), T::new_from_str("Blower1")?);
        assert_eq!(Blower_n_(2), T::new_from_str("Blower2")?);
        assert_eq!(Blower_n_(120), T::new_from_str("Blower120")?);
        assert_eq!(Fan_n_(1), T::new_from_str("Fan1")?);
        assert_eq!(Fan_n_(2), T::new_from_str("Fan2")?);
        assert_eq!(Fan_n_(120), T::new_from_str("Fan120")?);
        assert_eq!(Fog_n_(1), T::new_from_str("Fog1")?);
        assert_eq!(Fog_n_(2), T::new_from_str("Fog2")?);
        assert_eq!(Fog_n_(120), T::new_from_str("Fog120")?);
        assert_eq!(Haze_n_(1), T::new_from_str("Haze1")?);
        assert_eq!(Haze_n_(2), T::new_from_str("Haze2")?);
        assert_eq!(Haze_n_(120), T::new_from_str("Haze120")?);
        assert_eq!(LampPowerMode, T::new_from_str("LampPowerMode")?);
        assert_eq!(Fans, T::new_from_str("Fans")?);
        assert_eq!(Blade_n_A(1), T::new_from_str("Blade1A")?);
        assert_eq!(Blade_n_A(2), T::new_from_str("Blade2A")?);
        assert_eq!(Blade_n_A(120), T::new_from_str("Blade120A")?);
        assert_eq!(Blade_n_B(1), T::new_from_str("Blade1B")?);
        assert_eq!(Blade_n_B(2), T::new_from_str("Blade2B")?);
        assert_eq!(Blade_n_B(120), T::new_from_str("Blade120B")?);
        assert_eq!(Blade_n_Rot(1), T::new_from_str("Blade1Rot")?);
        assert_eq!(Blade_n_Rot(2), T::new_from_str("Blade2Rot")?);
        assert_eq!(Blade_n_Rot(120), T::new_from_str("Blade120Rot")?);
        assert_eq!(ShaperRot, T::new_from_str("ShaperRot")?);
        assert_eq!(ShaperMacros, T::new_from_str("ShaperMacros")?);
        assert_eq!(ShaperMacrosSpeed, T::new_from_str("ShaperMacrosSpeed")?);
        assert_eq!(BladeSoft_n_A(1), T::new_from_str("BladeSoft1A")?);
        assert_eq!(BladeSoft_n_A(2), T::new_from_str("BladeSoft2A")?);
        assert_eq!(BladeSoft_n_A(120), T::new_from_str("BladeSoft120A")?);
        assert_eq!(BladeSoft_n_B(1), T::new_from_str("BladeSoft1B")?);
        assert_eq!(BladeSoft_n_B(2), T::new_from_str("BladeSoft2B")?);
        assert_eq!(BladeSoft_n_B(120), T::new_from_str("BladeSoft120B")?);
        assert_eq!(KeyStone_n_A(1), T::new_from_str("KeyStone1A")?);
        assert_eq!(KeyStone_n_A(2), T::new_from_str("KeyStone2A")?);
        assert_eq!(KeyStone_n_A(120), T::new_from_str("KeyStone120A")?);
        assert_eq!(KeyStone_n_B(1), T::new_from_str("KeyStone1B")?);
        assert_eq!(KeyStone_n_B(2), T::new_from_str("KeyStone2B")?);
        assert_eq!(KeyStone_n_B(120), T::new_from_str("KeyStone120B")?);
        assert_eq!(Video, T::new_from_str("Video")?);
        assert_eq!(VideoEffect_n_Type(1), T::new_from_str("VideoEffect1Type")?);
        assert_eq!(VideoEffect_n_Type(2), T::new_from_str("VideoEffect2Type")?);
        assert_eq!(VideoEffect_n_Type(120), T::new_from_str("VideoEffect120Type")?);
        assert_eq!(VideoEffect_n_Parameter_m_(1, 1), T::new_from_str("VideoEffect1Parameter1")?);
        assert_eq!(VideoEffect_n_Parameter_m_(1, 2), T::new_from_str("VideoEffect1Parameter2")?);
        assert_eq!(VideoEffect_n_Parameter_m_(2, 1), T::new_from_str("VideoEffect2Parameter1")?);
        assert_eq!(VideoEffect_n_Parameter_m_(2, 2), T::new_from_str("VideoEffect2Parameter2")?);
        assert_eq!(VideoEffect_n_Parameter_m_(2, 120), T::new_from_str("VideoEffect2Parameter120")?);
        assert_eq!(VideoEffect_n_Parameter_m_(120, 2), T::new_from_str("VideoEffect120Parameter2")?);
        assert_eq!(VideoEffect_n_Parameter_m_(120, 120), T::new_from_str("VideoEffect120Parameter120")?);
        assert_eq!(VideoCamera_n_(1), T::new_from_str("VideoCamera1")?);
        assert_eq!(VideoCamera_n_(2), T::new_from_str("VideoCamera2")?);
        assert_eq!(VideoCamera_n_(120), T::new_from_str("VideoCamera120")?);
        assert_eq!(VideoSoundVolume_n_(1), T::new_from_str("VideoSoundVolume1")?);
        assert_eq!(VideoSoundVolume_n_(2), T::new_from_str("VideoSoundVolume2")?);
        assert_eq!(VideoSoundVolume_n_(120), T::new_from_str("VideoSoundVolume120")?);
        assert_eq!(VideoBlendMode, T::new_from_str("VideoBlendMode")?);
        assert_eq!(InputSource, T::new_from_str("InputSource")?);
        assert_eq!(FieldOfView, T::new_from_str("FieldOfView")?);

        assert!(T::new_from_str("something{invalid").is_err());
        assert!(T::new_from_str("something௸invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_attr_owned() -> Result<(), GdtfError> {
        use T::*;
        assert_eq!(UserDefined(Name::new("test")?), T::new_from_attr(testdata::to_attr_owned(b"test"))?);
        assert_eq!(UserDefined(Name::new("")?), T::new_from_attr(testdata::to_attr_owned(b""))?);
        assert_eq!(Dimmer, T::new_from_attr(testdata::to_attr_owned(b"Dimmer"))?);
        assert_eq!(Pan, T::new_from_attr(testdata::to_attr_owned(b"Pan"))?);
        assert_eq!(Tilt, T::new_from_attr(testdata::to_attr_owned(b"Tilt"))?);
        assert_eq!(Gobo_n_(1), T::new_from_attr(testdata::to_attr_owned(b"Gobo1"))?);
        assert_eq!(Gobo_n_SelectSpin(2), T::new_from_attr(testdata::to_attr_owned(b"Gobo2SelectSpin"))?);
        assert_eq!(Gobo_n_SelectShake(120), T::new_from_attr(testdata::to_attr_owned(b"Gobo120SelectShake"))?);
        assert_eq!(Dimmer, T::new_from_attr(testdata::to_attr_owned(b"Dimmer"))?);
        assert_eq!(Dimmer, T::new_from_attr(testdata::to_attr_owned(b"Dimmer"))?);

        assert_eq!(Effects_n_Adjust_m_(1, 1), T::new_from_attr(testdata::to_attr_owned(b"Effects1Adjust1"))?);
        assert_eq!(Effects_n_Adjust_m_(1, 2), T::new_from_attr(testdata::to_attr_owned(b"Effects1Adjust2"))?);
        assert_eq!(Effects_n_Adjust_m_(2, 1), T::new_from_attr(testdata::to_attr_owned(b"Effects2Adjust1"))?);
        assert_eq!(Effects_n_Adjust_m_(2, 2), T::new_from_attr(testdata::to_attr_owned(b"Effects2Adjust2"))?);
        assert_eq!(Effects_n_Adjust_m_(2, 120), T::new_from_attr(testdata::to_attr_owned(b"Effects2Adjust120"))?);
        assert_eq!(Effects_n_Adjust_m_(120, 2), T::new_from_attr(testdata::to_attr_owned(b"Effects120Adjust2"))?);
        assert_eq!(Effects_n_Adjust_m_(120, 120), T::new_from_attr(testdata::to_attr_owned(b"Effects120Adjust120"))?);

        Ok(())
    }

    #[test]
    fn test_new_from_attr_borrowed() -> Result<(), GdtfError> {
        use T::*;
        assert_eq!(UserDefined(Name::new("test")?), T::new_from_attr(testdata::to_attr_borrowed(b"test"))?);
        assert_eq!(UserDefined(Name::new("")?), T::new_from_attr(testdata::to_attr_borrowed(b""))?);
        assert_eq!(Dimmer, T::new_from_attr(testdata::to_attr_borrowed(b"Dimmer"))?);
        assert_eq!(Pan, T::new_from_attr(testdata::to_attr_borrowed(b"Pan"))?);
        assert_eq!(Tilt, T::new_from_attr(testdata::to_attr_borrowed(b"Tilt"))?);
        assert_eq!(Gobo_n_(1), T::new_from_attr(testdata::to_attr_borrowed(b"Gobo1"))?);
        assert_eq!(Gobo_n_SelectSpin(2), T::new_from_attr(testdata::to_attr_borrowed(b"Gobo2SelectSpin"))?);
        assert_eq!(Gobo_n_SelectShake(120), T::new_from_attr(testdata::to_attr_borrowed(b"Gobo120SelectShake"))?);
        assert_eq!(Dimmer, T::new_from_attr(testdata::to_attr_borrowed(b"Dimmer"))?);
        assert_eq!(Dimmer, T::new_from_attr(testdata::to_attr_borrowed(b"Dimmer"))?);

        assert_eq!(Effects_n_Adjust_m_(1, 1), T::new_from_attr(testdata::to_attr_borrowed(b"Effects1Adjust1"))?);
        assert_eq!(Effects_n_Adjust_m_(1, 2), T::new_from_attr(testdata::to_attr_borrowed(b"Effects1Adjust2"))?);
        assert_eq!(Effects_n_Adjust_m_(2, 1), T::new_from_attr(testdata::to_attr_borrowed(b"Effects2Adjust1"))?);
        assert_eq!(Effects_n_Adjust_m_(2, 2), T::new_from_attr(testdata::to_attr_borrowed(b"Effects2Adjust2"))?);
        assert_eq!(Effects_n_Adjust_m_(2, 120), T::new_from_attr(testdata::to_attr_borrowed(b"Effects2Adjust120"))?);
        assert_eq!(Effects_n_Adjust_m_(120, 2), T::new_from_attr(testdata::to_attr_borrowed(b"Effects120Adjust2"))?);
        assert_eq!(Effects_n_Adjust_m_(120, 120), T::new_from_attr(testdata::to_attr_borrowed(b"Effects120Adjust120"))?);

        Ok(())
    }
}
