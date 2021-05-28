//! AttributeName is a preferred Name for Attributes in GDTF Format.
#![allow(non_camel_case_types)]

use crate::utils::units::name::Name;

#[derive(Debug)]
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
    XYZ_X,
    ///Defines a fixture’s y-coordinate within an XYZ coordinate system.
    XYZ_Y,
    ///Defines a fixture‘s z-coordinate within an XYZ coordinate system.
    XYZ_Z,
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
    Scale_XYZ,
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
    ColorAdd_RY,
    ///Controls the intensity of the fixture’s lime emitters for direct additive color mixing.
    ColorAdd_GY,
    ///Controls the intensity of the fixture’s blue-green emitters for direct additive color mixing.
    ColorAdd_GC,
    ///Controls the intensity of the fixture’s light-blue emitters for direct additive color mixing.
    ColorAdd_BC,
    ///Controls the intensity of the fixture’s purple emitters for direct additive color mixing.
    ColorAdd_BM,
    ///Controls the intensity of the fixture’s pink emitters for direct additive color mixing.
    ColorAdd_RM,
    ///Controls the intensity of the fixture’s white emitters for direct additive color mixing.
    ColorAdd_W,
    ///Controls the intensity of the fixture’s warm white emitters for direct additive color mixing.
    ColorAdd_WW,
    ///Controls the intensity of the fixture’s cool white emitters for direct additive color mixing.
    ColorAdd_CW,
    ///Controls the intensity of the fixture’s UV emitters for direct additive color mixing.
    ColorAdd_UV,
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
    CTO,
    ///Controls the fixture’s “Correct to color” wheel or mixing system.
    CTC,
    ///Controls the fixture’s “Correct to blue” wheel or mixing system.
    CTB,
    ///Controls the fixture’s “Correct green to magenta” wheel or mixing system.
    Tint,
    ///Controls the fixture’s color attribute regarding the hue.
    HSB_Hue,
    ///Controls the fixture’s color attribute regarding the saturation.
    HSB_Saturation,
    ///Controls the fixture’s color attribute regarding the brightness.
    HSB_Brightness,
    ///Controls the fixture’s color attribute regarding the quality.
    HSB_Quality,
    ///Controls the fixture’s CIE 1931 color attribute regarding the chromaticity x.
    CIE_X,
    ///Controls the fixture’s CIE 1931 color attribute regarding the chromaticity y.
    CIE_Y,
    ///Controls the fixture’s CIE 1931 color attribute regarding the brightness (Y).
    CIE_Brightness,
    ///Controls the fixture’s red attribute for indirect RGB color mixing.
    ColorRGB_Red,
    ///Controls the fixture’s green attribute for indirect RGB color mixing.
    ColorRGB_Green,
    ///Controls the fixture’s blue attribute for indirect RGB color mixing.
    ColorRGB_Blue,
    ///Controls the fixture’s cyan attribute for indirect CMY color mixing.
    ColorRGB_Cyan,
    ///Controls the fixture’s magenta attribute for indirect CMY color mixing.
    ColorRGB_Magenta,
    ///Controls the fixture’s yellow attribute for indirect CMY color mixing.
    ColorRGB_Yellow,
    ///Controls the fixture’s quality attribute for indirect color mixing.
    ColorRGB_Quality,
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
    LEDFrequency,
    ///Changes zones of LEDs.
    LEDZoneMode,
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
    CRIMode,
    ///Custom color related functions (save, recall).
    CustomColor,
    ///Settings for UV stability color behavior.
    UVStability,
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
    CTBReset,
    ///Resets the fixture’s CTO.
    CTOReset,
    ///Resets the fixture’s CTC.
    CTCReset,
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
    DMXInput,
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

impl PartialEq for AttributeName {
    fn eq(&self, other: &Self) -> bool {
        use AttributeName::*;
        match self {
            UserDefined(n1) => if let UserDefined(n2) = other { n1 == n2 } else { false },
            Dimmer => if let Dimmer = other { true } else { false },
            Pan => if let Pan = other { true } else { false },
            Tilt => if let Tilt = other { true } else { false },
            PanRotate => if let PanRotate = other { true } else { false },
            TiltRotate => if let TiltRotate = other { true } else { false },
            PositionEffect => if let PositionEffect = other { true } else { false },
            PositionEffectRate => if let PositionEffectRate = other { true } else { false },
            PositionEffectFade => if let PositionEffectFade = other { true } else { false },
            XYZ_X => if let XYZ_X = other { true } else { false },
            XYZ_Y => if let XYZ_Y = other { true } else { false },
            XYZ_Z => if let XYZ_Z = other { true } else { false },
            Rot_X => if let Rot_X = other { true } else { false },
            Rot_Y => if let Rot_Y = other { true } else { false },
            Rot_Z => if let Rot_Z = other { true } else { false },
            Scale_X => if let Scale_X = other { true } else { false },
            Scale_Y => if let Scale_Y = other { true } else { false },
            Scale_Z => if let Scale_Z = other { true } else { false },
            Scale_XYZ => if let Scale_XYZ = other { true } else { false },
            Gobo_n_(n1) => if let Gobo_n_(n2) = other { n1 == n2 } else { false },
            Gobo_n_SelectSpin(n1) => if let Gobo_n_SelectSpin(n2) = other { n1 == n2 } else { false },
            Gobo_n_SelectShake(n1) => if let Gobo_n_SelectShake(n2) = other { n1 == n2 } else { false },
            Gobo_n_SelectEffects(n1) => if let Gobo_n_SelectEffects(n2) = other { n1 == n2 } else { false },
            Gobo_n_WheelIndex(n1) => if let Gobo_n_WheelIndex(n2) = other { n1 == n2 } else { false },
            Gobo_n_WheelSpin(n1) => if let Gobo_n_WheelSpin(n2) = other { n1 == n2 } else { false },
            Gobo_n_WheelShake(n1) => if let Gobo_n_WheelShake(n2) = other { n1 == n2 } else { false },
            Gobo_n_WheelRandom(n1) => if let Gobo_n_WheelRandom(n2) = other { n1 == n2 } else { false },
            Gobo_n_WheelAudio(n1) => if let Gobo_n_WheelAudio(n2) = other { n1 == n2 } else { false },
            Gobo_n_Pos(n1) => if let Gobo_n_Pos(n2) = other { n1 == n2 } else { false },
            Gobo_n_PosRotate(n1) => if let Gobo_n_PosRotate(n2) = other { n1 == n2 } else { false },
            Gobo_n_PosShake(n1) => if let Gobo_n_PosShake(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_(n1) => if let AnimationWheel_n_(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_Audio(n1) => if let AnimationWheel_n_Audio(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_Macro(n1) => if let AnimationWheel_n_Macro(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_Random(n1) => if let AnimationWheel_n_Random(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_SelectEffects(n1) => if let AnimationWheel_n_SelectEffects(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_SelectShake(n1) => if let AnimationWheel_n_SelectShake(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_SelectSpin(n1) => if let AnimationWheel_n_SelectSpin(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_Pos(n1) => if let AnimationWheel_n_Pos(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_PosRotate(n1) => if let AnimationWheel_n_PosRotate(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_PosShake(n1) => if let AnimationWheel_n_PosShake(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_(n1) => if let AnimationSystem_n_(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_Ramp(n1) => if let AnimationSystem_n_Ramp(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_Shake(n1) => if let AnimationSystem_n_Shake(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_Audio(n1) => if let AnimationSystem_n_Audio(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_Random(n1) => if let AnimationSystem_n_Random(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_Pos(n1) => if let AnimationSystem_n_Pos(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_PosRotate(n1) => if let AnimationSystem_n_PosRotate(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_PosShake(n1) => if let AnimationSystem_n_PosShake(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_PosRandom(n1) => if let AnimationSystem_n_PosRandom(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_PosAudio(n1) => if let AnimationSystem_n_PosAudio(n2) = other { n1 == n2 } else { false },
            AnimationSystem_n_Macro(n1) => if let AnimationSystem_n_Macro(n2) = other { n1 == n2 } else { false },
            MediaFolder_n_(n1) => if let MediaFolder_n_(n2) = other { n1 == n2 } else { false },
            MediaContent_n_(n1) => if let MediaContent_n_(n2) = other { n1 == n2 } else { false },
            ModelFolder_n_(n1) => if let ModelFolder_n_(n2) = other { n1 == n2 } else { false },
            ModelContent_n_(n1) => if let ModelContent_n_(n2) = other { n1 == n2 } else { false },
            PlayMode => if let PlayMode = other { true } else { false },
            PlayBegin => if let PlayBegin = other { true } else { false },
            PlayEnd => if let PlayEnd = other { true } else { false },
            PlaySpeed => if let PlaySpeed = other { true } else { false },
            ColorEffects_n_(n1) => if let ColorEffects_n_(n2) = other { n1 == n2 } else { false },
            Color_n_(n1) => if let Color_n_(n2) = other { n1 == n2 } else { false },
            Color_n_WheelIndex(n1) => if let Color_n_WheelIndex(n2) = other { n1 == n2 } else { false },
            Color_n_WheelSpin(n1) => if let Color_n_WheelSpin(n2) = other { n1 == n2 } else { false },
            Color_n_WheelRandom(n1) => if let Color_n_WheelRandom(n2) = other { n1 == n2 } else { false },
            Color_n_WheelAudio(n1) => if let Color_n_WheelAudio(n2) = other { n1 == n2 } else { false },
            ColorAdd_R => if let ColorAdd_R = other { true } else { false },
            ColorAdd_G => if let ColorAdd_G = other { true } else { false },
            ColorAdd_B => if let ColorAdd_B = other { true } else { false },
            ColorAdd_C => if let ColorAdd_C = other { true } else { false },
            ColorAdd_M => if let ColorAdd_M = other { true } else { false },
            ColorAdd_Y => if let ColorAdd_Y = other { true } else { false },
            ColorAdd_RY => if let ColorAdd_RY = other { true } else { false },
            ColorAdd_GY => if let ColorAdd_GY = other { true } else { false },
            ColorAdd_GC => if let ColorAdd_GC = other { true } else { false },
            ColorAdd_BC => if let ColorAdd_BC = other { true } else { false },
            ColorAdd_BM => if let ColorAdd_BM = other { true } else { false },
            ColorAdd_RM => if let ColorAdd_RM = other { true } else { false },
            ColorAdd_W => if let ColorAdd_W = other { true } else { false },
            ColorAdd_WW => if let ColorAdd_WW = other { true } else { false },
            ColorAdd_CW => if let ColorAdd_CW = other { true } else { false },
            ColorAdd_UV => if let ColorAdd_UV = other { true } else { false },
            ColorSub_R => if let ColorSub_R = other { true } else { false },
            ColorSub_G => if let ColorSub_G = other { true } else { false },
            ColorSub_B => if let ColorSub_B = other { true } else { false },
            ColorSub_C => if let ColorSub_C = other { true } else { false },
            ColorSub_M => if let ColorSub_M = other { true } else { false },
            ColorSub_Y => if let ColorSub_Y = other { true } else { false },
            ColorMacro_n_(n1) => if let ColorMacro_n_(n2) = other { n1 == n2 } else { false },
            ColorMacro_n_Rate(n1) => if let ColorMacro_n_Rate(n2) = other { n1 == n2 } else { false },
            CTO => if let CTO = other { true } else { false },
            CTC => if let CTC = other { true } else { false },
            CTB => if let CTB = other { true } else { false },
            Tint => if let Tint = other { true } else { false },
            HSB_Hue => if let HSB_Hue = other { true } else { false },
            HSB_Saturation => if let HSB_Saturation = other { true } else { false },
            HSB_Brightness => if let HSB_Brightness = other { true } else { false },
            HSB_Quality => if let HSB_Quality = other { true } else { false },
            CIE_X => if let CIE_X = other { true } else { false },
            CIE_Y => if let CIE_Y = other { true } else { false },
            CIE_Brightness => if let CIE_Brightness = other { true } else { false },
            ColorRGB_Red => if let ColorRGB_Red = other { true } else { false },
            ColorRGB_Green => if let ColorRGB_Green = other { true } else { false },
            ColorRGB_Blue => if let ColorRGB_Blue = other { true } else { false },
            ColorRGB_Cyan => if let ColorRGB_Cyan = other { true } else { false },
            ColorRGB_Magenta => if let ColorRGB_Magenta = other { true } else { false },
            ColorRGB_Yellow => if let ColorRGB_Yellow = other { true } else { false },
            ColorRGB_Quality => if let ColorRGB_Quality = other { true } else { false },
            VideoBoost_R => if let VideoBoost_R = other { true } else { false },
            VideoBoost_G => if let VideoBoost_G = other { true } else { false },
            VideoBoost_B => if let VideoBoost_B = other { true } else { false },
            VideoHueShift => if let VideoHueShift = other { true } else { false },
            VideoSaturation => if let VideoSaturation = other { true } else { false },
            VideoBrightness => if let VideoBrightness = other { true } else { false },
            VideoContrast => if let VideoContrast = other { true } else { false },
            VideoKeyColor_R => if let VideoKeyColor_R = other { true } else { false },
            VideoKeyColor_G => if let VideoKeyColor_G = other { true } else { false },
            VideoKeyColor_B => if let VideoKeyColor_B = other { true } else { false },
            VideoKeyIntensity => if let VideoKeyIntensity = other { true } else { false },
            VideoKeyTolerance => if let VideoKeyTolerance = other { true } else { false },
            StrobeDuration => if let StrobeDuration = other { true } else { false },
            StrobeRate => if let StrobeRate = other { true } else { false },
            Shutter_n_(n1) => if let Shutter_n_(n2) = other { n1 == n2 } else { false },
            Shutter_n_Strobe(n1) => if let Shutter_n_Strobe(n2) = other { n1 == n2 } else { false },
            Shutter_n_StrobePulse(n1) => if let Shutter_n_StrobePulse(n2) = other { n1 == n2 } else { false },
            Shutter_n_StrobePulseClose(n1) => if let Shutter_n_StrobePulseClose(n2) = other { n1 == n2 } else { false },
            Shutter_n_StrobePulseOpen(n1) => if let Shutter_n_StrobePulseOpen(n2) = other { n1 == n2 } else { false },
            Shutter_n_StrobeRandom(n1) => if let Shutter_n_StrobeRandom(n2) = other { n1 == n2 } else { false },
            Shutter_n_StrobeRandomPulse(n1) => if let Shutter_n_StrobeRandomPulse(n2) = other { n1 == n2 } else { false },
            Shutter_n_StrobeRandomPulseClose(n1) => if let Shutter_n_StrobeRandomPulseClose(n2) = other { n1 == n2 } else { false },
            Shutter_n_StrobeRandomPulseOpen(n1) => if let Shutter_n_StrobeRandomPulseOpen(n2) = other { n1 == n2 } else { false },
            Shutter_n_StrobeEffect(n1) => if let Shutter_n_StrobeEffect(n2) = other { n1 == n2 } else { false },
            Iris => if let Iris = other { true } else { false },
            IrisStrobe => if let IrisStrobe = other { true } else { false },
            IrisStrobeRandom => if let IrisStrobeRandom = other { true } else { false },
            IrisPulseClose => if let IrisPulseClose = other { true } else { false },
            IrisPulseOpen => if let IrisPulseOpen = other { true } else { false },
            IrisRandomPulseClose => if let IrisRandomPulseClose = other { true } else { false },
            IrisRandomPulseOpen => if let IrisRandomPulseOpen = other { true } else { false },
            Frost_n_(n1) => if let Frost_n_(n2) = other { n1 == n2 } else { false },
            Frost_n_PulseOpen(n1) => if let Frost_n_PulseOpen(n2) = other { n1 == n2 } else { false },
            Frost_n_PulseClose(n1) => if let Frost_n_PulseClose(n2) = other { n1 == n2 } else { false },
            Frost_n_Ramp(n1) => if let Frost_n_Ramp(n2) = other { n1 == n2 } else { false },
            Prism_n_(n1) => if let Prism_n_(n2) = other { n1 == n2 } else { false },
            Prism_n_SelectSpin(n1) => if let Prism_n_SelectSpin(n2) = other { n1 == n2 } else { false },
            Prism_n_Macro(n1) => if let Prism_n_Macro(n2) = other { n1 == n2 } else { false },
            Prism_n_Pos(n1) => if let Prism_n_Pos(n2) = other { n1 == n2 } else { false },
            Prism_n_PosRotate(n1) => if let Prism_n_PosRotate(n2) = other { n1 == n2 } else { false },
            Effects_n_(n1) => if let Effects_n_(n2) = other { n1 == n2 } else { false },
            Effects_n_Rate(n1) => if let Effects_n_Rate(n2) = other { n1 == n2 } else { false },
            Effects_n_Fade(n1) => if let Effects_n_Fade(n2) = other { n1 == n2 } else { false },
            Effects_n_Adjust_m_(n1, m1) => if let Effects_n_Adjust_m_(n2, m2) = other { n1 == n2 && m1 == m2 } else { false },
            Effects_n_Pos(n1) => if let Effects_n_Pos(n2) = other { n1 == n2 } else { false },
            Effects_n_PosRotate(n1) => if let Effects_n_PosRotate(n2) = other { n1 == n2 } else { false },
            EffectsSync => if let EffectsSync = other { true } else { false },
            BeamShaper => if let BeamShaper = other { true } else { false },
            BeamShaperMacro => if let BeamShaperMacro = other { true } else { false },
            BeamShaperPos => if let BeamShaperPos = other { true } else { false },
            BeamShaperPosRotate => if let BeamShaperPosRotate = other { true } else { false },
            Zoom => if let Zoom = other { true } else { false },
            ZoomModeSpot => if let ZoomModeSpot = other { true } else { false },
            ZoomModeBeam => if let ZoomModeBeam = other { true } else { false },
            Focus_n_(n1) => if let Focus_n_(n2) = other { n1 == n2 } else { false },
            Focus_n_Adjust(n1) => if let Focus_n_Adjust(n2) = other { n1 == n2 } else { false },
            Focus_n_Distance(n1) => if let Focus_n_Distance(n2) = other { n1 == n2 } else { false },
            Control_n_(n1) => if let Control_n_(n2) = other { n1 == n2 } else { false },
            DimmerMode => if let DimmerMode = other { true } else { false },
            DimmerCurve => if let DimmerCurve = other { true } else { false },
            BlackoutMode => if let BlackoutMode = other { true } else { false },
            LEDFrequency => if let LEDFrequency = other { true } else { false },
            LEDZoneMode => if let LEDZoneMode = other { true } else { false },
            PixelMode => if let PixelMode = other { true } else { false },
            PanMode => if let PanMode = other { true } else { false },
            TiltMode => if let TiltMode = other { true } else { false },
            PanTiltMode => if let PanTiltMode = other { true } else { false },
            PositionModes => if let PositionModes = other { true } else { false },
            Gobo_n_WheelMode(n1) => if let Gobo_n_WheelMode(n2) = other { n1 == n2 } else { false },
            AnimationWheel_n_Mode(n1) => if let AnimationWheel_n_Mode(n2) = other { n1 == n2 } else { false },
            AnimationWheelShortcutMode => if let AnimationWheelShortcutMode = other { true } else { false },
            Color_n_Mode(n1) => if let Color_n_Mode(n2) = other { n1 == n2 } else { false },
            ColorWheelShortcutMode => if let ColorWheelShortcutMode = other { true } else { false },
            CyanMode => if let CyanMode = other { true } else { false },
            MagentaMode => if let MagentaMode = other { true } else { false },
            YellowMode => if let YellowMode = other { true } else { false },
            ColorMixMode => if let ColorMixMode = other { true } else { false },
            ChromaticMode => if let ChromaticMode = other { true } else { false },
            ColorCalibrationMode => if let ColorCalibrationMode = other { true } else { false },
            ColorConsistency => if let ColorConsistency = other { true } else { false },
            ColorControl => if let ColorControl = other { true } else { false },
            ColorModelMode => if let ColorModelMode = other { true } else { false },
            ColorSettingsReset => if let ColorSettingsReset = other { true } else { false },
            ColorUniformity => if let ColorUniformity = other { true } else { false },
            CRIMode => if let CRIMode = other { true } else { false },
            CustomColor => if let CustomColor = other { true } else { false },
            UVStability => if let UVStability = other { true } else { false },
            WavelengthCorrection => if let WavelengthCorrection = other { true } else { false },
            WhiteCount => if let WhiteCount = other { true } else { false },
            StrobeMode => if let StrobeMode = other { true } else { false },
            ZoomMode => if let ZoomMode = other { true } else { false },
            FocusMode => if let FocusMode = other { true } else { false },
            IrisMode => if let IrisMode = other { true } else { false },
            Fan_n_Mode(n1) => if let Fan_n_Mode(n2) = other { n1 == n2 } else { false },
            FollowSpotMode => if let FollowSpotMode = other { true } else { false },
            BeamEffectIndexRotateMode => if let BeamEffectIndexRotateMode = other { true } else { false },
            IntensityMSpeed => if let IntensityMSpeed = other { true } else { false },
            PositionMSpeed => if let PositionMSpeed = other { true } else { false },
            ColorMixMSpeed => if let ColorMixMSpeed = other { true } else { false },
            ColorWheelSelectMSpeed => if let ColorWheelSelectMSpeed = other { true } else { false },
            GoboWheel_n_MSpeed(n1) => if let GoboWheel_n_MSpeed(n2) = other { n1 == n2 } else { false },
            IrisMSpeed => if let IrisMSpeed = other { true } else { false },
            Prism_n_MSpeed(n1) => if let Prism_n_MSpeed(n2) = other { n1 == n2 } else { false },
            FocusMSpeed => if let FocusMSpeed = other { true } else { false },
            Frost_n_MSpeed(n1) => if let Frost_n_MSpeed(n2) = other { n1 == n2 } else { false },
            ZoomMSpeed => if let ZoomMSpeed = other { true } else { false },
            FrameMSpeed => if let FrameMSpeed = other { true } else { false },
            GlobalMSpeed => if let GlobalMSpeed = other { true } else { false },
            ReflectorAdjust => if let ReflectorAdjust = other { true } else { false },
            FixtureGlobalReset => if let FixtureGlobalReset = other { true } else { false },
            ShutterReset => if let ShutterReset = other { true } else { false },
            BeamReset => if let BeamReset = other { true } else { false },
            ColorMixReset => if let ColorMixReset = other { true } else { false },
            ColorWheelReset => if let ColorWheelReset = other { true } else { false },
            FocusReset => if let FocusReset = other { true } else { false },
            FrameReset => if let FrameReset = other { true } else { false },
            GoboWheelReset => if let GoboWheelReset = other { true } else { false },
            IntensityReset => if let IntensityReset = other { true } else { false },
            IrisReset => if let IrisReset = other { true } else { false },
            PositionReset => if let PositionReset = other { true } else { false },
            PanReset => if let PanReset = other { true } else { false },
            TiltReset => if let TiltReset = other { true } else { false },
            ZoomReset => if let ZoomReset = other { true } else { false },
            CTBReset => if let CTBReset = other { true } else { false },
            CTOReset => if let CTOReset = other { true } else { false },
            CTCReset => if let CTCReset = other { true } else { false },
            AnimationSystemReset => if let AnimationSystemReset = other { true } else { false },
            FixtureCalibrationReset => if let FixtureCalibrationReset = other { true } else { false },
            Function => if let Function = other { true } else { false },
            LampControl => if let LampControl = other { true } else { false },
            DisplayIntensity => if let DisplayIntensity = other { true } else { false },
            DMXInput => if let DMXInput = other { true } else { false },
            NoFeature => if let NoFeature = other { true } else { false },
            Blower_n_(n1) => if let Blower_n_(n2) = other { n1 == n2 } else { false },
            Fan_n_(n1) => if let Fan_n_(n2) = other { n1 == n2 } else { false },
            Fog_n_(n1) => if let Fog_n_(n2) = other { n1 == n2 } else { false },
            Haze_n_(n1) => if let Haze_n_(n2) = other { n1 == n2 } else { false },
            LampPowerMode => if let LampPowerMode = other { true } else { false },
            Fans => if let Fans = other { true } else { false },
            Blade_n_A(n1) => if let Blade_n_A(n2) = other { n1 == n2 } else { false },
            Blade_n_B(n1) => if let Blade_n_B(n2) = other { n1 == n2 } else { false },
            Blade_n_Rot(n1) => if let Blade_n_Rot(n2) = other { n1 == n2 } else { false },
            ShaperRot => if let ShaperRot = other { true } else { false },
            ShaperMacros => if let ShaperMacros = other { true } else { false },
            ShaperMacrosSpeed => if let ShaperMacrosSpeed = other { true } else { false },
            BladeSoft_n_A(n1) => if let BladeSoft_n_A(n2) = other { n1 == n2 } else { false },
            BladeSoft_n_B(n1) => if let BladeSoft_n_B(n2) = other { n1 == n2 } else { false },
            KeyStone_n_A(n1) => if let KeyStone_n_A(n2) = other { n1 == n2 } else { false },
            KeyStone_n_B(n1) => if let KeyStone_n_B(n2) = other { n1 == n2 } else { false },
            Video => if let Video = other { true } else { false },
            VideoEffect_n_Type(n1) => if let VideoEffect_n_Type(n2) = other { n1 == n2 } else { false },
            VideoEffect_n_Parameter_m_(n1, m1) => if let VideoEffect_n_Parameter_m_(n2, m2) = other { n1 == n2 && m1 == m2 } else { false },
            VideoCamera_n_(n1) => if let VideoCamera_n_(n2) = other { n1 == n2 } else { false },
            VideoSoundVolume_n_(n1) => if let VideoSoundVolume_n_(n2) = other { n1 == n2 } else { false },
            VideoBlendMode => if let VideoBlendMode = other { true } else { false },
            InputSource => if let InputSource = other { true } else { false },
            FieldOfView => if let FieldOfView = other { true } else { false },
        }
    }
}

///Parses a string in gdtf-xml definition to AttributeName. In case of error it will return UserDefined with Empty Name
impl From<&str> for AttributeName {
    fn from(value: &str) -> Self {
        use AttributeName::*;
        if value == "" {
            return UserDefined(Name::new_unchecked(value));
        }
        let _chars: Vec<char> = value.chars().collect();


        return UserDefined(Name::new_unchecked(value));
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::name::Name;

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        use AttributeName::*;
        assert_eq!(UserDefined(Name::new("test")?), UserDefined(Name::new("test")?));
        assert_ne!(UserDefined(Name::new("")?), UserDefined(Name::new("")?));
        assert_eq!(Dimmer, Dimmer);
        assert_eq!(Pan, Pan);
        assert_eq!(Tilt, Tilt);
        assert_eq!(PanRotate, PanRotate);
        assert_eq!(TiltRotate, TiltRotate);
        assert_eq!(PositionEffect, PositionEffect);
        assert_eq!(PositionEffectRate, PositionEffectRate);
        assert_eq!(PositionEffectFade, PositionEffectFade);
        assert_eq!(XYZ_X, XYZ_X);
        assert_eq!(XYZ_Y, XYZ_Y);
        assert_eq!(XYZ_Z, XYZ_Z);
        assert_eq!(Rot_X, Rot_X);
        assert_eq!(Rot_Y, Rot_Y);
        assert_eq!(Rot_Z, Rot_Z);
        assert_eq!(Scale_X, Scale_X);
        assert_eq!(Scale_Y, Scale_Y);
        assert_eq!(Scale_Z, Scale_Z);
        assert_eq!(Scale_XYZ, Scale_XYZ);
        assert_eq!(Gobo_n_(1), Gobo_n_(1));
        assert_eq!(Gobo_n_(2), Gobo_n_(2));
        assert_ne!(Gobo_n_(1), Gobo_n_(2));
        assert_eq!(Gobo_n_SelectSpin(1), Gobo_n_SelectSpin(1));
        assert_eq!(Gobo_n_SelectSpin(2), Gobo_n_SelectSpin(2));
        assert_ne!(Gobo_n_SelectSpin(1), Gobo_n_SelectSpin(2));
        assert_eq!(Gobo_n_SelectShake(1), Gobo_n_SelectShake(1));
        assert_eq!(Gobo_n_SelectShake(2), Gobo_n_SelectShake(2));
        assert_ne!(Gobo_n_SelectShake(1), Gobo_n_SelectShake(2));
        assert_eq!(Gobo_n_SelectEffects(1), Gobo_n_SelectEffects(1));
        assert_eq!(Gobo_n_SelectEffects(2), Gobo_n_SelectEffects(2));
        assert_ne!(Gobo_n_SelectEffects(1), Gobo_n_SelectEffects(2));
        assert_eq!(Gobo_n_WheelIndex(1), Gobo_n_WheelIndex(1));
        assert_eq!(Gobo_n_WheelIndex(2), Gobo_n_WheelIndex(2));
        assert_ne!(Gobo_n_WheelIndex(1), Gobo_n_WheelIndex(2));
        assert_eq!(Gobo_n_WheelSpin(1), Gobo_n_WheelSpin(1));
        assert_eq!(Gobo_n_WheelSpin(2), Gobo_n_WheelSpin(2));
        assert_ne!(Gobo_n_WheelSpin(1), Gobo_n_WheelSpin(2));
        assert_eq!(Gobo_n_WheelShake(1), Gobo_n_WheelShake(1));
        assert_eq!(Gobo_n_WheelShake(2), Gobo_n_WheelShake(2));
        assert_ne!(Gobo_n_WheelShake(1), Gobo_n_WheelShake(2));
        assert_eq!(Gobo_n_WheelRandom(1), Gobo_n_WheelRandom(1));
        assert_eq!(Gobo_n_WheelRandom(2), Gobo_n_WheelRandom(2));
        assert_ne!(Gobo_n_WheelRandom(1), Gobo_n_WheelRandom(2));
        assert_eq!(Gobo_n_WheelAudio(1), Gobo_n_WheelAudio(1));
        assert_eq!(Gobo_n_WheelAudio(2), Gobo_n_WheelAudio(2));
        assert_ne!(Gobo_n_WheelAudio(1), Gobo_n_WheelAudio(2));
        assert_eq!(Gobo_n_Pos(1), Gobo_n_Pos(1));
        assert_eq!(Gobo_n_Pos(2), Gobo_n_Pos(2));
        assert_ne!(Gobo_n_Pos(1), Gobo_n_Pos(2));
        assert_eq!(Gobo_n_PosRotate(1), Gobo_n_PosRotate(1));
        assert_eq!(Gobo_n_PosRotate(2), Gobo_n_PosRotate(2));
        assert_ne!(Gobo_n_PosRotate(1), Gobo_n_PosRotate(2));
        assert_eq!(Gobo_n_PosShake(1), Gobo_n_PosShake(1));
        assert_eq!(Gobo_n_PosShake(2), Gobo_n_PosShake(2));
        assert_ne!(Gobo_n_PosShake(1), Gobo_n_PosShake(2));
        assert_eq!(AnimationWheel_n_(1), AnimationWheel_n_(1));
        assert_eq!(AnimationWheel_n_(2), AnimationWheel_n_(2));
        assert_ne!(AnimationWheel_n_(1), AnimationWheel_n_(2));
        assert_eq!(AnimationWheel_n_Audio(1), AnimationWheel_n_Audio(1));
        assert_eq!(AnimationWheel_n_Audio(2), AnimationWheel_n_Audio(2));
        assert_ne!(AnimationWheel_n_Audio(1), AnimationWheel_n_Audio(2));
        assert_eq!(AnimationWheel_n_Macro(1), AnimationWheel_n_Macro(1));
        assert_eq!(AnimationWheel_n_Macro(2), AnimationWheel_n_Macro(2));
        assert_ne!(AnimationWheel_n_Macro(1), AnimationWheel_n_Macro(2));
        assert_eq!(AnimationWheel_n_Random(1), AnimationWheel_n_Random(1));
        assert_eq!(AnimationWheel_n_Random(2), AnimationWheel_n_Random(2));
        assert_ne!(AnimationWheel_n_Random(1), AnimationWheel_n_Random(2));
        assert_eq!(AnimationWheel_n_SelectEffects(1), AnimationWheel_n_SelectEffects(1));
        assert_eq!(AnimationWheel_n_SelectEffects(2), AnimationWheel_n_SelectEffects(2));
        assert_ne!(AnimationWheel_n_SelectEffects(1), AnimationWheel_n_SelectEffects(2));
        assert_eq!(AnimationWheel_n_SelectShake(1), AnimationWheel_n_SelectShake(1));
        assert_eq!(AnimationWheel_n_SelectShake(2), AnimationWheel_n_SelectShake(2));
        assert_ne!(AnimationWheel_n_SelectShake(1), AnimationWheel_n_SelectShake(2));
        assert_eq!(AnimationWheel_n_SelectSpin(1), AnimationWheel_n_SelectSpin(1));
        assert_eq!(AnimationWheel_n_SelectSpin(2), AnimationWheel_n_SelectSpin(2));
        assert_ne!(AnimationWheel_n_SelectSpin(1), AnimationWheel_n_SelectSpin(2));
        assert_eq!(AnimationWheel_n_Pos(1), AnimationWheel_n_Pos(1));
        assert_eq!(AnimationWheel_n_Pos(2), AnimationWheel_n_Pos(2));
        assert_ne!(AnimationWheel_n_Pos(1), AnimationWheel_n_Pos(2));
        assert_eq!(AnimationWheel_n_PosRotate(1), AnimationWheel_n_PosRotate(1));
        assert_eq!(AnimationWheel_n_PosRotate(2), AnimationWheel_n_PosRotate(2));
        assert_ne!(AnimationWheel_n_PosRotate(1), AnimationWheel_n_PosRotate(2));
        assert_eq!(AnimationWheel_n_PosShake(1), AnimationWheel_n_PosShake(1));
        assert_eq!(AnimationWheel_n_PosShake(2), AnimationWheel_n_PosShake(2));
        assert_ne!(AnimationWheel_n_PosShake(1), AnimationWheel_n_PosShake(2));
        assert_eq!(AnimationSystem_n_(1), AnimationSystem_n_(1));
        assert_eq!(AnimationSystem_n_(2), AnimationSystem_n_(2));
        assert_ne!(AnimationSystem_n_(1), AnimationSystem_n_(2));
        assert_eq!(AnimationSystem_n_Ramp(1), AnimationSystem_n_Ramp(1));
        assert_eq!(AnimationSystem_n_Ramp(2), AnimationSystem_n_Ramp(2));
        assert_ne!(AnimationSystem_n_Ramp(1), AnimationSystem_n_Ramp(2));
        assert_eq!(AnimationSystem_n_Shake(1), AnimationSystem_n_Shake(1));
        assert_eq!(AnimationSystem_n_Shake(2), AnimationSystem_n_Shake(2));
        assert_ne!(AnimationSystem_n_Shake(1), AnimationSystem_n_Shake(2));
        assert_eq!(AnimationSystem_n_Audio(1), AnimationSystem_n_Audio(1));
        assert_eq!(AnimationSystem_n_Audio(2), AnimationSystem_n_Audio(2));
        assert_ne!(AnimationSystem_n_Audio(1), AnimationSystem_n_Audio(2));
        assert_eq!(AnimationSystem_n_Random(1), AnimationSystem_n_Random(1));
        assert_eq!(AnimationSystem_n_Random(2), AnimationSystem_n_Random(2));
        assert_ne!(AnimationSystem_n_Random(1), AnimationSystem_n_Random(2));
        assert_eq!(AnimationSystem_n_Pos(1), AnimationSystem_n_Pos(1));
        assert_eq!(AnimationSystem_n_Pos(2), AnimationSystem_n_Pos(2));
        assert_ne!(AnimationSystem_n_Pos(1), AnimationSystem_n_Pos(2));
        assert_eq!(AnimationSystem_n_PosRotate(1), AnimationSystem_n_PosRotate(1));
        assert_eq!(AnimationSystem_n_PosRotate(2), AnimationSystem_n_PosRotate(2));
        assert_ne!(AnimationSystem_n_PosRotate(1), AnimationSystem_n_PosRotate(2));
        assert_eq!(AnimationSystem_n_PosShake(1), AnimationSystem_n_PosShake(1));
        assert_eq!(AnimationSystem_n_PosShake(2), AnimationSystem_n_PosShake(2));
        assert_ne!(AnimationSystem_n_PosShake(1), AnimationSystem_n_PosShake(2));
        assert_eq!(AnimationSystem_n_PosRandom(1), AnimationSystem_n_PosRandom(1));
        assert_eq!(AnimationSystem_n_PosRandom(2), AnimationSystem_n_PosRandom(2));
        assert_ne!(AnimationSystem_n_PosRandom(1), AnimationSystem_n_PosRandom(2));
        assert_eq!(AnimationSystem_n_PosAudio(1), AnimationSystem_n_PosAudio(1));
        assert_eq!(AnimationSystem_n_PosAudio(2), AnimationSystem_n_PosAudio(2));
        assert_ne!(AnimationSystem_n_PosAudio(1), AnimationSystem_n_PosAudio(2));
        assert_eq!(AnimationSystem_n_Macro(1), AnimationSystem_n_Macro(1));
        assert_eq!(AnimationSystem_n_Macro(2), AnimationSystem_n_Macro(2));
        assert_ne!(AnimationSystem_n_Macro(1), AnimationSystem_n_Macro(2));
        assert_eq!(MediaFolder_n_(1), MediaFolder_n_(1));
        assert_eq!(MediaFolder_n_(2), MediaFolder_n_(2));
        assert_ne!(MediaFolder_n_(1), MediaFolder_n_(2));
        assert_eq!(MediaContent_n_(1), MediaContent_n_(1));
        assert_eq!(MediaContent_n_(2), MediaContent_n_(2));
        assert_ne!(MediaContent_n_(1), MediaContent_n_(2));
        assert_eq!(ModelFolder_n_(1), ModelFolder_n_(1));
        assert_eq!(ModelFolder_n_(2), ModelFolder_n_(2));
        assert_ne!(ModelFolder_n_(1), ModelFolder_n_(2));
        assert_eq!(ModelContent_n_(1), ModelContent_n_(1));
        assert_eq!(ModelContent_n_(2), ModelContent_n_(2));
        assert_ne!(ModelContent_n_(1), ModelContent_n_(2));
        assert_eq!(PlayMode, PlayMode);
        assert_eq!(PlayBegin, PlayBegin);
        assert_eq!(PlayEnd, PlayEnd);
        assert_eq!(PlaySpeed, PlaySpeed);
        assert_eq!(ColorEffects_n_(1), ColorEffects_n_(1));
        assert_eq!(ColorEffects_n_(2), ColorEffects_n_(2));
        assert_ne!(ColorEffects_n_(1), ColorEffects_n_(2));
        assert_eq!(Color_n_(1), Color_n_(1));
        assert_eq!(Color_n_(2), Color_n_(2));
        assert_ne!(Color_n_(1), Color_n_(2));
        assert_eq!(Color_n_WheelIndex(1), Color_n_WheelIndex(1));
        assert_eq!(Color_n_WheelIndex(2), Color_n_WheelIndex(2));
        assert_ne!(Color_n_WheelIndex(1), Color_n_WheelIndex(2));
        assert_eq!(Color_n_WheelSpin(1), Color_n_WheelSpin(1));
        assert_eq!(Color_n_WheelSpin(2), Color_n_WheelSpin(2));
        assert_ne!(Color_n_WheelSpin(1), Color_n_WheelSpin(2));
        assert_eq!(Color_n_WheelRandom(1), Color_n_WheelRandom(1));
        assert_eq!(Color_n_WheelRandom(2), Color_n_WheelRandom(2));
        assert_ne!(Color_n_WheelRandom(1), Color_n_WheelRandom(2));
        assert_eq!(Color_n_WheelAudio(1), Color_n_WheelAudio(1));
        assert_eq!(Color_n_WheelAudio(2), Color_n_WheelAudio(2));
        assert_ne!(Color_n_WheelAudio(1), Color_n_WheelAudio(2));
        assert_eq!(ColorAdd_R, ColorAdd_R);
        assert_eq!(ColorAdd_G, ColorAdd_G);
        assert_eq!(ColorAdd_B, ColorAdd_B);
        assert_eq!(ColorAdd_C, ColorAdd_C);
        assert_eq!(ColorAdd_M, ColorAdd_M);
        assert_eq!(ColorAdd_Y, ColorAdd_Y);
        assert_eq!(ColorAdd_RY, ColorAdd_RY);
        assert_eq!(ColorAdd_GY, ColorAdd_GY);
        assert_eq!(ColorAdd_GC, ColorAdd_GC);
        assert_eq!(ColorAdd_BC, ColorAdd_BC);
        assert_eq!(ColorAdd_BM, ColorAdd_BM);
        assert_eq!(ColorAdd_RM, ColorAdd_RM);
        assert_eq!(ColorAdd_W, ColorAdd_W);
        assert_eq!(ColorAdd_WW, ColorAdd_WW);
        assert_eq!(ColorAdd_CW, ColorAdd_CW);
        assert_eq!(ColorAdd_UV, ColorAdd_UV);
        assert_eq!(ColorSub_R, ColorSub_R);
        assert_eq!(ColorSub_G, ColorSub_G);
        assert_eq!(ColorSub_B, ColorSub_B);
        assert_eq!(ColorSub_C, ColorSub_C);
        assert_eq!(ColorSub_M, ColorSub_M);
        assert_eq!(ColorSub_Y, ColorSub_Y);
        assert_eq!(ColorMacro_n_(1), ColorMacro_n_(1));
        assert_eq!(ColorMacro_n_(2), ColorMacro_n_(2));
        assert_ne!(ColorMacro_n_(1), ColorMacro_n_(2));
        assert_eq!(ColorMacro_n_Rate(1), ColorMacro_n_Rate(1));
        assert_eq!(ColorMacro_n_Rate(2), ColorMacro_n_Rate(2));
        assert_ne!(ColorMacro_n_Rate(1), ColorMacro_n_Rate(2));
        assert_eq!(CTO, CTO);
        assert_eq!(CTC, CTC);
        assert_eq!(CTB, CTB);
        assert_eq!(Tint, Tint);
        assert_eq!(HSB_Hue, HSB_Hue);
        assert_eq!(HSB_Saturation, HSB_Saturation);
        assert_eq!(HSB_Brightness, HSB_Brightness);
        assert_eq!(HSB_Quality, HSB_Quality);
        assert_eq!(CIE_X, CIE_X);
        assert_eq!(CIE_Y, CIE_Y);
        assert_eq!(CIE_Brightness, CIE_Brightness);
        assert_eq!(ColorRGB_Red, ColorRGB_Red);
        assert_eq!(ColorRGB_Green, ColorRGB_Green);
        assert_eq!(ColorRGB_Blue, ColorRGB_Blue);
        assert_eq!(ColorRGB_Cyan, ColorRGB_Cyan);
        assert_eq!(ColorRGB_Magenta, ColorRGB_Magenta);
        assert_eq!(ColorRGB_Yellow, ColorRGB_Yellow);
        assert_eq!(ColorRGB_Quality, ColorRGB_Quality);
        assert_eq!(VideoBoost_R, VideoBoost_R);
        assert_eq!(VideoBoost_G, VideoBoost_G);
        assert_eq!(VideoBoost_B, VideoBoost_B);
        assert_eq!(VideoHueShift, VideoHueShift);
        assert_eq!(VideoSaturation, VideoSaturation);
        assert_eq!(VideoBrightness, VideoBrightness);
        assert_eq!(VideoContrast, VideoContrast);
        assert_eq!(VideoKeyColor_R, VideoKeyColor_R);
        assert_eq!(VideoKeyColor_G, VideoKeyColor_G);
        assert_eq!(VideoKeyColor_B, VideoKeyColor_B);
        assert_eq!(VideoKeyIntensity, VideoKeyIntensity);
        assert_eq!(VideoKeyTolerance, VideoKeyTolerance);
        assert_eq!(StrobeDuration, StrobeDuration);
        assert_eq!(StrobeRate, StrobeRate);
        assert_eq!(Shutter_n_(1), Shutter_n_(1));
        assert_eq!(Shutter_n_(2), Shutter_n_(2));
        assert_ne!(Shutter_n_(1), Shutter_n_(2));
        assert_eq!(Shutter_n_Strobe(1), Shutter_n_Strobe(1));
        assert_eq!(Shutter_n_Strobe(2), Shutter_n_Strobe(2));
        assert_ne!(Shutter_n_Strobe(1), Shutter_n_Strobe(2));
        assert_eq!(Shutter_n_StrobePulse(1), Shutter_n_StrobePulse(1));
        assert_eq!(Shutter_n_StrobePulse(2), Shutter_n_StrobePulse(2));
        assert_ne!(Shutter_n_StrobePulse(1), Shutter_n_StrobePulse(2));
        assert_eq!(Shutter_n_StrobePulseClose(1), Shutter_n_StrobePulseClose(1));
        assert_eq!(Shutter_n_StrobePulseClose(2), Shutter_n_StrobePulseClose(2));
        assert_ne!(Shutter_n_StrobePulseClose(1), Shutter_n_StrobePulseClose(2));
        assert_eq!(Shutter_n_StrobePulseOpen(1), Shutter_n_StrobePulseOpen(1));
        assert_eq!(Shutter_n_StrobePulseOpen(2), Shutter_n_StrobePulseOpen(2));
        assert_ne!(Shutter_n_StrobePulseOpen(1), Shutter_n_StrobePulseOpen(2));
        assert_eq!(Shutter_n_StrobeRandom(1), Shutter_n_StrobeRandom(1));
        assert_eq!(Shutter_n_StrobeRandom(2), Shutter_n_StrobeRandom(2));
        assert_ne!(Shutter_n_StrobeRandom(1), Shutter_n_StrobeRandom(2));
        assert_eq!(Shutter_n_StrobeRandomPulse(1), Shutter_n_StrobeRandomPulse(1));
        assert_eq!(Shutter_n_StrobeRandomPulse(2), Shutter_n_StrobeRandomPulse(2));
        assert_ne!(Shutter_n_StrobeRandomPulse(1), Shutter_n_StrobeRandomPulse(2));
        assert_eq!(Shutter_n_StrobeRandomPulseClose(1), Shutter_n_StrobeRandomPulseClose(1));
        assert_eq!(Shutter_n_StrobeRandomPulseClose(2), Shutter_n_StrobeRandomPulseClose(2));
        assert_ne!(Shutter_n_StrobeRandomPulseClose(1), Shutter_n_StrobeRandomPulseClose(2));
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(1), Shutter_n_StrobeRandomPulseOpen(1));
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(2), Shutter_n_StrobeRandomPulseOpen(2));
        assert_ne!(Shutter_n_StrobeRandomPulseOpen(1), Shutter_n_StrobeRandomPulseOpen(2));
        assert_eq!(Shutter_n_StrobeEffect(1), Shutter_n_StrobeEffect(1));
        assert_eq!(Shutter_n_StrobeEffect(2), Shutter_n_StrobeEffect(2));
        assert_ne!(Shutter_n_StrobeEffect(1), Shutter_n_StrobeEffect(2));
        assert_eq!(Iris, Iris);
        assert_eq!(IrisStrobe, IrisStrobe);
        assert_eq!(IrisStrobeRandom, IrisStrobeRandom);
        assert_eq!(IrisPulseClose, IrisPulseClose);
        assert_eq!(IrisPulseOpen, IrisPulseOpen);
        assert_eq!(IrisRandomPulseClose, IrisRandomPulseClose);
        assert_eq!(IrisRandomPulseOpen, IrisRandomPulseOpen);
        assert_eq!(Frost_n_(1), Frost_n_(1));
        assert_eq!(Frost_n_(2), Frost_n_(2));
        assert_ne!(Frost_n_(1), Frost_n_(2));
        assert_eq!(Frost_n_PulseOpen(1), Frost_n_PulseOpen(1));
        assert_eq!(Frost_n_PulseOpen(2), Frost_n_PulseOpen(2));
        assert_ne!(Frost_n_PulseOpen(1), Frost_n_PulseOpen(2));
        assert_eq!(Frost_n_PulseClose(1), Frost_n_PulseClose(1));
        assert_eq!(Frost_n_PulseClose(2), Frost_n_PulseClose(2));
        assert_ne!(Frost_n_PulseClose(1), Frost_n_PulseClose(2));
        assert_eq!(Frost_n_Ramp(1), Frost_n_Ramp(1));
        assert_eq!(Frost_n_Ramp(2), Frost_n_Ramp(2));
        assert_ne!(Frost_n_Ramp(1), Frost_n_Ramp(2));
        assert_eq!(Prism_n_(1), Prism_n_(1));
        assert_eq!(Prism_n_(2), Prism_n_(2));
        assert_ne!(Prism_n_(1), Prism_n_(2));
        assert_eq!(Prism_n_SelectSpin(1), Prism_n_SelectSpin(1));
        assert_eq!(Prism_n_SelectSpin(2), Prism_n_SelectSpin(2));
        assert_ne!(Prism_n_SelectSpin(1), Prism_n_SelectSpin(2));
        assert_eq!(Prism_n_Macro(1), Prism_n_Macro(1));
        assert_eq!(Prism_n_Macro(2), Prism_n_Macro(2));
        assert_ne!(Prism_n_Macro(1), Prism_n_Macro(2));
        assert_eq!(Prism_n_Pos(1), Prism_n_Pos(1));
        assert_eq!(Prism_n_Pos(2), Prism_n_Pos(2));
        assert_ne!(Prism_n_Pos(1), Prism_n_Pos(2));
        assert_eq!(Prism_n_PosRotate(1), Prism_n_PosRotate(1));
        assert_eq!(Prism_n_PosRotate(2), Prism_n_PosRotate(2));
        assert_ne!(Prism_n_PosRotate(1), Prism_n_PosRotate(2));
        assert_eq!(Effects_n_(1), Effects_n_(1));
        assert_eq!(Effects_n_(2), Effects_n_(2));
        assert_ne!(Effects_n_(1), Effects_n_(2));
        assert_eq!(Effects_n_Rate(1), Effects_n_Rate(1));
        assert_eq!(Effects_n_Rate(2), Effects_n_Rate(2));
        assert_ne!(Effects_n_Rate(1), Effects_n_Rate(2));
        assert_eq!(Effects_n_Fade(1), Effects_n_Fade(1));
        assert_eq!(Effects_n_Fade(2), Effects_n_Fade(2));
        assert_ne!(Effects_n_Fade(1), Effects_n_Fade(2));
        assert_eq!(Effects_n_Adjust_m_(1, 1), Effects_n_Adjust_m_(1, 1));
        assert_eq!(Effects_n_Adjust_m_(1, 2), Effects_n_Adjust_m_(1, 2));
        assert_eq!(Effects_n_Adjust_m_(2, 1), Effects_n_Adjust_m_(2, 1));
        assert_eq!(Effects_n_Adjust_m_(2, 2), Effects_n_Adjust_m_(2, 2));
        assert_ne!(Effects_n_Adjust_m_(1, 1), Effects_n_Adjust_m_(1, 2));
        assert_ne!(Effects_n_Adjust_m_(1, 1), Effects_n_Adjust_m_(2, 1));
        assert_ne!(Effects_n_Adjust_m_(2, 1), Effects_n_Adjust_m_(1, 1));
        assert_ne!(Effects_n_Adjust_m_(2, 1), Effects_n_Adjust_m_(2, 2));
        assert_ne!(Effects_n_Adjust_m_(1, 2), Effects_n_Adjust_m_(1, 1));
        assert_ne!(Effects_n_Adjust_m_(1, 2), Effects_n_Adjust_m_(2, 2));
        assert_ne!(Effects_n_Adjust_m_(2, 2), Effects_n_Adjust_m_(1, 1));
        assert_ne!(Effects_n_Adjust_m_(1, 1), Effects_n_Adjust_m_(2, 2));
        assert_ne!(Effects_n_Adjust_m_(2, 2), Effects_n_Adjust_m_(1, 1));
        assert_ne!(Effects_n_Adjust_m_(1, 1), Effects_n_Adjust_m_(2, 2));
        assert_eq!(Effects_n_Pos(1), Effects_n_Pos(1));
        assert_eq!(Effects_n_Pos(2), Effects_n_Pos(2));
        assert_ne!(Effects_n_Pos(1), Effects_n_Pos(2));
        assert_eq!(Effects_n_PosRotate(1), Effects_n_PosRotate(1));
        assert_eq!(Effects_n_PosRotate(2), Effects_n_PosRotate(2));
        assert_ne!(Effects_n_PosRotate(1), Effects_n_PosRotate(2));
        assert_eq!(EffectsSync, EffectsSync);
        assert_eq!(BeamShaper, BeamShaper);
        assert_eq!(BeamShaperMacro, BeamShaperMacro);
        assert_eq!(BeamShaperPos, BeamShaperPos);
        assert_eq!(BeamShaperPosRotate, BeamShaperPosRotate);
        assert_eq!(Zoom, Zoom);
        assert_eq!(ZoomModeSpot, ZoomModeSpot);
        assert_eq!(ZoomModeBeam, ZoomModeBeam);
        assert_eq!(Focus_n_(1), Focus_n_(1));
        assert_eq!(Focus_n_(2), Focus_n_(2));
        assert_ne!(Focus_n_(1), Focus_n_(2));
        assert_eq!(Focus_n_Adjust(1), Focus_n_Adjust(1));
        assert_eq!(Focus_n_Adjust(2), Focus_n_Adjust(2));
        assert_ne!(Focus_n_Adjust(1), Focus_n_Adjust(2));
        assert_eq!(Focus_n_Distance(1), Focus_n_Distance(1));
        assert_eq!(Focus_n_Distance(2), Focus_n_Distance(2));
        assert_ne!(Focus_n_Distance(1), Focus_n_Distance(2));
        assert_eq!(Control_n_(1), Control_n_(1));
        assert_eq!(Control_n_(2), Control_n_(2));
        assert_ne!(Control_n_(1), Control_n_(2));
        assert_eq!(DimmerMode, DimmerMode);
        assert_eq!(DimmerCurve, DimmerCurve);
        assert_eq!(BlackoutMode, BlackoutMode);
        assert_eq!(LEDFrequency, LEDFrequency);
        assert_eq!(LEDZoneMode, LEDZoneMode);
        assert_eq!(PixelMode, PixelMode);
        assert_eq!(PanMode, PanMode);
        assert_eq!(TiltMode, TiltMode);
        assert_eq!(PanTiltMode, PanTiltMode);
        assert_eq!(PositionModes, PositionModes);
        assert_eq!(Gobo_n_WheelMode(1), Gobo_n_WheelMode(1));
        assert_eq!(Gobo_n_WheelMode(2), Gobo_n_WheelMode(2));
        assert_ne!(Gobo_n_WheelMode(1), Gobo_n_WheelMode(2));
        assert_eq!(AnimationWheel_n_Mode(1), AnimationWheel_n_Mode(1));
        assert_eq!(AnimationWheel_n_Mode(2), AnimationWheel_n_Mode(2));
        assert_ne!(AnimationWheel_n_Mode(1), AnimationWheel_n_Mode(2));
        assert_eq!(AnimationWheelShortcutMode, AnimationWheelShortcutMode);
        assert_eq!(Color_n_Mode(1), Color_n_Mode(1));
        assert_eq!(Color_n_Mode(2), Color_n_Mode(2));
        assert_ne!(Color_n_Mode(1), Color_n_Mode(2));
        assert_eq!(ColorWheelShortcutMode, ColorWheelShortcutMode);
        assert_eq!(CyanMode, CyanMode);
        assert_eq!(MagentaMode, MagentaMode);
        assert_eq!(YellowMode, YellowMode);
        assert_eq!(ColorMixMode, ColorMixMode);
        assert_eq!(ChromaticMode, ChromaticMode);
        assert_eq!(ColorCalibrationMode, ColorCalibrationMode);
        assert_eq!(ColorConsistency, ColorConsistency);
        assert_eq!(ColorControl, ColorControl);
        assert_eq!(ColorModelMode, ColorModelMode);
        assert_eq!(ColorSettingsReset, ColorSettingsReset);
        assert_eq!(ColorUniformity, ColorUniformity);
        assert_eq!(CRIMode, CRIMode);
        assert_eq!(CustomColor, CustomColor);
        assert_eq!(UVStability, UVStability);
        assert_eq!(WavelengthCorrection, WavelengthCorrection);
        assert_eq!(WhiteCount, WhiteCount);
        assert_eq!(StrobeMode, StrobeMode);
        assert_eq!(ZoomMode, ZoomMode);
        assert_eq!(FocusMode, FocusMode);
        assert_eq!(IrisMode, IrisMode);
        assert_eq!(Fan_n_Mode(1), Fan_n_Mode(1));
        assert_eq!(Fan_n_Mode(2), Fan_n_Mode(2));
        assert_ne!(Fan_n_Mode(1), Fan_n_Mode(2));
        assert_eq!(FollowSpotMode, FollowSpotMode);
        assert_eq!(BeamEffectIndexRotateMode, BeamEffectIndexRotateMode);
        assert_eq!(IntensityMSpeed, IntensityMSpeed);
        assert_eq!(PositionMSpeed, PositionMSpeed);
        assert_eq!(ColorMixMSpeed, ColorMixMSpeed);
        assert_eq!(ColorWheelSelectMSpeed, ColorWheelSelectMSpeed);
        assert_eq!(GoboWheel_n_MSpeed(1), GoboWheel_n_MSpeed(1));
        assert_eq!(GoboWheel_n_MSpeed(2), GoboWheel_n_MSpeed(2));
        assert_ne!(GoboWheel_n_MSpeed(1), GoboWheel_n_MSpeed(2));
        assert_eq!(IrisMSpeed, IrisMSpeed);
        assert_eq!(Prism_n_MSpeed(1), Prism_n_MSpeed(1));
        assert_eq!(Prism_n_MSpeed(2), Prism_n_MSpeed(2));
        assert_ne!(Prism_n_MSpeed(1), Prism_n_MSpeed(2));
        assert_eq!(FocusMSpeed, FocusMSpeed);
        assert_eq!(Frost_n_MSpeed(1), Frost_n_MSpeed(1));
        assert_eq!(Frost_n_MSpeed(2), Frost_n_MSpeed(2));
        assert_ne!(Frost_n_MSpeed(1), Frost_n_MSpeed(2));
        assert_eq!(ZoomMSpeed, ZoomMSpeed);
        assert_eq!(FrameMSpeed, FrameMSpeed);
        assert_eq!(GlobalMSpeed, GlobalMSpeed);
        assert_eq!(ReflectorAdjust, ReflectorAdjust);
        assert_eq!(FixtureGlobalReset, FixtureGlobalReset);
        assert_eq!(ShutterReset, ShutterReset);
        assert_eq!(BeamReset, BeamReset);
        assert_eq!(ColorMixReset, ColorMixReset);
        assert_eq!(ColorWheelReset, ColorWheelReset);
        assert_eq!(FocusReset, FocusReset);
        assert_eq!(FrameReset, FrameReset);
        assert_eq!(GoboWheelReset, GoboWheelReset);
        assert_eq!(IntensityReset, IntensityReset);
        assert_eq!(IrisReset, IrisReset);
        assert_eq!(PositionReset, PositionReset);
        assert_eq!(PanReset, PanReset);
        assert_eq!(TiltReset, TiltReset);
        assert_eq!(ZoomReset, ZoomReset);
        assert_eq!(CTBReset, CTBReset);
        assert_eq!(CTOReset, CTOReset);
        assert_eq!(CTCReset, CTCReset);
        assert_eq!(AnimationSystemReset, AnimationSystemReset);
        assert_eq!(FixtureCalibrationReset, FixtureCalibrationReset);
        assert_eq!(Function, Function);
        assert_eq!(LampControl, LampControl);
        assert_eq!(DisplayIntensity, DisplayIntensity);
        assert_eq!(DMXInput, DMXInput);
        assert_eq!(NoFeature, NoFeature);
        assert_eq!(Blower_n_(1), Blower_n_(1));
        assert_eq!(Blower_n_(2), Blower_n_(2));
        assert_ne!(Blower_n_(1), Blower_n_(2));
        assert_eq!(Fan_n_(1), Fan_n_(1));
        assert_eq!(Fan_n_(2), Fan_n_(2));
        assert_ne!(Fan_n_(1), Fan_n_(2));
        assert_eq!(Fog_n_(1), Fog_n_(1));
        assert_eq!(Fog_n_(2), Fog_n_(2));
        assert_ne!(Fog_n_(1), Fog_n_(2));
        assert_eq!(Haze_n_(1), Haze_n_(1));
        assert_eq!(Haze_n_(2), Haze_n_(2));
        assert_ne!(Haze_n_(1), Haze_n_(2));
        assert_eq!(LampPowerMode, LampPowerMode);
        assert_eq!(Fans, Fans);
        assert_eq!(Blade_n_A(1), Blade_n_A(1));
        assert_eq!(Blade_n_A(2), Blade_n_A(2));
        assert_ne!(Blade_n_A(1), Blade_n_A(2));
        assert_eq!(Blade_n_B(1), Blade_n_B(1));
        assert_eq!(Blade_n_B(2), Blade_n_B(2));
        assert_ne!(Blade_n_B(1), Blade_n_B(2));
        assert_eq!(Blade_n_Rot(1), Blade_n_Rot(1));
        assert_eq!(Blade_n_Rot(2), Blade_n_Rot(2));
        assert_ne!(Blade_n_Rot(1), Blade_n_Rot(2));
        assert_eq!(ShaperRot, ShaperRot);
        assert_eq!(ShaperMacros, ShaperMacros);
        assert_eq!(ShaperMacrosSpeed, ShaperMacrosSpeed);
        assert_eq!(BladeSoft_n_A(1), BladeSoft_n_A(1));
        assert_eq!(BladeSoft_n_A(2), BladeSoft_n_A(2));
        assert_ne!(BladeSoft_n_A(1), BladeSoft_n_A(2));
        assert_eq!(BladeSoft_n_B(1), BladeSoft_n_B(1));
        assert_eq!(BladeSoft_n_B(2), BladeSoft_n_B(2));
        assert_ne!(BladeSoft_n_B(1), BladeSoft_n_B(2));
        assert_eq!(KeyStone_n_A(1), KeyStone_n_A(1));
        assert_eq!(KeyStone_n_A(2), KeyStone_n_A(2));
        assert_ne!(KeyStone_n_A(1), KeyStone_n_A(2));
        assert_eq!(KeyStone_n_B(1), KeyStone_n_B(1));
        assert_eq!(KeyStone_n_B(2), KeyStone_n_B(2));
        assert_ne!(KeyStone_n_B(1), KeyStone_n_B(2));
        assert_eq!(Video, Video);
        assert_eq!(VideoEffect_n_Type(1), VideoEffect_n_Type(1));
        assert_eq!(VideoEffect_n_Type(2), VideoEffect_n_Type(2));
        assert_ne!(VideoEffect_n_Type(1), VideoEffect_n_Type(2));
        assert_eq!(VideoEffect_n_Parameter_m_(1, 1), VideoEffect_n_Parameter_m_(1, 1));
        assert_eq!(VideoEffect_n_Parameter_m_(1, 2), VideoEffect_n_Parameter_m_(1, 2));
        assert_eq!(VideoEffect_n_Parameter_m_(2, 1), VideoEffect_n_Parameter_m_(2, 1));
        assert_eq!(VideoEffect_n_Parameter_m_(2, 2), VideoEffect_n_Parameter_m_(2, 2));
        assert_ne!(VideoEffect_n_Parameter_m_(1, 1), VideoEffect_n_Parameter_m_(1, 2));
        assert_ne!(VideoEffect_n_Parameter_m_(1, 1), VideoEffect_n_Parameter_m_(2, 1));
        assert_ne!(VideoEffect_n_Parameter_m_(2, 1), VideoEffect_n_Parameter_m_(1, 1));
        assert_ne!(VideoEffect_n_Parameter_m_(2, 1), VideoEffect_n_Parameter_m_(2, 2));
        assert_ne!(VideoEffect_n_Parameter_m_(1, 2), VideoEffect_n_Parameter_m_(1, 1));
        assert_ne!(VideoEffect_n_Parameter_m_(1, 2), VideoEffect_n_Parameter_m_(2, 2));
        assert_ne!(VideoEffect_n_Parameter_m_(2, 2), VideoEffect_n_Parameter_m_(1, 1));
        assert_ne!(VideoEffect_n_Parameter_m_(1, 1), VideoEffect_n_Parameter_m_(2, 2));
        assert_ne!(VideoEffect_n_Parameter_m_(2, 2), VideoEffect_n_Parameter_m_(1, 1));
        assert_ne!(VideoEffect_n_Parameter_m_(1, 1), VideoEffect_n_Parameter_m_(2, 2));
        assert_eq!(VideoCamera_n_(1), VideoCamera_n_(1));
        assert_eq!(VideoCamera_n_(2), VideoCamera_n_(2));
        assert_ne!(VideoCamera_n_(1), VideoCamera_n_(2));
        assert_eq!(VideoSoundVolume_n_(1), VideoSoundVolume_n_(1));
        assert_eq!(VideoSoundVolume_n_(2), VideoSoundVolume_n_(2));
        assert_ne!(VideoSoundVolume_n_(1), VideoSoundVolume_n_(2));
        assert_eq!(VideoBlendMode, VideoBlendMode);
        assert_eq!(InputSource, InputSource);
        assert_eq!(FieldOfView, FieldOfView);

        Ok(())
    }

    #[test]
    fn test_from_str() -> Result<(), GdtfError> {
        use AttributeName::*;
        assert_eq!(UserDefined(Name::new("test")?), "test".into());
        if let UserDefined(n) = AttributeName::from("something{invalid") {
            n.assert_eq_allow_empty(&Name::new("")?, true);
        } else {
            panic!("something invalid was not parsed to empty user defined");
        }
        if let UserDefined(n) = AttributeName::from("") {
            n.assert_eq_allow_empty(&Name::new("")?, true);
        } else {
            panic!("empty str was not parsed to empty user defined");
        }
        assert_eq!(Dimmer, "Dimmer".into());
        assert_eq!(Pan, "Pan".into());
        assert_eq!(Tilt, "Tilt".into());
        assert_eq!(PanRotate, "PanRotate".into());
        assert_eq!(TiltRotate, "TiltRotate".into());
        assert_eq!(PositionEffect, "PositionEffect".into());
        assert_eq!(PositionEffectRate, "PositionEffectRate".into());
        assert_eq!(PositionEffectFade, "PositionEffectFade".into());
        assert_eq!(XYZ_X, "XYZ_X".into());
        assert_eq!(XYZ_Y, "XYZ_Y".into());
        assert_eq!(XYZ_Z, "XYZ_Z".into());
        assert_eq!(Rot_X, "Rot_X".into());
        assert_eq!(Rot_Y, "Rot_Y".into());
        assert_eq!(Rot_Z, "Rot_Z".into());
        assert_eq!(Scale_X, "Scale_X".into());
        assert_eq!(Scale_Y, "Scale_Y".into());
        assert_eq!(Scale_Z, "Scale_Z".into());
        assert_eq!(Scale_XYZ, "Scale_XYZ".into());
        assert_eq!(Gobo_n_(1), "Gobo1".into());
        assert_eq!(Gobo_n_(2), "Gobo2".into());
        assert_eq!(Gobo_n_SelectSpin(1), "Gobo1SelectSpin".into());
        assert_eq!(Gobo_n_SelectSpin(2), "Gobo2SelectSpin".into());
        assert_eq!(Gobo_n_SelectShake(1), "Gobo1SelectShake".into());
        assert_eq!(Gobo_n_SelectShake(2), "Gobo2SelectShake".into());
        assert_eq!(Gobo_n_SelectEffects(1), "Gobo1SelectEffects".into());
        assert_eq!(Gobo_n_SelectEffects(2), "Gobo2SelectEffects".into());
        assert_eq!(Gobo_n_WheelIndex(1), "Gobo1WheelIndex".into());
        assert_eq!(Gobo_n_WheelIndex(2), "Gobo2WheelIndex".into());
        assert_eq!(Gobo_n_WheelSpin(1), "Gobo1WheelSpin".into());
        assert_eq!(Gobo_n_WheelSpin(2), "Gobo2WheelSpin".into());
        assert_eq!(Gobo_n_WheelShake(1), "Gobo1WheelShake".into());
        assert_eq!(Gobo_n_WheelShake(2), "Gobo2WheelShake".into());
        assert_eq!(Gobo_n_WheelRandom(1), "Gobo1WheelRandom".into());
        assert_eq!(Gobo_n_WheelRandom(2), "Gobo2WheelRandom".into());
        assert_eq!(Gobo_n_WheelAudio(1), "Gobo1WheelAudio".into());
        assert_eq!(Gobo_n_WheelAudio(2), "Gobo2WheelAudio".into());
        assert_eq!(Gobo_n_Pos(1), "Gobo1Pos".into());
        assert_eq!(Gobo_n_Pos(2), "Gobo2Pos".into());
        assert_eq!(Gobo_n_PosRotate(1), "Gobo1PosRotate".into());
        assert_eq!(Gobo_n_PosRotate(2), "Gobo2PosRotate".into());
        assert_eq!(Gobo_n_PosShake(1), "Gobo1PosShake".into());
        assert_eq!(Gobo_n_PosShake(2), "Gobo2PosShake".into());
        assert_eq!(AnimationWheel_n_(1), "AnimationWheel1".into());
        assert_eq!(AnimationWheel_n_(2), "AnimationWheel2".into());
        assert_eq!(AnimationWheel_n_Audio(1), "AnimationWheel1Audio".into());
        assert_eq!(AnimationWheel_n_Audio(2), "AnimationWheel2Audio".into());
        assert_eq!(AnimationWheel_n_Macro(1), "AnimationWheel1Macro".into());
        assert_eq!(AnimationWheel_n_Macro(2), "AnimationWheel2Macro".into());
        assert_eq!(AnimationWheel_n_Random(1), "AnimationWheel1Random".into());
        assert_eq!(AnimationWheel_n_Random(2), "AnimationWheel2Random".into());
        assert_eq!(AnimationWheel_n_SelectEffects(1), "AnimationWheel1SelectEffects".into());
        assert_eq!(AnimationWheel_n_SelectEffects(2), "AnimationWheel2SelectEffects".into());
        assert_eq!(AnimationWheel_n_SelectShake(1), "AnimationWheel1SelectShake".into());
        assert_eq!(AnimationWheel_n_SelectShake(2), "AnimationWheel2SelectShake".into());
        assert_eq!(AnimationWheel_n_SelectSpin(1), "AnimationWheel1SelectSpin".into());
        assert_eq!(AnimationWheel_n_SelectSpin(2), "AnimationWheel2SelectSpin".into());
        assert_eq!(AnimationWheel_n_Pos(1), "AnimationWheel1Pos".into());
        assert_eq!(AnimationWheel_n_Pos(2), "AnimationWheel2Pos".into());
        assert_eq!(AnimationWheel_n_PosRotate(1), "AnimationWheel1PosRotate".into());
        assert_eq!(AnimationWheel_n_PosRotate(2), "AnimationWheel2PosRotate".into());
        assert_eq!(AnimationWheel_n_PosShake(1), "AnimationWheel1PosShake".into());
        assert_eq!(AnimationWheel_n_PosShake(2), "AnimationWheel2PosShake".into());
        assert_eq!(AnimationSystem_n_(1), "AnimationSystem1".into());
        assert_eq!(AnimationSystem_n_(2), "AnimationSystem2".into());
        assert_eq!(AnimationSystem_n_Ramp(1), "AnimationSystem1Ramp".into());
        assert_eq!(AnimationSystem_n_Ramp(2), "AnimationSystem2Ramp".into());
        assert_eq!(AnimationSystem_n_Shake(1), "AnimationSystem1Shake".into());
        assert_eq!(AnimationSystem_n_Shake(2), "AnimationSystem2Shake".into());
        assert_eq!(AnimationSystem_n_Audio(1), "AnimationSystem1Audio".into());
        assert_eq!(AnimationSystem_n_Audio(2), "AnimationSystem2Audio".into());
        assert_eq!(AnimationSystem_n_Random(1), "AnimationSystem1Random".into());
        assert_eq!(AnimationSystem_n_Random(2), "AnimationSystem2Random".into());
        assert_eq!(AnimationSystem_n_Pos(1), "AnimationSystem1Pos".into());
        assert_eq!(AnimationSystem_n_Pos(2), "AnimationSystem2Pos".into());
        assert_eq!(AnimationSystem_n_PosRotate(1), "AnimationSystem1PosRotate".into());
        assert_eq!(AnimationSystem_n_PosRotate(2), "AnimationSystem2PosRotate".into());
        assert_eq!(AnimationSystem_n_PosShake(1), "AnimationSystem1PosShake".into());
        assert_eq!(AnimationSystem_n_PosShake(2), "AnimationSystem2PosShake".into());
        assert_eq!(AnimationSystem_n_PosRandom(1), "AnimationSystem1PosRandom".into());
        assert_eq!(AnimationSystem_n_PosRandom(2), "AnimationSystem2PosRandom".into());
        assert_eq!(AnimationSystem_n_PosAudio(1), "AnimationSystem1PosAudio".into());
        assert_eq!(AnimationSystem_n_PosAudio(2), "AnimationSystem2PosAudio".into());
        assert_eq!(AnimationSystem_n_Macro(1), "AnimationSystem1Macro".into());
        assert_eq!(AnimationSystem_n_Macro(2), "AnimationSystem2Macro".into());
        assert_eq!(MediaFolder_n_(1), "MediaFolder1".into());
        assert_eq!(MediaFolder_n_(2), "MediaFolder2".into());
        assert_eq!(MediaContent_n_(1), "MediaContent1".into());
        assert_eq!(MediaContent_n_(2), "MediaContent2".into());
        assert_eq!(ModelFolder_n_(1), "ModelFolder1".into());
        assert_eq!(ModelFolder_n_(2), "ModelFolder2".into());
        assert_eq!(ModelContent_n_(1), "ModelContent1".into());
        assert_eq!(ModelContent_n_(2), "ModelContent2".into());
        assert_eq!(PlayMode, "PlayMode".into());
        assert_eq!(PlayBegin, "PlayBegin".into());
        assert_eq!(PlayEnd, "PlayEnd".into());
        assert_eq!(PlaySpeed, "PlaySpeed".into());
        assert_eq!(ColorEffects_n_(1), "ColorEffects1".into());
        assert_eq!(ColorEffects_n_(2), "ColorEffects2".into());
        assert_eq!(Color_n_(1), "Color1".into());
        assert_eq!(Color_n_(2), "Color2".into());
        assert_eq!(Color_n_WheelIndex(1), "Color1WheelIndex".into());
        assert_eq!(Color_n_WheelIndex(2), "Color2WheelIndex".into());
        assert_eq!(Color_n_WheelSpin(1), "Color1WheelSpin".into());
        assert_eq!(Color_n_WheelSpin(2), "Color2WheelSpin".into());
        assert_eq!(Color_n_WheelRandom(1), "Color1WheelRandom".into());
        assert_eq!(Color_n_WheelRandom(2), "Color2WheelRandom".into());
        assert_eq!(Color_n_WheelAudio(1), "Color1WheelAudio".into());
        assert_eq!(Color_n_WheelAudio(2), "Color2WheelAudio".into());
        assert_eq!(ColorAdd_R, "ColorAdd_R".into());
        assert_eq!(ColorAdd_G, "ColorAdd_G".into());
        assert_eq!(ColorAdd_B, "ColorAdd_B".into());
        assert_eq!(ColorAdd_C, "ColorAdd_C".into());
        assert_eq!(ColorAdd_M, "ColorAdd_M".into());
        assert_eq!(ColorAdd_Y, "ColorAdd_Y".into());
        assert_eq!(ColorAdd_RY, "ColorAdd_RY".into());
        assert_eq!(ColorAdd_GY, "ColorAdd_GY".into());
        assert_eq!(ColorAdd_GC, "ColorAdd_GC".into());
        assert_eq!(ColorAdd_BC, "ColorAdd_BC".into());
        assert_eq!(ColorAdd_BM, "ColorAdd_BM".into());
        assert_eq!(ColorAdd_RM, "ColorAdd_RM".into());
        assert_eq!(ColorAdd_W, "ColorAdd_W".into());
        assert_eq!(ColorAdd_WW, "ColorAdd_WW".into());
        assert_eq!(ColorAdd_CW, "ColorAdd_CW".into());
        assert_eq!(ColorAdd_UV, "ColorAdd_UV".into());
        assert_eq!(ColorSub_R, "ColorSub_R".into());
        assert_eq!(ColorSub_G, "ColorSub_G".into());
        assert_eq!(ColorSub_B, "ColorSub_B".into());
        assert_eq!(ColorSub_C, "ColorSub_C".into());
        assert_eq!(ColorSub_M, "ColorSub_M".into());
        assert_eq!(ColorSub_Y, "ColorSub_Y".into());
        assert_eq!(ColorMacro_n_(1), "ColorMacro1".into());
        assert_eq!(ColorMacro_n_(2), "ColorMacro2".into());
        assert_eq!(ColorMacro_n_Rate(1), "ColorMacro1Rate".into());
        assert_eq!(ColorMacro_n_Rate(2), "ColorMacro2Rate".into());
        assert_eq!(CTO, "CTO".into());
        assert_eq!(CTC, "CTC".into());
        assert_eq!(CTB, "CTB".into());
        assert_eq!(Tint, "Tint".into());
        assert_eq!(HSB_Hue, "HSB_Hue".into());
        assert_eq!(HSB_Saturation, "HSB_Saturation".into());
        assert_eq!(HSB_Brightness, "HSB_Brightness".into());
        assert_eq!(HSB_Quality, "HSB_Quality".into());
        assert_eq!(CIE_X, "CIE_X".into());
        assert_eq!(CIE_Y, "CIE_Y".into());
        assert_eq!(CIE_Brightness, "CIE_Brightness".into());
        assert_eq!(ColorRGB_Red, "ColorRGB_Red".into());
        assert_eq!(ColorRGB_Green, "ColorRGB_Green".into());
        assert_eq!(ColorRGB_Blue, "ColorRGB_Blue".into());
        assert_eq!(ColorRGB_Cyan, "ColorRGB_Cyan".into());
        assert_eq!(ColorRGB_Magenta, "ColorRGB_Magenta".into());
        assert_eq!(ColorRGB_Yellow, "ColorRGB_Yellow".into());
        assert_eq!(ColorRGB_Quality, "ColorRGB_Quality".into());
        assert_eq!(VideoBoost_R, "VideoBoost_R".into());
        assert_eq!(VideoBoost_G, "VideoBoost_G".into());
        assert_eq!(VideoBoost_B, "VideoBoost_B".into());
        assert_eq!(VideoHueShift, "VideoHueShift".into());
        assert_eq!(VideoSaturation, "VideoSaturation".into());
        assert_eq!(VideoBrightness, "VideoBrightness".into());
        assert_eq!(VideoContrast, "VideoContrast".into());
        assert_eq!(VideoKeyColor_R, "VideoKeyColor_R".into());
        assert_eq!(VideoKeyColor_G, "VideoKeyColor_G".into());
        assert_eq!(VideoKeyColor_B, "VideoKeyColor_B".into());
        assert_eq!(VideoKeyIntensity, "VideoKeyIntensity".into());
        assert_eq!(VideoKeyTolerance, "VideoKeyTolerance".into());
        assert_eq!(StrobeDuration, "StrobeDuration".into());
        assert_eq!(StrobeRate, "StrobeRate".into());
        assert_eq!(Shutter_n_(1), "Shutter11)".into());
        assert_eq!(Shutter_n_(2), "Shutter2".into());
        assert_eq!(Shutter_n_Strobe(1), "Shutter1Strobe".into());
        assert_eq!(Shutter_n_Strobe(2), "Shutter2Strobe".into());
        assert_eq!(Shutter_n_StrobePulse(1), "Shutter1StrobePulse".into());
        assert_eq!(Shutter_n_StrobePulse(2), "Shutter2StrobePulse".into());
        assert_eq!(Shutter_n_StrobePulseClose(1), "Shutter1StrobePulseClose".into());
        assert_eq!(Shutter_n_StrobePulseClose(2), "Shutter2StrobePulseClose".into());
        assert_eq!(Shutter_n_StrobePulseOpen(1), "Shutter1StrobePulseOpen".into());
        assert_eq!(Shutter_n_StrobePulseOpen(2), "Shutter2StrobePulseOpen".into());
        assert_eq!(Shutter_n_StrobeRandom(1), "Shutter1StrobeRandom".into());
        assert_eq!(Shutter_n_StrobeRandom(2), "Shutter2StrobeRandom".into());
        assert_eq!(Shutter_n_StrobeRandomPulse(1), "Shutter1StrobeRandomPulse".into());
        assert_eq!(Shutter_n_StrobeRandomPulse(2), "Shutter2StrobeRandomPulse".into());
        assert_eq!(Shutter_n_StrobeRandomPulseClose(1), "Shutter1StrobeRandomPulseClose".into());
        assert_eq!(Shutter_n_StrobeRandomPulseClose(2), "Shutter2StrobeRandomPulseClose".into());
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(1), "Shutter1StrobeRandomPulseOpen".into());
        assert_eq!(Shutter_n_StrobeRandomPulseOpen(2), "Shutter2StrobeRandomPulseOpen".into());
        assert_eq!(Shutter_n_StrobeEffect(1), "Shutter1StrobeEffect".into());
        assert_eq!(Shutter_n_StrobeEffect(2), "Shutter2StrobeEffect".into());
        assert_eq!(Iris, "Iris".into());
        assert_eq!(IrisStrobe, "IrisStrobe".into());
        assert_eq!(IrisStrobeRandom, "IrisStrobeRandom".into());
        assert_eq!(IrisPulseClose, "IrisPulseClose".into());
        assert_eq!(IrisPulseOpen, "IrisPulseOpen".into());
        assert_eq!(IrisRandomPulseClose, "IrisRandomPulseClose".into());
        assert_eq!(IrisRandomPulseOpen, "IrisRandomPulseOpen".into());
        assert_eq!(Frost_n_(1), "Frost1".into());
        assert_eq!(Frost_n_(2), "Frost2".into());
        assert_eq!(Frost_n_PulseOpen(1), "Frost1PulseOpen".into());
        assert_eq!(Frost_n_PulseOpen(2), "Frost2PulseOpen".into());
        assert_eq!(Frost_n_PulseClose(1), "Frost1PulseClose".into());
        assert_eq!(Frost_n_PulseClose(2), "Frost2PulseClose".into());
        assert_eq!(Frost_n_Ramp(1), "Frost1Ramp".into());
        assert_eq!(Frost_n_Ramp(2), "Frost2Ramp".into());
        assert_eq!(Prism_n_(1), "Prism1".into());
        assert_eq!(Prism_n_(2), "Prism2".into());
        assert_eq!(Prism_n_SelectSpin(1), "Prism1SelectSpin".into());
        assert_eq!(Prism_n_SelectSpin(2), "Prism2SelectSpin".into());
        assert_eq!(Prism_n_Macro(1), "Prism1Macro".into());
        assert_eq!(Prism_n_Macro(2), "Prism2Macro".into());
        assert_eq!(Prism_n_Pos(1), "Prism1Pos".into());
        assert_eq!(Prism_n_Pos(2), "Prism2Pos".into());
        assert_eq!(Prism_n_PosRotate(1), "Prism1PosRotate".into());
        assert_eq!(Prism_n_PosRotate(2), "Prism2PosRotate".into());
        assert_eq!(Effects_n_(1), "Effects1".into());
        assert_eq!(Effects_n_(2), "Effects2".into());
        assert_eq!(Effects_n_Rate(1), "Effects1Rate".into());
        assert_eq!(Effects_n_Rate(2), "Effects2Rate".into());
        assert_eq!(Effects_n_Fade(1), "Effects1Fade".into());
        assert_eq!(Effects_n_Fade(2), "Effects2Fade".into());
        assert_eq!(Effects_n_Adjust_m_(1, 1), "Effects1Adjust1".into());
        assert_eq!(Effects_n_Adjust_m_(1, 2), "Effects1Adjust2".into());
        assert_eq!(Effects_n_Adjust_m_(2, 1), "Effects2Adjust1".into());
        assert_eq!(Effects_n_Adjust_m_(2, 2), "Effects1Adjust2".into());
        assert_eq!(Effects_n_Pos(1), "Effects1Pos".into());
        assert_eq!(Effects_n_Pos(2), "Effects2Pos".into());
        assert_eq!(Effects_n_PosRotate(1), "Effects1PosRotate".into());
        assert_eq!(Effects_n_PosRotate(2), "Effects2PosRotate".into());
        assert_eq!(EffectsSync, "EffectsSync".into());
        assert_eq!(BeamShaper, "BeamShaper".into());
        assert_eq!(BeamShaperMacro, "BeamShaperMacro".into());
        assert_eq!(BeamShaperPos, "BeamShaperPos".into());
        assert_eq!(BeamShaperPosRotate, "BeamShaperPosRotate".into());
        assert_eq!(Zoom, "Zoom".into());
        assert_eq!(ZoomModeSpot, "ZoomModeSpot".into());
        assert_eq!(ZoomModeBeam, "ZoomModeBeam".into());
        assert_eq!(Focus_n_(1), "Focus1".into());
        assert_eq!(Focus_n_(2), "Focus2".into());
        assert_eq!(Focus_n_Adjust(1), "Focus1Adjust".into());
        assert_eq!(Focus_n_Adjust(2), "Focus2Adjust".into());
        assert_eq!(Focus_n_Distance(1), "Focus1Distance".into());
        assert_eq!(Focus_n_Distance(2), "Focus2Distance".into());
        assert_eq!(Control_n_(1), "Control1".into());
        assert_eq!(Control_n_(2), "Control2".into());
        assert_eq!(DimmerMode, "DimmerMode".into());
        assert_eq!(DimmerCurve, "DimmerCurve".into());
        assert_eq!(BlackoutMode, "BlackoutMode".into());
        assert_eq!(LEDFrequency, "LEDFrequency".into());
        assert_eq!(LEDZoneMode, "LEDZoneMode".into());
        assert_eq!(PixelMode, "PixelMode".into());
        assert_eq!(PanMode, "PanMode".into());
        assert_eq!(TiltMode, "TiltMode".into());
        assert_eq!(PanTiltMode, "PanTiltMode".into());
        assert_eq!(PositionModes, "PositionModes".into());
        assert_eq!(Gobo_n_WheelMode(1), "Gobo1WheelMode".into());
        assert_eq!(Gobo_n_WheelMode(2), "Gobo2WheelMode".into());
        assert_eq!(AnimationWheel_n_Mode(1), "AnimationWheel1Mode".into());
        assert_eq!(AnimationWheel_n_Mode(2), "AnimationWheel2Mode".into());
        assert_eq!(AnimationWheelShortcutMode, "AnimationWheelShortcutMode".into());
        assert_eq!(Color_n_Mode(1), "Color1Mode".into());
        assert_eq!(Color_n_Mode(2), "Color2Mode".into());
        assert_eq!(ColorWheelShortcutMode, "ColorWheelShortcutMode".into());
        assert_eq!(CyanMode, "CyanMode".into());
        assert_eq!(MagentaMode, "MagentaMode".into());
        assert_eq!(YellowMode, "YellowMode".into());
        assert_eq!(ColorMixMode, "ColorMixMode".into());
        assert_eq!(ChromaticMode, "ChromaticMode".into());
        assert_eq!(ColorCalibrationMode, "ColorCalibrationMode".into());
        assert_eq!(ColorConsistency, "ColorConsistency".into());
        assert_eq!(ColorControl, "ColorControl".into());
        assert_eq!(ColorModelMode, "ColorModelMode".into());
        assert_eq!(ColorSettingsReset, "ColorSettingsReset".into());
        assert_eq!(ColorUniformity, "ColorUniformity".into());
        assert_eq!(CRIMode, "CRIMode".into());
        assert_eq!(CustomColor, "CustomColor".into());
        assert_eq!(UVStability, "UVStability".into());
        assert_eq!(WavelengthCorrection, "WavelengthCorrection".into());
        assert_eq!(WhiteCount, "WhiteCount".into());
        assert_eq!(StrobeMode, "StrobeMode".into());
        assert_eq!(ZoomMode, "ZoomMode".into());
        assert_eq!(FocusMode, "FocusMode".into());
        assert_eq!(IrisMode, "IrisMode".into());
        assert_eq!(Fan_n_Mode(1), "Fan1Mode".into());
        assert_eq!(Fan_n_Mode(2), "Fan2Mode".into());
        assert_eq!(FollowSpotMode, "FollowSpotMode".into());
        assert_eq!(BeamEffectIndexRotateMode, "BeamEffectIndexRotateMode".into());
        assert_eq!(IntensityMSpeed, "IntensityMSpeed".into());
        assert_eq!(PositionMSpeed, "PositionMSpeed".into());
        assert_eq!(ColorMixMSpeed, "ColorMixMSpeed".into());
        assert_eq!(ColorWheelSelectMSpeed, "ColorWheelSelectMSpeed".into());
        assert_eq!(GoboWheel_n_MSpeed(1), "GoboWheel1MSpeed".into());
        assert_eq!(GoboWheel_n_MSpeed(2), "GoboWheel2MSpeed".into());
        assert_eq!(IrisMSpeed, "IrisMSpeed".into());
        assert_eq!(Prism_n_MSpeed(1), "Prism1MSpeed".into());
        assert_eq!(Prism_n_MSpeed(2), "Prism2MSpeed".into());
        assert_eq!(FocusMSpeed, "FocusMSpeed".into());
        assert_eq!(Frost_n_MSpeed(1), "Frost1MSpeed".into());
        assert_eq!(Frost_n_MSpeed(2), "Frost2MSpeed".into());
        assert_eq!(ZoomMSpeed, "ZoomMSpeed".into());
        assert_eq!(FrameMSpeed, "FrameMSpeed".into());
        assert_eq!(GlobalMSpeed, "GlobalMSpeed".into());
        assert_eq!(ReflectorAdjust, "ReflectorAdjust".into());
        assert_eq!(FixtureGlobalReset, "FixtureGlobalReset".into());
        assert_eq!(ShutterReset, "ShutterReset".into());
        assert_eq!(BeamReset, "BeamReset".into());
        assert_eq!(ColorMixReset, "ColorMixReset".into());
        assert_eq!(ColorWheelReset, "ColorWheelReset".into());
        assert_eq!(FocusReset, "FocusReset".into());
        assert_eq!(FrameReset, "FrameReset".into());
        assert_eq!(GoboWheelReset, "GoboWheelReset".into());
        assert_eq!(IntensityReset, "IntensityReset".into());
        assert_eq!(IrisReset, "IrisReset".into());
        assert_eq!(PositionReset, "PositionReset".into());
        assert_eq!(PanReset, "PanReset".into());
        assert_eq!(TiltReset, "TiltReset".into());
        assert_eq!(ZoomReset, "ZoomReset".into());
        assert_eq!(CTBReset, "CTBReset".into());
        assert_eq!(CTOReset, "CTOReset".into());
        assert_eq!(CTCReset, "CTCReset".into());
        assert_eq!(AnimationSystemReset, "AnimationSystemReset".into());
        assert_eq!(FixtureCalibrationReset, "FixtureCalibrationReset".into());
        assert_eq!(Function, "Function".into());
        assert_eq!(LampControl, "LampControl".into());
        assert_eq!(DisplayIntensity, "DisplayIntensity".into());
        assert_eq!(DMXInput, "DMXInput".into());
        assert_eq!(NoFeature, "NoFeature".into());
        assert_eq!(Blower_n_(1), "Blower1".into());
        assert_eq!(Blower_n_(2), "Blower2".into());
        assert_eq!(Fan_n_(1), "Fan1".into());
        assert_eq!(Fan_n_(2), "Fan2".into());
        assert_eq!(Fog_n_(1), "Fog1".into());
        assert_eq!(Fog_n_(2), "Fog2".into());
        assert_eq!(Haze_n_(1), "Haze1".into());
        assert_eq!(Haze_n_(2), "Haze2".into());
        assert_eq!(LampPowerMode, "LampPowerMode".into());
        assert_eq!(Fans, "Fans".into());
        assert_eq!(Blade_n_A(1), "Blade1A".into());
        assert_eq!(Blade_n_A(2), "Blade2A".into());
        assert_eq!(Blade_n_B(1), "Blade1B".into());
        assert_eq!(Blade_n_B(2), "Blade2B".into());
        assert_eq!(Blade_n_Rot(1), "Blade1Rot".into());
        assert_eq!(Blade_n_Rot(2), "Blade2Rot".into());
        assert_eq!(ShaperRot, "ShaperRot".into());
        assert_eq!(ShaperMacros, "ShaperMacros".into());
        assert_eq!(ShaperMacrosSpeed, "ShaperMacrosSpeed".into());
        assert_eq!(BladeSoft_n_A(1), "BladeSoft1A".into());
        assert_eq!(BladeSoft_n_A(2), "BladeSoft2A".into());
        assert_eq!(BladeSoft_n_B(1), "BladeSoft1B".into());
        assert_eq!(BladeSoft_n_B(2), "BladeSoft2B".into());
        assert_eq!(KeyStone_n_A(1), "KeyStone1A".into());
        assert_eq!(KeyStone_n_A(2), "KeyStone2A".into());
        assert_eq!(KeyStone_n_B(1), "KeyStone1B".into());
        assert_eq!(KeyStone_n_B(2), "KeyStone2B".into());
        assert_eq!(Video, "Video".into());
        assert_eq!(VideoEffect_n_Type(1), "VideoEffect1Type".into());
        assert_eq!(VideoEffect_n_Type(2), "VideoEffect2Type".into());
        assert_eq!(VideoEffect_n_Parameter_m_(1, 1), "VideoEffect1Parameter1".into());
        assert_eq!(VideoEffect_n_Parameter_m_(1, 2), "VideoEffect1Parameter2".into());
        assert_eq!(VideoEffect_n_Parameter_m_(2, 1), "VideoEffect2Parameter1".into());
        assert_eq!(VideoEffect_n_Parameter_m_(2, 2), "VideoEffect2Parameter2".into());
        assert_eq!(VideoCamera_n_(1), "VideoCamera1".into());
        assert_eq!(VideoCamera_n_(2), "VideoCamera2".into());
        assert_eq!(VideoSoundVolume_n_(1), "VideoSoundVolume1".into());
        assert_eq!(VideoSoundVolume_n_(2), "VideoSoundVolume2".into());
        assert_eq!(VideoBlendMode, "VideoBlendMode".into());
        assert_eq!(InputSource, "InputSource".into());
        assert_eq!(FieldOfView, "FieldOfView".into());

        Ok(())
    }
}

/*
Set for copying if needed


UserDefined(n1)
Dimmer
Pan
Tilt
PanRotate
TiltRotate
PositionEffect
PositionEffectRate
PositionEffectFade
XYZ_X
XYZ_Y
XYZ_Z
Rot_X
Rot_Y
Rot_Z
Scale_X
Scale_Y
Scale_Z
Scale_XYZ
Gobo_n_(n1)
Gobo_n_SelectSpin(n1)
Gobo_n_SelectShake(n1)
Gobo_n_SelectEffects(n1)
Gobo_n_WheelIndex(n1)
Gobo_n_WheelSpin(n1)
Gobo_n_WheelShake(n1)
Gobo_n_WheelRandom(n1)
Gobo_n_WheelAudio(n1)
Gobo_n_Pos(n1)
Gobo_n_PosRotate(n1)
Gobo_n_PosShake(n1)
AnimationWheel_n_(n1)
AnimationWheel_n_Audio(n1)
AnimationWheel_n_Macro(n1)
AnimationWheel_n_Random(n1)
AnimationWheel_n_SelectEffects(n1)
AnimationWheel_n_SelectShake(n1)
AnimationWheel_n_SelectSpin(n1)
AnimationWheel_n_Pos(n1)
AnimationWheel_n_PosRotate(n1)
AnimationWheel_n_PosShake(n1)
AnimationSystem_n_(n1)
AnimationSystem_n_Ramp(n1)
AnimationSystem_n_Shake(n1)
AnimationSystem_n_Audio(n1)
AnimationSystem_n_Random(n1)
AnimationSystem_n_Pos(n1)
AnimationSystem_n_PosRotate(n1)
AnimationSystem_n_PosShake(n1)
AnimationSystem_n_PosRandom(n1)
AnimationSystem_n_PosAudio(n1)
AnimationSystem_n_Macro(n1)
MediaFolder_n_(n1)
MediaContent_n_(n1)
ModelFolder_n_(n1)
ModelContent_n_(n1)
PlayMode
PlayBegin
PlayEnd
PlaySpeed
ColorEffects_n_(n1)
Color_n_(n1)
Color_n_WheelIndex(n1)
Color_n_WheelSpin(n1)
Color_n_WheelRandom(n1)
Color_n_WheelAudio(n1)
ColorAdd_R
ColorAdd_G
ColorAdd_B
ColorAdd_C
ColorAdd_M
ColorAdd_Y
ColorAdd_RY
ColorAdd_GY
ColorAdd_GC
ColorAdd_BC
ColorAdd_BM
ColorAdd_RM
ColorAdd_W
ColorAdd_WW
ColorAdd_CW
ColorAdd_UV
ColorSub_R
ColorSub_G
ColorSub_B
ColorSub_C
ColorSub_M
ColorSub_Y
ColorMacro_n_(n1)
ColorMacro_n_Rate(n1)
CTO
CTC
CTB
Tint
HSB_Hue
HSB_Saturation
HSB_Brightness
HSB_Quality
CIE_X
CIE_Y
CIE_Brightness
ColorRGB_Red
ColorRGB_Green
ColorRGB_Blue
ColorRGB_Cyan
ColorRGB_Magenta
ColorRGB_Yellow
ColorRGB_Quality
VideoBoost_R
VideoBoost_G
VideoBoost_B
VideoHueShift
VideoSaturation
VideoBrightness
VideoContrast
VideoKeyColor_R
VideoKeyColor_G
VideoKeyColor_B
VideoKeyIntensity
VideoKeyTolerance
StrobeDuration
StrobeRate
Shutter_n_(n1)
Shutter_n_Strobe(n1)
Shutter_n_StrobePulse(n1)
Shutter_n_StrobePulseClose(n1)
Shutter_n_StrobePulseOpen(n1)
Shutter_n_StrobeRandom(n1)
Shutter_n_StrobeRandomPulse(n1)
Shutter_n_StrobeRandomPulseClose(n1)
Shutter_n_StrobeRandomPulseOpen(n1)
Shutter_n_StrobeEffect(n1)
Iris
IrisStrobe
IrisStrobeRandom
IrisPulseClose
IrisPulseOpen
IrisRandomPulseClose
IrisRandomPulseOpen
Frost_n_(n1)
Frost_n_PulseOpen(n1)
Frost_n_PulseClose(n1)
Frost_n_Ramp(n1)
Prism_n_(n1)
Prism_n_SelectSpin(n1)
Prism_n_Macro(n1)
Prism_n_Pos(n1)
Prism_n_PosRotate(n1)
Effects_n_(n1)
Effects_n_Rate(n1)
Effects_n_Fade(n1)
Effects_n_Adjust_m_(n1,m1)
Effects_n_Pos(n1)
Effects_n_PosRotate(n1)
EffectsSync
BeamShaper
BeamShaperMacro
BeamShaperPos
BeamShaperPosRotate
Zoom
ZoomModeSpot
ZoomModeBeam
Focus_n_(n1)
Focus_n_Adjust(n1)
Focus_n_Distance(n1)
Control_n_(n1)
DimmerMode
DimmerCurve
BlackoutMode
LEDFrequency
LEDZoneMode
PixelMode
PanMode
TiltMode
PanTiltMode
PositionModes
Gobo_n_WheelMode(n1)
AnimationWheel_n_Mode(n1)
AnimationWheelShortcutMode
Color_n_Mode(n1)
ColorWheelShortcutMode
CyanMode
MagentaMode
YellowMode
ColorMixMode
ChromaticMode
ColorCalibrationMode
ColorConsistency
ColorControl
ColorModelMode
ColorSettingsReset
ColorUniformity
CRIMode
CustomColor
UVStability
WavelengthCorrection
WhiteCount
StrobeMode
ZoomMode
FocusMode
IrisMode
Fan_n_Mode(n1)
FollowSpotMode
BeamEffectIndexRotateMode
IntensityMSpeed
PositionMSpeed
ColorMixMSpeed
ColorWheelSelectMSpeed
GoboWheel_n_MSpeed(n1)
IrisMSpeed
Prism_n_MSpeed(n1)
FocusMSpeed
Frost_n_MSpeed(n1)
ZoomMSpeed
FrameMSpeed
GlobalMSpeed
ReflectorAdjust
FixtureGlobalReset
ShutterReset
BeamReset
ColorMixReset
ColorWheelReset
FocusReset
FrameReset
GoboWheelReset
IntensityReset
IrisReset
PositionReset
PanReset
TiltReset
ZoomReset
CTBReset
CTOReset
CTCReset
AnimationSystemReset
FixtureCalibrationReset
Function
LampControl
DisplayIntensity
DMXInput
NoFeature
Blower_n_(n1)
Fan_n_(n1)
Fog_n_(n1)
Haze_n_(n1)
LampPowerMode
Fans
Blade_n_A(n1)
Blade_n_B(n1)
Blade_n_Rot(n1)
ShaperRot
ShaperMacros
ShaperMacrosSpeed
BladeSoft_n_A(n1)
BladeSoft_n_B(n1)
KeyStone_n_A(n1)
KeyStone_n_B(n1)
Video
VideoEffect_n_Type(n1)
VideoEffect_n_Parameter_m_(n1,m1)
VideoCamera_n_(n1)
VideoSoundVolume_n_(n1)
VideoBlendMode
InputSource
FieldOfView

 */