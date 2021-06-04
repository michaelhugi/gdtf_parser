//! AttributeName is a preferred Name for Attributes in GDTF Format.
#![allow(non_camel_case_types)]

use std::borrow::Borrow;
use std::convert::TryFrom;
use std::str::FromStr;

use lazy_static::lazy_static;
use quick_xml::events::attributes::Attribute;
use regex::{Regex, RegexSet, SetMatches};

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

///Default is an Empty user defined name
impl Default for AttributeName {
    fn default() -> Self {
        AttributeName::new_from_str("").unwrap()
    }
}


impl AttributeName {
    ///Creates an new AttributeName of &str defined by xml of GDTF. This method does not check if all chars are valid for Name defined by GDTF spec
    pub(crate) fn new_from_str(value: &str) -> Result<Self, GdtfNameError> {
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
}

///Creates an new AttributeName of &str defined by xml of GDTF. This method does check if all chars are valid for Name defined by GDTF spec
impl TryFrom<&str> for AttributeName {
    type Error = GdtfNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AttributeName::new_from_str(value)
    }
}

///Creates an new AttributeName of an xml-attribute defined by xml of GDTF. This method does check if all chars are valid for Name defined by GDTF spec
impl TryFrom<Attribute<'_>> for AttributeName {
    type Error = GdtfNameError;

    fn try_from(attr: Attribute<'_>) -> Result<Self, Self::Error> {
        AttributeName::new_from_str(std::str::from_utf8(attr.value.borrow()).unwrap_or(""))
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::name::Name;

    #[test]
    fn test_default() -> Result<(), GdtfError> {
        use AttributeName::*;
        assert_eq!(UserDefined(Name::new("")?), AttributeName::default());
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        use AttributeName::*;
        assert_eq!(UserDefined(Name::new("test")?), "test".try_into()?);
        assert_eq!(AttributeName::UserDefined(Name::new("")?), AttributeName::try_from("")?);
        assert_eq!(Dimmer, "Dimmer".try_into()?);
        assert_eq!(Pan, "Pan".try_into()?);
        assert_eq!(Tilt, "Tilt".try_into()?);
        assert_eq!(PanRotate, "PanRotate".try_into()?);
        assert_eq!(TiltRotate, "TiltRotate".try_into()?);
        assert_eq!(PositionEffect, "PositionEffect".try_into()?);
        assert_eq!(PositionEffectRate, "PositionEffectRate".try_into()?);
        assert_eq!(PositionEffectFade, "PositionEffectFade".try_into()?);
        assert_eq!(Xyz_X, "XYZ_X".try_into()?);
        assert_eq!(Xyz_Y, "XYZ_Y".try_into()?);
        assert_eq!(Xyz_Z, "XYZ_Z".try_into()?);
        assert_eq!(Rot_X, "Rot_X".try_into()?);
        assert_eq!(Rot_Y, "Rot_Y".try_into()?);
        assert_eq!(Rot_Z, "Rot_Z".try_into()?);
        assert_eq!(Scale_X, "Scale_X".try_into()?);
        assert_eq!(Scale_Y, "Scale_Y".try_into()?);
        assert_eq!(Scale_Z, "Scale_Z".try_into()?);
        assert_eq!(Scale_Xyz, "Scale_XYZ".try_into()?);
        assert_eq!(Gobo_n_(1), "Gobo1".try_into()?);
        assert_eq!(Gobo_n_(2), "Gobo2".try_into()?);
        assert_eq!(Gobo_n_(120), "Gobo120".try_into()?);
        assert_eq!(Gobo_n_SelectSpin(1), "Gobo1SelectSpin".try_into()?);
        assert_eq!(Gobo_n_SelectSpin(2), "Gobo2SelectSpin".try_into()?);
        assert_eq!(Gobo_n_SelectSpin(120), "Gobo120SelectSpin".try_into()?);
        assert_eq!(Gobo_n_SelectShake(1), "Gobo1SelectShake".try_into()?);
        assert_eq!(Gobo_n_SelectShake(2), "Gobo2SelectShake".try_into()?);
        assert_eq!(Gobo_n_SelectShake(120), "Gobo120SelectShake".try_into()?);
        assert_eq!(Gobo_n_SelectEffects(1), "Gobo1SelectEffects".try_into()?);
        assert_eq!(Gobo_n_SelectEffects(2), "Gobo2SelectEffects".try_into()?);
        assert_eq!(Gobo_n_SelectEffects(120), "Gobo120SelectEffects".try_into()?);
        assert_eq!(Gobo_n_WheelIndex(1), "Gobo1WheelIndex".try_into()?);
        assert_eq!(Gobo_n_WheelIndex(2), "Gobo2WheelIndex".try_into()?);
        assert_eq!(Gobo_n_WheelIndex(120), "Gobo120WheelIndex".try_into()?);
        assert_eq!(Gobo_n_WheelSpin(1), "Gobo1WheelSpin".try_into()?);
        assert_eq!(Gobo_n_WheelSpin(2), "Gobo2WheelSpin".try_into()?);
        assert_eq!(Gobo_n_WheelSpin(120), "Gobo120WheelSpin".try_into()?);
        assert_eq!(Gobo_n_WheelShake(1), "Gobo1WheelShake".try_into()?);
        assert_eq!(Gobo_n_WheelShake(2), "Gobo2WheelShake".try_into()?);
        assert_eq!(Gobo_n_WheelShake(120), "Gobo120WheelShake".try_into()?);
        assert_eq!(Gobo_n_WheelRandom(1), "Gobo1WheelRandom".try_into()?);
        assert_eq!(Gobo_n_WheelRandom(2), "Gobo2WheelRandom".try_into()?);
        assert_eq!(Gobo_n_WheelRandom(120), "Gobo120WheelRandom".try_into()?);
        assert_eq!(Gobo_n_WheelAudio(1), "Gobo1WheelAudio".try_into()?);
        assert_eq!(Gobo_n_WheelAudio(2), "Gobo2WheelAudio".try_into()?);
        assert_eq!(Gobo_n_WheelAudio(120), "Gobo120WheelAudio".try_into()?);
        assert_eq!(Gobo_n_Pos(1), "Gobo1Pos".try_into()?);
        assert_eq!(Gobo_n_Pos(2), "Gobo2Pos".try_into()?);
        assert_eq!(Gobo_n_Pos(120), "Gobo120Pos".try_into()?);
        assert_eq!(Gobo_n_PosRotate(1), "Gobo1PosRotate".try_into()?);
        assert_eq!(Gobo_n_PosRotate(2), "Gobo2PosRotate".try_into()?);
        assert_eq!(Gobo_n_PosRotate(120), "Gobo120PosRotate".try_into()?);
        assert_eq!(Gobo_n_PosShake(1), "Gobo1PosShake".try_into()?);
        assert_eq!(Gobo_n_PosShake(2), "Gobo2PosShake".try_into()?);
        assert_eq!(Gobo_n_PosShake(120), "Gobo120PosShake".try_into()?);
        assert_eq!(AnimationWheel_n_(1), "AnimationWheel1".try_into()?);
        assert_eq!(AnimationWheel_n_(2), "AnimationWheel2".try_into()?);
        assert_eq!(AnimationWheel_n_(120), "AnimationWheel120".try_into()?);
        assert_eq!(AnimationWheel_n_Audio(1), "AnimationWheel1Audio".try_into()?);
        assert_eq!(AnimationWheel_n_Audio(2), "AnimationWheel2Audio".try_into()?);
        assert_eq!(AnimationWheel_n_Audio(120), "AnimationWheel120Audio".try_into()?);
        assert_eq!(AnimationWheel_n_Macro(1), "AnimationWheel1Macro".try_into()?);
        assert_eq!(AnimationWheel_n_Macro(2), "AnimationWheel2Macro".try_into()?);
        assert_eq!(AnimationWheel_n_Macro(120), "AnimationWheel120Macro".try_into()?);
        assert_eq!(AnimationWheel_n_Random(1), "AnimationWheel1Random".try_into()?);
        assert_eq!(AnimationWheel_n_Random(2), "AnimationWheel2Random".try_into()?);
        assert_eq!(AnimationWheel_n_Random(120), "AnimationWheel120Random".try_into()?);
        assert_eq!(AnimationWheel_n_SelectEffects(1), "AnimationWheel1SelectEffects".try_into()?);
        assert_eq!(AnimationWheel_n_SelectEffects(2), "AnimationWheel2SelectEffects".try_into()?);
        assert_eq!(AnimationWheel_n_SelectEffects(120), "AnimationWheel120SelectEffects".try_into()?);
        assert_eq!(AnimationWheel_n_SelectShake(1), "AnimationWheel1SelectShake".try_into()?);
        assert_eq!(AnimationWheel_n_SelectShake(2), "AnimationWheel2SelectShake".try_into()?);
        assert_eq!(AnimationWheel_n_SelectShake(120), "AnimationWheel120SelectShake".try_into()?);
        assert_eq!(AnimationWheel_n_SelectSpin(1), "AnimationWheel1SelectSpin".try_into()?);
        assert_eq!(AnimationWheel_n_SelectSpin(2), "AnimationWheel2SelectSpin".try_into()?);
        assert_eq!(AnimationWheel_n_SelectSpin(120), "AnimationWheel120SelectSpin".try_into()?);
        assert_eq!(AnimationWheel_n_Pos(1), "AnimationWheel1Pos".try_into()?);
        assert_eq!(AnimationWheel_n_Pos(2), "AnimationWheel2Pos".try_into()?);
        assert_eq!(AnimationWheel_n_Pos(120), "AnimationWheel120Pos".try_into()?);
        assert_eq!(AnimationWheel_n_PosRotate(1), "AnimationWheel1PosRotate".try_into()?);
        assert_eq!(AnimationWheel_n_PosRotate(2), "AnimationWheel2PosRotate".try_into()?);
        assert_eq!(AnimationWheel_n_PosRotate(120), "AnimationWheel120PosRotate".try_into()?);
        assert_eq!(AnimationWheel_n_PosShake(1), "AnimationWheel1PosShake".try_into()?);
        assert_eq!(AnimationWheel_n_PosShake(2), "AnimationWheel2PosShake".try_into()?);
        assert_eq!(AnimationWheel_n_PosShake(120), "AnimationWheel120PosShake".try_into()?);
        assert_eq!(AnimationSystem_n_(1), "AnimationSystem1".try_into()?);
        assert_eq!(AnimationSystem_n_(2), "AnimationSystem2".try_into()?);
        assert_eq!(AnimationSystem_n_(120), "AnimationSystem120".try_into()?);
        assert_eq!(AnimationSystem_n_Ramp(1), "AnimationSystem1Ramp".try_into()?);
        assert_eq!(AnimationSystem_n_Ramp(2), "AnimationSystem2Ramp".try_into()?);
        assert_eq!(AnimationSystem_n_Ramp(120), "AnimationSystem120Ramp".try_into()?);
        assert_eq!(AnimationSystem_n_Shake(1), "AnimationSystem1Shake".try_into()?);
        assert_eq!(AnimationSystem_n_Shake(2), "AnimationSystem2Shake".try_into()?);
        assert_eq!(AnimationSystem_n_Shake(120), "AnimationSystem120Shake".try_into()?);
        assert_eq!(AnimationSystem_n_Audio(1), "AnimationSystem1Audio".try_into()?);
        assert_eq!(AnimationSystem_n_Audio(2), "AnimationSystem2Audio".try_into()?);
        assert_eq!(AnimationSystem_n_Audio(120), "AnimationSystem120Audio".try_into()?);
        assert_eq!(AnimationSystem_n_Random(1), "AnimationSystem1Random".try_into()?);
        assert_eq!(AnimationSystem_n_Random(2), "AnimationSystem2Random".try_into()?);
        assert_eq!(AnimationSystem_n_Random(120), "AnimationSystem120Random".try_into()?);
        assert_eq!(AnimationSystem_n_Pos(1), "AnimationSystem1Pos".try_into()?);
        assert_eq!(AnimationSystem_n_Pos(2), "AnimationSystem2Pos".try_into()?);
        assert_eq!(AnimationSystem_n_Pos(120), "AnimationSystem120Pos".try_into()?);
        assert_eq!(AnimationSystem_n_PosRotate(1), "AnimationSystem1PosRotate".try_into()?);
        assert_eq!(AnimationSystem_n_PosRotate(2), "AnimationSystem2PosRotate".try_into()?);
        assert_eq!(AnimationSystem_n_PosRotate(120), "AnimationSystem120PosRotate".try_into()?);
        assert_eq!(AnimationSystem_n_PosShake(1), "AnimationSystem1PosShake".try_into()?);
        assert_eq!(AnimationSystem_n_PosShake(2), "AnimationSystem2PosShake".try_into()?);
        assert_eq!(AnimationSystem_n_PosShake(120), "AnimationSystem120PosShake".try_into()?);
        assert_eq!(AnimationSystem_n_PosRandom(1), "AnimationSystem1PosRandom".try_into()?);
        assert_eq!(AnimationSystem_n_PosRandom(2), "AnimationSystem2PosRandom".try_into()?);
        assert_eq!(AnimationSystem_n_PosRandom(120), "AnimationSystem120PosRandom".try_into()?);
        assert_eq!(AnimationSystem_n_PosAudio(1), "AnimationSystem1PosAudio".try_into()?);
        assert_eq!(AnimationSystem_n_PosAudio(2), "AnimationSystem2PosAudio".try_into()?);
        assert_eq!(AnimationSystem_n_PosAudio(120), "AnimationSystem120PosAudio".try_into()?);
        assert_eq!(AnimationSystem_n_Macro(1), "AnimationSystem1Macro".try_into()?);
        assert_eq!(AnimationSystem_n_Macro(2), "AnimationSystem2Macro".try_into()?);
        assert_eq!(AnimationSystem_n_Macro(120), "AnimationSystem120Macro".try_into()?);
        assert_eq!(MediaFolder_n_(1), "MediaFolder1".try_into()?);
        assert_eq!(MediaFolder_n_(2), "MediaFolder2".try_into()?);
        assert_eq!(MediaFolder_n_(120), "MediaFolder120".try_into()?);
        assert_eq!(MediaContent_n_(1), "MediaContent1".try_into()?);
        assert_eq!(MediaContent_n_(2), "MediaContent2".try_into()?);
        assert_eq!(MediaContent_n_(120), "MediaContent120".try_into()?);
        assert_eq!(ModelFolder_n_(1), "ModelFolder1".try_into()?);
        assert_eq!(ModelFolder_n_(2), "ModelFolder2".try_into()?);
        assert_eq!(ModelFolder_n_(120), "ModelFolder120".try_into()?);
        assert_eq!(ModelContent_n_(1), "ModelContent1".try_into()?);
        assert_eq!(ModelContent_n_(2), "ModelContent2".try_into()?);
        assert_eq!(ModelContent_n_(120), "ModelContent120".try_into()?);
        assert_eq!(PlayMode, "PlayMode".try_into()?);
        assert_eq!(PlayBegin, "PlayBegin".try_into()?);
        assert_eq!(PlayEnd, "PlayEnd".try_into()?);
        assert_eq!(PlaySpeed, "PlaySpeed".try_into()?);
        assert_eq!(ColorEffects_n_(1), "ColorEffects1".try_into()?);
        assert_eq!(ColorEffects_n_(2), "ColorEffects2".try_into()?);
        assert_eq!(ColorEffects_n_(120), "ColorEffects120".try_into()?);
        assert_eq!(Color_n_(1), "Color1".try_into()?);
        assert_eq!(Color_n_(2), "Color2".try_into()?);
        assert_eq!(Color_n_(120), "Color120".try_into()?);
        assert_eq!(Color_n_WheelIndex(1), "Color1WheelIndex".try_into()?);
        assert_eq!(Color_n_WheelIndex(2), "Color2WheelIndex".try_into()?);
        assert_eq!(Color_n_WheelIndex(120), "Color120WheelIndex".try_into()?);
        assert_eq!(Color_n_WheelSpin(1), "Color1WheelSpin".try_into()?);
        assert_eq!(Color_n_WheelSpin(2), "Color2WheelSpin".try_into()?);
        assert_eq!(Color_n_WheelSpin(120), "Color120WheelSpin".try_into()?);
        assert_eq!(Color_n_WheelRandom(1), "Color1WheelRandom".try_into()?);
        assert_eq!(Color_n_WheelRandom(2), "Color2WheelRandom".try_into()?);
        assert_eq!(Color_n_WheelRandom(120), "Color120WheelRandom".try_into()?);
        assert_eq!(Color_n_WheelAudio(1), "Color1WheelAudio".try_into()?);
        assert_eq!(Color_n_WheelAudio(2), "Color2WheelAudio".try_into()?);
        assert_eq!(Color_n_WheelAudio(120), "Color120WheelAudio".try_into()?);
        assert_eq!(ColorAdd_R, "ColorAdd_R".try_into()?);
        assert_eq!(ColorAdd_G, "ColorAdd_G".try_into()?);
        assert_eq!(ColorAdd_B, "ColorAdd_B".try_into()?);
        assert_eq!(ColorAdd_C, "ColorAdd_C".try_into()?);
        assert_eq!(ColorAdd_M, "ColorAdd_M".try_into()?);
        assert_eq!(ColorAdd_Y, "ColorAdd_Y".try_into()?);
        assert_eq!(ColorAdd_Ry, "ColorAdd_RY".try_into()?);
        assert_eq!(ColorAdd_Gy, "ColorAdd_GY".try_into()?);
        assert_eq!(ColorAdd_Gc, "ColorAdd_GC".try_into()?);
        assert_eq!(ColorAdd_Bc, "ColorAdd_BC".try_into()?);
        assert_eq!(ColorAdd_Bm, "ColorAdd_BM".try_into()?);
        assert_eq!(ColorAdd_Rm, "ColorAdd_RM".try_into()?);
        assert_eq!(ColorAdd_W, "ColorAdd_W".try_into()?);
        assert_eq!(ColorAdd_Ww, "ColorAdd_WW".try_into()?);
        assert_eq!(ColorAdd_Cw, "ColorAdd_CW".try_into()?);
        assert_eq!(ColorAdd_Uv, "ColorAdd_UV".try_into()?);
        assert_eq!(ColorSub_R, "ColorSub_R".try_into()?);
        assert_eq!(ColorSub_G, "ColorSub_G".try_into()?);
        assert_eq!(ColorSub_B, "ColorSub_B".try_into()?);
        assert_eq!(ColorSub_C, "ColorSub_C".try_into()?);
        assert_eq!(ColorSub_M, "ColorSub_M".try_into()?);
        assert_eq!(ColorSub_Y, "ColorSub_Y".try_into()?);
        assert_eq!(ColorMacro_n_(1), "ColorMacro1".try_into()?);
        assert_eq!(ColorMacro_n_(2), "ColorMacro2".try_into()?);
        assert_eq!(ColorMacro_n_(120), "ColorMacro120".try_into()?);
        assert_eq!(ColorMacro_n_Rate(1), "ColorMacro1Rate".try_into()?);
        assert_eq!(ColorMacro_n_Rate(2), "ColorMacro2Rate".try_into()?);
        assert_eq!(ColorMacro_n_Rate(120), "ColorMacro120Rate".try_into()?);
        assert_eq!(Cto, "CTO".try_into()?);
        assert_eq!(Ctc, "CTC".try_into()?);
        assert_eq!(Ctb, "CTB".try_into()?);
        assert_eq!(Tint, "Tint".try_into()?);
        assert_eq!(Hsb_Hue, "HSB_Hue".try_into()?);
        assert_eq!(Hsb_Saturation, "HSB_Saturation".try_into()?);
        assert_eq!(Hsb_Brightness, "HSB_Brightness".try_into()?);
        assert_eq!(Hsb_Quality, "HSB_Quality".try_into()?);
        assert_eq!(Cie_X, "CIE_X".try_into()?);
        assert_eq!(Cie_Y, "CIE_Y".try_into()?);
        assert_eq!(Cie_Brightness, "CIE_Brightness".try_into()?);
        assert_eq!(ColorRgb_Red, "ColorRGB_Red".try_into()?);
        assert_eq!(ColorRgb_Green, "ColorRGB_Green".try_into()?);
        assert_eq!(ColorRgb_Blue, "ColorRGB_Blue".try_into()?);
        assert_eq!(ColorRgb_Cyan, "ColorRGB_Cyan".try_into()?);
        assert_eq!(ColorRgb_Magenta, "ColorRGB_Magenta".try_into()?);
        assert_eq!(ColorRgb_Yellow, "ColorRGB_Yellow".try_into()?);
        assert_eq!(ColorRgb_Quality, "ColorRGB_Quality".try_into()?);
        assert_eq!(VideoBoost_R, "VideoBoost_R".try_into()?);
        assert_eq!(VideoBoost_G, "VideoBoost_G".try_into()?);
        assert_eq!(VideoBoost_B, "VideoBoost_B".try_into()?);
        assert_eq!(VideoHueShift, "VideoHueShift".try_into()?);
        assert_eq!(VideoSaturation, "VideoSaturation".try_into()?);
        assert_eq!(VideoBrightness, "VideoBrightness".try_into()?);
        assert_eq!(VideoContrast, "VideoContrast".try_into()?);
        assert_eq!(VideoKeyColor_R, "VideoKeyColor_R".try_into()?);
        assert_eq!(VideoKeyColor_G, "VideoKeyColor_G".try_into()?);
        assert_eq!(VideoKeyColor_B, "VideoKeyColor_B".try_into()?);
        assert_eq!(VideoKeyIntensity, "VideoKeyIntensity".try_into()?);
        assert_eq!(VideoKeyTolerance, "VideoKeyTolerance".try_into()?);
        assert_eq!(StrobeDuration, "StrobeDuration".try_into()?);
        assert_eq!(StrobeRate, "StrobeRate".try_into()?);
        assert_eq!(Shutter_n_(1), "Shutter1".try_into()?);
        assert_eq!(Shutter_n_(2), "Shutter2".try_into()?);
        assert_eq!(Shutter_n_(120), "Shutter120".try_into()?);
        assert_eq!(Shutter_n_Strobe(1), "Shutter1Strobe".try_into()?);
        assert_eq!(Shutter_n_Strobe(2), "Shutter2Strobe".try_into()?);
        assert_eq!(Shutter_n_Strobe(120), "Shutter120Strobe".try_into()?);
        assert_eq!(Shutter_n_StrobePulse(1), "Shutter1StrobePulse".try_into()?);
        assert_eq!(Shutter_n_StrobePulse(2), "Shutter2StrobePulse".try_into()?);
        assert_eq!(Shutter_n_StrobePulse(120), "Shutter120StrobePulse".try_into()?);
        assert_eq!(Shutter_n_StrobePulseClose(1), "Shutter1StrobePulseClose".try_into()?);
        assert_eq!(Shutter_n_StrobePulseClose(2), "Shutter2StrobePulseClose".try_into()?);
        assert_eq!(Shutter_n_StrobePulseOpen(1), "Shutter1StrobePulseOpen".try_into()?);
        assert_eq!(Shutter_n_StrobePulseOpen(2), "Shutter2StrobePulseOpen".try_into()?);
        assert_eq!(Shutter_n_StrobePulseOpen(120), "Shutter120StrobePulseOpen".try_into()?);
        assert_eq!(Shutter_n_StrobeRandom(1), "Shutter1StrobeRandom".try_into()?);
        assert_eq!(Shutter_n_StrobeRandom(2), "Shutter2StrobeRandom".try_into()?);
        assert_eq!(Shutter_n_StrobeRandom(120), "Shutter120StrobeRandom".try_into()?);
        assert_eq!(Shutter_n_StrobeRandomPulse(1), "Shutter1StrobeRandomPulse".try_into()?);
        assert_eq!(Shutter_n_StrobeRandomPulse(2), "Shutter2StrobeRandomPulse".try_into()?);
        assert_eq!(Shutter_n_StrobeRandomPulse(120), "Shutter120StrobeRandomPulse".try_into()?);
        assert_eq!(Shutter_n_StrobeRandomPulseClose(1), "Shutter1StrobeRandomPulseClose".try_into()?);
        assert_eq!(Shutter_n_StrobeRandomPulseClose(2), "Shutter2StrobeRandomPulseClose".try_into()?);
        assert_eq!(Shutter_n_StrobeRandomPulseClose(120), "Shutter120StrobeRandomPulseClose".try_into()?);
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(1), "Shutter1StrobeRandomPulseOpen".try_into()?);
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(2), "Shutter2StrobeRandomPulseOpen".try_into()?);
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(120), "Shutter120StrobeRandomPulseOpen".try_into()?);
        assert_eq!(Shutter_n_StrobeEffect(1), "Shutter1StrobeEffect".try_into()?);
        assert_eq!(Shutter_n_StrobeEffect(2), "Shutter2StrobeEffect".try_into()?);
        assert_eq!(Shutter_n_StrobeEffect(120), "Shutter120StrobeEffect".try_into()?);
        assert_eq!(Iris, "Iris".try_into()?);
        assert_eq!(IrisStrobe, "IrisStrobe".try_into()?);
        assert_eq!(IrisStrobeRandom, "IrisStrobeRandom".try_into()?);
        assert_eq!(IrisPulseClose, "IrisPulseClose".try_into()?);
        assert_eq!(IrisPulseOpen, "IrisPulseOpen".try_into()?);
        assert_eq!(IrisRandomPulseClose, "IrisRandomPulseClose".try_into()?);
        assert_eq!(IrisRandomPulseOpen, "IrisRandomPulseOpen".try_into()?);
        assert_eq!(Frost_n_(1), "Frost1".try_into()?);
        assert_eq!(Frost_n_(2), "Frost2".try_into()?);
        assert_eq!(Frost_n_(120), "Frost120".try_into()?);
        assert_eq!(Frost_n_PulseOpen(1), "Frost1PulseOpen".try_into()?);
        assert_eq!(Frost_n_PulseOpen(2), "Frost2PulseOpen".try_into()?);
        assert_eq!(Frost_n_PulseOpen(120), "Frost120PulseOpen".try_into()?);
        assert_eq!(Frost_n_PulseClose(1), "Frost1PulseClose".try_into()?);
        assert_eq!(Frost_n_PulseClose(2), "Frost2PulseClose".try_into()?);
        assert_eq!(Frost_n_PulseClose(120), "Frost120PulseClose".try_into()?);
        assert_eq!(Frost_n_Ramp(1), "Frost1Ramp".try_into()?);
        assert_eq!(Frost_n_Ramp(2), "Frost2Ramp".try_into()?);
        assert_eq!(Frost_n_Ramp(120), "Frost120Ramp".try_into()?);
        assert_eq!(Prism_n_(1), "Prism1".try_into()?);
        assert_eq!(Prism_n_(2), "Prism2".try_into()?);
        assert_eq!(Prism_n_(120), "Prism120".try_into()?);
        assert_eq!(Prism_n_SelectSpin(1), "Prism1SelectSpin".try_into()?);
        assert_eq!(Prism_n_SelectSpin(2), "Prism2SelectSpin".try_into()?);
        assert_eq!(Prism_n_SelectSpin(120), "Prism120SelectSpin".try_into()?);
        assert_eq!(Prism_n_Macro(1), "Prism1Macro".try_into()?);
        assert_eq!(Prism_n_Macro(2), "Prism2Macro".try_into()?);
        assert_eq!(Prism_n_Macro(120), "Prism120Macro".try_into()?);
        assert_eq!(Prism_n_Pos(1), "Prism1Pos".try_into()?);
        assert_eq!(Prism_n_Pos(2), "Prism2Pos".try_into()?);
        assert_eq!(Prism_n_Pos(120), "Prism120Pos".try_into()?);
        assert_eq!(Prism_n_PosRotate(1), "Prism1PosRotate".try_into()?);
        assert_eq!(Prism_n_PosRotate(2), "Prism2PosRotate".try_into()?);
        assert_eq!(Prism_n_PosRotate(120), "Prism120PosRotate".try_into()?);
        assert_eq!(Effects_n_(1), "Effects1".try_into()?);
        assert_eq!(Effects_n_(2), "Effects2".try_into()?);
        assert_eq!(Effects_n_(120), "Effects120".try_into()?);
        assert_eq!(Effects_n_Rate(1), "Effects1Rate".try_into()?);
        assert_eq!(Effects_n_Rate(2), "Effects2Rate".try_into()?);
        assert_eq!(Effects_n_Rate(120), "Effects120Rate".try_into()?);
        assert_eq!(Effects_n_Fade(1), "Effects1Fade".try_into()?);
        assert_eq!(Effects_n_Fade(2), "Effects2Fade".try_into()?);
        assert_eq!(Effects_n_Fade(120), "Effects120Fade".try_into()?);
        assert_eq!(Effects_n_Adjust_m_(1, 1), "Effects1Adjust1".try_into()?);
        assert_eq!(Effects_n_Adjust_m_(1, 2), "Effects1Adjust2".try_into()?);
        assert_eq!(Effects_n_Adjust_m_(2, 1), "Effects2Adjust1".try_into()?);
        assert_eq!(Effects_n_Adjust_m_(2, 2), "Effects2Adjust2".try_into()?);
        assert_eq!(Effects_n_Adjust_m_(2, 120), "Effects2Adjust120".try_into()?);
        assert_eq!(Effects_n_Adjust_m_(120, 2), "Effects120Adjust2".try_into()?);
        assert_eq!(Effects_n_Adjust_m_(120, 120), "Effects120Adjust120".try_into()?);
        assert_eq!(Effects_n_Pos(1), "Effects1Pos".try_into()?);
        assert_eq!(Effects_n_Pos(2), "Effects2Pos".try_into()?);
        assert_eq!(Effects_n_Pos(120), "Effects120Pos".try_into()?);
        assert_eq!(Effects_n_PosRotate(1), "Effects1PosRotate".try_into()?);
        assert_eq!(Effects_n_PosRotate(2), "Effects2PosRotate".try_into()?);
        assert_eq!(Effects_n_PosRotate(120), "Effects120PosRotate".try_into()?);
        assert_eq!(EffectsSync, "EffectsSync".try_into()?);
        assert_eq!(BeamShaper, "BeamShaper".try_into()?);
        assert_eq!(BeamShaperMacro, "BeamShaperMacro".try_into()?);
        assert_eq!(BeamShaperPos, "BeamShaperPos".try_into()?);
        assert_eq!(BeamShaperPosRotate, "BeamShaperPosRotate".try_into()?);
        assert_eq!(Zoom, "Zoom".try_into()?);
        assert_eq!(ZoomModeSpot, "ZoomModeSpot".try_into()?);
        assert_eq!(ZoomModeBeam, "ZoomModeBeam".try_into()?);
        assert_eq!(Focus_n_(1), "Focus1".try_into()?);
        assert_eq!(Focus_n_(2), "Focus2".try_into()?);
        assert_eq!(Focus_n_(120), "Focus120".try_into()?);
        assert_eq!(Focus_n_Adjust(1), "Focus1Adjust".try_into()?);
        assert_eq!(Focus_n_Adjust(2), "Focus2Adjust".try_into()?);
        assert_eq!(Focus_n_Adjust(120), "Focus120Adjust".try_into()?);
        assert_eq!(Focus_n_Distance(1), "Focus1Distance".try_into()?);
        assert_eq!(Focus_n_Distance(2), "Focus2Distance".try_into()?);
        assert_eq!(Focus_n_Distance(120), "Focus120Distance".try_into()?);
        assert_eq!(Control_n_(1), "Control1".try_into()?);
        assert_eq!(Control_n_(2), "Control2".try_into()?);
        assert_eq!(Control_n_(120), "Control120".try_into()?);
        assert_eq!(DimmerMode, "DimmerMode".try_into()?);
        assert_eq!(DimmerCurve, "DimmerCurve".try_into()?);
        assert_eq!(BlackoutMode, "BlackoutMode".try_into()?);
        assert_eq!(LedFrequency, "LEDFrequency".try_into()?);
        assert_eq!(LedZoneMode, "LEDZoneMode".try_into()?);
        assert_eq!(PixelMode, "PixelMode".try_into()?);
        assert_eq!(PanMode, "PanMode".try_into()?);
        assert_eq!(TiltMode, "TiltMode".try_into()?);
        assert_eq!(PanTiltMode, "PanTiltMode".try_into()?);
        assert_eq!(PositionModes, "PositionModes".try_into()?);
        assert_eq!(Gobo_n_WheelMode(1), "Gobo1WheelMode".try_into()?);
        assert_eq!(Gobo_n_WheelMode(2), "Gobo2WheelMode".try_into()?);
        assert_eq!(Gobo_n_WheelMode(120), "Gobo120WheelMode".try_into()?);
        assert_eq!(AnimationWheel_n_Mode(1), "AnimationWheel1Mode".try_into()?);
        assert_eq!(AnimationWheel_n_Mode(2), "AnimationWheel2Mode".try_into()?);
        assert_eq!(AnimationWheel_n_Mode(120), "AnimationWheel120Mode".try_into()?);
        assert_eq!(AnimationWheelShortcutMode, "AnimationWheelShortcutMode".try_into()?);
        assert_eq!(Color_n_Mode(1), "Color1Mode".try_into()?);
        assert_eq!(Color_n_Mode(2), "Color2Mode".try_into()?);
        assert_eq!(Color_n_Mode(120), "Color120Mode".try_into()?);
        assert_eq!(ColorWheelShortcutMode, "ColorWheelShortcutMode".try_into()?);
        assert_eq!(CyanMode, "CyanMode".try_into()?);
        assert_eq!(MagentaMode, "MagentaMode".try_into()?);
        assert_eq!(YellowMode, "YellowMode".try_into()?);
        assert_eq!(ColorMixMode, "ColorMixMode".try_into()?);
        assert_eq!(ChromaticMode, "ChromaticMode".try_into()?);
        assert_eq!(ColorCalibrationMode, "ColorCalibrationMode".try_into()?);
        assert_eq!(ColorConsistency, "ColorConsistency".try_into()?);
        assert_eq!(ColorControl, "ColorControl".try_into()?);
        assert_eq!(ColorModelMode, "ColorModelMode".try_into()?);
        assert_eq!(ColorSettingsReset, "ColorSettingsReset".try_into()?);
        assert_eq!(ColorUniformity, "ColorUniformity".try_into()?);
        assert_eq!(CriMode, "CRIMode".try_into()?);
        assert_eq!(CustomColor, "CustomColor".try_into()?);
        assert_eq!(UvStability, "UVStability".try_into()?);
        assert_eq!(WavelengthCorrection, "WavelengthCorrection".try_into()?);
        assert_eq!(WhiteCount, "WhiteCount".try_into()?);
        assert_eq!(StrobeMode, "StrobeMode".try_into()?);
        assert_eq!(ZoomMode, "ZoomMode".try_into()?);
        assert_eq!(FocusMode, "FocusMode".try_into()?);
        assert_eq!(IrisMode, "IrisMode".try_into()?);
        assert_eq!(Fan_n_Mode(1), "Fan1Mode".try_into()?);
        assert_eq!(Fan_n_Mode(2), "Fan2Mode".try_into()?);
        assert_eq!(Fan_n_Mode(120), "Fan120Mode".try_into()?);
        assert_eq!(FollowSpotMode, "FollowSpotMode".try_into()?);
        assert_eq!(BeamEffectIndexRotateMode, "BeamEffectIndexRotateMode".try_into()?);
        assert_eq!(IntensityMSpeed, "IntensityMSpeed".try_into()?);
        assert_eq!(PositionMSpeed, "PositionMSpeed".try_into()?);
        assert_eq!(ColorMixMSpeed, "ColorMixMSpeed".try_into()?);
        assert_eq!(ColorWheelSelectMSpeed, "ColorWheelSelectMSpeed".try_into()?);
        assert_eq!(GoboWheel_n_MSpeed(1), "GoboWheel1MSpeed".try_into()?);
        assert_eq!(GoboWheel_n_MSpeed(2), "GoboWheel2MSpeed".try_into()?);
        assert_eq!(GoboWheel_n_MSpeed(120), "GoboWheel120MSpeed".try_into()?);
        assert_eq!(IrisMSpeed, "IrisMSpeed".try_into()?);
        assert_eq!(Prism_n_MSpeed(1), "Prism1MSpeed".try_into()?);
        assert_eq!(Prism_n_MSpeed(2), "Prism2MSpeed".try_into()?);
        assert_eq!(Prism_n_MSpeed(120), "Prism120MSpeed".try_into()?);
        assert_eq!(FocusMSpeed, "FocusMSpeed".try_into()?);
        assert_eq!(Frost_n_MSpeed(1), "Frost1MSpeed".try_into()?);
        assert_eq!(Frost_n_MSpeed(2), "Frost2MSpeed".try_into()?);
        assert_eq!(Frost_n_MSpeed(120), "Frost120MSpeed".try_into()?);
        assert_eq!(ZoomMSpeed, "ZoomMSpeed".try_into()?);
        assert_eq!(FrameMSpeed, "FrameMSpeed".try_into()?);
        assert_eq!(GlobalMSpeed, "GlobalMSpeed".try_into()?);
        assert_eq!(ReflectorAdjust, "ReflectorAdjust".try_into()?);
        assert_eq!(FixtureGlobalReset, "FixtureGlobalReset".try_into()?);
        assert_eq!(ShutterReset, "ShutterReset".try_into()?);
        assert_eq!(BeamReset, "BeamReset".try_into()?);
        assert_eq!(ColorMixReset, "ColorMixReset".try_into()?);
        assert_eq!(ColorWheelReset, "ColorWheelReset".try_into()?);
        assert_eq!(FocusReset, "FocusReset".try_into()?);
        assert_eq!(FrameReset, "FrameReset".try_into()?);
        assert_eq!(GoboWheelReset, "GoboWheelReset".try_into()?);
        assert_eq!(IntensityReset, "IntensityReset".try_into()?);
        assert_eq!(IrisReset, "IrisReset".try_into()?);
        assert_eq!(PositionReset, "PositionReset".try_into()?);
        assert_eq!(PanReset, "PanReset".try_into()?);
        assert_eq!(TiltReset, "TiltReset".try_into()?);
        assert_eq!(ZoomReset, "ZoomReset".try_into()?);
        assert_eq!(CtbReset, "CTBReset".try_into()?);
        assert_eq!(CtoReset, "CTOReset".try_into()?);
        assert_eq!(CtcReset, "CTCReset".try_into()?);
        assert_eq!(AnimationSystemReset, "AnimationSystemReset".try_into()?);
        assert_eq!(FixtureCalibrationReset, "FixtureCalibrationReset".try_into()?);
        assert_eq!(Function, "Function".try_into()?);
        assert_eq!(LampControl, "LampControl".try_into()?);
        assert_eq!(DisplayIntensity, "DisplayIntensity".try_into()?);
        assert_eq!(DmxInput, "DMXInput".try_into()?);
        assert_eq!(NoFeature, "NoFeature".try_into()?);
        assert_eq!(Blower_n_(1), "Blower1".try_into()?);
        assert_eq!(Blower_n_(2), "Blower2".try_into()?);
        assert_eq!(Blower_n_(120), "Blower120".try_into()?);
        assert_eq!(Fan_n_(1), "Fan1".try_into()?);
        assert_eq!(Fan_n_(2), "Fan2".try_into()?);
        assert_eq!(Fan_n_(120), "Fan120".try_into()?);
        assert_eq!(Fog_n_(1), "Fog1".try_into()?);
        assert_eq!(Fog_n_(2), "Fog2".try_into()?);
        assert_eq!(Fog_n_(120), "Fog120".try_into()?);
        assert_eq!(Haze_n_(1), "Haze1".try_into()?);
        assert_eq!(Haze_n_(2), "Haze2".try_into()?);
        assert_eq!(Haze_n_(120), "Haze120".try_into()?);
        assert_eq!(LampPowerMode, "LampPowerMode".try_into()?);
        assert_eq!(Fans, "Fans".try_into()?);
        assert_eq!(Blade_n_A(1), "Blade1A".try_into()?);
        assert_eq!(Blade_n_A(2), "Blade2A".try_into()?);
        assert_eq!(Blade_n_A(120), "Blade120A".try_into()?);
        assert_eq!(Blade_n_B(1), "Blade1B".try_into()?);
        assert_eq!(Blade_n_B(2), "Blade2B".try_into()?);
        assert_eq!(Blade_n_B(120), "Blade120B".try_into()?);
        assert_eq!(Blade_n_Rot(1), "Blade1Rot".try_into()?);
        assert_eq!(Blade_n_Rot(2), "Blade2Rot".try_into()?);
        assert_eq!(Blade_n_Rot(120), "Blade120Rot".try_into()?);
        assert_eq!(ShaperRot, "ShaperRot".try_into()?);
        assert_eq!(ShaperMacros, "ShaperMacros".try_into()?);
        assert_eq!(ShaperMacrosSpeed, "ShaperMacrosSpeed".try_into()?);
        assert_eq!(BladeSoft_n_A(1), "BladeSoft1A".try_into()?);
        assert_eq!(BladeSoft_n_A(2), "BladeSoft2A".try_into()?);
        assert_eq!(BladeSoft_n_A(120), "BladeSoft120A".try_into()?);
        assert_eq!(BladeSoft_n_B(1), "BladeSoft1B".try_into()?);
        assert_eq!(BladeSoft_n_B(2), "BladeSoft2B".try_into()?);
        assert_eq!(BladeSoft_n_B(120), "BladeSoft120B".try_into()?);
        assert_eq!(KeyStone_n_A(1), "KeyStone1A".try_into()?);
        assert_eq!(KeyStone_n_A(2), "KeyStone2A".try_into()?);
        assert_eq!(KeyStone_n_A(120), "KeyStone120A".try_into()?);
        assert_eq!(KeyStone_n_B(1), "KeyStone1B".try_into()?);
        assert_eq!(KeyStone_n_B(2), "KeyStone2B".try_into()?);
        assert_eq!(KeyStone_n_B(120), "KeyStone120B".try_into()?);
        assert_eq!(Video, "Video".try_into()?);
        assert_eq!(VideoEffect_n_Type(1), "VideoEffect1Type".try_into()?);
        assert_eq!(VideoEffect_n_Type(2), "VideoEffect2Type".try_into()?);
        assert_eq!(VideoEffect_n_Type(120), "VideoEffect120Type".try_into()?);
        assert_eq!(VideoEffect_n_Parameter_m_(1, 1), "VideoEffect1Parameter1".try_into()?);
        assert_eq!(VideoEffect_n_Parameter_m_(1, 2), "VideoEffect1Parameter2".try_into()?);
        assert_eq!(VideoEffect_n_Parameter_m_(2, 1), "VideoEffect2Parameter1".try_into()?);
        assert_eq!(VideoEffect_n_Parameter_m_(2, 2), "VideoEffect2Parameter2".try_into()?);
        assert_eq!(VideoEffect_n_Parameter_m_(2, 120), "VideoEffect2Parameter120".try_into()?);
        assert_eq!(VideoEffect_n_Parameter_m_(120, 2), "VideoEffect120Parameter2".try_into()?);
        assert_eq!(VideoEffect_n_Parameter_m_(120, 120), "VideoEffect120Parameter120".try_into()?);
        assert_eq!(VideoCamera_n_(1), "VideoCamera1".try_into()?);
        assert_eq!(VideoCamera_n_(2), "VideoCamera2".try_into()?);
        assert_eq!(VideoCamera_n_(120), "VideoCamera120".try_into()?);
        assert_eq!(VideoSoundVolume_n_(1), "VideoSoundVolume1".try_into()?);
        assert_eq!(VideoSoundVolume_n_(2), "VideoSoundVolume2".try_into()?);
        assert_eq!(VideoSoundVolume_n_(120), "VideoSoundVolume120".try_into()?);
        assert_eq!(VideoBlendMode, "VideoBlendMode".try_into()?);
        assert_eq!(InputSource, "InputSource".try_into()?);
        assert_eq!(FieldOfView, "FieldOfView".try_into()?);

        assert!(AttributeName::try_from("something{invalid").is_err());
        assert!(AttributeName::try_from("something௸invalid").is_err());
        Ok(())
    }


    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        use AttributeName::*;
        assert_eq!(UserDefined(Name::new("test")?), "test".try_into()?);

        if let UserDefined(n) = AttributeName::try_from("")? {
            assert_eq!(n, Name::new("")?);
        } else {
            panic!("empty str was not parsed to empty user defined");
        }
        assert_eq!(Dimmer, testdata::to_attr_owned(b"Dimmer").try_into()?);
        assert_eq!(Pan, testdata::to_attr_owned(b"Pan").try_into()?);
        assert_eq!(Tilt, testdata::to_attr_owned(b"Tilt").try_into()?);
        assert_eq!(Gobo_n_(1), testdata::to_attr_owned(b"Gobo1").try_into()?);
        assert_eq!(Gobo_n_SelectSpin(2), testdata::to_attr_owned(b"Gobo2SelectSpin").try_into()?);
        assert_eq!(Gobo_n_SelectShake(120), testdata::to_attr_owned(b"Gobo120SelectShake").try_into()?);
        assert_eq!(Dimmer, testdata::to_attr_owned(b"Dimmer").try_into()?);
        assert_eq!(Dimmer, testdata::to_attr_owned(b"Dimmer").try_into()?);

        assert_eq!(Effects_n_Adjust_m_(1, 1), testdata::to_attr_owned(b"Effects1Adjust1").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(1, 2), testdata::to_attr_owned(b"Effects1Adjust2").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(2, 1), testdata::to_attr_owned(b"Effects2Adjust1").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(2, 2), testdata::to_attr_owned(b"Effects2Adjust2").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(2, 120), testdata::to_attr_owned(b"Effects2Adjust120").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(120, 2), testdata::to_attr_owned(b"Effects120Adjust2").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(120, 120), testdata::to_attr_owned(b"Effects120Adjust120").try_into()?);

        Ok(())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        use AttributeName::*;
        assert_eq!(UserDefined(Name::new("test")?), "test".try_into()?);

        if let UserDefined(n) = AttributeName::try_from("")? {
            assert_eq!(n, Name::new("")?);
        } else {
            panic!("empty str was not parsed to empty user defined");
        }
        assert_eq!(Dimmer, testdata::to_attr_borrowed(b"Dimmer").try_into()?);
        assert_eq!(Pan, testdata::to_attr_borrowed(b"Pan").try_into()?);
        assert_eq!(Tilt, testdata::to_attr_borrowed(b"Tilt").try_into()?);
        assert_eq!(Gobo_n_(1), testdata::to_attr_borrowed(b"Gobo1").try_into()?);
        assert_eq!(Gobo_n_SelectSpin(2), testdata::to_attr_borrowed(b"Gobo2SelectSpin").try_into()?);
        assert_eq!(Gobo_n_SelectShake(120), testdata::to_attr_borrowed(b"Gobo120SelectShake").try_into()?);
        assert_eq!(Dimmer, testdata::to_attr_borrowed(b"Dimmer").try_into()?);
        assert_eq!(Dimmer, testdata::to_attr_borrowed(b"Dimmer").try_into()?);

        assert_eq!(Effects_n_Adjust_m_(1, 1), testdata::to_attr_borrowed(b"Effects1Adjust1").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(1, 2), testdata::to_attr_borrowed(b"Effects1Adjust2").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(2, 1), testdata::to_attr_borrowed(b"Effects2Adjust1").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(2, 2), testdata::to_attr_borrowed(b"Effects2Adjust2").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(2, 120), testdata::to_attr_borrowed(b"Effects2Adjust120").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(120, 2), testdata::to_attr_borrowed(b"Effects120Adjust2").try_into()?);
        assert_eq!(Effects_n_Adjust_m_(120, 120), testdata::to_attr_borrowed(b"Effects120Adjust120").try_into()?);

        Ok(())
    }

    #[test]
    fn test_new_from_str() -> Result<(), GdtfError> {
        assert_eq!(AttributeName::Effects_n_(2), AttributeName::new_from_str("Effects2")?);
        assert_eq!(AttributeName::Control_n_(3), AttributeName::new_from_str("Control3")?);
        assert_eq!(AttributeName::Dimmer, AttributeName::new_from_str("Dimmer")?);
        assert_eq!(AttributeName::UserDefined(Name::new("")?), AttributeName::new_from_str("")?);
        assert_eq!(AttributeName::UserDefined(Name::new("Something weird")?), AttributeName::new_from_str("Something weird")?);
        assert!(Name::new("asdf{").is_err());
        Ok(())
    }
}
