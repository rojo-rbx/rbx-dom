//! Test to ensure the formatting of files does not change.

use rbx_dom_weak::DomViewer;

const INPUT: &str = r#"<roblox version="4">
    <Item class="TestClass" referent="Parent">
        <Properties>
            <Axes name="TestAxes"><axes>5</axes></Axes>
            <BinaryString name="TestBinaryString">SGVsbG8sIHdvcmxkIQ==</BinaryString>
            <bool name="TestBool">true</bool>
            <CoordinateFrame name="TestCFrame">
                <X>123</X>
                <Y>456</Y>
                <Z>789</Z>
                <R00>987</R00>
                <R01>654</R01>
                <R02>432</R02>
                <R10>210</R10>
                <R11>0</R11>
                <R12>-12345</R12>
                <R20>765</R20>
                <R21>234</R21>
                <R22>123123</R22>
            </CoordinateFrame>
            <ColorSequence name="TestColorSequence">0 0 0.5 1 0 1 1 0.5 0 0 </ColorSequence>
            <Color3 name="TestColor3">
                <R>1</R>
                <G>0.5</G>
                <B>125600</B>
            </Color3>
            <Content name="TestContent1"><url>Wow!</url></Content>
            <Content name="TestContent2"><null></null></Content>
        </Properties>
        <Item class="TestClass" referent="Child1">
            <Properties>
                <double name="TestDouble">INF</double>
                <Faces name="TestFace"><faces>42</faces></Faces>
                <float name="TestFloat">NAN</float>
                <Font name="TestFont">
                    <Family><url>Font Family</url></Family>
                    <Weight>100</Weight>
                    <Style>Normal</Style>
                    <CachedFaceId><null></null></CachedFaceId>
                </Font>
                <int name="TestInt">1337</int>
                <int64 name="TestInt64">8675309</int64>
            </Properties>
        </Item>
        <Item class="TestClass" referent="Child2">
            <Properties>
                <NumberRange name="TestNumberRange">-1337 1337</NumberRange>
                <NumberSequence name="TestNumberSequence">0 10 20 1 30 40</NumberSequence>
                <OptionalCoordinateFrame name="TestOptionCFrame1"></OptionalCoordinateFrame>
                <OptionalCoordinateFrame name="TestOptionCFrame2">
                    <CFrame>
                        <X>100</X>
                        <Y>200</Y>
                        <Z>300</Z>
                        <R00>-100</R00>
                        <R01>-200</R01>
                        <R02>-300</R02>
                        <R10>123</R10>
                        <R11>456</R11>
                        <R12>-123</R12>
                        <R20>-456</R20>
                        <R21>INF</R21>
                        <R22>-INF</R22>
                    </CFrame>
                </OptionalCoordinateFrame>
            </Properties>
            <Item class="TestClass" referent="Grandchild">
                <Properties>
                    <PhysicalProperties name="TestPhysicalProperties1">
                        <CustomPhysics>false</CustomPhysics>
                    </PhysicalProperties>
                    <PhysicalProperties name="TestPhysicalProperties2">
                        <CustomPhysics>true</CustomPhysics>
                        <Density>1</Density>
                        <Friction>-1</Friction>
                        <Elasticity>0.15625</Elasticity>
                        <FrictionWeight>-0.15625</FrictionWeight>
                        <ElasticityWeight>NAN</ElasticityWeight>
                    </PhysicalProperties>
                    <ProtectedString name="TestProtectedString">Hello world, again!</ProtectedString>
                    <Ray name="TestRay">
                        <origin>
                            <X>10</X>
                            <Y>20</Y>
                            <Z>30</Z>
                        </origin>
                        <direction>
                            <X>30</X>
                            <Y>20</Y>
                            <Z>10</Z>
                        </direction>
                    </Ray>
                    <Rect2D name="TestRect">
                        <min>
                            <X>1</X>
                            <Y>2</Y>
                        </min>
                        <max>
                            <X>0.0</X>
                            <Y>INF</Y>
                        </max>
                    </Rect2D>
                    <Ref name="TestRef1">Parent</Ref>
                    <Ref name="TestRef2">null</Ref>
                    <SharedString name="TestSharedString">TestHash</SharedString>
                    <string name="TestString">Hello, world!</string>
                    <token name="TestEnum">1337</token>
                    <UDim name="TestUDim">
                        <S>1234.5</S>
                        <O>-123</O>
                    </UDim>
                    <UDim2 name="TestUDim2">
                        <XS>1234.5</XS>
                        <XO>-123</XO>
                        <YS>-1234.5</YS>
                        <YO>123</YO>
                    </UDim2>
                    <UniqueId name="TestUniqueId1">00000000000000000000000000000000</UniqueId>
                    <UniqueId name="TestUniqueId2">1234567890abcdef00c0ffeebadf00d0</UniqueId>
                    <Vector2 name="TestVector2">
                        <X>INF</X>
                        <Y>0</Y>
                    </Vector2>
                    <Vector3 name="TestVector3">
                        <X>0</X>
                        <Y>INF</Y>
                        <Z>123</Z>
                    </Vector3>
                    <Vector3int16 name="TestVector3int16">
                        <X>-10</X>
                        <Y>0</Y>
                        <Z>10</Z>
                    </Vector3int16>
                </Properties>
            </Item>
        </Item>
    </Item>
    <SharedStrings>
        <SharedString md5="TestHash">SGVsbG8sIHdvcmxkIQ==</SharedString>
    </SharedStrings>
</roblox>"#;

#[test]
fn formatting() {
    let _ = env_logger::try_init();

    let de = crate::from_str(
        INPUT,
        crate::DecodeOptions::new().property_behavior(crate::DecodePropertyBehavior::NoReflection),
    )
    .map_err(|e| panic!("cannot deserialize: {}", e))
    .unwrap();

    insta::assert_yaml_snapshot!("deserialized", DomViewer::new().view_children(&de));

    let mut ser = Vec::with_capacity(INPUT.len());
    crate::to_writer(
        &mut ser,
        &de,
        de.root().children(),
        crate::EncodeOptions::new().property_behavior(crate::EncodePropertyBehavior::NoReflection),
    )
    .map_err(|e| panic!("cannot serialize: {}", e))
    .unwrap();

    let ser_str = String::from_utf8(ser)
        .map_err(|e| panic!("serialized result not string: {}", e))
        .unwrap();

    insta::assert_snapshot!("serialized", ser_str)
}
