#![cfg(test)]

use crate::fixture_type::dmx_mode::DMXMode;
use std::convert::TryInto;

pub fn expect() -> Vec<DMXMode> {
    vec![
        DMXMode {
            geometry: "Base".try_into().unwrap(),
            name: "Mode 1 12 DMX".try_into().unwrap(),
            dmx_channels: vec![]
        }
    ]
}
/*      <DMXModes>
      <DMXMode Geometry="Base" Name="Mode 1 12 DMX">
      <DMXChannels>
      <DMXChannel DMXBreak="1" Default="32768/2" Geometry="Yoke" Highlight="None" Offset="1,2">
      <LogicalChannel Attribute="Pan" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="Pan" DMXFrom="0/2" Name="Pan 1" OriginalAttribute="" PhysicalFrom="270.000000" PhysicalTo="-270.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="0/2" Name="Min" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="1/2" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="32768/2" Name="Center" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="32769/2" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="65535/2" Name="Max" WheelSlotIndex="0"/>
      </ChannelFunction>
      </LogicalChannel>
      </DMXChannel>
      <DMXChannel DMXBreak="1" Default="32767/2" Geometry="Head" Highlight="None" Offset="3,4">
      <LogicalChannel Attribute="Tilt" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="Tilt" DMXFrom="0/2" Name="Tilt 1" OriginalAttribute="" PhysicalFrom="-125.000000" PhysicalTo="125.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="0/2" Name="Min" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="1/2" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="32767/2" Name="Center" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="32768/2" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="65535/2" Name="Max" WheelSlotIndex="0"/>
      </ChannelFunction>
      </LogicalChannel>
      </DMXChannel>
      <DMXChannel DMXBreak="1" Default="0/1" Geometry="Head" Highlight="None" Offset="5">
      <LogicalChannel Attribute="PT Speed" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="PT Speed" DMXFrom="0/1" Name="PT Speed 1" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealFade="0.000000"/>
      </LogicalChannel>
      </DMXChannel>
      <DMXChannel DMXBreak="1" Default="0/1" Geometry="Head" Highlight="255/1" Offset="6">
      <LogicalChannel Attribute="Dimmer" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="Dimmer" DMXFrom="0/1" Name="Dimmer 1" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="0/1" Name="Closed" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="1/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="255/1" Name="Open" WheelSlotIndex="0"/>
      </ChannelFunction>
      </LogicalChannel>
      </DMXChannel>
      <DMXChannel DMXBreak="1" Default="12/1" Geometry="Head" Highlight="12/1" Offset="7">
      <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="Shutter1" DMXFrom="0/1" Name="Shutter1 1" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="0/1" Name="OFF" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="8/1" Name="Open" PhysicalFrom="1.000000" PhysicalTo="1.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Shutter1Strobe" DMXFrom="16/1" Name="Strobe" OriginalAttribute="" PhysicalFrom="0.900000" PhysicalTo="12.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="16/1" Name="min Strobe" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="17/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="131/1" Name="max Strobe" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Shutter1" DMXFrom="132/1" Name="Open2" OriginalAttribute="" PhysicalFrom="1.000000" PhysicalTo="1.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="132/1" Name="Open 2" PhysicalFrom="1.000000" PhysicalTo="1.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Shutter1StrobePulseOpen" DMXFrom="140/1" Name="PulseOpen" OriginalAttribute="" PhysicalFrom="0.900000" PhysicalTo="5.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="140/1" Name="min PulseOpen" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="141/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="181/1" Name="max PulseOpen" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Shutter1" DMXFrom="182/1" Name="Open3" OriginalAttribute="" PhysicalFrom="1.000000" PhysicalTo="1.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="182/1" Name="Open 3" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Shutter1StrobePulseClose" DMXFrom="190/1" Name="PulseClose" OriginalAttribute="" PhysicalFrom="0.900000" PhysicalTo="5.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="190/1" Name="min PulseClose" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="191/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="231/1" Name="max PulseClose" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Shutter1" DMXFrom="232/1" Name="Open4" OriginalAttribute="" PhysicalFrom="1.000000" PhysicalTo="1.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="232/1" Name="Open4" PhysicalFrom="1.000000" PhysicalTo="1.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Shutter1StrobeRandom" DMXFrom="240/1" Name="Strobe Ran" OriginalAttribute="" PhysicalFrom="10.000000" PhysicalTo="12.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="240/1" Name="Strobe Rnd" PhysicalFrom="10.000000" PhysicalTo="12.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Shutter1" DMXFrom="248/1" Name="Open 5" OriginalAttribute="" PhysicalFrom="1.000000" PhysicalTo="1.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="248/1" Name="Open 2" PhysicalFrom="1.000000" PhysicalTo="1.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      </LogicalChannel>
      </DMXChannel>
      <DMXChannel DMXBreak="1" Default="0/1" Geometry="Head" Highlight="0/1" Offset="8">
      <LogicalChannel Attribute="Color1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="Color1" DMXFrom="0/1" Name="Color1 1" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000" Wheel="Color1">
      <ChannelSet DMXFrom="0/1" Name="Open" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="1"/>
      <ChannelSet DMXFrom="9/1" Name="Color 1" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="2"/>
      <ChannelSet DMXFrom="17/1" Name="Color 2" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="3"/>
      <ChannelSet DMXFrom="26/1" Name="Color 3" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="4"/>
      <ChannelSet DMXFrom="34/1" Name="Color 4" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="5"/>
      <ChannelSet DMXFrom="43/1" Name="Color 5" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="6"/>
      <ChannelSet DMXFrom="51/1" Name="Color 6" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="7"/>
      <ChannelSet DMXFrom="60/1" Name="Color 7" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="8"/>
      <ChannelSet DMXFrom="68/1" Name="Color 8" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="9"/>
      <ChannelSet DMXFrom="77/1" Name="Color 9" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="10"/>
      <ChannelSet DMXFrom="85/1" Name="Color 10" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="11"/>
      <ChannelSet DMXFrom="94/1" Name="Color 11" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="12"/>
      <ChannelSet DMXFrom="102/1" Name="Color 12" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="13"/>
      <ChannelSet DMXFrom="111/1" Name="Color 13" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="14"/>
      <ChannelSet DMXFrom="120/1" Name="Color 14" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="15"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Color1WheelSpin" DMXFrom="128/1" Name="Color Spin" OriginalAttribute="" PhysicalFrom="120.000000" PhysicalTo="1.000000" RealFade="0.000000" Wheel="Color1">
      <ChannelSet DMXFrom="128/1" Name="max CCW" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="129/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="189/1" Name="min CCW" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Color1WheelSpin" DMXFrom="190/1" Name="Stop" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000" Wheel="Color1">
      <ChannelSet DMXFrom="190/1" Name="Stop" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Color1WheelSpin" DMXFrom="194/1" Name="Color Spin 1" OriginalAttribute="" PhysicalFrom="1.000000" PhysicalTo="120.000000" RealFade="0.000000" Wheel="Color1">
      <ChannelSet DMXFrom="194/1" Name="min CCW" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="195/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="255/1" Name="max CCW" WheelSlotIndex="0"/>
      </ChannelFunction>
      </LogicalChannel>
      </DMXChannel>
      <DMXChannel DMXBreak="1" Default="0/1" Geometry="Head" Highlight="0/1" Offset="9">
      <LogicalChannel Attribute="Gobo1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="Gobo1" DMXFrom="0/1" Name="Gobo1 1" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000" Wheel="Gobo1">
      <ChannelSet DMXFrom="0/1" Name="Open" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="1"/>
      <ChannelSet DMXFrom="4/1" Name="Gobo 1" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="2"/>
      <ChannelSet DMXFrom="8/1" Name="Gobo 2" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="3"/>
      <ChannelSet DMXFrom="12/1" Name="Gobo 3" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="4"/>
      <ChannelSet DMXFrom="16/1" Name="Gobo 4" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="5"/>
      <ChannelSet DMXFrom="20/1" Name="Gobo 5" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="6"/>
      <ChannelSet DMXFrom="24/1" Name="Gobo 6" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="7"/>
      <ChannelSet DMXFrom="28/1" Name="Gobo 7" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="8"/>
      <ChannelSet DMXFrom="32/1" Name="Gobo 8" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="9"/>
      <ChannelSet DMXFrom="36/1" Name="Gobo 9" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="10"/>
      <ChannelSet DMXFrom="40/1" Name="Gobo 10" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="11"/>
      <ChannelSet DMXFrom="44/1" Name="Gobo 11" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="12"/>
      <ChannelSet DMXFrom="48/1" Name="Gobo 12" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="13"/>
      <ChannelSet DMXFrom="52/1" Name="Gobo 13" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="14"/>
      <ChannelSet DMXFrom="56/1" Name="Gobo 14" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="15"/>
      <ChannelSet DMXFrom="60/1" Name="Gobo 15" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="16"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Gobo1SelectShake" DMXFrom="64/1" Name="NoFeature 2" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000" Wheel="Gobo1">
      <ChannelSet DMXFrom="64/1" Name="Gobo 1 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="2"/>
      <ChannelSet DMXFrom="69/1" Name="Gobo 2 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="3"/>
      <ChannelSet DMXFrom="73/1" Name="Gobo 3 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="4"/>
      <ChannelSet DMXFrom="77/1" Name="Gobo 4 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="5"/>
      <ChannelSet DMXFrom="81/1" Name="Gobo 5 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="6"/>
      <ChannelSet DMXFrom="85/1" Name="Gobo 6 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="7"/>
      <ChannelSet DMXFrom="90/1" Name="Gobo 7 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="8"/>
      <ChannelSet DMXFrom="94/1" Name="Gobo 8 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="9"/>
      <ChannelSet DMXFrom="98/1" Name="Gobo 9 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="10"/>
      <ChannelSet DMXFrom="102/1" Name="Gobo 10 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="11"/>
      <ChannelSet DMXFrom="106/1" Name="Gobo 11 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="12"/>
      <ChannelSet DMXFrom="111/1" Name="Gobo 12 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="13"/>
      <ChannelSet DMXFrom="115/1" Name="Gobo 13 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="14"/>
      <ChannelSet DMXFrom="119/1" Name="Gobo 14 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="15"/>
      <ChannelSet DMXFrom="123/1" Name="Gobo 15 Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="16"/>
      <ChannelSet DMXFrom="127/1" Name="Open Shake" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="1"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Gobo1WheelSpin" DMXFrom="128/1" Name="Gobo1WheelSpin" OriginalAttribute="" PhysicalFrom="80.000000" PhysicalTo="1.000000" RealFade="0.000000" Wheel="Gobo1">
      <ChannelSet DMXFrom="128/1" Name="max CCW" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="129/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="189/1" Name="min CCW" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Gobo1WheelSpin" DMXFrom="190/1" Name="Stop" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000" Wheel="Gobo1">
      <ChannelSet DMXFrom="190/1" Name="Stop" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Gobo1WheelSpin" DMXFrom="194/1" Name="NoFeature 5" OriginalAttribute="" PhysicalFrom="-1.000000" PhysicalTo="-80.000000" RealFade="0.000000" Wheel="Gobo1">
      <ChannelSet DMXFrom="194/1" Name="min CW" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="195/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="255/1" Name="max CW" WheelSlotIndex="0"/>
      </ChannelFunction>
      </LogicalChannel>
      </DMXChannel>
      <DMXChannel DMXBreak="1" Default="0/1" Geometry="Head" Highlight="0/1" Offset="10">
      <LogicalChannel Attribute="Prism1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="Prism1" DMXFrom="0/1" Name="Open" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000" Wheel="Prism 1">
      <ChannelSet DMXFrom="0/1" Name="Open" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="1"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Prism1" DMXFrom="8/1" Name="Prism 1" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000" Wheel="Prism 1">
      <ChannelSet DMXFrom="8/1" Name="Prism 1" PhysicalFrom="1.000000" PhysicalTo="1.000000" WheelSlotIndex="2"/>
      </ChannelFunction>
      </LogicalChannel>
      </DMXChannel>
      <DMXChannel DMXBreak="1" Default="0/1" Geometry="Head" Highlight="0/1" Offset="11">
      <LogicalChannel Attribute="Prism1Pos" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="Prism1Pos" DMXFrom="0/1" Name="Prism1Pos 1" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealFade="0.000000" Wheel="Prism 1">
      <ChannelSet DMXFrom="0/1" Name="Open" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="1/1" Name="Prism1Index" PhysicalFrom="1.000000" PhysicalTo="360.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Prism1PosRotate" DMXFrom="128/1" Name="Prism 1 Rot" OriginalAttribute="" PhysicalFrom="80.000000" PhysicalTo="1.000000" RealFade="0.000000" Wheel="Prism 1">
      <ChannelSet DMXFrom="128/1" Name="max CCW" PhysicalFrom="80.000000" PhysicalTo="80.000000" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="129/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="189/1" Name="min CCW" PhysicalFrom="1.000000" PhysicalTo="1.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Prism1PosRotate" DMXFrom="190/1" Name="NoFeature 3" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000" Wheel="Prism 1">
      <ChannelSet DMXFrom="190/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="191/1" Name="Stop" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="193/1" Name="" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Prism1PosRotate" DMXFrom="194/1" Name="NoFeature 4" OriginalAttribute="" PhysicalFrom="-1.000000" PhysicalTo="-80.000000" RealFade="0.000000" Wheel="Prism 1">
      <ChannelSet DMXFrom="194/1" Name="min CW" PhysicalFrom="-1.000000" PhysicalTo="-1.000000" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="195/1" Name="" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="255/1" Name="max CW" PhysicalFrom="-80.000000" PhysicalTo="-80.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      </LogicalChannel>
      </DMXChannel>
      <DMXChannel DMXBreak="1" Default="0/1" Geometry="Head" Highlight="None" Offset="12">
      <LogicalChannel Attribute="LampControl" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">
      <ChannelFunction Attribute="Function" DMXFrom="0/1" Name="Control " OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="0/1" Name="NoFeature 1" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="50/1" Name="" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="PositionReset" DMXFrom="80/1" Name="Reset PT " OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="80/1" Name="Reset PT" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="ColorMixReset" DMXFrom="85/1" Name="Reset Color" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="85/1" Name="Reset Color" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="GoboWheelReset" DMXFrom="90/1" Name="Reset Gobo" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="90/1" Name="Reset Gobo" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="ShutterReset" DMXFrom="95/1" Name="Reset Shutter 1" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="95/1" Name="Reset Shutter" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Function" DMXFrom="100/1" Name="Function" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="100/1" Name="BO PT" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="110/1" Name="BO Color" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      <ChannelSet DMXFrom="120/1" Name="BO Gobo" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Lamp On" DMXFrom="130/1" Name="Lamp On" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="130/1" Name="Lamp On" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="NoFeature" DMXFrom="140/1" Name="NoFeature 8" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="140/1" Name="NoFeature2" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="FixtureGlobalReset" DMXFrom="200/1" Name="Reset All" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="200/1" Name="Reset All" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="NoFeature" DMXFrom="210/1" Name="NoFeature 10" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="210/1" Name="NoFeature 3" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Lamp Off" DMXFrom="230/1" Name="Lamp Off" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="230/1" Name="Lamp Off" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="NoFeature" DMXFrom="240/1" Name="NoFeature 3" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="240/1" Name="NoFeature 3" PhysicalFrom="0.000000" PhysicalTo="0.000000" WheelSlotIndex="0"/>
      </ChannelFunction>
      <ChannelFunction Attribute="Sound" DMXFrom="250/1" Name="Sound" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="0.000000" RealFade="0.000000">
      <ChannelSet DMXFrom="250/1" Name="Sound" WheelSlotIndex="0"/>
      </ChannelFunction>
      </LogicalChannel>
      </DMXChannel>
      </DMXChannels>
      <Relations/>
      <FTMacros/>
      </DMXMode>
      </DMXModes>*/
