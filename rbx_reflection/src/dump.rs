# ! [ allow ( unused_mut ) ]use crate::types::*;
use std::collections::HashMap;
pub fn generate_classes() -> HashMap<&'static str, RbxInstanceClass> {
    let mut output = HashMap::new();
    output.insert(
        "Instance",
        RbxInstanceClass {
            name: "Instance",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Archivable",
                    RbxInstanceProperty {
                        name: "Archivable",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ClassName",
                    RbxInstanceProperty {
                        name: "ClassName",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "DataCost",
                    RbxInstanceProperty {
                        name: "DataCost",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Name",
                    RbxInstanceProperty {
                        name: "Name",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Parent",
                    RbxInstanceProperty {
                        name: "Parent",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "RobloxLocked",
                    RbxInstanceProperty {
                        name: "RobloxLocked",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "archivable",
                    RbxInstanceProperty {
                        name: "archivable",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "className",
                    RbxInstanceProperty {
                        name: "className",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ABTestService",
        RbxInstanceClass {
            name: "ABTestService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Accoutrement",
        RbxInstanceClass {
            name: "Accoutrement",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AttachmentForward",
                    RbxInstanceProperty {
                        name: "AttachmentForward",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "AttachmentPoint",
                    RbxInstanceProperty {
                        name: "AttachmentPoint",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "AttachmentPos",
                    RbxInstanceProperty {
                        name: "AttachmentPos",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "AttachmentRight",
                    RbxInstanceProperty {
                        name: "AttachmentRight",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "AttachmentUp",
                    RbxInstanceProperty {
                        name: "AttachmentUp",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Accessory",
        RbxInstanceClass {
            name: "Accessory",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Hat",
        RbxInstanceClass {
            name: "Hat",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "AdService",
        RbxInstanceClass {
            name: "AdService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "AdvancedDragger",
        RbxInstanceClass {
            name: "AdvancedDragger",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "AnalyticsService",
        RbxInstanceClass {
            name: "AnalyticsService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Animation",
        RbxInstanceClass {
            name: "Animation",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AnimationId",
                    RbxInstanceProperty {
                        name: "AnimationId",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "AnimationController",
        RbxInstanceClass {
            name: "AnimationController",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "AnimationTrack",
        RbxInstanceClass {
            name: "AnimationTrack",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Animation",
                    RbxInstanceProperty {
                        name: "Animation",
                        value_type: "Animation",
                    },
                );
                properties.insert(
                    "IsPlaying",
                    RbxInstanceProperty {
                        name: "IsPlaying",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Length",
                    RbxInstanceProperty {
                        name: "Length",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Looped",
                    RbxInstanceProperty {
                        name: "Looped",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Priority",
                    RbxInstanceProperty {
                        name: "Priority",
                        value_type: "AnimationPriority",
                    },
                );
                properties.insert(
                    "Speed",
                    RbxInstanceProperty {
                        name: "Speed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TimePosition",
                    RbxInstanceProperty {
                        name: "TimePosition",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WeightCurrent",
                    RbxInstanceProperty {
                        name: "WeightCurrent",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WeightTarget",
                    RbxInstanceProperty {
                        name: "WeightTarget",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Animator",
        RbxInstanceClass {
            name: "Animator",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "AssetService",
        RbxInstanceClass {
            name: "AssetService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Attachment",
        RbxInstanceClass {
            name: "Attachment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Axis",
                    RbxInstanceProperty {
                        name: "Axis",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "CFrame",
                    RbxInstanceProperty {
                        name: "CFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "Orientation",
                    RbxInstanceProperty {
                        name: "Orientation",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Position",
                    RbxInstanceProperty {
                        name: "Position",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Rotation",
                    RbxInstanceProperty {
                        name: "Rotation",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "SecondaryAxis",
                    RbxInstanceProperty {
                        name: "SecondaryAxis",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Visible",
                    RbxInstanceProperty {
                        name: "Visible",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "WorldAxis",
                    RbxInstanceProperty {
                        name: "WorldAxis",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "WorldCFrame",
                    RbxInstanceProperty {
                        name: "WorldCFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "WorldOrientation",
                    RbxInstanceProperty {
                        name: "WorldOrientation",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "WorldPosition",
                    RbxInstanceProperty {
                        name: "WorldPosition",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "WorldRotation",
                    RbxInstanceProperty {
                        name: "WorldRotation",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "WorldSecondaryAxis",
                    RbxInstanceProperty {
                        name: "WorldSecondaryAxis",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BadgeService",
        RbxInstanceClass {
            name: "BadgeService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BasePlayerGui",
        RbxInstanceClass {
            name: "BasePlayerGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CoreGui",
        RbxInstanceClass {
            name: "CoreGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "SelectionImageObject",
                    RbxInstanceProperty {
                        name: "SelectionImageObject",
                        value_type: "GuiObject",
                    },
                );
                properties.insert(
                    "Version",
                    RbxInstanceProperty {
                        name: "Version",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PlayerGui",
        RbxInstanceClass {
            name: "PlayerGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CurrentScreenOrientation",
                    RbxInstanceProperty {
                        name: "CurrentScreenOrientation",
                        value_type: "ScreenOrientation",
                    },
                );
                properties.insert(
                    "ScreenOrientation",
                    RbxInstanceProperty {
                        name: "ScreenOrientation",
                        value_type: "ScreenOrientation",
                    },
                );
                properties.insert(
                    "SelectionImageObject",
                    RbxInstanceProperty {
                        name: "SelectionImageObject",
                        value_type: "GuiObject",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "StarterGui",
        RbxInstanceClass {
            name: "StarterGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ProcessUserInput",
                    RbxInstanceProperty {
                        name: "ProcessUserInput",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ResetPlayerGuiOnSpawn",
                    RbxInstanceProperty {
                        name: "ResetPlayerGuiOnSpawn",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ScreenOrientation",
                    RbxInstanceProperty {
                        name: "ScreenOrientation",
                        value_type: "ScreenOrientation",
                    },
                );
                properties.insert(
                    "ShowDevelopmentGui",
                    RbxInstanceProperty {
                        name: "ShowDevelopmentGui",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Beam",
        RbxInstanceClass {
            name: "Beam",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Attachment0",
                    RbxInstanceProperty {
                        name: "Attachment0",
                        value_type: "Attachment",
                    },
                );
                properties.insert(
                    "Attachment1",
                    RbxInstanceProperty {
                        name: "Attachment1",
                        value_type: "Attachment",
                    },
                );
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "ColorSequence",
                    },
                );
                properties.insert(
                    "CurveSize0",
                    RbxInstanceProperty {
                        name: "CurveSize0",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "CurveSize1",
                    RbxInstanceProperty {
                        name: "CurveSize1",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "FaceCamera",
                    RbxInstanceProperty {
                        name: "FaceCamera",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LightEmission",
                    RbxInstanceProperty {
                        name: "LightEmission",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LightInfluence",
                    RbxInstanceProperty {
                        name: "LightInfluence",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Segments",
                    RbxInstanceProperty {
                        name: "Segments",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Texture",
                    RbxInstanceProperty {
                        name: "Texture",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "TextureLength",
                    RbxInstanceProperty {
                        name: "TextureLength",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextureMode",
                    RbxInstanceProperty {
                        name: "TextureMode",
                        value_type: "TextureMode",
                    },
                );
                properties.insert(
                    "TextureSpeed",
                    RbxInstanceProperty {
                        name: "TextureSpeed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Transparency",
                    RbxInstanceProperty {
                        name: "Transparency",
                        value_type: "NumberSequence",
                    },
                );
                properties.insert(
                    "Width0",
                    RbxInstanceProperty {
                        name: "Width0",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Width1",
                    RbxInstanceProperty {
                        name: "Width1",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ZOffset",
                    RbxInstanceProperty {
                        name: "ZOffset",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BindableEvent",
        RbxInstanceClass {
            name: "BindableEvent",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BindableFunction",
        RbxInstanceClass {
            name: "BindableFunction",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BodyMover",
        RbxInstanceClass {
            name: "BodyMover",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BodyAngularVelocity",
        RbxInstanceClass {
            name: "BodyAngularVelocity",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AngularVelocity",
                    RbxInstanceProperty {
                        name: "AngularVelocity",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "MaxTorque",
                    RbxInstanceProperty {
                        name: "MaxTorque",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "P",
                    RbxInstanceProperty {
                        name: "P",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "angularvelocity",
                    RbxInstanceProperty {
                        name: "angularvelocity",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "maxTorque",
                    RbxInstanceProperty {
                        name: "maxTorque",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BodyForce",
        RbxInstanceClass {
            name: "BodyForce",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Force",
                    RbxInstanceProperty {
                        name: "Force",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "force",
                    RbxInstanceProperty {
                        name: "force",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BodyGyro",
        RbxInstanceClass {
            name: "BodyGyro",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CFrame",
                    RbxInstanceProperty {
                        name: "CFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "D",
                    RbxInstanceProperty {
                        name: "D",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxTorque",
                    RbxInstanceProperty {
                        name: "MaxTorque",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "P",
                    RbxInstanceProperty {
                        name: "P",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "cframe",
                    RbxInstanceProperty {
                        name: "cframe",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "maxTorque",
                    RbxInstanceProperty {
                        name: "maxTorque",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BodyPosition",
        RbxInstanceClass {
            name: "BodyPosition",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "D",
                    RbxInstanceProperty {
                        name: "D",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxForce",
                    RbxInstanceProperty {
                        name: "MaxForce",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "P",
                    RbxInstanceProperty {
                        name: "P",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Position",
                    RbxInstanceProperty {
                        name: "Position",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "maxForce",
                    RbxInstanceProperty {
                        name: "maxForce",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "position",
                    RbxInstanceProperty {
                        name: "position",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BodyThrust",
        RbxInstanceClass {
            name: "BodyThrust",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Force",
                    RbxInstanceProperty {
                        name: "Force",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Location",
                    RbxInstanceProperty {
                        name: "Location",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "force",
                    RbxInstanceProperty {
                        name: "force",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "location",
                    RbxInstanceProperty {
                        name: "location",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BodyVelocity",
        RbxInstanceClass {
            name: "BodyVelocity",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "MaxForce",
                    RbxInstanceProperty {
                        name: "MaxForce",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "P",
                    RbxInstanceProperty {
                        name: "P",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Velocity",
                    RbxInstanceProperty {
                        name: "Velocity",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "maxForce",
                    RbxInstanceProperty {
                        name: "maxForce",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "velocity",
                    RbxInstanceProperty {
                        name: "velocity",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "RocketPropulsion",
        RbxInstanceClass {
            name: "RocketPropulsion",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CartoonFactor",
                    RbxInstanceProperty {
                        name: "CartoonFactor",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxSpeed",
                    RbxInstanceProperty {
                        name: "MaxSpeed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxThrust",
                    RbxInstanceProperty {
                        name: "MaxThrust",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxTorque",
                    RbxInstanceProperty {
                        name: "MaxTorque",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Target",
                    RbxInstanceProperty {
                        name: "Target",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "TargetOffset",
                    RbxInstanceProperty {
                        name: "TargetOffset",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "TargetRadius",
                    RbxInstanceProperty {
                        name: "TargetRadius",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ThrustD",
                    RbxInstanceProperty {
                        name: "ThrustD",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ThrustP",
                    RbxInstanceProperty {
                        name: "ThrustP",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TurnD",
                    RbxInstanceProperty {
                        name: "TurnD",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TurnP",
                    RbxInstanceProperty {
                        name: "TurnP",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BrowserService",
        RbxInstanceClass {
            name: "BrowserService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CacheableContentProvider",
        RbxInstanceClass {
            name: "CacheableContentProvider",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "MeshContentProvider",
        RbxInstanceClass {
            name: "MeshContentProvider",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "SolidModelContentProvider",
        RbxInstanceClass {
            name: "SolidModelContentProvider",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Camera",
        RbxInstanceClass {
            name: "Camera",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CFrame",
                    RbxInstanceProperty {
                        name: "CFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "CameraSubject",
                    RbxInstanceProperty {
                        name: "CameraSubject",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "CameraType",
                    RbxInstanceProperty {
                        name: "CameraType",
                        value_type: "CameraType",
                    },
                );
                properties.insert(
                    "CoordinateFrame",
                    RbxInstanceProperty {
                        name: "CoordinateFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "FieldOfView",
                    RbxInstanceProperty {
                        name: "FieldOfView",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Focus",
                    RbxInstanceProperty {
                        name: "Focus",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "HeadLocked",
                    RbxInstanceProperty {
                        name: "HeadLocked",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "HeadScale",
                    RbxInstanceProperty {
                        name: "HeadScale",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "NearPlaneZ",
                    RbxInstanceProperty {
                        name: "NearPlaneZ",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ViewportSize",
                    RbxInstanceProperty {
                        name: "ViewportSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "focus",
                    RbxInstanceProperty {
                        name: "focus",
                        value_type: "CFrame",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ChangeHistoryService",
        RbxInstanceClass {
            name: "ChangeHistoryService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CharacterAppearance",
        RbxInstanceClass {
            name: "CharacterAppearance",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BodyColors",
        RbxInstanceClass {
            name: "BodyColors",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "HeadColor",
                    RbxInstanceProperty {
                        name: "HeadColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "HeadColor3",
                    RbxInstanceProperty {
                        name: "HeadColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "LeftArmColor",
                    RbxInstanceProperty {
                        name: "LeftArmColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "LeftArmColor3",
                    RbxInstanceProperty {
                        name: "LeftArmColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "LeftLegColor",
                    RbxInstanceProperty {
                        name: "LeftLegColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "LeftLegColor3",
                    RbxInstanceProperty {
                        name: "LeftLegColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "RightArmColor",
                    RbxInstanceProperty {
                        name: "RightArmColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "RightArmColor3",
                    RbxInstanceProperty {
                        name: "RightArmColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "RightLegColor",
                    RbxInstanceProperty {
                        name: "RightLegColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "RightLegColor3",
                    RbxInstanceProperty {
                        name: "RightLegColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "TorsoColor",
                    RbxInstanceProperty {
                        name: "TorsoColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "TorsoColor3",
                    RbxInstanceProperty {
                        name: "TorsoColor3",
                        value_type: "Color3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "CharacterMesh",
        RbxInstanceClass {
            name: "CharacterMesh",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "BaseTextureId",
                    RbxInstanceProperty {
                        name: "BaseTextureId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "BodyPart",
                    RbxInstanceProperty {
                        name: "BodyPart",
                        value_type: "BodyPart",
                    },
                );
                properties.insert(
                    "MeshId",
                    RbxInstanceProperty {
                        name: "MeshId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "OverlayTextureId",
                    RbxInstanceProperty {
                        name: "OverlayTextureId",
                        value_type: "int64",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Clothing",
        RbxInstanceClass {
            name: "Clothing",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Color3",
                    RbxInstanceProperty {
                        name: "Color3",
                        value_type: "Color3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Pants",
        RbxInstanceClass {
            name: "Pants",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "PantsTemplate",
                    RbxInstanceProperty {
                        name: "PantsTemplate",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Shirt",
        RbxInstanceClass {
            name: "Shirt",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ShirtTemplate",
                    RbxInstanceProperty {
                        name: "ShirtTemplate",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ShirtGraphic",
        RbxInstanceClass {
            name: "ShirtGraphic",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Color3",
                    RbxInstanceProperty {
                        name: "Color3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Graphic",
                    RbxInstanceProperty {
                        name: "Graphic",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Skin",
        RbxInstanceClass {
            name: "Skin",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "SkinColor",
                    RbxInstanceProperty {
                        name: "SkinColor",
                        value_type: "BrickColor",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Chat",
        RbxInstanceClass {
            name: "Chat",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "BubbleChatEnabled",
                    RbxInstanceProperty {
                        name: "BubbleChatEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LoadDefaultChat",
                    RbxInstanceProperty {
                        name: "LoadDefaultChat",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ClickDetector",
        RbxInstanceClass {
            name: "ClickDetector",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CursorIcon",
                    RbxInstanceProperty {
                        name: "CursorIcon",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "MaxActivationDistance",
                    RbxInstanceProperty {
                        name: "MaxActivationDistance",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ClusterPacketCache",
        RbxInstanceClass {
            name: "ClusterPacketCache",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CollectionService",
        RbxInstanceClass {
            name: "CollectionService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Configuration",
        RbxInstanceClass {
            name: "Configuration",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Constraint",
        RbxInstanceClass {
            name: "Constraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Active",
                    RbxInstanceProperty {
                        name: "Active",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Attachment0",
                    RbxInstanceProperty {
                        name: "Attachment0",
                        value_type: "Attachment",
                    },
                );
                properties.insert(
                    "Attachment1",
                    RbxInstanceProperty {
                        name: "Attachment1",
                        value_type: "Attachment",
                    },
                );
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Visible",
                    RbxInstanceProperty {
                        name: "Visible",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "AlignOrientation",
        RbxInstanceClass {
            name: "AlignOrientation",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AlignType",
                    RbxInstanceProperty {
                        name: "AlignType",
                        value_type: "AlignType",
                    },
                );
                properties.insert(
                    "MaxAngularVelocity",
                    RbxInstanceProperty {
                        name: "MaxAngularVelocity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxTorque",
                    RbxInstanceProperty {
                        name: "MaxTorque",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "PrimaryAxisOnly",
                    RbxInstanceProperty {
                        name: "PrimaryAxisOnly",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ReactionTorqueEnabled",
                    RbxInstanceProperty {
                        name: "ReactionTorqueEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Responsiveness",
                    RbxInstanceProperty {
                        name: "Responsiveness",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "RigidityEnabled",
                    RbxInstanceProperty {
                        name: "RigidityEnabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "AlignPosition",
        RbxInstanceClass {
            name: "AlignPosition",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ApplyAtCenterOfMass",
                    RbxInstanceProperty {
                        name: "ApplyAtCenterOfMass",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MaxForce",
                    RbxInstanceProperty {
                        name: "MaxForce",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxVelocity",
                    RbxInstanceProperty {
                        name: "MaxVelocity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ReactionForceEnabled",
                    RbxInstanceProperty {
                        name: "ReactionForceEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Responsiveness",
                    RbxInstanceProperty {
                        name: "Responsiveness",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "RigidityEnabled",
                    RbxInstanceProperty {
                        name: "RigidityEnabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BallSocketConstraint",
        RbxInstanceClass {
            name: "BallSocketConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "LimitsEnabled",
                    RbxInstanceProperty {
                        name: "LimitsEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Radius",
                    RbxInstanceProperty {
                        name: "Radius",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Restitution",
                    RbxInstanceProperty {
                        name: "Restitution",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TwistLimitsEnabled",
                    RbxInstanceProperty {
                        name: "TwistLimitsEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TwistLowerAngle",
                    RbxInstanceProperty {
                        name: "TwistLowerAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TwistUpperAngle",
                    RbxInstanceProperty {
                        name: "TwistUpperAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "UpperAngle",
                    RbxInstanceProperty {
                        name: "UpperAngle",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "HingeConstraint",
        RbxInstanceClass {
            name: "HingeConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ActuatorType",
                    RbxInstanceProperty {
                        name: "ActuatorType",
                        value_type: "ActuatorType",
                    },
                );
                properties.insert(
                    "AngularSpeed",
                    RbxInstanceProperty {
                        name: "AngularSpeed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "AngularVelocity",
                    RbxInstanceProperty {
                        name: "AngularVelocity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "CurrentAngle",
                    RbxInstanceProperty {
                        name: "CurrentAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LimitsEnabled",
                    RbxInstanceProperty {
                        name: "LimitsEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LowerAngle",
                    RbxInstanceProperty {
                        name: "LowerAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MotorMaxAcceleration",
                    RbxInstanceProperty {
                        name: "MotorMaxAcceleration",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MotorMaxTorque",
                    RbxInstanceProperty {
                        name: "MotorMaxTorque",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Radius",
                    RbxInstanceProperty {
                        name: "Radius",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Restitution",
                    RbxInstanceProperty {
                        name: "Restitution",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ServoMaxTorque",
                    RbxInstanceProperty {
                        name: "ServoMaxTorque",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TargetAngle",
                    RbxInstanceProperty {
                        name: "TargetAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "UpperAngle",
                    RbxInstanceProperty {
                        name: "UpperAngle",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LineForce",
        RbxInstanceClass {
            name: "LineForce",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ApplyAtCenterOfMass",
                    RbxInstanceProperty {
                        name: "ApplyAtCenterOfMass",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "InverseSquareLaw",
                    RbxInstanceProperty {
                        name: "InverseSquareLaw",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Magnitude",
                    RbxInstanceProperty {
                        name: "Magnitude",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxForce",
                    RbxInstanceProperty {
                        name: "MaxForce",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ReactionForceEnabled",
                    RbxInstanceProperty {
                        name: "ReactionForceEnabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "RodConstraint",
        RbxInstanceClass {
            name: "RodConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CurrentDistance",
                    RbxInstanceProperty {
                        name: "CurrentDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Length",
                    RbxInstanceProperty {
                        name: "Length",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Thickness",
                    RbxInstanceProperty {
                        name: "Thickness",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "RopeConstraint",
        RbxInstanceClass {
            name: "RopeConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CurrentDistance",
                    RbxInstanceProperty {
                        name: "CurrentDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Length",
                    RbxInstanceProperty {
                        name: "Length",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Restitution",
                    RbxInstanceProperty {
                        name: "Restitution",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Thickness",
                    RbxInstanceProperty {
                        name: "Thickness",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SlidingBallConstraint",
        RbxInstanceClass {
            name: "SlidingBallConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ActuatorType",
                    RbxInstanceProperty {
                        name: "ActuatorType",
                        value_type: "ActuatorType",
                    },
                );
                properties.insert(
                    "CurrentPosition",
                    RbxInstanceProperty {
                        name: "CurrentPosition",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LimitsEnabled",
                    RbxInstanceProperty {
                        name: "LimitsEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LowerLimit",
                    RbxInstanceProperty {
                        name: "LowerLimit",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MotorMaxAcceleration",
                    RbxInstanceProperty {
                        name: "MotorMaxAcceleration",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MotorMaxForce",
                    RbxInstanceProperty {
                        name: "MotorMaxForce",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Restitution",
                    RbxInstanceProperty {
                        name: "Restitution",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ServoMaxForce",
                    RbxInstanceProperty {
                        name: "ServoMaxForce",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Speed",
                    RbxInstanceProperty {
                        name: "Speed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TargetPosition",
                    RbxInstanceProperty {
                        name: "TargetPosition",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "UpperLimit",
                    RbxInstanceProperty {
                        name: "UpperLimit",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Velocity",
                    RbxInstanceProperty {
                        name: "Velocity",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "CylindricalConstraint",
        RbxInstanceClass {
            name: "CylindricalConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AngularActuatorType",
                    RbxInstanceProperty {
                        name: "AngularActuatorType",
                        value_type: "ActuatorType",
                    },
                );
                properties.insert(
                    "AngularLimitsEnabled",
                    RbxInstanceProperty {
                        name: "AngularLimitsEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AngularRestitution",
                    RbxInstanceProperty {
                        name: "AngularRestitution",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "AngularSpeed",
                    RbxInstanceProperty {
                        name: "AngularSpeed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "AngularVelocity",
                    RbxInstanceProperty {
                        name: "AngularVelocity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "CurrentAngle",
                    RbxInstanceProperty {
                        name: "CurrentAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "InclinationAngle",
                    RbxInstanceProperty {
                        name: "InclinationAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LowerAngle",
                    RbxInstanceProperty {
                        name: "LowerAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MotorMaxAngularAcceleration",
                    RbxInstanceProperty {
                        name: "MotorMaxAngularAcceleration",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MotorMaxTorque",
                    RbxInstanceProperty {
                        name: "MotorMaxTorque",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "RotationAxisVisible",
                    RbxInstanceProperty {
                        name: "RotationAxisVisible",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ServoMaxTorque",
                    RbxInstanceProperty {
                        name: "ServoMaxTorque",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TargetAngle",
                    RbxInstanceProperty {
                        name: "TargetAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "UpperAngle",
                    RbxInstanceProperty {
                        name: "UpperAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WorldRotationAxis",
                    RbxInstanceProperty {
                        name: "WorldRotationAxis",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PrismaticConstraint",
        RbxInstanceClass {
            name: "PrismaticConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "SpringConstraint",
        RbxInstanceClass {
            name: "SpringConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Coils",
                    RbxInstanceProperty {
                        name: "Coils",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "CurrentLength",
                    RbxInstanceProperty {
                        name: "CurrentLength",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Damping",
                    RbxInstanceProperty {
                        name: "Damping",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "FreeLength",
                    RbxInstanceProperty {
                        name: "FreeLength",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LimitsEnabled",
                    RbxInstanceProperty {
                        name: "LimitsEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MaxForce",
                    RbxInstanceProperty {
                        name: "MaxForce",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxLength",
                    RbxInstanceProperty {
                        name: "MaxLength",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MinLength",
                    RbxInstanceProperty {
                        name: "MinLength",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Radius",
                    RbxInstanceProperty {
                        name: "Radius",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Stiffness",
                    RbxInstanceProperty {
                        name: "Stiffness",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Thickness",
                    RbxInstanceProperty {
                        name: "Thickness",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Torque",
        RbxInstanceClass {
            name: "Torque",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "RelativeTo",
                    RbxInstanceProperty {
                        name: "RelativeTo",
                        value_type: "ActuatorRelativeTo",
                    },
                );
                properties.insert(
                    "Torque",
                    RbxInstanceProperty {
                        name: "Torque",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "VectorForce",
        RbxInstanceClass {
            name: "VectorForce",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ApplyAtCenterOfMass",
                    RbxInstanceProperty {
                        name: "ApplyAtCenterOfMass",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Force",
                    RbxInstanceProperty {
                        name: "Force",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "RelativeTo",
                    RbxInstanceProperty {
                        name: "RelativeTo",
                        value_type: "ActuatorRelativeTo",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ContentProvider",
        RbxInstanceClass {
            name: "ContentProvider",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "BaseUrl",
                    RbxInstanceProperty {
                        name: "BaseUrl",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "RequestQueueSize",
                    RbxInstanceProperty {
                        name: "RequestQueueSize",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ContextActionService",
        RbxInstanceClass {
            name: "ContextActionService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Controller",
        RbxInstanceClass {
            name: "Controller",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "HumanoidController",
        RbxInstanceClass {
            name: "HumanoidController",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "SkateboardController",
        RbxInstanceClass {
            name: "SkateboardController",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Steer",
                    RbxInstanceProperty {
                        name: "Steer",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Throttle",
                    RbxInstanceProperty {
                        name: "Throttle",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "VehicleController",
        RbxInstanceClass {
            name: "VehicleController",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ControllerService",
        RbxInstanceClass {
            name: "ControllerService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CookiesService",
        RbxInstanceClass {
            name: "CookiesService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CorePackages",
        RbxInstanceClass {
            name: "CorePackages",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CoreScriptSyncService",
        RbxInstanceClass {
            name: "CoreScriptSyncService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CustomEvent",
        RbxInstanceClass {
            name: "CustomEvent",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CustomEventReceiver",
        RbxInstanceClass {
            name: "CustomEventReceiver",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Source",
                    RbxInstanceProperty {
                        name: "Source",
                        value_type: "Instance",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DataModelMesh",
        RbxInstanceClass {
            name: "DataModelMesh",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Offset",
                    RbxInstanceProperty {
                        name: "Offset",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Scale",
                    RbxInstanceProperty {
                        name: "Scale",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "VertexColor",
                    RbxInstanceProperty {
                        name: "VertexColor",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BevelMesh",
        RbxInstanceClass {
            name: "BevelMesh",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BlockMesh",
        RbxInstanceClass {
            name: "BlockMesh",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CylinderMesh",
        RbxInstanceClass {
            name: "CylinderMesh",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "FileMesh",
        RbxInstanceClass {
            name: "FileMesh",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "MeshId",
                    RbxInstanceProperty {
                        name: "MeshId",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "TextureId",
                    RbxInstanceProperty {
                        name: "TextureId",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SpecialMesh",
        RbxInstanceClass {
            name: "SpecialMesh",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "MeshType",
                    RbxInstanceProperty {
                        name: "MeshType",
                        value_type: "MeshType",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DataStoreService",
        RbxInstanceClass {
            name: "DataStoreService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AutomaticRetry",
                    RbxInstanceProperty {
                        name: "AutomaticRetry",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LegacyNamingScheme",
                    RbxInstanceProperty {
                        name: "LegacyNamingScheme",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Debris",
        RbxInstanceClass {
            name: "Debris",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "MaxItems",
                    RbxInstanceProperty {
                        name: "MaxItems",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DebugSettings",
        RbxInstanceClass {
            name: "DebugSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "DataModel",
                    RbxInstanceProperty {
                        name: "DataModel",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ErrorReporting",
                    RbxInstanceProperty {
                        name: "ErrorReporting",
                        value_type: "ErrorReporting",
                    },
                );
                properties.insert(
                    "GfxCard",
                    RbxInstanceProperty {
                        name: "GfxCard",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "InstanceCount",
                    RbxInstanceProperty {
                        name: "InstanceCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "IsFmodProfilingEnabled",
                    RbxInstanceProperty {
                        name: "IsFmodProfilingEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsScriptStackTracingEnabled",
                    RbxInstanceProperty {
                        name: "IsScriptStackTracingEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "JobCount",
                    RbxInstanceProperty {
                        name: "JobCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "LuaRamLimit",
                    RbxInstanceProperty {
                        name: "LuaRamLimit",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "OsIs64Bit",
                    RbxInstanceProperty {
                        name: "OsIs64Bit",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "OsPlatform",
                    RbxInstanceProperty {
                        name: "OsPlatform",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "OsPlatformId",
                    RbxInstanceProperty {
                        name: "OsPlatformId",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "OsVer",
                    RbxInstanceProperty {
                        name: "OsVer",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "PlayerCount",
                    RbxInstanceProperty {
                        name: "PlayerCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ReportSoundWarnings",
                    RbxInstanceProperty {
                        name: "ReportSoundWarnings",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RobloxProductName",
                    RbxInstanceProperty {
                        name: "RobloxProductName",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "RobloxVersion",
                    RbxInstanceProperty {
                        name: "RobloxVersion",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "SIMD",
                    RbxInstanceProperty {
                        name: "SIMD",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "SystemProductName",
                    RbxInstanceProperty {
                        name: "SystemProductName",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "TickCountPreciseOverride",
                    RbxInstanceProperty {
                        name: "TickCountPreciseOverride",
                        value_type: "TickCountSampleMethod",
                    },
                );
                properties.insert(
                    "VideoMemory",
                    RbxInstanceProperty {
                        name: "VideoMemory",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DebuggerBreakpoint",
        RbxInstanceClass {
            name: "DebuggerBreakpoint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Condition",
                    RbxInstanceProperty {
                        name: "Condition",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "IsEnabled",
                    RbxInstanceProperty {
                        name: "IsEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Line",
                    RbxInstanceProperty {
                        name: "Line",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DebuggerManager",
        RbxInstanceClass {
            name: "DebuggerManager",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "DebuggingEnabled",
                    RbxInstanceProperty {
                        name: "DebuggingEnabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DebuggerWatch",
        RbxInstanceClass {
            name: "DebuggerWatch",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Expression",
                    RbxInstanceProperty {
                        name: "Expression",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Dialog",
        RbxInstanceClass {
            name: "Dialog",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "BehaviorType",
                    RbxInstanceProperty {
                        name: "BehaviorType",
                        value_type: "DialogBehaviorType",
                    },
                );
                properties.insert(
                    "ConversationDistance",
                    RbxInstanceProperty {
                        name: "ConversationDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "GoodbyeChoiceActive",
                    RbxInstanceProperty {
                        name: "GoodbyeChoiceActive",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "GoodbyeDialog",
                    RbxInstanceProperty {
                        name: "GoodbyeDialog",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "InUse",
                    RbxInstanceProperty {
                        name: "InUse",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "InitialPrompt",
                    RbxInstanceProperty {
                        name: "InitialPrompt",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Purpose",
                    RbxInstanceProperty {
                        name: "Purpose",
                        value_type: "DialogPurpose",
                    },
                );
                properties.insert(
                    "Tone",
                    RbxInstanceProperty {
                        name: "Tone",
                        value_type: "DialogTone",
                    },
                );
                properties.insert(
                    "TriggerDistance",
                    RbxInstanceProperty {
                        name: "TriggerDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TriggerOffset",
                    RbxInstanceProperty {
                        name: "TriggerOffset",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DialogChoice",
        RbxInstanceClass {
            name: "DialogChoice",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "GoodbyeChoiceActive",
                    RbxInstanceProperty {
                        name: "GoodbyeChoiceActive",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "GoodbyeDialog",
                    RbxInstanceProperty {
                        name: "GoodbyeDialog",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "ResponseDialog",
                    RbxInstanceProperty {
                        name: "ResponseDialog",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "UserDialog",
                    RbxInstanceProperty {
                        name: "UserDialog",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Dragger",
        RbxInstanceClass {
            name: "Dragger",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Explosion",
        RbxInstanceClass {
            name: "Explosion",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "BlastPressure",
                    RbxInstanceProperty {
                        name: "BlastPressure",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "BlastRadius",
                    RbxInstanceProperty {
                        name: "BlastRadius",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "DestroyJointRadiusPercent",
                    RbxInstanceProperty {
                        name: "DestroyJointRadiusPercent",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ExplosionType",
                    RbxInstanceProperty {
                        name: "ExplosionType",
                        value_type: "ExplosionType",
                    },
                );
                properties.insert(
                    "Position",
                    RbxInstanceProperty {
                        name: "Position",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Visible",
                    RbxInstanceProperty {
                        name: "Visible",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "FaceInstance",
        RbxInstanceClass {
            name: "FaceInstance",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Face",
                    RbxInstanceProperty {
                        name: "Face",
                        value_type: "NormalId",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Decal",
        RbxInstanceClass {
            name: "Decal",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Color3",
                    RbxInstanceProperty {
                        name: "Color3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "LocalTransparencyModifier",
                    RbxInstanceProperty {
                        name: "LocalTransparencyModifier",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Shiny",
                    RbxInstanceProperty {
                        name: "Shiny",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Specular",
                    RbxInstanceProperty {
                        name: "Specular",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Texture",
                    RbxInstanceProperty {
                        name: "Texture",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "Transparency",
                    RbxInstanceProperty {
                        name: "Transparency",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Texture",
        RbxInstanceClass {
            name: "Texture",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "StudsPerTileU",
                    RbxInstanceProperty {
                        name: "StudsPerTileU",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "StudsPerTileV",
                    RbxInstanceProperty {
                        name: "StudsPerTileV",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Feature",
        RbxInstanceClass {
            name: "Feature",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "FaceId",
                    RbxInstanceProperty {
                        name: "FaceId",
                        value_type: "NormalId",
                    },
                );
                properties.insert(
                    "InOut",
                    RbxInstanceProperty {
                        name: "InOut",
                        value_type: "InOut",
                    },
                );
                properties.insert(
                    "LeftRight",
                    RbxInstanceProperty {
                        name: "LeftRight",
                        value_type: "LeftRight",
                    },
                );
                properties.insert(
                    "TopBottom",
                    RbxInstanceProperty {
                        name: "TopBottom",
                        value_type: "TopBottom",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Hole",
        RbxInstanceClass {
            name: "Hole",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "MotorFeature",
        RbxInstanceClass {
            name: "MotorFeature",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Fire",
        RbxInstanceClass {
            name: "Fire",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Heat",
                    RbxInstanceProperty {
                        name: "Heat",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "SecondaryColor",
                    RbxInstanceProperty {
                        name: "SecondaryColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "size",
                    RbxInstanceProperty {
                        name: "size",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "FlagStandService",
        RbxInstanceClass {
            name: "FlagStandService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "FlyweightService",
        RbxInstanceClass {
            name: "FlyweightService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "CSGDictionaryService",
        RbxInstanceClass {
            name: "CSGDictionaryService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "NonReplicatedCSGDictionaryService",
        RbxInstanceClass {
            name: "NonReplicatedCSGDictionaryService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Folder",
        RbxInstanceClass {
            name: "Folder",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ForceField",
        RbxInstanceClass {
            name: "ForceField",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Visible",
                    RbxInstanceProperty {
                        name: "Visible",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "FriendService",
        RbxInstanceClass {
            name: "FriendService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "FunctionalTest",
        RbxInstanceClass {
            name: "FunctionalTest",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Description",
                    RbxInstanceProperty {
                        name: "Description",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GamePassService",
        RbxInstanceClass {
            name: "GamePassService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "GameSettings",
        RbxInstanceClass {
            name: "GameSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AdditionalCoreIncludeDirs",
                    RbxInstanceProperty {
                        name: "AdditionalCoreIncludeDirs",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "BubbleChatLifetime",
                    RbxInstanceProperty {
                        name: "BubbleChatLifetime",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "BubbleChatMaxBubbles",
                    RbxInstanceProperty {
                        name: "BubbleChatMaxBubbles",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ChatHistory",
                    RbxInstanceProperty {
                        name: "ChatHistory",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ChatScrollLength",
                    RbxInstanceProperty {
                        name: "ChatScrollLength",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "CollisionSoundEnabled",
                    RbxInstanceProperty {
                        name: "CollisionSoundEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CollisionSoundVolume",
                    RbxInstanceProperty {
                        name: "CollisionSoundVolume",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "HardwareMouse",
                    RbxInstanceProperty {
                        name: "HardwareMouse",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MaxCollisionSounds",
                    RbxInstanceProperty {
                        name: "MaxCollisionSounds",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "OverrideStarterScript",
                    RbxInstanceProperty {
                        name: "OverrideStarterScript",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "ReportAbuseChatHistory",
                    RbxInstanceProperty {
                        name: "ReportAbuseChatHistory",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "SoftwareSound",
                    RbxInstanceProperty {
                        name: "SoftwareSound",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "VideoCaptureEnabled",
                    RbxInstanceProperty {
                        name: "VideoCaptureEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "VideoQuality",
                    RbxInstanceProperty {
                        name: "VideoQuality",
                        value_type: "VideoQualitySettings",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GamepadService",
        RbxInstanceClass {
            name: "GamepadService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Geometry",
        RbxInstanceClass {
            name: "Geometry",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "GlobalDataStore",
        RbxInstanceClass {
            name: "GlobalDataStore",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "OrderedDataStore",
        RbxInstanceClass {
            name: "OrderedDataStore",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "GoogleAnalyticsConfiguration",
        RbxInstanceClass {
            name: "GoogleAnalyticsConfiguration",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "GroupService",
        RbxInstanceClass {
            name: "GroupService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "GuiBase",
        RbxInstanceClass {
            name: "GuiBase",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "GuiBase2d",
        RbxInstanceClass {
            name: "GuiBase2d",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AbsolutePosition",
                    RbxInstanceProperty {
                        name: "AbsolutePosition",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "AbsoluteRotation",
                    RbxInstanceProperty {
                        name: "AbsoluteRotation",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "AbsoluteSize",
                    RbxInstanceProperty {
                        name: "AbsoluteSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "AutoLocalize",
                    RbxInstanceProperty {
                        name: "AutoLocalize",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Localize",
                    RbxInstanceProperty {
                        name: "Localize",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RootLocalizationTable",
                    RbxInstanceProperty {
                        name: "RootLocalizationTable",
                        value_type: "LocalizationTable",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GuiObject",
        RbxInstanceClass {
            name: "GuiObject",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Active",
                    RbxInstanceProperty {
                        name: "Active",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AnchorPoint",
                    RbxInstanceProperty {
                        name: "AnchorPoint",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "BackgroundColor",
                    RbxInstanceProperty {
                        name: "BackgroundColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "BackgroundColor3",
                    RbxInstanceProperty {
                        name: "BackgroundColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "BackgroundTransparency",
                    RbxInstanceProperty {
                        name: "BackgroundTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "BorderColor",
                    RbxInstanceProperty {
                        name: "BorderColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "BorderColor3",
                    RbxInstanceProperty {
                        name: "BorderColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "BorderSizePixel",
                    RbxInstanceProperty {
                        name: "BorderSizePixel",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ClipsDescendants",
                    RbxInstanceProperty {
                        name: "ClipsDescendants",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Draggable",
                    RbxInstanceProperty {
                        name: "Draggable",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LayoutOrder",
                    RbxInstanceProperty {
                        name: "LayoutOrder",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "NextSelectionDown",
                    RbxInstanceProperty {
                        name: "NextSelectionDown",
                        value_type: "GuiObject",
                    },
                );
                properties.insert(
                    "NextSelectionLeft",
                    RbxInstanceProperty {
                        name: "NextSelectionLeft",
                        value_type: "GuiObject",
                    },
                );
                properties.insert(
                    "NextSelectionRight",
                    RbxInstanceProperty {
                        name: "NextSelectionRight",
                        value_type: "GuiObject",
                    },
                );
                properties.insert(
                    "NextSelectionUp",
                    RbxInstanceProperty {
                        name: "NextSelectionUp",
                        value_type: "GuiObject",
                    },
                );
                properties.insert(
                    "Position",
                    RbxInstanceProperty {
                        name: "Position",
                        value_type: "UDim2",
                    },
                );
                properties.insert(
                    "Rotation",
                    RbxInstanceProperty {
                        name: "Rotation",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Selectable",
                    RbxInstanceProperty {
                        name: "Selectable",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "SelectionImageObject",
                    RbxInstanceProperty {
                        name: "SelectionImageObject",
                        value_type: "GuiObject",
                    },
                );
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "UDim2",
                    },
                );
                properties.insert(
                    "SizeConstraint",
                    RbxInstanceProperty {
                        name: "SizeConstraint",
                        value_type: "SizeConstraint",
                    },
                );
                properties.insert(
                    "Transparency",
                    RbxInstanceProperty {
                        name: "Transparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Visible",
                    RbxInstanceProperty {
                        name: "Visible",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ZIndex",
                    RbxInstanceProperty {
                        name: "ZIndex",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Frame",
        RbxInstanceClass {
            name: "Frame",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Style",
                    RbxInstanceProperty {
                        name: "Style",
                        value_type: "FrameStyle",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GuiButton",
        RbxInstanceClass {
            name: "GuiButton",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AutoButtonColor",
                    RbxInstanceProperty {
                        name: "AutoButtonColor",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Modal",
                    RbxInstanceProperty {
                        name: "Modal",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Selected",
                    RbxInstanceProperty {
                        name: "Selected",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Style",
                    RbxInstanceProperty {
                        name: "Style",
                        value_type: "ButtonStyle",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ImageButton",
        RbxInstanceClass {
            name: "ImageButton",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "HoverImage",
                    RbxInstanceProperty {
                        name: "HoverImage",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "Image",
                    RbxInstanceProperty {
                        name: "Image",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "ImageColor3",
                    RbxInstanceProperty {
                        name: "ImageColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "ImageRectOffset",
                    RbxInstanceProperty {
                        name: "ImageRectOffset",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "ImageRectSize",
                    RbxInstanceProperty {
                        name: "ImageRectSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "ImageTransparency",
                    RbxInstanceProperty {
                        name: "ImageTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "IsLoaded",
                    RbxInstanceProperty {
                        name: "IsLoaded",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PressedImage",
                    RbxInstanceProperty {
                        name: "PressedImage",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "ScaleType",
                    RbxInstanceProperty {
                        name: "ScaleType",
                        value_type: "ScaleType",
                    },
                );
                properties.insert(
                    "SliceCenter",
                    RbxInstanceProperty {
                        name: "SliceCenter",
                        value_type: "Rect",
                    },
                );
                properties.insert(
                    "SliceScale",
                    RbxInstanceProperty {
                        name: "SliceScale",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TileSize",
                    RbxInstanceProperty {
                        name: "TileSize",
                        value_type: "UDim2",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TextButton",
        RbxInstanceClass {
            name: "TextButton",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Font",
                    RbxInstanceProperty {
                        name: "Font",
                        value_type: "Font",
                    },
                );
                properties.insert(
                    "FontSize",
                    RbxInstanceProperty {
                        name: "FontSize",
                        value_type: "FontSize",
                    },
                );
                properties.insert(
                    "LineHeight",
                    RbxInstanceProperty {
                        name: "LineHeight",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LocalizedText",
                    RbxInstanceProperty {
                        name: "LocalizedText",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Text",
                    RbxInstanceProperty {
                        name: "Text",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "TextBounds",
                    RbxInstanceProperty {
                        name: "TextBounds",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "TextColor",
                    RbxInstanceProperty {
                        name: "TextColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "TextColor3",
                    RbxInstanceProperty {
                        name: "TextColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "TextFits",
                    RbxInstanceProperty {
                        name: "TextFits",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextScaled",
                    RbxInstanceProperty {
                        name: "TextScaled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextSize",
                    RbxInstanceProperty {
                        name: "TextSize",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextStrokeColor3",
                    RbxInstanceProperty {
                        name: "TextStrokeColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "TextStrokeTransparency",
                    RbxInstanceProperty {
                        name: "TextStrokeTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextTransparency",
                    RbxInstanceProperty {
                        name: "TextTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextTruncate",
                    RbxInstanceProperty {
                        name: "TextTruncate",
                        value_type: "TextTruncate",
                    },
                );
                properties.insert(
                    "TextWrap",
                    RbxInstanceProperty {
                        name: "TextWrap",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextWrapped",
                    RbxInstanceProperty {
                        name: "TextWrapped",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextXAlignment",
                    RbxInstanceProperty {
                        name: "TextXAlignment",
                        value_type: "TextXAlignment",
                    },
                );
                properties.insert(
                    "TextYAlignment",
                    RbxInstanceProperty {
                        name: "TextYAlignment",
                        value_type: "TextYAlignment",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GuiLabel",
        RbxInstanceClass {
            name: "GuiLabel",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ImageLabel",
        RbxInstanceClass {
            name: "ImageLabel",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Image",
                    RbxInstanceProperty {
                        name: "Image",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "ImageColor3",
                    RbxInstanceProperty {
                        name: "ImageColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "ImageRectOffset",
                    RbxInstanceProperty {
                        name: "ImageRectOffset",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "ImageRectSize",
                    RbxInstanceProperty {
                        name: "ImageRectSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "ImageTransparency",
                    RbxInstanceProperty {
                        name: "ImageTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "IsLoaded",
                    RbxInstanceProperty {
                        name: "IsLoaded",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ScaleType",
                    RbxInstanceProperty {
                        name: "ScaleType",
                        value_type: "ScaleType",
                    },
                );
                properties.insert(
                    "SliceCenter",
                    RbxInstanceProperty {
                        name: "SliceCenter",
                        value_type: "Rect",
                    },
                );
                properties.insert(
                    "SliceScale",
                    RbxInstanceProperty {
                        name: "SliceScale",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TileSize",
                    RbxInstanceProperty {
                        name: "TileSize",
                        value_type: "UDim2",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TextLabel",
        RbxInstanceClass {
            name: "TextLabel",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Font",
                    RbxInstanceProperty {
                        name: "Font",
                        value_type: "Font",
                    },
                );
                properties.insert(
                    "FontSize",
                    RbxInstanceProperty {
                        name: "FontSize",
                        value_type: "FontSize",
                    },
                );
                properties.insert(
                    "LineHeight",
                    RbxInstanceProperty {
                        name: "LineHeight",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LocalizedText",
                    RbxInstanceProperty {
                        name: "LocalizedText",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Text",
                    RbxInstanceProperty {
                        name: "Text",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "TextBounds",
                    RbxInstanceProperty {
                        name: "TextBounds",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "TextColor",
                    RbxInstanceProperty {
                        name: "TextColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "TextColor3",
                    RbxInstanceProperty {
                        name: "TextColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "TextFits",
                    RbxInstanceProperty {
                        name: "TextFits",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextScaled",
                    RbxInstanceProperty {
                        name: "TextScaled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextSize",
                    RbxInstanceProperty {
                        name: "TextSize",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextStrokeColor3",
                    RbxInstanceProperty {
                        name: "TextStrokeColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "TextStrokeTransparency",
                    RbxInstanceProperty {
                        name: "TextStrokeTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextTransparency",
                    RbxInstanceProperty {
                        name: "TextTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextTruncate",
                    RbxInstanceProperty {
                        name: "TextTruncate",
                        value_type: "TextTruncate",
                    },
                );
                properties.insert(
                    "TextWrap",
                    RbxInstanceProperty {
                        name: "TextWrap",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextWrapped",
                    RbxInstanceProperty {
                        name: "TextWrapped",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextXAlignment",
                    RbxInstanceProperty {
                        name: "TextXAlignment",
                        value_type: "TextXAlignment",
                    },
                );
                properties.insert(
                    "TextYAlignment",
                    RbxInstanceProperty {
                        name: "TextYAlignment",
                        value_type: "TextYAlignment",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ScrollingFrame",
        RbxInstanceClass {
            name: "ScrollingFrame",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AbsoluteWindowSize",
                    RbxInstanceProperty {
                        name: "AbsoluteWindowSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "BottomImage",
                    RbxInstanceProperty {
                        name: "BottomImage",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "CanvasPosition",
                    RbxInstanceProperty {
                        name: "CanvasPosition",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "CanvasSize",
                    RbxInstanceProperty {
                        name: "CanvasSize",
                        value_type: "UDim2",
                    },
                );
                properties.insert(
                    "ElasticBehavior",
                    RbxInstanceProperty {
                        name: "ElasticBehavior",
                        value_type: "ElasticBehavior",
                    },
                );
                properties.insert(
                    "HorizontalScrollBarInset",
                    RbxInstanceProperty {
                        name: "HorizontalScrollBarInset",
                        value_type: "ScrollBarInset",
                    },
                );
                properties.insert(
                    "MidImage",
                    RbxInstanceProperty {
                        name: "MidImage",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "ScrollBarImageColor3",
                    RbxInstanceProperty {
                        name: "ScrollBarImageColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "ScrollBarImageTransparency",
                    RbxInstanceProperty {
                        name: "ScrollBarImageTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ScrollBarThickness",
                    RbxInstanceProperty {
                        name: "ScrollBarThickness",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ScrollingDirection",
                    RbxInstanceProperty {
                        name: "ScrollingDirection",
                        value_type: "ScrollingDirection",
                    },
                );
                properties.insert(
                    "ScrollingEnabled",
                    RbxInstanceProperty {
                        name: "ScrollingEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TopImage",
                    RbxInstanceProperty {
                        name: "TopImage",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "VerticalScrollBarInset",
                    RbxInstanceProperty {
                        name: "VerticalScrollBarInset",
                        value_type: "ScrollBarInset",
                    },
                );
                properties.insert(
                    "VerticalScrollBarPosition",
                    RbxInstanceProperty {
                        name: "VerticalScrollBarPosition",
                        value_type: "VerticalScrollBarPosition",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TextBox",
        RbxInstanceClass {
            name: "TextBox",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ClearTextOnFocus",
                    RbxInstanceProperty {
                        name: "ClearTextOnFocus",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CursorPosition",
                    RbxInstanceProperty {
                        name: "CursorPosition",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Font",
                    RbxInstanceProperty {
                        name: "Font",
                        value_type: "Font",
                    },
                );
                properties.insert(
                    "FontSize",
                    RbxInstanceProperty {
                        name: "FontSize",
                        value_type: "FontSize",
                    },
                );
                properties.insert(
                    "LineHeight",
                    RbxInstanceProperty {
                        name: "LineHeight",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ManualFocusRelease",
                    RbxInstanceProperty {
                        name: "ManualFocusRelease",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MultiLine",
                    RbxInstanceProperty {
                        name: "MultiLine",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "OverlayNativeInput",
                    RbxInstanceProperty {
                        name: "OverlayNativeInput",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PlaceholderColor3",
                    RbxInstanceProperty {
                        name: "PlaceholderColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "PlaceholderText",
                    RbxInstanceProperty {
                        name: "PlaceholderText",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "ShowNativeInput",
                    RbxInstanceProperty {
                        name: "ShowNativeInput",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Text",
                    RbxInstanceProperty {
                        name: "Text",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "TextBounds",
                    RbxInstanceProperty {
                        name: "TextBounds",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "TextColor",
                    RbxInstanceProperty {
                        name: "TextColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "TextColor3",
                    RbxInstanceProperty {
                        name: "TextColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "TextFits",
                    RbxInstanceProperty {
                        name: "TextFits",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextScaled",
                    RbxInstanceProperty {
                        name: "TextScaled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextSize",
                    RbxInstanceProperty {
                        name: "TextSize",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextStrokeColor3",
                    RbxInstanceProperty {
                        name: "TextStrokeColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "TextStrokeTransparency",
                    RbxInstanceProperty {
                        name: "TextStrokeTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextTransparency",
                    RbxInstanceProperty {
                        name: "TextTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextTruncate",
                    RbxInstanceProperty {
                        name: "TextTruncate",
                        value_type: "TextTruncate",
                    },
                );
                properties.insert(
                    "TextWrap",
                    RbxInstanceProperty {
                        name: "TextWrap",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextWrapped",
                    RbxInstanceProperty {
                        name: "TextWrapped",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TextXAlignment",
                    RbxInstanceProperty {
                        name: "TextXAlignment",
                        value_type: "TextXAlignment",
                    },
                );
                properties.insert(
                    "TextYAlignment",
                    RbxInstanceProperty {
                        name: "TextYAlignment",
                        value_type: "TextYAlignment",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ViewportFrame",
        RbxInstanceClass {
            name: "ViewportFrame",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CurrentCamera",
                    RbxInstanceProperty {
                        name: "CurrentCamera",
                        value_type: "Camera",
                    },
                );
                properties.insert(
                    "ImageColor3",
                    RbxInstanceProperty {
                        name: "ImageColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "ImageTransparency",
                    RbxInstanceProperty {
                        name: "ImageTransparency",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LayerCollector",
        RbxInstanceClass {
            name: "LayerCollector",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ResetOnSpawn",
                    RbxInstanceProperty {
                        name: "ResetOnSpawn",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ZIndexBehavior",
                    RbxInstanceProperty {
                        name: "ZIndexBehavior",
                        value_type: "ZIndexBehavior",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BillboardGui",
        RbxInstanceClass {
            name: "BillboardGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Active",
                    RbxInstanceProperty {
                        name: "Active",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Adornee",
                    RbxInstanceProperty {
                        name: "Adornee",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "AlwaysOnTop",
                    RbxInstanceProperty {
                        name: "AlwaysOnTop",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ClipsDescendants",
                    RbxInstanceProperty {
                        name: "ClipsDescendants",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ExtentsOffset",
                    RbxInstanceProperty {
                        name: "ExtentsOffset",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "ExtentsOffsetWorldSpace",
                    RbxInstanceProperty {
                        name: "ExtentsOffsetWorldSpace",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "LightInfluence",
                    RbxInstanceProperty {
                        name: "LightInfluence",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxDistance",
                    RbxInstanceProperty {
                        name: "MaxDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "PlayerToHideFrom",
                    RbxInstanceProperty {
                        name: "PlayerToHideFrom",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "UDim2",
                    },
                );
                properties.insert(
                    "SizeOffset",
                    RbxInstanceProperty {
                        name: "SizeOffset",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "StudsOffset",
                    RbxInstanceProperty {
                        name: "StudsOffset",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "StudsOffsetWorldSpace",
                    RbxInstanceProperty {
                        name: "StudsOffsetWorldSpace",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PluginGui",
        RbxInstanceClass {
            name: "PluginGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Title",
                    RbxInstanceProperty {
                        name: "Title",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DockWidgetPluginGui",
        RbxInstanceClass {
            name: "DockWidgetPluginGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "HostWidgetWasRestored",
                    RbxInstanceProperty {
                        name: "HostWidgetWasRestored",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "QWidgetPluginGui",
        RbxInstanceClass {
            name: "QWidgetPluginGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ScreenGui",
        RbxInstanceClass {
            name: "ScreenGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "DisplayOrder",
                    RbxInstanceProperty {
                        name: "DisplayOrder",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "IgnoreGuiInset",
                    RbxInstanceProperty {
                        name: "IgnoreGuiInset",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "OnTopOfCoreBlur",
                    RbxInstanceProperty {
                        name: "OnTopOfCoreBlur",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GuiMain",
        RbxInstanceClass {
            name: "GuiMain",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "SurfaceGui",
        RbxInstanceClass {
            name: "SurfaceGui",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Active",
                    RbxInstanceProperty {
                        name: "Active",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Adornee",
                    RbxInstanceProperty {
                        name: "Adornee",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "AlwaysOnTop",
                    RbxInstanceProperty {
                        name: "AlwaysOnTop",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CanvasSize",
                    RbxInstanceProperty {
                        name: "CanvasSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "ClipsDescendants",
                    RbxInstanceProperty {
                        name: "ClipsDescendants",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Face",
                    RbxInstanceProperty {
                        name: "Face",
                        value_type: "NormalId",
                    },
                );
                properties.insert(
                    "LightInfluence",
                    RbxInstanceProperty {
                        name: "LightInfluence",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ToolPunchThroughDistance",
                    RbxInstanceProperty {
                        name: "ToolPunchThroughDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ZOffset",
                    RbxInstanceProperty {
                        name: "ZOffset",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GuiBase3d",
        RbxInstanceClass {
            name: "GuiBase3d",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "Color3",
                    RbxInstanceProperty {
                        name: "Color3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Transparency",
                    RbxInstanceProperty {
                        name: "Transparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Visible",
                    RbxInstanceProperty {
                        name: "Visible",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "FloorWire",
        RbxInstanceClass {
            name: "FloorWire",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CycleOffset",
                    RbxInstanceProperty {
                        name: "CycleOffset",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "From",
                    RbxInstanceProperty {
                        name: "From",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "StudsBetweenTextures",
                    RbxInstanceProperty {
                        name: "StudsBetweenTextures",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Texture",
                    RbxInstanceProperty {
                        name: "Texture",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "TextureSize",
                    RbxInstanceProperty {
                        name: "TextureSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "To",
                    RbxInstanceProperty {
                        name: "To",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "Velocity",
                    RbxInstanceProperty {
                        name: "Velocity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WireRadius",
                    RbxInstanceProperty {
                        name: "WireRadius",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PVAdornment",
        RbxInstanceClass {
            name: "PVAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Adornee",
                    RbxInstanceProperty {
                        name: "Adornee",
                        value_type: "PVInstance",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "HandleAdornment",
        RbxInstanceClass {
            name: "HandleAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AlwaysOnTop",
                    RbxInstanceProperty {
                        name: "AlwaysOnTop",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CFrame",
                    RbxInstanceProperty {
                        name: "CFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "SizeRelativeOffset",
                    RbxInstanceProperty {
                        name: "SizeRelativeOffset",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "ZIndex",
                    RbxInstanceProperty {
                        name: "ZIndex",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BoxHandleAdornment",
        RbxInstanceClass {
            name: "BoxHandleAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ConeHandleAdornment",
        RbxInstanceClass {
            name: "ConeHandleAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Height",
                    RbxInstanceProperty {
                        name: "Height",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Radius",
                    RbxInstanceProperty {
                        name: "Radius",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "CylinderHandleAdornment",
        RbxInstanceClass {
            name: "CylinderHandleAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Height",
                    RbxInstanceProperty {
                        name: "Height",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Radius",
                    RbxInstanceProperty {
                        name: "Radius",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ImageHandleAdornment",
        RbxInstanceClass {
            name: "ImageHandleAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Image",
                    RbxInstanceProperty {
                        name: "Image",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "Vector2",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LineHandleAdornment",
        RbxInstanceClass {
            name: "LineHandleAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Length",
                    RbxInstanceProperty {
                        name: "Length",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Thickness",
                    RbxInstanceProperty {
                        name: "Thickness",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SphereHandleAdornment",
        RbxInstanceClass {
            name: "SphereHandleAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Radius",
                    RbxInstanceProperty {
                        name: "Radius",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ParabolaAdornment",
        RbxInstanceClass {
            name: "ParabolaAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "A",
                    RbxInstanceProperty {
                        name: "A",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "B",
                    RbxInstanceProperty {
                        name: "B",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "C",
                    RbxInstanceProperty {
                        name: "C",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Range",
                    RbxInstanceProperty {
                        name: "Range",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Thickness",
                    RbxInstanceProperty {
                        name: "Thickness",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SelectionBox",
        RbxInstanceClass {
            name: "SelectionBox",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "LineThickness",
                    RbxInstanceProperty {
                        name: "LineThickness",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "SurfaceColor",
                    RbxInstanceProperty {
                        name: "SurfaceColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "SurfaceColor3",
                    RbxInstanceProperty {
                        name: "SurfaceColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "SurfaceTransparency",
                    RbxInstanceProperty {
                        name: "SurfaceTransparency",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SelectionSphere",
        RbxInstanceClass {
            name: "SelectionSphere",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "SurfaceColor",
                    RbxInstanceProperty {
                        name: "SurfaceColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "SurfaceColor3",
                    RbxInstanceProperty {
                        name: "SurfaceColor3",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "SurfaceTransparency",
                    RbxInstanceProperty {
                        name: "SurfaceTransparency",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PartAdornment",
        RbxInstanceClass {
            name: "PartAdornment",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Adornee",
                    RbxInstanceProperty {
                        name: "Adornee",
                        value_type: "BasePart",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "HandlesBase",
        RbxInstanceClass {
            name: "HandlesBase",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ArcHandles",
        RbxInstanceClass {
            name: "ArcHandles",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Axes",
                    RbxInstanceProperty {
                        name: "Axes",
                        value_type: "Axes",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Handles",
        RbxInstanceClass {
            name: "Handles",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Faces",
                    RbxInstanceProperty {
                        name: "Faces",
                        value_type: "Faces",
                    },
                );
                properties.insert(
                    "Style",
                    RbxInstanceProperty {
                        name: "Style",
                        value_type: "HandlesStyle",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SurfaceSelection",
        RbxInstanceClass {
            name: "SurfaceSelection",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "TargetSurface",
                    RbxInstanceProperty {
                        name: "TargetSurface",
                        value_type: "NormalId",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SelectionLasso",
        RbxInstanceClass {
            name: "SelectionLasso",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Humanoid",
                    RbxInstanceProperty {
                        name: "Humanoid",
                        value_type: "Humanoid",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SelectionPartLasso",
        RbxInstanceClass {
            name: "SelectionPartLasso",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Part",
                    RbxInstanceProperty {
                        name: "Part",
                        value_type: "BasePart",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SelectionPointLasso",
        RbxInstanceClass {
            name: "SelectionPointLasso",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Point",
                    RbxInstanceProperty {
                        name: "Point",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GuiItem",
        RbxInstanceClass {
            name: "GuiItem",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Backpack",
        RbxInstanceClass {
            name: "Backpack",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BackpackItem",
        RbxInstanceClass {
            name: "BackpackItem",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "TextureId",
                    RbxInstanceProperty {
                        name: "TextureId",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "HopperBin",
        RbxInstanceClass {
            name: "HopperBin",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Active",
                    RbxInstanceProperty {
                        name: "Active",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "BinType",
                    RbxInstanceProperty {
                        name: "BinType",
                        value_type: "BinType",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Tool",
        RbxInstanceClass {
            name: "Tool",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CanBeDropped",
                    RbxInstanceProperty {
                        name: "CanBeDropped",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Grip",
                    RbxInstanceProperty {
                        name: "Grip",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "GripForward",
                    RbxInstanceProperty {
                        name: "GripForward",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "GripPos",
                    RbxInstanceProperty {
                        name: "GripPos",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "GripRight",
                    RbxInstanceProperty {
                        name: "GripRight",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "GripUp",
                    RbxInstanceProperty {
                        name: "GripUp",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "ManualActivationOnly",
                    RbxInstanceProperty {
                        name: "ManualActivationOnly",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RequiresHandle",
                    RbxInstanceProperty {
                        name: "RequiresHandle",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ToolTip",
                    RbxInstanceProperty {
                        name: "ToolTip",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Flag",
        RbxInstanceClass {
            name: "Flag",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "TeamColor",
                    RbxInstanceProperty {
                        name: "TeamColor",
                        value_type: "BrickColor",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ButtonBindingWidget",
        RbxInstanceClass {
            name: "ButtonBindingWidget",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "GuiRoot",
        RbxInstanceClass {
            name: "GuiRoot",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Hopper",
        RbxInstanceClass {
            name: "Hopper",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "StarterPack",
        RbxInstanceClass {
            name: "StarterPack",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "GuiService",
        RbxInstanceClass {
            name: "GuiService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AutoSelectGuiEnabled",
                    RbxInstanceProperty {
                        name: "AutoSelectGuiEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CoreEffectFolder",
                    RbxInstanceProperty {
                        name: "CoreEffectFolder",
                        value_type: "Folder",
                    },
                );
                properties.insert(
                    "CoreGuiFolder",
                    RbxInstanceProperty {
                        name: "CoreGuiFolder",
                        value_type: "Folder",
                    },
                );
                properties.insert(
                    "CoreGuiNavigationEnabled",
                    RbxInstanceProperty {
                        name: "CoreGuiNavigationEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "GuiNavigationEnabled",
                    RbxInstanceProperty {
                        name: "GuiNavigationEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsModalDialog",
                    RbxInstanceProperty {
                        name: "IsModalDialog",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsWindows",
                    RbxInstanceProperty {
                        name: "IsWindows",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MenuIsOpen",
                    RbxInstanceProperty {
                        name: "MenuIsOpen",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "SelectedCoreObject",
                    RbxInstanceProperty {
                        name: "SelectedCoreObject",
                        value_type: "GuiObject",
                    },
                );
                properties.insert(
                    "SelectedObject",
                    RbxInstanceProperty {
                        name: "SelectedObject",
                        value_type: "GuiObject",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GuidRegistryService",
        RbxInstanceClass {
            name: "GuidRegistryService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "HapticService",
        RbxInstanceClass {
            name: "HapticService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "HttpRbxApiService",
        RbxInstanceClass {
            name: "HttpRbxApiService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "HttpRequest",
        RbxInstanceClass {
            name: "HttpRequest",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "HttpService",
        RbxInstanceClass {
            name: "HttpService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "HttpEnabled",
                    RbxInstanceProperty {
                        name: "HttpEnabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Humanoid",
        RbxInstanceClass {
            name: "Humanoid",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AutoJumpEnabled",
                    RbxInstanceProperty {
                        name: "AutoJumpEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AutoRotate",
                    RbxInstanceProperty {
                        name: "AutoRotate",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AutomaticScalingEnabled",
                    RbxInstanceProperty {
                        name: "AutomaticScalingEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "BreakJointsOnDeath",
                    RbxInstanceProperty {
                        name: "BreakJointsOnDeath",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CameraOffset",
                    RbxInstanceProperty {
                        name: "CameraOffset",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "DisplayDistanceType",
                    RbxInstanceProperty {
                        name: "DisplayDistanceType",
                        value_type: "HumanoidDisplayDistanceType",
                    },
                );
                properties.insert(
                    "FloorMaterial",
                    RbxInstanceProperty {
                        name: "FloorMaterial",
                        value_type: "Material",
                    },
                );
                properties.insert(
                    "Health",
                    RbxInstanceProperty {
                        name: "Health",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "HealthDisplayDistance",
                    RbxInstanceProperty {
                        name: "HealthDisplayDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "HealthDisplayType",
                    RbxInstanceProperty {
                        name: "HealthDisplayType",
                        value_type: "HumanoidHealthDisplayType",
                    },
                );
                properties.insert(
                    "HipHeight",
                    RbxInstanceProperty {
                        name: "HipHeight",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Jump",
                    RbxInstanceProperty {
                        name: "Jump",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "JumpPower",
                    RbxInstanceProperty {
                        name: "JumpPower",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LeftLeg",
                    RbxInstanceProperty {
                        name: "LeftLeg",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "MaxHealth",
                    RbxInstanceProperty {
                        name: "MaxHealth",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxSlopeAngle",
                    RbxInstanceProperty {
                        name: "MaxSlopeAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MoveDirection",
                    RbxInstanceProperty {
                        name: "MoveDirection",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "NameDisplayDistance",
                    RbxInstanceProperty {
                        name: "NameDisplayDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "NameOcclusion",
                    RbxInstanceProperty {
                        name: "NameOcclusion",
                        value_type: "NameOcclusion",
                    },
                );
                properties.insert(
                    "PlatformStand",
                    RbxInstanceProperty {
                        name: "PlatformStand",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RigType",
                    RbxInstanceProperty {
                        name: "RigType",
                        value_type: "HumanoidRigType",
                    },
                );
                properties.insert(
                    "RightLeg",
                    RbxInstanceProperty {
                        name: "RightLeg",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "RootPart",
                    RbxInstanceProperty {
                        name: "RootPart",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "SeatPart",
                    RbxInstanceProperty {
                        name: "SeatPart",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "Sit",
                    RbxInstanceProperty {
                        name: "Sit",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TargetPoint",
                    RbxInstanceProperty {
                        name: "TargetPoint",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Torso",
                    RbxInstanceProperty {
                        name: "Torso",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "WalkSpeed",
                    RbxInstanceProperty {
                        name: "WalkSpeed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WalkToPart",
                    RbxInstanceProperty {
                        name: "WalkToPart",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "WalkToPoint",
                    RbxInstanceProperty {
                        name: "WalkToPoint",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "maxHealth",
                    RbxInstanceProperty {
                        name: "maxHealth",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "HumanoidDescription",
        RbxInstanceClass {
            name: "HumanoidDescription",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "BackAccessory",
                    RbxInstanceProperty {
                        name: "BackAccessory",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "BodyTypeScale",
                    RbxInstanceProperty {
                        name: "BodyTypeScale",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ClimbAnimation",
                    RbxInstanceProperty {
                        name: "ClimbAnimation",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "DepthScale",
                    RbxInstanceProperty {
                        name: "DepthScale",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Face",
                    RbxInstanceProperty {
                        name: "Face",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "FaceAccessory",
                    RbxInstanceProperty {
                        name: "FaceAccessory",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "FallAnimation",
                    RbxInstanceProperty {
                        name: "FallAnimation",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "FrontAccessory",
                    RbxInstanceProperty {
                        name: "FrontAccessory",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "GraphicTShirt",
                    RbxInstanceProperty {
                        name: "GraphicTShirt",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "HairAccessory",
                    RbxInstanceProperty {
                        name: "HairAccessory",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "HatAccessory",
                    RbxInstanceProperty {
                        name: "HatAccessory",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Head",
                    RbxInstanceProperty {
                        name: "Head",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "HeadColor",
                    RbxInstanceProperty {
                        name: "HeadColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "HeadScale",
                    RbxInstanceProperty {
                        name: "HeadScale",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "HeightScale",
                    RbxInstanceProperty {
                        name: "HeightScale",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "IdleAnimation",
                    RbxInstanceProperty {
                        name: "IdleAnimation",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "JumpAnimation",
                    RbxInstanceProperty {
                        name: "JumpAnimation",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "LeftArm",
                    RbxInstanceProperty {
                        name: "LeftArm",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "LeftArmColor",
                    RbxInstanceProperty {
                        name: "LeftArmColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "LeftLeg",
                    RbxInstanceProperty {
                        name: "LeftLeg",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "LeftLegColor",
                    RbxInstanceProperty {
                        name: "LeftLegColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "NeckAccessory",
                    RbxInstanceProperty {
                        name: "NeckAccessory",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Pants",
                    RbxInstanceProperty {
                        name: "Pants",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "ProportionScale",
                    RbxInstanceProperty {
                        name: "ProportionScale",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "RightArm",
                    RbxInstanceProperty {
                        name: "RightArm",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "RightArmColor",
                    RbxInstanceProperty {
                        name: "RightArmColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "RightLeg",
                    RbxInstanceProperty {
                        name: "RightLeg",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "RightLegColor",
                    RbxInstanceProperty {
                        name: "RightLegColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "RunAnimation",
                    RbxInstanceProperty {
                        name: "RunAnimation",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "Shirt",
                    RbxInstanceProperty {
                        name: "Shirt",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "ShouldersAccessory",
                    RbxInstanceProperty {
                        name: "ShouldersAccessory",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "SwimAnimation",
                    RbxInstanceProperty {
                        name: "SwimAnimation",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "Torso",
                    RbxInstanceProperty {
                        name: "Torso",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "TorsoColor",
                    RbxInstanceProperty {
                        name: "TorsoColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "WaistAccessory",
                    RbxInstanceProperty {
                        name: "WaistAccessory",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "WalkAnimation",
                    RbxInstanceProperty {
                        name: "WalkAnimation",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "WidthScale",
                    RbxInstanceProperty {
                        name: "WidthScale",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "InputObject",
        RbxInstanceClass {
            name: "InputObject",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Delta",
                    RbxInstanceProperty {
                        name: "Delta",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "KeyCode",
                    RbxInstanceProperty {
                        name: "KeyCode",
                        value_type: "KeyCode",
                    },
                );
                properties.insert(
                    "Position",
                    RbxInstanceProperty {
                        name: "Position",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "UserInputState",
                    RbxInstanceProperty {
                        name: "UserInputState",
                        value_type: "UserInputState",
                    },
                );
                properties.insert(
                    "UserInputType",
                    RbxInstanceProperty {
                        name: "UserInputType",
                        value_type: "UserInputType",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "InsertService",
        RbxInstanceClass {
            name: "InsertService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AllowClientInsertModels",
                    RbxInstanceProperty {
                        name: "AllowClientInsertModels",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AllowInsertFreeModels",
                    RbxInstanceProperty {
                        name: "AllowInsertFreeModels",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "JointInstance",
        RbxInstanceClass {
            name: "JointInstance",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Active",
                    RbxInstanceProperty {
                        name: "Active",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "C0",
                    RbxInstanceProperty {
                        name: "C0",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "C1",
                    RbxInstanceProperty {
                        name: "C1",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "Part0",
                    RbxInstanceProperty {
                        name: "Part0",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "Part1",
                    RbxInstanceProperty {
                        name: "Part1",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "part1",
                    RbxInstanceProperty {
                        name: "part1",
                        value_type: "BasePart",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DynamicRotate",
        RbxInstanceClass {
            name: "DynamicRotate",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "BaseAngle",
                    RbxInstanceProperty {
                        name: "BaseAngle",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "RotateP",
        RbxInstanceClass {
            name: "RotateP",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RotateV",
        RbxInstanceClass {
            name: "RotateV",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Glue",
        RbxInstanceClass {
            name: "Glue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "F0",
                    RbxInstanceProperty {
                        name: "F0",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "F1",
                    RbxInstanceProperty {
                        name: "F1",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "F2",
                    RbxInstanceProperty {
                        name: "F2",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "F3",
                    RbxInstanceProperty {
                        name: "F3",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ManualSurfaceJointInstance",
        RbxInstanceClass {
            name: "ManualSurfaceJointInstance",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ManualGlue",
        RbxInstanceClass {
            name: "ManualGlue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ManualWeld",
        RbxInstanceClass {
            name: "ManualWeld",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Motor",
        RbxInstanceClass {
            name: "Motor",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CurrentAngle",
                    RbxInstanceProperty {
                        name: "CurrentAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "DesiredAngle",
                    RbxInstanceProperty {
                        name: "DesiredAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxVelocity",
                    RbxInstanceProperty {
                        name: "MaxVelocity",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Motor6D",
        RbxInstanceClass {
            name: "Motor6D",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Transform",
                    RbxInstanceProperty {
                        name: "Transform",
                        value_type: "CFrame",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Rotate",
        RbxInstanceClass {
            name: "Rotate",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Snap",
        RbxInstanceClass {
            name: "Snap",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "VelocityMotor",
        RbxInstanceClass {
            name: "VelocityMotor",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CurrentAngle",
                    RbxInstanceProperty {
                        name: "CurrentAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "DesiredAngle",
                    RbxInstanceProperty {
                        name: "DesiredAngle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Hole",
                    RbxInstanceProperty {
                        name: "Hole",
                        value_type: "Hole",
                    },
                );
                properties.insert(
                    "MaxVelocity",
                    RbxInstanceProperty {
                        name: "MaxVelocity",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Weld",
        RbxInstanceClass {
            name: "Weld",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "JointsService",
        RbxInstanceClass {
            name: "JointsService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "KeyboardService",
        RbxInstanceClass {
            name: "KeyboardService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Keyframe",
        RbxInstanceClass {
            name: "Keyframe",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Time",
                    RbxInstanceProperty {
                        name: "Time",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "KeyframeMarker",
        RbxInstanceClass {
            name: "KeyframeMarker",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "KeyframeSequence",
        RbxInstanceClass {
            name: "KeyframeSequence",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AuthoredHipHeight",
                    RbxInstanceProperty {
                        name: "AuthoredHipHeight",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Loop",
                    RbxInstanceProperty {
                        name: "Loop",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Priority",
                    RbxInstanceProperty {
                        name: "Priority",
                        value_type: "AnimationPriority",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "KeyframeSequenceProvider",
        RbxInstanceClass {
            name: "KeyframeSequenceProvider",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Light",
        RbxInstanceClass {
            name: "Light",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Brightness",
                    RbxInstanceProperty {
                        name: "Brightness",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Shadows",
                    RbxInstanceProperty {
                        name: "Shadows",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PointLight",
        RbxInstanceClass {
            name: "PointLight",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Range",
                    RbxInstanceProperty {
                        name: "Range",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SpotLight",
        RbxInstanceClass {
            name: "SpotLight",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Angle",
                    RbxInstanceProperty {
                        name: "Angle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Face",
                    RbxInstanceProperty {
                        name: "Face",
                        value_type: "NormalId",
                    },
                );
                properties.insert(
                    "Range",
                    RbxInstanceProperty {
                        name: "Range",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SurfaceLight",
        RbxInstanceClass {
            name: "SurfaceLight",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Angle",
                    RbxInstanceProperty {
                        name: "Angle",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Face",
                    RbxInstanceProperty {
                        name: "Face",
                        value_type: "NormalId",
                    },
                );
                properties.insert(
                    "Range",
                    RbxInstanceProperty {
                        name: "Range",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Lighting",
        RbxInstanceClass {
            name: "Lighting",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Ambient",
                    RbxInstanceProperty {
                        name: "Ambient",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Brightness",
                    RbxInstanceProperty {
                        name: "Brightness",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ClockTime",
                    RbxInstanceProperty {
                        name: "ClockTime",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ColorShift_Bottom",
                    RbxInstanceProperty {
                        name: "ColorShift_Bottom",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "ColorShift_Top",
                    RbxInstanceProperty {
                        name: "ColorShift_Top",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "ExposureCompensation",
                    RbxInstanceProperty {
                        name: "ExposureCompensation",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "FogColor",
                    RbxInstanceProperty {
                        name: "FogColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "FogEnd",
                    RbxInstanceProperty {
                        name: "FogEnd",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "FogStart",
                    RbxInstanceProperty {
                        name: "FogStart",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "GeographicLatitude",
                    RbxInstanceProperty {
                        name: "GeographicLatitude",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "GlobalShadows",
                    RbxInstanceProperty {
                        name: "GlobalShadows",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "OutdoorAmbient",
                    RbxInstanceProperty {
                        name: "OutdoorAmbient",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Outlines",
                    RbxInstanceProperty {
                        name: "Outlines",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ShadowColor",
                    RbxInstanceProperty {
                        name: "ShadowColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Technology",
                    RbxInstanceProperty {
                        name: "Technology",
                        value_type: "Technology",
                    },
                );
                properties.insert(
                    "TimeOfDay",
                    RbxInstanceProperty {
                        name: "TimeOfDay",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LocalAsset",
        RbxInstanceClass {
            name: "LocalAsset",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "FileName",
                    RbxInstanceProperty {
                        name: "FileName",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "FileSize",
                    RbxInstanceProperty {
                        name: "FileSize",
                        value_type: "int64",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LocalStorageService",
        RbxInstanceClass {
            name: "LocalStorageService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "AppStorageService",
        RbxInstanceClass {
            name: "AppStorageService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "UserStorageService",
        RbxInstanceClass {
            name: "UserStorageService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "LocalizationService",
        RbxInstanceClass {
            name: "LocalizationService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ForcePlayModeGameLocaleId",
                    RbxInstanceProperty {
                        name: "ForcePlayModeGameLocaleId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "ForcePlayModeRobloxLocaleId",
                    RbxInstanceProperty {
                        name: "ForcePlayModeRobloxLocaleId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "IsTextScraperRunning",
                    RbxInstanceProperty {
                        name: "IsTextScraperRunning",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RobloxForcePlayModeGameLocaleId",
                    RbxInstanceProperty {
                        name: "RobloxForcePlayModeGameLocaleId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "RobloxForcePlayModeRobloxLocaleId",
                    RbxInstanceProperty {
                        name: "RobloxForcePlayModeRobloxLocaleId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "RobloxLocaleId",
                    RbxInstanceProperty {
                        name: "RobloxLocaleId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "SystemLocaleId",
                    RbxInstanceProperty {
                        name: "SystemLocaleId",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LocalizationTable",
        RbxInstanceClass {
            name: "LocalizationTable",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "DevelopmentLanguage",
                    RbxInstanceProperty {
                        name: "DevelopmentLanguage",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Root",
                    RbxInstanceProperty {
                        name: "Root",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "SourceLocaleId",
                    RbxInstanceProperty {
                        name: "SourceLocaleId",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LogService",
        RbxInstanceClass {
            name: "LogService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "LoginService",
        RbxInstanceClass {
            name: "LoginService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "LuaSettings",
        RbxInstanceClass {
            name: "LuaSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AreScriptStartsReported",
                    RbxInstanceProperty {
                        name: "AreScriptStartsReported",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "DefaultWaitTime",
                    RbxInstanceProperty {
                        name: "DefaultWaitTime",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "GcFrequency",
                    RbxInstanceProperty {
                        name: "GcFrequency",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "GcLimit",
                    RbxInstanceProperty {
                        name: "GcLimit",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "GcPause",
                    RbxInstanceProperty {
                        name: "GcPause",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "GcStepMul",
                    RbxInstanceProperty {
                        name: "GcStepMul",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "WaitingThreadsBudget",
                    RbxInstanceProperty {
                        name: "WaitingThreadsBudget",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LuaSourceContainer",
        RbxInstanceClass {
            name: "LuaSourceContainer",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CurrentEditor",
                    RbxInstanceProperty {
                        name: "CurrentEditor",
                        value_type: "Instance",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BaseScript",
        RbxInstanceClass {
            name: "BaseScript",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Disabled",
                    RbxInstanceProperty {
                        name: "Disabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LinkedSource",
                    RbxInstanceProperty {
                        name: "LinkedSource",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "CoreScript",
        RbxInstanceClass {
            name: "CoreScript",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Script",
        RbxInstanceClass {
            name: "Script",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Source",
                    RbxInstanceProperty {
                        name: "Source",
                        value_type: "ProtectedString",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LocalScript",
        RbxInstanceClass {
            name: "LocalScript",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ModuleScript",
        RbxInstanceClass {
            name: "ModuleScript",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "LinkedSource",
                    RbxInstanceProperty {
                        name: "LinkedSource",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "Source",
                    RbxInstanceProperty {
                        name: "Source",
                        value_type: "ProtectedString",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "LuaWebService",
        RbxInstanceClass {
            name: "LuaWebService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "MarketplaceService",
        RbxInstanceClass {
            name: "MarketplaceService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Message",
        RbxInstanceClass {
            name: "Message",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Text",
                    RbxInstanceProperty {
                        name: "Text",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Hint",
        RbxInstanceClass {
            name: "Hint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "MessagingService",
        RbxInstanceClass {
            name: "MessagingService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Mouse",
        RbxInstanceClass {
            name: "Mouse",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Hit",
                    RbxInstanceProperty {
                        name: "Hit",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "Icon",
                    RbxInstanceProperty {
                        name: "Icon",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "Origin",
                    RbxInstanceProperty {
                        name: "Origin",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "Target",
                    RbxInstanceProperty {
                        name: "Target",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "TargetFilter",
                    RbxInstanceProperty {
                        name: "TargetFilter",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "TargetSurface",
                    RbxInstanceProperty {
                        name: "TargetSurface",
                        value_type: "NormalId",
                    },
                );
                properties.insert(
                    "UnitRay",
                    RbxInstanceProperty {
                        name: "UnitRay",
                        value_type: "Ray",
                    },
                );
                properties.insert(
                    "ViewSizeX",
                    RbxInstanceProperty {
                        name: "ViewSizeX",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ViewSizeY",
                    RbxInstanceProperty {
                        name: "ViewSizeY",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "X",
                    RbxInstanceProperty {
                        name: "X",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Y",
                    RbxInstanceProperty {
                        name: "Y",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "hit",
                    RbxInstanceProperty {
                        name: "hit",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "target",
                    RbxInstanceProperty {
                        name: "target",
                        value_type: "BasePart",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PlayerMouse",
        RbxInstanceClass {
            name: "PlayerMouse",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "PluginMouse",
        RbxInstanceClass {
            name: "PluginMouse",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "MouseService",
        RbxInstanceClass {
            name: "MouseService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "NetworkMarker",
        RbxInstanceClass {
            name: "NetworkMarker",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "NetworkPeer",
        RbxInstanceClass {
            name: "NetworkPeer",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "NetworkClient",
        RbxInstanceClass {
            name: "NetworkClient",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Ticket",
                    RbxInstanceProperty {
                        name: "Ticket",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "NetworkServer",
        RbxInstanceClass {
            name: "NetworkServer",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Port",
                    RbxInstanceProperty {
                        name: "Port",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "NetworkReplicator",
        RbxInstanceClass {
            name: "NetworkReplicator",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ClientReplicator",
        RbxInstanceClass {
            name: "ClientReplicator",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ServerReplicator",
        RbxInstanceClass {
            name: "ServerReplicator",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "NetworkSettings",
        RbxInstanceClass {
            name: "NetworkSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ArePhysicsRejectionsReported",
                    RbxInstanceProperty {
                        name: "ArePhysicsRejectionsReported",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ClientPhysicsSendRate",
                    RbxInstanceProperty {
                        name: "ClientPhysicsSendRate",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "DataGCRate",
                    RbxInstanceProperty {
                        name: "DataGCRate",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "DataMtuAdjust",
                    RbxInstanceProperty {
                        name: "DataMtuAdjust",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "DataSendPriority",
                    RbxInstanceProperty {
                        name: "DataSendPriority",
                        value_type: "PacketPriority",
                    },
                );
                properties.insert(
                    "DataSendRate",
                    RbxInstanceProperty {
                        name: "DataSendRate",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ExtraMemoryUsed",
                    RbxInstanceProperty {
                        name: "ExtraMemoryUsed",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "FreeMemoryMBytes",
                    RbxInstanceProperty {
                        name: "FreeMemoryMBytes",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "IncommingReplicationLag",
                    RbxInstanceProperty {
                        name: "IncommingReplicationLag",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "IsQueueErrorComputed",
                    RbxInstanceProperty {
                        name: "IsQueueErrorComputed",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "NetworkOwnerRate",
                    RbxInstanceProperty {
                        name: "NetworkOwnerRate",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "PhysicsMtuAdjust",
                    RbxInstanceProperty {
                        name: "PhysicsMtuAdjust",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "PhysicsSendPriority",
                    RbxInstanceProperty {
                        name: "PhysicsSendPriority",
                        value_type: "PacketPriority",
                    },
                );
                properties.insert(
                    "PhysicsSendRate",
                    RbxInstanceProperty {
                        name: "PhysicsSendRate",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "PreferredClientPort",
                    RbxInstanceProperty {
                        name: "PreferredClientPort",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "PrintBits",
                    RbxInstanceProperty {
                        name: "PrintBits",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PrintEvents",
                    RbxInstanceProperty {
                        name: "PrintEvents",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PrintFilters",
                    RbxInstanceProperty {
                        name: "PrintFilters",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PrintInstances",
                    RbxInstanceProperty {
                        name: "PrintInstances",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PrintPhysicsErrors",
                    RbxInstanceProperty {
                        name: "PrintPhysicsErrors",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PrintProperties",
                    RbxInstanceProperty {
                        name: "PrintProperties",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PrintSplitMessage",
                    RbxInstanceProperty {
                        name: "PrintSplitMessage",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PrintStreamInstanceQuota",
                    RbxInstanceProperty {
                        name: "PrintStreamInstanceQuota",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PrintTouches",
                    RbxInstanceProperty {
                        name: "PrintTouches",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ProxyEnabled",
                    RbxInstanceProperty {
                        name: "ProxyEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ProxyURL",
                    RbxInstanceProperty {
                        name: "ProxyURL",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "ReceiveRate",
                    RbxInstanceProperty {
                        name: "ReceiveRate",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "RenderStreamedRegions",
                    RbxInstanceProperty {
                        name: "RenderStreamedRegions",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ShowActiveAnimationAsset",
                    RbxInstanceProperty {
                        name: "ShowActiveAnimationAsset",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TouchSendRate",
                    RbxInstanceProperty {
                        name: "TouchSendRate",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TrackDataTypes",
                    RbxInstanceProperty {
                        name: "TrackDataTypes",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TrackPhysicsDetails",
                    RbxInstanceProperty {
                        name: "TrackPhysicsDetails",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "UsePhysicsPacketCache",
                    RbxInstanceProperty {
                        name: "UsePhysicsPacketCache",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "NotificationService",
        RbxInstanceClass {
            name: "NotificationService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "IsLuaBottomBarEnabled",
                    RbxInstanceProperty {
                        name: "IsLuaBottomBarEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsLuaChatEnabled",
                    RbxInstanceProperty {
                        name: "IsLuaChatEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsLuaGameDetailsEnabled",
                    RbxInstanceProperty {
                        name: "IsLuaGameDetailsEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsLuaGamesPageEnabled",
                    RbxInstanceProperty {
                        name: "IsLuaGamesPageEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsLuaHomePageEnabled",
                    RbxInstanceProperty {
                        name: "IsLuaHomePageEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "SelectedTheme",
                    RbxInstanceProperty {
                        name: "SelectedTheme",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PVInstance",
        RbxInstanceClass {
            name: "PVInstance",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BasePart",
        RbxInstanceClass {
            name: "BasePart",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Anchored",
                    RbxInstanceProperty {
                        name: "Anchored",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "BackParamA",
                    RbxInstanceProperty {
                        name: "BackParamA",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "BackParamB",
                    RbxInstanceProperty {
                        name: "BackParamB",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "BackSurface",
                    RbxInstanceProperty {
                        name: "BackSurface",
                        value_type: "SurfaceType",
                    },
                );
                properties.insert(
                    "BackSurfaceInput",
                    RbxInstanceProperty {
                        name: "BackSurfaceInput",
                        value_type: "InputType",
                    },
                );
                properties.insert(
                    "BottomParamA",
                    RbxInstanceProperty {
                        name: "BottomParamA",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "BottomParamB",
                    RbxInstanceProperty {
                        name: "BottomParamB",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "BottomSurface",
                    RbxInstanceProperty {
                        name: "BottomSurface",
                        value_type: "SurfaceType",
                    },
                );
                properties.insert(
                    "BottomSurfaceInput",
                    RbxInstanceProperty {
                        name: "BottomSurfaceInput",
                        value_type: "InputType",
                    },
                );
                properties.insert(
                    "BrickColor",
                    RbxInstanceProperty {
                        name: "BrickColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "CFrame",
                    RbxInstanceProperty {
                        name: "CFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "CanCollide",
                    RbxInstanceProperty {
                        name: "CanCollide",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CenterOfMass",
                    RbxInstanceProperty {
                        name: "CenterOfMass",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "CollisionGroupId",
                    RbxInstanceProperty {
                        name: "CollisionGroupId",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "CustomPhysicalProperties",
                    RbxInstanceProperty {
                        name: "CustomPhysicalProperties",
                        value_type: "PhysicalProperties",
                    },
                );
                properties.insert(
                    "Elasticity",
                    RbxInstanceProperty {
                        name: "Elasticity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Friction",
                    RbxInstanceProperty {
                        name: "Friction",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "FrontParamA",
                    RbxInstanceProperty {
                        name: "FrontParamA",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "FrontParamB",
                    RbxInstanceProperty {
                        name: "FrontParamB",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "FrontSurface",
                    RbxInstanceProperty {
                        name: "FrontSurface",
                        value_type: "SurfaceType",
                    },
                );
                properties.insert(
                    "FrontSurfaceInput",
                    RbxInstanceProperty {
                        name: "FrontSurfaceInput",
                        value_type: "InputType",
                    },
                );
                properties.insert(
                    "LeftParamA",
                    RbxInstanceProperty {
                        name: "LeftParamA",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LeftParamB",
                    RbxInstanceProperty {
                        name: "LeftParamB",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LeftSurface",
                    RbxInstanceProperty {
                        name: "LeftSurface",
                        value_type: "SurfaceType",
                    },
                );
                properties.insert(
                    "LeftSurfaceInput",
                    RbxInstanceProperty {
                        name: "LeftSurfaceInput",
                        value_type: "InputType",
                    },
                );
                properties.insert(
                    "LocalTransparencyModifier",
                    RbxInstanceProperty {
                        name: "LocalTransparencyModifier",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Locked",
                    RbxInstanceProperty {
                        name: "Locked",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Massless",
                    RbxInstanceProperty {
                        name: "Massless",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Material",
                    RbxInstanceProperty {
                        name: "Material",
                        value_type: "Material",
                    },
                );
                properties.insert(
                    "Orientation",
                    RbxInstanceProperty {
                        name: "Orientation",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Position",
                    RbxInstanceProperty {
                        name: "Position",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "ReceiveAge",
                    RbxInstanceProperty {
                        name: "ReceiveAge",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Reflectance",
                    RbxInstanceProperty {
                        name: "Reflectance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ResizeIncrement",
                    RbxInstanceProperty {
                        name: "ResizeIncrement",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ResizeableFaces",
                    RbxInstanceProperty {
                        name: "ResizeableFaces",
                        value_type: "Faces",
                    },
                );
                properties.insert(
                    "RightParamA",
                    RbxInstanceProperty {
                        name: "RightParamA",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "RightParamB",
                    RbxInstanceProperty {
                        name: "RightParamB",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "RightSurface",
                    RbxInstanceProperty {
                        name: "RightSurface",
                        value_type: "SurfaceType",
                    },
                );
                properties.insert(
                    "RightSurfaceInput",
                    RbxInstanceProperty {
                        name: "RightSurfaceInput",
                        value_type: "InputType",
                    },
                );
                properties.insert(
                    "RootPriority",
                    RbxInstanceProperty {
                        name: "RootPriority",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "RotVelocity",
                    RbxInstanceProperty {
                        name: "RotVelocity",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Rotation",
                    RbxInstanceProperty {
                        name: "Rotation",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "SpecificGravity",
                    RbxInstanceProperty {
                        name: "SpecificGravity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TopParamA",
                    RbxInstanceProperty {
                        name: "TopParamA",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TopParamB",
                    RbxInstanceProperty {
                        name: "TopParamB",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TopSurface",
                    RbxInstanceProperty {
                        name: "TopSurface",
                        value_type: "SurfaceType",
                    },
                );
                properties.insert(
                    "TopSurfaceInput",
                    RbxInstanceProperty {
                        name: "TopSurfaceInput",
                        value_type: "InputType",
                    },
                );
                properties.insert(
                    "Transparency",
                    RbxInstanceProperty {
                        name: "Transparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Velocity",
                    RbxInstanceProperty {
                        name: "Velocity",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "brickColor",
                    RbxInstanceProperty {
                        name: "brickColor",
                        value_type: "BrickColor",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "CornerWedgePart",
        RbxInstanceClass {
            name: "CornerWedgePart",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "FormFactorPart",
        RbxInstanceClass {
            name: "FormFactorPart",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "FormFactor",
                    RbxInstanceProperty {
                        name: "FormFactor",
                        value_type: "FormFactor",
                    },
                );
                properties.insert(
                    "formFactor",
                    RbxInstanceProperty {
                        name: "formFactor",
                        value_type: "FormFactor",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Part",
        RbxInstanceClass {
            name: "Part",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Shape",
                    RbxInstanceProperty {
                        name: "Shape",
                        value_type: "PartType",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "FlagStand",
        RbxInstanceClass {
            name: "FlagStand",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "TeamColor",
                    RbxInstanceProperty {
                        name: "TeamColor",
                        value_type: "BrickColor",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Platform",
        RbxInstanceClass {
            name: "Platform",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Seat",
        RbxInstanceClass {
            name: "Seat",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Disabled",
                    RbxInstanceProperty {
                        name: "Disabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Occupant",
                    RbxInstanceProperty {
                        name: "Occupant",
                        value_type: "Humanoid",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SkateboardPlatform",
        RbxInstanceClass {
            name: "SkateboardPlatform",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Controller",
                    RbxInstanceProperty {
                        name: "Controller",
                        value_type: "SkateboardController",
                    },
                );
                properties.insert(
                    "ControllingHumanoid",
                    RbxInstanceProperty {
                        name: "ControllingHumanoid",
                        value_type: "Humanoid",
                    },
                );
                properties.insert(
                    "Steer",
                    RbxInstanceProperty {
                        name: "Steer",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "StickyWheels",
                    RbxInstanceProperty {
                        name: "StickyWheels",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Throttle",
                    RbxInstanceProperty {
                        name: "Throttle",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SpawnLocation",
        RbxInstanceClass {
            name: "SpawnLocation",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AllowTeamChangeOnTouch",
                    RbxInstanceProperty {
                        name: "AllowTeamChangeOnTouch",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Duration",
                    RbxInstanceProperty {
                        name: "Duration",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Neutral",
                    RbxInstanceProperty {
                        name: "Neutral",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TeamColor",
                    RbxInstanceProperty {
                        name: "TeamColor",
                        value_type: "BrickColor",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "WedgePart",
        RbxInstanceClass {
            name: "WedgePart",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "MeshPart",
        RbxInstanceClass {
            name: "MeshPart",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CollisionFidelity",
                    RbxInstanceProperty {
                        name: "CollisionFidelity",
                        value_type: "CollisionFidelity",
                    },
                );
                properties.insert(
                    "MeshID",
                    RbxInstanceProperty {
                        name: "MeshID",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "MeshId",
                    RbxInstanceProperty {
                        name: "MeshId",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "TextureID",
                    RbxInstanceProperty {
                        name: "TextureID",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PartOperation",
        RbxInstanceClass {
            name: "PartOperation",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CollisionFidelity",
                    RbxInstanceProperty {
                        name: "CollisionFidelity",
                        value_type: "CollisionFidelity",
                    },
                );
                properties.insert(
                    "RenderFidelity",
                    RbxInstanceProperty {
                        name: "RenderFidelity",
                        value_type: "RenderFidelity",
                    },
                );
                properties.insert(
                    "TriangleCount",
                    RbxInstanceProperty {
                        name: "TriangleCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "UsePartColor",
                    RbxInstanceProperty {
                        name: "UsePartColor",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "NegateOperation",
        RbxInstanceClass {
            name: "NegateOperation",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "UnionOperation",
        RbxInstanceClass {
            name: "UnionOperation",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Terrain",
        RbxInstanceClass {
            name: "Terrain",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "IsSmooth",
                    RbxInstanceProperty {
                        name: "IsSmooth",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MaterialColors",
                    RbxInstanceProperty {
                        name: "MaterialColors",
                        value_type: "BinaryString",
                    },
                );
                properties.insert(
                    "MaxExtents",
                    RbxInstanceProperty {
                        name: "MaxExtents",
                        value_type: "Region3int16",
                    },
                );
                properties.insert(
                    "WaterColor",
                    RbxInstanceProperty {
                        name: "WaterColor",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "WaterReflectance",
                    RbxInstanceProperty {
                        name: "WaterReflectance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WaterTransparency",
                    RbxInstanceProperty {
                        name: "WaterTransparency",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WaterWaveSize",
                    RbxInstanceProperty {
                        name: "WaterWaveSize",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WaterWaveSpeed",
                    RbxInstanceProperty {
                        name: "WaterWaveSpeed",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TrussPart",
        RbxInstanceClass {
            name: "TrussPart",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Style",
                    RbxInstanceProperty {
                        name: "Style",
                        value_type: "Style",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "VehicleSeat",
        RbxInstanceClass {
            name: "VehicleSeat",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AreHingesDetected",
                    RbxInstanceProperty {
                        name: "AreHingesDetected",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Disabled",
                    RbxInstanceProperty {
                        name: "Disabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "HeadsUpDisplay",
                    RbxInstanceProperty {
                        name: "HeadsUpDisplay",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MaxSpeed",
                    RbxInstanceProperty {
                        name: "MaxSpeed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Occupant",
                    RbxInstanceProperty {
                        name: "Occupant",
                        value_type: "Humanoid",
                    },
                );
                properties.insert(
                    "Steer",
                    RbxInstanceProperty {
                        name: "Steer",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "SteerFloat",
                    RbxInstanceProperty {
                        name: "SteerFloat",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Throttle",
                    RbxInstanceProperty {
                        name: "Throttle",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ThrottleFloat",
                    RbxInstanceProperty {
                        name: "ThrottleFloat",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Torque",
                    RbxInstanceProperty {
                        name: "Torque",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TurnSpeed",
                    RbxInstanceProperty {
                        name: "TurnSpeed",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Model",
        RbxInstanceClass {
            name: "Model",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "PrimaryPart",
                    RbxInstanceProperty {
                        name: "PrimaryPart",
                        value_type: "BasePart",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Status",
        RbxInstanceClass {
            name: "Status",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Workspace",
        RbxInstanceClass {
            name: "Workspace",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AllowThirdPartySales",
                    RbxInstanceProperty {
                        name: "AllowThirdPartySales",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AutoJointsMode",
                    RbxInstanceProperty {
                        name: "AutoJointsMode",
                        value_type: "AutoJointsMode",
                    },
                );
                properties.insert(
                    "CurrentCamera",
                    RbxInstanceProperty {
                        name: "CurrentCamera",
                        value_type: "Camera",
                    },
                );
                properties.insert(
                    "DistributedGameTime",
                    RbxInstanceProperty {
                        name: "DistributedGameTime",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "FallenPartsDestroyHeight",
                    RbxInstanceProperty {
                        name: "FallenPartsDestroyHeight",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "FilteringEnabled",
                    RbxInstanceProperty {
                        name: "FilteringEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Gravity",
                    RbxInstanceProperty {
                        name: "Gravity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "PGSPhysicsSolverEnabled",
                    RbxInstanceProperty {
                        name: "PGSPhysicsSolverEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "StreamingEnabled",
                    RbxInstanceProperty {
                        name: "StreamingEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "StreamingMinRadius",
                    RbxInstanceProperty {
                        name: "StreamingMinRadius",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "StreamingTargetRadius",
                    RbxInstanceProperty {
                        name: "StreamingTargetRadius",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "TemporaryLegacyPhysicsSolverOverride",
                    RbxInstanceProperty {
                        name: "TemporaryLegacyPhysicsSolverOverride",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Terrain",
                    RbxInstanceProperty {
                        name: "Terrain",
                        value_type: "Instance",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PackageLink",
        RbxInstanceClass {
            name: "PackageLink",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "PackageId",
                    RbxInstanceProperty {
                        name: "PackageId",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "Status",
                    RbxInstanceProperty {
                        name: "Status",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "VersionNumber",
                    RbxInstanceProperty {
                        name: "VersionNumber",
                        value_type: "int64",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Pages",
        RbxInstanceClass {
            name: "Pages",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "IsFinished",
                    RbxInstanceProperty {
                        name: "IsFinished",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DataStorePages",
        RbxInstanceClass {
            name: "DataStorePages",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "FriendPages",
        RbxInstanceClass {
            name: "FriendPages",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "InventoryPages",
        RbxInstanceClass {
            name: "InventoryPages",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "StandardPages",
        RbxInstanceClass {
            name: "StandardPages",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "PartOperationAsset",
        RbxInstanceClass {
            name: "PartOperationAsset",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ParticleEmitter",
        RbxInstanceClass {
            name: "ParticleEmitter",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Acceleration",
                    RbxInstanceProperty {
                        name: "Acceleration",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "ColorSequence",
                    },
                );
                properties.insert(
                    "Drag",
                    RbxInstanceProperty {
                        name: "Drag",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "EmissionDirection",
                    RbxInstanceProperty {
                        name: "EmissionDirection",
                        value_type: "NormalId",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Lifetime",
                    RbxInstanceProperty {
                        name: "Lifetime",
                        value_type: "NumberRange",
                    },
                );
                properties.insert(
                    "LightEmission",
                    RbxInstanceProperty {
                        name: "LightEmission",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LightInfluence",
                    RbxInstanceProperty {
                        name: "LightInfluence",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LockedToPart",
                    RbxInstanceProperty {
                        name: "LockedToPart",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Rate",
                    RbxInstanceProperty {
                        name: "Rate",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "RotSpeed",
                    RbxInstanceProperty {
                        name: "RotSpeed",
                        value_type: "NumberRange",
                    },
                );
                properties.insert(
                    "Rotation",
                    RbxInstanceProperty {
                        name: "Rotation",
                        value_type: "NumberRange",
                    },
                );
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "NumberSequence",
                    },
                );
                properties.insert(
                    "Speed",
                    RbxInstanceProperty {
                        name: "Speed",
                        value_type: "NumberRange",
                    },
                );
                properties.insert(
                    "SpreadAngle",
                    RbxInstanceProperty {
                        name: "SpreadAngle",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "Texture",
                    RbxInstanceProperty {
                        name: "Texture",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "Transparency",
                    RbxInstanceProperty {
                        name: "Transparency",
                        value_type: "NumberSequence",
                    },
                );
                properties.insert(
                    "VelocityInheritance",
                    RbxInstanceProperty {
                        name: "VelocityInheritance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "VelocitySpread",
                    RbxInstanceProperty {
                        name: "VelocitySpread",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "ZOffset",
                    RbxInstanceProperty {
                        name: "ZOffset",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Path",
        RbxInstanceClass {
            name: "Path",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Status",
                    RbxInstanceProperty {
                        name: "Status",
                        value_type: "PathStatus",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PathfindingService",
        RbxInstanceClass {
            name: "PathfindingService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "EmptyCutoff",
                    RbxInstanceProperty {
                        name: "EmptyCutoff",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PhysicsPacketCache",
        RbxInstanceClass {
            name: "PhysicsPacketCache",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "PhysicsService",
        RbxInstanceClass {
            name: "PhysicsService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "PhysicsSettings",
        RbxInstanceClass {
            name: "PhysicsSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AllowSleep",
                    RbxInstanceProperty {
                        name: "AllowSleep",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreAnchorsShown",
                    RbxInstanceProperty {
                        name: "AreAnchorsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreAssembliesShown",
                    RbxInstanceProperty {
                        name: "AreAssembliesShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreAwakePartsHighlighted",
                    RbxInstanceProperty {
                        name: "AreAwakePartsHighlighted",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreBodyTypesShown",
                    RbxInstanceProperty {
                        name: "AreBodyTypesShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreContactIslandsShown",
                    RbxInstanceProperty {
                        name: "AreContactIslandsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreContactPointsShown",
                    RbxInstanceProperty {
                        name: "AreContactPointsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreJointCoordinatesShown",
                    RbxInstanceProperty {
                        name: "AreJointCoordinatesShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreMechanismsShown",
                    RbxInstanceProperty {
                        name: "AreMechanismsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreModelCoordsShown",
                    RbxInstanceProperty {
                        name: "AreModelCoordsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreOwnersShown",
                    RbxInstanceProperty {
                        name: "AreOwnersShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ArePartCoordsShown",
                    RbxInstanceProperty {
                        name: "ArePartCoordsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreRegionsShown",
                    RbxInstanceProperty {
                        name: "AreRegionsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreUnalignedPartsShown",
                    RbxInstanceProperty {
                        name: "AreUnalignedPartsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AreWorldCoordsShown",
                    RbxInstanceProperty {
                        name: "AreWorldCoordsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "DisableCSGv2",
                    RbxInstanceProperty {
                        name: "DisableCSGv2",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsReceiveAgeShown",
                    RbxInstanceProperty {
                        name: "IsReceiveAgeShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsTreeShown",
                    RbxInstanceProperty {
                        name: "IsTreeShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PhysicsEnvironmentalThrottle",
                    RbxInstanceProperty {
                        name: "PhysicsEnvironmentalThrottle",
                        value_type: "EnviromentalPhysicsThrottle",
                    },
                );
                properties.insert(
                    "ShowDecompositionGeometry",
                    RbxInstanceProperty {
                        name: "ShowDecompositionGeometry",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ThrottleAdjustTime",
                    RbxInstanceProperty {
                        name: "ThrottleAdjustTime",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "UseCSGv2",
                    RbxInstanceProperty {
                        name: "UseCSGv2",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Player",
        RbxInstanceClass {
            name: "Player",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AccountAge",
                    RbxInstanceProperty {
                        name: "AccountAge",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "AppearanceDidLoad",
                    RbxInstanceProperty {
                        name: "AppearanceDidLoad",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AutoJumpEnabled",
                    RbxInstanceProperty {
                        name: "AutoJumpEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CameraMaxZoomDistance",
                    RbxInstanceProperty {
                        name: "CameraMaxZoomDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "CameraMinZoomDistance",
                    RbxInstanceProperty {
                        name: "CameraMinZoomDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "CameraMode",
                    RbxInstanceProperty {
                        name: "CameraMode",
                        value_type: "CameraMode",
                    },
                );
                properties.insert(
                    "CanLoadCharacterAppearance",
                    RbxInstanceProperty {
                        name: "CanLoadCharacterAppearance",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Character",
                    RbxInstanceProperty {
                        name: "Character",
                        value_type: "Model",
                    },
                );
                properties.insert(
                    "CharacterAppearance",
                    RbxInstanceProperty {
                        name: "CharacterAppearance",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "CharacterAppearanceId",
                    RbxInstanceProperty {
                        name: "CharacterAppearanceId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "ChatMode",
                    RbxInstanceProperty {
                        name: "ChatMode",
                        value_type: "ChatMode",
                    },
                );
                properties.insert(
                    "DataComplexity",
                    RbxInstanceProperty {
                        name: "DataComplexity",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "DataComplexityLimit",
                    RbxInstanceProperty {
                        name: "DataComplexityLimit",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "DataReady",
                    RbxInstanceProperty {
                        name: "DataReady",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "DevCameraOcclusionMode",
                    RbxInstanceProperty {
                        name: "DevCameraOcclusionMode",
                        value_type: "DevCameraOcclusionMode",
                    },
                );
                properties.insert(
                    "DevComputerCameraMode",
                    RbxInstanceProperty {
                        name: "DevComputerCameraMode",
                        value_type: "DevComputerCameraMovementMode",
                    },
                );
                properties.insert(
                    "DevComputerMovementMode",
                    RbxInstanceProperty {
                        name: "DevComputerMovementMode",
                        value_type: "DevComputerMovementMode",
                    },
                );
                properties.insert(
                    "DevEnableMouseLock",
                    RbxInstanceProperty {
                        name: "DevEnableMouseLock",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "DevTouchCameraMode",
                    RbxInstanceProperty {
                        name: "DevTouchCameraMode",
                        value_type: "DevTouchCameraMovementMode",
                    },
                );
                properties.insert(
                    "DevTouchMovementMode",
                    RbxInstanceProperty {
                        name: "DevTouchMovementMode",
                        value_type: "DevTouchMovementMode",
                    },
                );
                properties.insert(
                    "DisplayName",
                    RbxInstanceProperty {
                        name: "DisplayName",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "FollowUserId",
                    RbxInstanceProperty {
                        name: "FollowUserId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "Guest",
                    RbxInstanceProperty {
                        name: "Guest",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "HealthDisplayDistance",
                    RbxInstanceProperty {
                        name: "HealthDisplayDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LocaleId",
                    RbxInstanceProperty {
                        name: "LocaleId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "MaximumSimulationRadius",
                    RbxInstanceProperty {
                        name: "MaximumSimulationRadius",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MembershipType",
                    RbxInstanceProperty {
                        name: "MembershipType",
                        value_type: "MembershipType",
                    },
                );
                properties.insert(
                    "NameDisplayDistance",
                    RbxInstanceProperty {
                        name: "NameDisplayDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Neutral",
                    RbxInstanceProperty {
                        name: "Neutral",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "OsPlatform",
                    RbxInstanceProperty {
                        name: "OsPlatform",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "ReplicationFocus",
                    RbxInstanceProperty {
                        name: "ReplicationFocus",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "RespawnLocation",
                    RbxInstanceProperty {
                        name: "RespawnLocation",
                        value_type: "SpawnLocation",
                    },
                );
                properties.insert(
                    "SimulationRadius",
                    RbxInstanceProperty {
                        name: "SimulationRadius",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Team",
                    RbxInstanceProperty {
                        name: "Team",
                        value_type: "Team",
                    },
                );
                properties.insert(
                    "TeamColor",
                    RbxInstanceProperty {
                        name: "TeamColor",
                        value_type: "BrickColor",
                    },
                );
                properties.insert(
                    "Teleported",
                    RbxInstanceProperty {
                        name: "Teleported",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TeleportedIn",
                    RbxInstanceProperty {
                        name: "TeleportedIn",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "UserId",
                    RbxInstanceProperty {
                        name: "UserId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "VRDevice",
                    RbxInstanceProperty {
                        name: "VRDevice",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "userId",
                    RbxInstanceProperty {
                        name: "userId",
                        value_type: "int64",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PlayerScripts",
        RbxInstanceClass {
            name: "PlayerScripts",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Players",
        RbxInstanceClass {
            name: "Players",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "BubbleChat",
                    RbxInstanceProperty {
                        name: "BubbleChat",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CharacterAutoLoads",
                    RbxInstanceProperty {
                        name: "CharacterAutoLoads",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ClassicChat",
                    RbxInstanceProperty {
                        name: "ClassicChat",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LocalPlayer",
                    RbxInstanceProperty {
                        name: "LocalPlayer",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "MaxPlayers",
                    RbxInstanceProperty {
                        name: "MaxPlayers",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "MaxPlayersInternal",
                    RbxInstanceProperty {
                        name: "MaxPlayersInternal",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "NumPlayers",
                    RbxInstanceProperty {
                        name: "NumPlayers",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "PreferredPlayers",
                    RbxInstanceProperty {
                        name: "PreferredPlayers",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "PreferredPlayersInternal",
                    RbxInstanceProperty {
                        name: "PreferredPlayersInternal",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "RespawnTime",
                    RbxInstanceProperty {
                        name: "RespawnTime",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "localPlayer",
                    RbxInstanceProperty {
                        name: "localPlayer",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "numPlayers",
                    RbxInstanceProperty {
                        name: "numPlayers",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Plugin",
        RbxInstanceClass {
            name: "Plugin",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CollisionEnabled",
                    RbxInstanceProperty {
                        name: "CollisionEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "GridSize",
                    RbxInstanceProperty {
                        name: "GridSize",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "UsesAssetInsertionDrag",
                    RbxInstanceProperty {
                        name: "UsesAssetInsertionDrag",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PluginAction",
        RbxInstanceClass {
            name: "PluginAction",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ActionId",
                    RbxInstanceProperty {
                        name: "ActionId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "AllowBinding",
                    RbxInstanceProperty {
                        name: "AllowBinding",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "StatusTip",
                    RbxInstanceProperty {
                        name: "StatusTip",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Text",
                    RbxInstanceProperty {
                        name: "Text",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PluginDragEvent",
        RbxInstanceClass {
            name: "PluginDragEvent",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Data",
                    RbxInstanceProperty {
                        name: "Data",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "MimeType",
                    RbxInstanceProperty {
                        name: "MimeType",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Position",
                    RbxInstanceProperty {
                        name: "Position",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "Sender",
                    RbxInstanceProperty {
                        name: "Sender",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PluginGuiService",
        RbxInstanceClass {
            name: "PluginGuiService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "PluginManager",
        RbxInstanceClass {
            name: "PluginManager",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "PluginMenu",
        RbxInstanceClass {
            name: "PluginMenu",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Icon",
                    RbxInstanceProperty {
                        name: "Icon",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Title",
                    RbxInstanceProperty {
                        name: "Title",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PluginToolbar",
        RbxInstanceClass {
            name: "PluginToolbar",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "PluginToolbarButton",
        RbxInstanceClass {
            name: "PluginToolbarButton",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ClickableWhenViewportHidden",
                    RbxInstanceProperty {
                        name: "ClickableWhenViewportHidden",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Icon",
                    RbxInstanceProperty {
                        name: "Icon",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PointsService",
        RbxInstanceClass {
            name: "PointsService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Pose",
        RbxInstanceClass {
            name: "Pose",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CFrame",
                    RbxInstanceProperty {
                        name: "CFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "EasingDirection",
                    RbxInstanceProperty {
                        name: "EasingDirection",
                        value_type: "PoseEasingDirection",
                    },
                );
                properties.insert(
                    "EasingStyle",
                    RbxInstanceProperty {
                        name: "EasingStyle",
                        value_type: "PoseEasingStyle",
                    },
                );
                properties.insert(
                    "MaskWeight",
                    RbxInstanceProperty {
                        name: "MaskWeight",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Weight",
                    RbxInstanceProperty {
                        name: "Weight",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PostEffect",
        RbxInstanceClass {
            name: "PostEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BloomEffect",
        RbxInstanceClass {
            name: "BloomEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Intensity",
                    RbxInstanceProperty {
                        name: "Intensity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Threshold",
                    RbxInstanceProperty {
                        name: "Threshold",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BlurEffect",
        RbxInstanceClass {
            name: "BlurEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ColorCorrectionEffect",
        RbxInstanceClass {
            name: "ColorCorrectionEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Brightness",
                    RbxInstanceProperty {
                        name: "Brightness",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Contrast",
                    RbxInstanceProperty {
                        name: "Contrast",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Saturation",
                    RbxInstanceProperty {
                        name: "Saturation",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TintColor",
                    RbxInstanceProperty {
                        name: "TintColor",
                        value_type: "Color3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SunRaysEffect",
        RbxInstanceClass {
            name: "SunRaysEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Intensity",
                    RbxInstanceProperty {
                        name: "Intensity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Spread",
                    RbxInstanceProperty {
                        name: "Spread",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadata",
        RbxInstanceClass {
            name: "ReflectionMetadata",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataCallbacks",
        RbxInstanceClass {
            name: "ReflectionMetadataCallbacks",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataClasses",
        RbxInstanceClass {
            name: "ReflectionMetadataClasses",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataEnums",
        RbxInstanceClass {
            name: "ReflectionMetadataEnums",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataEvents",
        RbxInstanceClass {
            name: "ReflectionMetadataEvents",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataFunctions",
        RbxInstanceClass {
            name: "ReflectionMetadataFunctions",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataItem",
        RbxInstanceClass {
            name: "ReflectionMetadataItem",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Browsable",
                    RbxInstanceProperty {
                        name: "Browsable",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ClassCategory",
                    RbxInstanceProperty {
                        name: "ClassCategory",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "ClientOnly",
                    RbxInstanceProperty {
                        name: "ClientOnly",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Constraint",
                    RbxInstanceProperty {
                        name: "Constraint",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "Deprecated",
                    RbxInstanceProperty {
                        name: "Deprecated",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "EditingDisabled",
                    RbxInstanceProperty {
                        name: "EditingDisabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsBackend",
                    RbxInstanceProperty {
                        name: "IsBackend",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ScriptContext",
                    RbxInstanceProperty {
                        name: "ScriptContext",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "ServerOnly",
                    RbxInstanceProperty {
                        name: "ServerOnly",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "UIMaximum",
                    RbxInstanceProperty {
                        name: "UIMaximum",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "UIMinimum",
                    RbxInstanceProperty {
                        name: "UIMinimum",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "UINumTicks",
                    RbxInstanceProperty {
                        name: "UINumTicks",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "summary",
                    RbxInstanceProperty {
                        name: "summary",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataClass",
        RbxInstanceClass {
            name: "ReflectionMetadataClass",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ExplorerImageIndex",
                    RbxInstanceProperty {
                        name: "ExplorerImageIndex",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ExplorerOrder",
                    RbxInstanceProperty {
                        name: "ExplorerOrder",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Insertable",
                    RbxInstanceProperty {
                        name: "Insertable",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PreferredParent",
                    RbxInstanceProperty {
                        name: "PreferredParent",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "PreferredParents",
                    RbxInstanceProperty {
                        name: "PreferredParents",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataEnum",
        RbxInstanceClass {
            name: "ReflectionMetadataEnum",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataEnumItem",
        RbxInstanceClass {
            name: "ReflectionMetadataEnumItem",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataMember",
        RbxInstanceClass {
            name: "ReflectionMetadataMember",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataProperties",
        RbxInstanceClass {
            name: "ReflectionMetadataProperties",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReflectionMetadataYieldFunctions",
        RbxInstanceClass {
            name: "ReflectionMetadataYieldFunctions",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RemoteEvent",
        RbxInstanceClass {
            name: "RemoteEvent",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RemoteFunction",
        RbxInstanceClass {
            name: "RemoteFunction",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RenderSettings",
        RbxInstanceClass {
            name: "RenderSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AutoFRMLevel",
                    RbxInstanceProperty {
                        name: "AutoFRMLevel",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "EagerBulkExecution",
                    RbxInstanceProperty {
                        name: "EagerBulkExecution",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "EditQualityLevel",
                    RbxInstanceProperty {
                        name: "EditQualityLevel",
                        value_type: "QualityLevel",
                    },
                );
                properties.insert(
                    "EnableFRM",
                    RbxInstanceProperty {
                        name: "EnableFRM",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ExportMergeByMaterial",
                    RbxInstanceProperty {
                        name: "ExportMergeByMaterial",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "FrameRateManager",
                    RbxInstanceProperty {
                        name: "FrameRateManager",
                        value_type: "FramerateManagerMode",
                    },
                );
                properties.insert(
                    "GraphicsMode",
                    RbxInstanceProperty {
                        name: "GraphicsMode",
                        value_type: "GraphicsMode",
                    },
                );
                properties.insert(
                    "MeshCacheSize",
                    RbxInstanceProperty {
                        name: "MeshCacheSize",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "QualityLevel",
                    RbxInstanceProperty {
                        name: "QualityLevel",
                        value_type: "QualityLevel",
                    },
                );
                properties.insert(
                    "ReloadAssets",
                    RbxInstanceProperty {
                        name: "ReloadAssets",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RenderCSGTrianglesDebug",
                    RbxInstanceProperty {
                        name: "RenderCSGTrianglesDebug",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ShowBoundingBoxes",
                    RbxInstanceProperty {
                        name: "ShowBoundingBoxes",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "RenderingTest",
        RbxInstanceClass {
            name: "RenderingTest",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CFrame",
                    RbxInstanceProperty {
                        name: "CFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "ComparisonDiffThreshold",
                    RbxInstanceProperty {
                        name: "ComparisonDiffThreshold",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ComparisonMethod",
                    RbxInstanceProperty {
                        name: "ComparisonMethod",
                        value_type: "RenderingTestComparisonMethod",
                    },
                );
                properties.insert(
                    "ComparisonPsnrThreshold",
                    RbxInstanceProperty {
                        name: "ComparisonPsnrThreshold",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Description",
                    RbxInstanceProperty {
                        name: "Description",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "FieldOfView",
                    RbxInstanceProperty {
                        name: "FieldOfView",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Orientation",
                    RbxInstanceProperty {
                        name: "Orientation",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "Position",
                    RbxInstanceProperty {
                        name: "Position",
                        value_type: "Vector3",
                    },
                );
                properties.insert(
                    "QualityLevel",
                    RbxInstanceProperty {
                        name: "QualityLevel",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ShouldSkip",
                    RbxInstanceProperty {
                        name: "ShouldSkip",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Ticket",
                    RbxInstanceProperty {
                        name: "Ticket",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ReplicatedFirst",
        RbxInstanceClass {
            name: "ReplicatedFirst",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ReplicatedStorage",
        RbxInstanceClass {
            name: "ReplicatedStorage",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RobloxReplicatedStorage",
        RbxInstanceClass {
            name: "RobloxReplicatedStorage",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RunService",
        RbxInstanceClass {
            name: "RunService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RuntimeScriptService",
        RbxInstanceClass {
            name: "RuntimeScriptService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ScriptContext",
        RbxInstanceClass {
            name: "ScriptContext",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ScriptsDisabled",
                    RbxInstanceProperty {
                        name: "ScriptsDisabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ScriptDebugger",
        RbxInstanceClass {
            name: "ScriptDebugger",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CurrentLine",
                    RbxInstanceProperty {
                        name: "CurrentLine",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "IsDebugging",
                    RbxInstanceProperty {
                        name: "IsDebugging",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsPaused",
                    RbxInstanceProperty {
                        name: "IsPaused",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Script",
                    RbxInstanceProperty {
                        name: "Script",
                        value_type: "Instance",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ScriptService",
        RbxInstanceClass {
            name: "ScriptService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Selection",
        RbxInstanceClass {
            name: "Selection",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ServerScriptService",
        RbxInstanceClass {
            name: "ServerScriptService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "LoadStringEnabled",
                    RbxInstanceProperty {
                        name: "LoadStringEnabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ServerStorage",
        RbxInstanceClass {
            name: "ServerStorage",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ServiceProvider",
        RbxInstanceClass {
            name: "ServiceProvider",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "DataModel",
        RbxInstanceClass {
            name: "DataModel",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CreatorId",
                    RbxInstanceProperty {
                        name: "CreatorId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "CreatorType",
                    RbxInstanceProperty {
                        name: "CreatorType",
                        value_type: "CreatorType",
                    },
                );
                properties.insert(
                    "GameId",
                    RbxInstanceProperty {
                        name: "GameId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "GearGenreSetting",
                    RbxInstanceProperty {
                        name: "GearGenreSetting",
                        value_type: "GearGenreSetting",
                    },
                );
                properties.insert(
                    "Genre",
                    RbxInstanceProperty {
                        name: "Genre",
                        value_type: "Genre",
                    },
                );
                properties.insert(
                    "IsSFFlagsLoaded",
                    RbxInstanceProperty {
                        name: "IsSFFlagsLoaded",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "JobId",
                    RbxInstanceProperty {
                        name: "JobId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "PlaceId",
                    RbxInstanceProperty {
                        name: "PlaceId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "PlaceVersion",
                    RbxInstanceProperty {
                        name: "PlaceVersion",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "PrivateServerId",
                    RbxInstanceProperty {
                        name: "PrivateServerId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "PrivateServerOwnerId",
                    RbxInstanceProperty {
                        name: "PrivateServerOwnerId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "VIPServerId",
                    RbxInstanceProperty {
                        name: "VIPServerId",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "VIPServerOwnerId",
                    RbxInstanceProperty {
                        name: "VIPServerOwnerId",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "Workspace",
                    RbxInstanceProperty {
                        name: "Workspace",
                        value_type: "Workspace",
                    },
                );
                properties.insert(
                    "lighting",
                    RbxInstanceProperty {
                        name: "lighting",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "workspace",
                    RbxInstanceProperty {
                        name: "workspace",
                        value_type: "Workspace",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "GenericSettings",
        RbxInstanceClass {
            name: "GenericSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "AnalysticsSettings",
        RbxInstanceClass {
            name: "AnalysticsSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "GlobalSettings",
        RbxInstanceClass {
            name: "GlobalSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "UserSettings",
        RbxInstanceClass {
            name: "UserSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Sky",
        RbxInstanceClass {
            name: "Sky",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CelestialBodiesShown",
                    RbxInstanceProperty {
                        name: "CelestialBodiesShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MoonAngularSize",
                    RbxInstanceProperty {
                        name: "MoonAngularSize",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MoonTextureId",
                    RbxInstanceProperty {
                        name: "MoonTextureId",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "SkyboxBk",
                    RbxInstanceProperty {
                        name: "SkyboxBk",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "SkyboxDn",
                    RbxInstanceProperty {
                        name: "SkyboxDn",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "SkyboxFt",
                    RbxInstanceProperty {
                        name: "SkyboxFt",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "SkyboxLf",
                    RbxInstanceProperty {
                        name: "SkyboxLf",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "SkyboxRt",
                    RbxInstanceProperty {
                        name: "SkyboxRt",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "SkyboxUp",
                    RbxInstanceProperty {
                        name: "SkyboxUp",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "StarCount",
                    RbxInstanceProperty {
                        name: "StarCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "SunAngularSize",
                    RbxInstanceProperty {
                        name: "SunAngularSize",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "SunTextureId",
                    RbxInstanceProperty {
                        name: "SunTextureId",
                        value_type: "Content",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Smoke",
        RbxInstanceClass {
            name: "Smoke",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Opacity",
                    RbxInstanceProperty {
                        name: "Opacity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "RiseVelocity",
                    RbxInstanceProperty {
                        name: "RiseVelocity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Size",
                    RbxInstanceProperty {
                        name: "Size",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SocialService",
        RbxInstanceClass {
            name: "SocialService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Sound",
        RbxInstanceClass {
            name: "Sound",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "EmitterSize",
                    RbxInstanceProperty {
                        name: "EmitterSize",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "IsLoaded",
                    RbxInstanceProperty {
                        name: "IsLoaded",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsPaused",
                    RbxInstanceProperty {
                        name: "IsPaused",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsPlaying",
                    RbxInstanceProperty {
                        name: "IsPlaying",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Looped",
                    RbxInstanceProperty {
                        name: "Looped",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MaxDistance",
                    RbxInstanceProperty {
                        name: "MaxDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MinDistance",
                    RbxInstanceProperty {
                        name: "MinDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Pitch",
                    RbxInstanceProperty {
                        name: "Pitch",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "PlayOnRemove",
                    RbxInstanceProperty {
                        name: "PlayOnRemove",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "PlaybackLoudness",
                    RbxInstanceProperty {
                        name: "PlaybackLoudness",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "PlaybackSpeed",
                    RbxInstanceProperty {
                        name: "PlaybackSpeed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Playing",
                    RbxInstanceProperty {
                        name: "Playing",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RollOffMode",
                    RbxInstanceProperty {
                        name: "RollOffMode",
                        value_type: "RollOffMode",
                    },
                );
                properties.insert(
                    "SoundGroup",
                    RbxInstanceProperty {
                        name: "SoundGroup",
                        value_type: "SoundGroup",
                    },
                );
                properties.insert(
                    "SoundId",
                    RbxInstanceProperty {
                        name: "SoundId",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "TimeLength",
                    RbxInstanceProperty {
                        name: "TimeLength",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "TimePosition",
                    RbxInstanceProperty {
                        name: "TimePosition",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "Volume",
                    RbxInstanceProperty {
                        name: "Volume",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "isPlaying",
                    RbxInstanceProperty {
                        name: "isPlaying",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SoundEffect",
        RbxInstanceClass {
            name: "SoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Priority",
                    RbxInstanceProperty {
                        name: "Priority",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ChorusSoundEffect",
        RbxInstanceClass {
            name: "ChorusSoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Depth",
                    RbxInstanceProperty {
                        name: "Depth",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Mix",
                    RbxInstanceProperty {
                        name: "Mix",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Rate",
                    RbxInstanceProperty {
                        name: "Rate",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "CompressorSoundEffect",
        RbxInstanceClass {
            name: "CompressorSoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Attack",
                    RbxInstanceProperty {
                        name: "Attack",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "GainMakeup",
                    RbxInstanceProperty {
                        name: "GainMakeup",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Ratio",
                    RbxInstanceProperty {
                        name: "Ratio",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Release",
                    RbxInstanceProperty {
                        name: "Release",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "SideChain",
                    RbxInstanceProperty {
                        name: "SideChain",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "Threshold",
                    RbxInstanceProperty {
                        name: "Threshold",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DistortionSoundEffect",
        RbxInstanceClass {
            name: "DistortionSoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Level",
                    RbxInstanceProperty {
                        name: "Level",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "EchoSoundEffect",
        RbxInstanceClass {
            name: "EchoSoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Delay",
                    RbxInstanceProperty {
                        name: "Delay",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "DryLevel",
                    RbxInstanceProperty {
                        name: "DryLevel",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Feedback",
                    RbxInstanceProperty {
                        name: "Feedback",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WetLevel",
                    RbxInstanceProperty {
                        name: "WetLevel",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "EqualizerSoundEffect",
        RbxInstanceClass {
            name: "EqualizerSoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "HighGain",
                    RbxInstanceProperty {
                        name: "HighGain",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LowGain",
                    RbxInstanceProperty {
                        name: "LowGain",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MidGain",
                    RbxInstanceProperty {
                        name: "MidGain",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "FlangeSoundEffect",
        RbxInstanceClass {
            name: "FlangeSoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Depth",
                    RbxInstanceProperty {
                        name: "Depth",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Mix",
                    RbxInstanceProperty {
                        name: "Mix",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Rate",
                    RbxInstanceProperty {
                        name: "Rate",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "PitchShiftSoundEffect",
        RbxInstanceClass {
            name: "PitchShiftSoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Octave",
                    RbxInstanceProperty {
                        name: "Octave",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ReverbSoundEffect",
        RbxInstanceClass {
            name: "ReverbSoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "DecayTime",
                    RbxInstanceProperty {
                        name: "DecayTime",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Density",
                    RbxInstanceProperty {
                        name: "Density",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Diffusion",
                    RbxInstanceProperty {
                        name: "Diffusion",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "DryLevel",
                    RbxInstanceProperty {
                        name: "DryLevel",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "WetLevel",
                    RbxInstanceProperty {
                        name: "WetLevel",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TremoloSoundEffect",
        RbxInstanceClass {
            name: "TremoloSoundEffect",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Depth",
                    RbxInstanceProperty {
                        name: "Depth",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Duty",
                    RbxInstanceProperty {
                        name: "Duty",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Frequency",
                    RbxInstanceProperty {
                        name: "Frequency",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SoundGroup",
        RbxInstanceClass {
            name: "SoundGroup",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Volume",
                    RbxInstanceProperty {
                        name: "Volume",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SoundService",
        RbxInstanceClass {
            name: "SoundService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AmbientReverb",
                    RbxInstanceProperty {
                        name: "AmbientReverb",
                        value_type: "ReverbType",
                    },
                );
                properties.insert(
                    "DistanceFactor",
                    RbxInstanceProperty {
                        name: "DistanceFactor",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "DopplerScale",
                    RbxInstanceProperty {
                        name: "DopplerScale",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "RespectFilteringEnabled",
                    RbxInstanceProperty {
                        name: "RespectFilteringEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RolloffScale",
                    RbxInstanceProperty {
                        name: "RolloffScale",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Sparkles",
        RbxInstanceClass {
            name: "Sparkles",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "SparkleColor",
                    RbxInstanceProperty {
                        name: "SparkleColor",
                        value_type: "Color3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "SpawnerService",
        RbxInstanceClass {
            name: "SpawnerService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "StarterGear",
        RbxInstanceClass {
            name: "StarterGear",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "StarterPlayer",
        RbxInstanceClass {
            name: "StarterPlayer",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AllowCustomAnimations",
                    RbxInstanceProperty {
                        name: "AllowCustomAnimations",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AutoJumpEnabled",
                    RbxInstanceProperty {
                        name: "AutoJumpEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CameraMaxZoomDistance",
                    RbxInstanceProperty {
                        name: "CameraMaxZoomDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "CameraMinZoomDistance",
                    RbxInstanceProperty {
                        name: "CameraMinZoomDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "CameraMode",
                    RbxInstanceProperty {
                        name: "CameraMode",
                        value_type: "CameraMode",
                    },
                );
                properties.insert(
                    "DevCameraOcclusionMode",
                    RbxInstanceProperty {
                        name: "DevCameraOcclusionMode",
                        value_type: "DevCameraOcclusionMode",
                    },
                );
                properties.insert(
                    "DevComputerCameraMovementMode",
                    RbxInstanceProperty {
                        name: "DevComputerCameraMovementMode",
                        value_type: "DevComputerCameraMovementMode",
                    },
                );
                properties.insert(
                    "DevComputerMovementMode",
                    RbxInstanceProperty {
                        name: "DevComputerMovementMode",
                        value_type: "DevComputerMovementMode",
                    },
                );
                properties.insert(
                    "DevTouchCameraMovementMode",
                    RbxInstanceProperty {
                        name: "DevTouchCameraMovementMode",
                        value_type: "DevTouchCameraMovementMode",
                    },
                );
                properties.insert(
                    "DevTouchMovementMode",
                    RbxInstanceProperty {
                        name: "DevTouchMovementMode",
                        value_type: "DevTouchMovementMode",
                    },
                );
                properties.insert(
                    "EnableMouseLockOption",
                    RbxInstanceProperty {
                        name: "EnableMouseLockOption",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "HealthDisplayDistance",
                    RbxInstanceProperty {
                        name: "HealthDisplayDistance",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LoadCharacterAppearance",
                    RbxInstanceProperty {
                        name: "LoadCharacterAppearance",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "NameDisplayDistance",
                    RbxInstanceProperty {
                        name: "NameDisplayDistance",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "StarterPlayerScripts",
        RbxInstanceClass {
            name: "StarterPlayerScripts",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "StarterCharacterScripts",
        RbxInstanceClass {
            name: "StarterCharacterScripts",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Stats",
        RbxInstanceClass {
            name: "Stats",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ContactsCount",
                    RbxInstanceProperty {
                        name: "ContactsCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "DataReceiveKbps",
                    RbxInstanceProperty {
                        name: "DataReceiveKbps",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "DataSendKbps",
                    RbxInstanceProperty {
                        name: "DataSendKbps",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "HeartbeatTimeMs",
                    RbxInstanceProperty {
                        name: "HeartbeatTimeMs",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "InstanceCount",
                    RbxInstanceProperty {
                        name: "InstanceCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "MovingPrimitivesCount",
                    RbxInstanceProperty {
                        name: "MovingPrimitivesCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "PhysicsReceiveKbps",
                    RbxInstanceProperty {
                        name: "PhysicsReceiveKbps",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "PhysicsSendKbps",
                    RbxInstanceProperty {
                        name: "PhysicsSendKbps",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "PhysicsStepTimeMs",
                    RbxInstanceProperty {
                        name: "PhysicsStepTimeMs",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "PrimitivesCount",
                    RbxInstanceProperty {
                        name: "PrimitivesCount",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "StatsItem",
        RbxInstanceClass {
            name: "StatsItem",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RunningAverageItemDouble",
        RbxInstanceClass {
            name: "RunningAverageItemDouble",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RunningAverageItemInt",
        RbxInstanceClass {
            name: "RunningAverageItemInt",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "RunningAverageTimeIntervalItem",
        RbxInstanceClass {
            name: "RunningAverageTimeIntervalItem",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "TotalCountTimeIntervalItem",
        RbxInstanceClass {
            name: "TotalCountTimeIntervalItem",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "StopWatchReporter",
        RbxInstanceClass {
            name: "StopWatchReporter",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Studio",
        RbxInstanceClass {
            name: "Studio",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Always Save Script Changes",
                    RbxInstanceProperty {
                        name: "Always Save Script Changes",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Animate Hover Over",
                    RbxInstanceProperty {
                        name: "Animate Hover Over",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Attach Debugger To",
                    RbxInstanceProperty {
                        name: "Attach Debugger To",
                        value_type: "DEPRECATED_DebuggerDataModelPreference",
                    },
                );
                properties.insert(
                    "Auto Indent",
                    RbxInstanceProperty {
                        name: "Auto Indent",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Auto-Save Enabled",
                    RbxInstanceProperty {
                        name: "Auto-Save Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Auto-Save Interval (Minutes)",
                    RbxInstanceProperty {
                        name: "Auto-Save Interval (Minutes)",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Auto-Save Path",
                    RbxInstanceProperty {
                        name: "Auto-Save Path",
                        value_type: "QDir",
                    },
                );
                properties.insert(
                    "Background Color",
                    RbxInstanceProperty {
                        name: "Background Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Basic Objects Display Mode",
                    RbxInstanceProperty {
                        name: "Basic Objects Display Mode",
                        value_type: "ListDisplayMode",
                    },
                );
                properties.insert(
                    "Built-in Function Color",
                    RbxInstanceProperty {
                        name: "Built-in Function Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Camera Mouse Wheel Speed",
                    RbxInstanceProperty {
                        name: "Camera Mouse Wheel Speed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Camera Shift Speed",
                    RbxInstanceProperty {
                        name: "Camera Shift Speed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Camera Speed",
                    RbxInstanceProperty {
                        name: "Camera Speed",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Camera Zoom to Mouse Position",
                    RbxInstanceProperty {
                        name: "Camera Zoom to Mouse Position",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Clear Output On Start",
                    RbxInstanceProperty {
                        name: "Clear Output On Start",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Comment Color",
                    RbxInstanceProperty {
                        name: "Comment Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "DefaultScriptFileDir",
                    RbxInstanceProperty {
                        name: "DefaultScriptFileDir",
                        value_type: "QDir",
                    },
                );
                properties.insert(
                    "DeprecatedObjectsShown",
                    RbxInstanceProperty {
                        name: "DeprecatedObjectsShown",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Device Pairing Code",
                    RbxInstanceProperty {
                        name: "Device Pairing Code",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Disable Accurate Play Solo",
                    RbxInstanceProperty {
                        name: "Disable Accurate Play Solo",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Drag Multiple Parts As Single Part",
                    RbxInstanceProperty {
                        name: "Drag Multiple Parts As Single Part",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Enable Autocomplete",
                    RbxInstanceProperty {
                        name: "Enable Autocomplete",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Enable CoreScript Debugger",
                    RbxInstanceProperty {
                        name: "Enable CoreScript Debugger",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Enable Intellisense",
                    RbxInstanceProperty {
                        name: "Enable Intellisense",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Error Color",
                    RbxInstanceProperty {
                        name: "Error Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Find Selection Background Color",
                    RbxInstanceProperty {
                        name: "Find Selection Background Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Font",
                    RbxInstanceProperty {
                        name: "Font",
                        value_type: "QFont",
                    },
                );
                properties.insert(
                    "Hover Animate Speed",
                    RbxInstanceProperty {
                        name: "Hover Animate Speed",
                        value_type: "HoverAnimateSpeed",
                    },
                );
                properties.insert(
                    "Hover Over Color",
                    RbxInstanceProperty {
                        name: "Hover Over Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Import mesh files as single mesh",
                    RbxInstanceProperty {
                        name: "Import mesh files as single mesh",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Keyword Color",
                    RbxInstanceProperty {
                        name: "Keyword Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Line Thickness",
                    RbxInstanceProperty {
                        name: "Line Thickness",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LuaDebuggerEnabled",
                    RbxInstanceProperty {
                        name: "LuaDebuggerEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LuaDebuggerEnabledAtStartup",
                    RbxInstanceProperty {
                        name: "LuaDebuggerEnabledAtStartup",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Matching Word Background Color",
                    RbxInstanceProperty {
                        name: "Matching Word Background Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Maximum Output Lines",
                    RbxInstanceProperty {
                        name: "Maximum Output Lines",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Number Color",
                    RbxInstanceProperty {
                        name: "Number Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Only Play Audio from Window in Focus",
                    RbxInstanceProperty {
                        name: "Only Play Audio from Window in Focus",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Operator Color",
                    RbxInstanceProperty {
                        name: "Operator Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Output Font",
                    RbxInstanceProperty {
                        name: "Output Font",
                        value_type: "QFont",
                    },
                );
                properties.insert(
                    "Output Layout Mode",
                    RbxInstanceProperty {
                        name: "Output Layout Mode",
                        value_type: "OutputLayoutMode",
                    },
                );
                properties.insert(
                    "OverrideCoreScripts",
                    RbxInstanceProperty {
                        name: "OverrideCoreScripts",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "OverrideCoreScriptsDir",
                    RbxInstanceProperty {
                        name: "OverrideCoreScriptsDir",
                        value_type: "QDir",
                    },
                );
                properties.insert(
                    "PermissionLevelShown",
                    RbxInstanceProperty {
                        name: "PermissionLevelShown",
                        value_type: "PermissionLevelShown",
                    },
                );
                properties.insert(
                    "PluginsDir",
                    RbxInstanceProperty {
                        name: "PluginsDir",
                        value_type: "QDir",
                    },
                );
                properties.insert(
                    "Preprocessor Color",
                    RbxInstanceProperty {
                        name: "Preprocessor Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "RecentSavesDir",
                    RbxInstanceProperty {
                        name: "RecentSavesDir",
                        value_type: "QDir",
                    },
                );
                properties.insert(
                    "Render Throttle Percentage",
                    RbxInstanceProperty {
                        name: "Render Throttle Percentage",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Respect Studio shortcuts when game has focus",
                    RbxInstanceProperty {
                        name: "Respect Studio shortcuts when game has focus",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RuntimeUndoBehavior",
                    RbxInstanceProperty {
                        name: "RuntimeUndoBehavior",
                        value_type: "RuntimeUndoBehavior",
                    },
                );
                properties.insert(
                    "ScriptTimeoutLength",
                    RbxInstanceProperty {
                        name: "ScriptTimeoutLength",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Select Color",
                    RbxInstanceProperty {
                        name: "Select Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Select/Hover Color",
                    RbxInstanceProperty {
                        name: "Select/Hover Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Selection Background Color",
                    RbxInstanceProperty {
                        name: "Selection Background Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Selection Color",
                    RbxInstanceProperty {
                        name: "Selection Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Server Audio Behavior",
                    RbxInstanceProperty {
                        name: "Server Audio Behavior",
                        value_type: "ServerAudioBehavior",
                    },
                );
                properties.insert(
                    "Show Core GUI in Explorer while Playing",
                    RbxInstanceProperty {
                        name: "Show Core GUI in Explorer while Playing",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Show Diagnostics Bar",
                    RbxInstanceProperty {
                        name: "Show Diagnostics Bar",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Show Hidden Objects in Explorer",
                    RbxInstanceProperty {
                        name: "Show Hidden Objects in Explorer",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Show Hover Over",
                    RbxInstanceProperty {
                        name: "Show Hover Over",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Show Navigation Mesh",
                    RbxInstanceProperty {
                        name: "Show Navigation Mesh",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Show Plugin GUI Service in Explorer",
                    RbxInstanceProperty {
                        name: "Show Plugin GUI Service in Explorer",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Show QT warnings in output",
                    RbxInstanceProperty {
                        name: "Show QT warnings in output",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Show plus button on hover in Explorer",
                    RbxInstanceProperty {
                        name: "Show plus button on hover in Explorer",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "String Color",
                    RbxInstanceProperty {
                        name: "String Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Tab Width",
                    RbxInstanceProperty {
                        name: "Tab Width",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Text Color",
                    RbxInstanceProperty {
                        name: "Text Color",
                        value_type: "Color3",
                    },
                );
                properties.insert(
                    "Text Wrapping",
                    RbxInstanceProperty {
                        name: "Text Wrapping",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Theme",
                    RbxInstanceProperty {
                        name: "Theme",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "UI Theme",
                    RbxInstanceProperty {
                        name: "UI Theme",
                        value_type: "UITheme",
                    },
                );
                properties.insert(
                    "Warning Color",
                    RbxInstanceProperty {
                        name: "Warning Color",
                        value_type: "Color3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "StudioService",
        RbxInstanceClass {
            name: "StudioService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ActiveScript",
                    RbxInstanceProperty {
                        name: "ActiveScript",
                        value_type: "Instance",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "StudioTheme",
        RbxInstanceClass {
            name: "StudioTheme",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "TaskScheduler",
        RbxInstanceClass {
            name: "TaskScheduler",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "SchedulerDutyCycle",
                    RbxInstanceProperty {
                        name: "SchedulerDutyCycle",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "SchedulerRate",
                    RbxInstanceProperty {
                        name: "SchedulerRate",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "ThreadPoolConfig",
                    RbxInstanceProperty {
                        name: "ThreadPoolConfig",
                        value_type: "ThreadPoolConfig",
                    },
                );
                properties.insert(
                    "ThreadPoolSize",
                    RbxInstanceProperty {
                        name: "ThreadPoolSize",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Team",
        RbxInstanceClass {
            name: "Team",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AutoAssignable",
                    RbxInstanceProperty {
                        name: "AutoAssignable",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "AutoColorCharacters",
                    RbxInstanceProperty {
                        name: "AutoColorCharacters",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Score",
                    RbxInstanceProperty {
                        name: "Score",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "TeamColor",
                    RbxInstanceProperty {
                        name: "TeamColor",
                        value_type: "BrickColor",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Teams",
        RbxInstanceClass {
            name: "Teams",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "TeleportService",
        RbxInstanceClass {
            name: "TeleportService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CustomizedTeleportUI",
                    RbxInstanceProperty {
                        name: "CustomizedTeleportUI",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TerrainRegion",
        RbxInstanceClass {
            name: "TerrainRegion",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "IsSmooth",
                    RbxInstanceProperty {
                        name: "IsSmooth",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "SizeInCells",
                    RbxInstanceProperty {
                        name: "SizeInCells",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TestService",
        RbxInstanceClass {
            name: "TestService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AutoRuns",
                    RbxInstanceProperty {
                        name: "AutoRuns",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Description",
                    RbxInstanceProperty {
                        name: "Description",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "ErrorCount",
                    RbxInstanceProperty {
                        name: "ErrorCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "ExecuteWithStudioRun",
                    RbxInstanceProperty {
                        name: "ExecuteWithStudioRun",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Is30FpsThrottleEnabled",
                    RbxInstanceProperty {
                        name: "Is30FpsThrottleEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsPhysicsEnvironmentalThrottled",
                    RbxInstanceProperty {
                        name: "IsPhysicsEnvironmentalThrottled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsSleepAllowed",
                    RbxInstanceProperty {
                        name: "IsSleepAllowed",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "NumberOfPlayers",
                    RbxInstanceProperty {
                        name: "NumberOfPlayers",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "SimulateSecondsLag",
                    RbxInstanceProperty {
                        name: "SimulateSecondsLag",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "TestCount",
                    RbxInstanceProperty {
                        name: "TestCount",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "Timeout",
                    RbxInstanceProperty {
                        name: "Timeout",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "WarnCount",
                    RbxInstanceProperty {
                        name: "WarnCount",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TextFilterResult",
        RbxInstanceClass {
            name: "TextFilterResult",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "TextService",
        RbxInstanceClass {
            name: "TextService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "ThirdPartyUserService",
        RbxInstanceClass {
            name: "ThirdPartyUserService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "TimerService",
        RbxInstanceClass {
            name: "TimerService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "TouchInputService",
        RbxInstanceClass {
            name: "TouchInputService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "TouchTransmitter",
        RbxInstanceClass {
            name: "TouchTransmitter",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Trail",
        RbxInstanceClass {
            name: "Trail",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Attachment0",
                    RbxInstanceProperty {
                        name: "Attachment0",
                        value_type: "Attachment",
                    },
                );
                properties.insert(
                    "Attachment1",
                    RbxInstanceProperty {
                        name: "Attachment1",
                        value_type: "Attachment",
                    },
                );
                properties.insert(
                    "Color",
                    RbxInstanceProperty {
                        name: "Color",
                        value_type: "ColorSequence",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "FaceCamera",
                    RbxInstanceProperty {
                        name: "FaceCamera",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Lifetime",
                    RbxInstanceProperty {
                        name: "Lifetime",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LightEmission",
                    RbxInstanceProperty {
                        name: "LightEmission",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "LightInfluence",
                    RbxInstanceProperty {
                        name: "LightInfluence",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MaxLength",
                    RbxInstanceProperty {
                        name: "MaxLength",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MinLength",
                    RbxInstanceProperty {
                        name: "MinLength",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "Texture",
                    RbxInstanceProperty {
                        name: "Texture",
                        value_type: "Content",
                    },
                );
                properties.insert(
                    "TextureLength",
                    RbxInstanceProperty {
                        name: "TextureLength",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "TextureMode",
                    RbxInstanceProperty {
                        name: "TextureMode",
                        value_type: "TextureMode",
                    },
                );
                properties.insert(
                    "Transparency",
                    RbxInstanceProperty {
                        name: "Transparency",
                        value_type: "NumberSequence",
                    },
                );
                properties.insert(
                    "WidthScale",
                    RbxInstanceProperty {
                        name: "WidthScale",
                        value_type: "NumberSequence",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Translator",
        RbxInstanceClass {
            name: "Translator",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "LocaleId",
                    RbxInstanceProperty {
                        name: "LocaleId",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TweenBase",
        RbxInstanceClass {
            name: "TweenBase",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "PlaybackState",
                    RbxInstanceProperty {
                        name: "PlaybackState",
                        value_type: "PlaybackState",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Tween",
        RbxInstanceClass {
            name: "Tween",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Instance",
                    RbxInstanceProperty {
                        name: "Instance",
                        value_type: "Instance",
                    },
                );
                properties.insert(
                    "TweenInfo",
                    RbxInstanceProperty {
                        name: "TweenInfo",
                        value_type: "TweenInfo",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "TweenService",
        RbxInstanceClass {
            name: "TweenService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "UIBase",
        RbxInstanceClass {
            name: "UIBase",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "UIComponent",
        RbxInstanceClass {
            name: "UIComponent",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "UIConstraint",
        RbxInstanceClass {
            name: "UIConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "UIAspectRatioConstraint",
        RbxInstanceClass {
            name: "UIAspectRatioConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AspectRatio",
                    RbxInstanceProperty {
                        name: "AspectRatio",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "AspectType",
                    RbxInstanceProperty {
                        name: "AspectType",
                        value_type: "AspectType",
                    },
                );
                properties.insert(
                    "DominantAxis",
                    RbxInstanceProperty {
                        name: "DominantAxis",
                        value_type: "DominantAxis",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UISizeConstraint",
        RbxInstanceClass {
            name: "UISizeConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "MaxSize",
                    RbxInstanceProperty {
                        name: "MaxSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "MinSize",
                    RbxInstanceProperty {
                        name: "MinSize",
                        value_type: "Vector2",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UITextSizeConstraint",
        RbxInstanceClass {
            name: "UITextSizeConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "MaxTextSize",
                    RbxInstanceProperty {
                        name: "MaxTextSize",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "MinTextSize",
                    RbxInstanceProperty {
                        name: "MinTextSize",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UILayout",
        RbxInstanceClass {
            name: "UILayout",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "UIGridStyleLayout",
        RbxInstanceClass {
            name: "UIGridStyleLayout",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AbsoluteContentSize",
                    RbxInstanceProperty {
                        name: "AbsoluteContentSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "FillDirection",
                    RbxInstanceProperty {
                        name: "FillDirection",
                        value_type: "FillDirection",
                    },
                );
                properties.insert(
                    "HorizontalAlignment",
                    RbxInstanceProperty {
                        name: "HorizontalAlignment",
                        value_type: "HorizontalAlignment",
                    },
                );
                properties.insert(
                    "SortOrder",
                    RbxInstanceProperty {
                        name: "SortOrder",
                        value_type: "SortOrder",
                    },
                );
                properties.insert(
                    "VerticalAlignment",
                    RbxInstanceProperty {
                        name: "VerticalAlignment",
                        value_type: "VerticalAlignment",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UIGridLayout",
        RbxInstanceClass {
            name: "UIGridLayout",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "CellPadding",
                    RbxInstanceProperty {
                        name: "CellPadding",
                        value_type: "UDim2",
                    },
                );
                properties.insert(
                    "CellSize",
                    RbxInstanceProperty {
                        name: "CellSize",
                        value_type: "UDim2",
                    },
                );
                properties.insert(
                    "FillDirectionMaxCells",
                    RbxInstanceProperty {
                        name: "FillDirectionMaxCells",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "StartCorner",
                    RbxInstanceProperty {
                        name: "StartCorner",
                        value_type: "StartCorner",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UIListLayout",
        RbxInstanceClass {
            name: "UIListLayout",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Padding",
                    RbxInstanceProperty {
                        name: "Padding",
                        value_type: "UDim",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UIPageLayout",
        RbxInstanceClass {
            name: "UIPageLayout",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Animated",
                    RbxInstanceProperty {
                        name: "Animated",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Circular",
                    RbxInstanceProperty {
                        name: "Circular",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CurrentPage",
                    RbxInstanceProperty {
                        name: "CurrentPage",
                        value_type: "GuiObject",
                    },
                );
                properties.insert(
                    "EasingDirection",
                    RbxInstanceProperty {
                        name: "EasingDirection",
                        value_type: "EasingDirection",
                    },
                );
                properties.insert(
                    "EasingStyle",
                    RbxInstanceProperty {
                        name: "EasingStyle",
                        value_type: "EasingStyle",
                    },
                );
                properties.insert(
                    "GamepadInputEnabled",
                    RbxInstanceProperty {
                        name: "GamepadInputEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Padding",
                    RbxInstanceProperty {
                        name: "Padding",
                        value_type: "UDim",
                    },
                );
                properties.insert(
                    "ScrollWheelInputEnabled",
                    RbxInstanceProperty {
                        name: "ScrollWheelInputEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TouchInputEnabled",
                    RbxInstanceProperty {
                        name: "TouchInputEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "TweenTime",
                    RbxInstanceProperty {
                        name: "TweenTime",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UITableLayout",
        RbxInstanceClass {
            name: "UITableLayout",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "FillEmptySpaceColumns",
                    RbxInstanceProperty {
                        name: "FillEmptySpaceColumns",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "FillEmptySpaceRows",
                    RbxInstanceProperty {
                        name: "FillEmptySpaceRows",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MajorAxis",
                    RbxInstanceProperty {
                        name: "MajorAxis",
                        value_type: "TableMajorAxis",
                    },
                );
                properties.insert(
                    "Padding",
                    RbxInstanceProperty {
                        name: "Padding",
                        value_type: "UDim2",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UIPadding",
        RbxInstanceClass {
            name: "UIPadding",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "PaddingBottom",
                    RbxInstanceProperty {
                        name: "PaddingBottom",
                        value_type: "UDim",
                    },
                );
                properties.insert(
                    "PaddingLeft",
                    RbxInstanceProperty {
                        name: "PaddingLeft",
                        value_type: "UDim",
                    },
                );
                properties.insert(
                    "PaddingRight",
                    RbxInstanceProperty {
                        name: "PaddingRight",
                        value_type: "UDim",
                    },
                );
                properties.insert(
                    "PaddingTop",
                    RbxInstanceProperty {
                        name: "PaddingTop",
                        value_type: "UDim",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UIScale",
        RbxInstanceClass {
            name: "UIScale",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Scale",
                    RbxInstanceProperty {
                        name: "Scale",
                        value_type: "float",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UserGameSettings",
        RbxInstanceClass {
            name: "UserGameSettings",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AllTutorialsDisabled",
                    RbxInstanceProperty {
                        name: "AllTutorialsDisabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "CameraMode",
                    RbxInstanceProperty {
                        name: "CameraMode",
                        value_type: "CustomCameraMode",
                    },
                );
                properties.insert(
                    "CameraYInverted",
                    RbxInstanceProperty {
                        name: "CameraYInverted",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ChatVisible",
                    RbxInstanceProperty {
                        name: "ChatVisible",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ComputerCameraMovementMode",
                    RbxInstanceProperty {
                        name: "ComputerCameraMovementMode",
                        value_type: "ComputerCameraMovementMode",
                    },
                );
                properties.insert(
                    "ComputerMovementMode",
                    RbxInstanceProperty {
                        name: "ComputerMovementMode",
                        value_type: "ComputerMovementMode",
                    },
                );
                properties.insert(
                    "ControlMode",
                    RbxInstanceProperty {
                        name: "ControlMode",
                        value_type: "ControlMode",
                    },
                );
                properties.insert(
                    "Fullscreen",
                    RbxInstanceProperty {
                        name: "Fullscreen",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "GamepadCameraSensitivity",
                    RbxInstanceProperty {
                        name: "GamepadCameraSensitivity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "HasEverUsedVR",
                    RbxInstanceProperty {
                        name: "HasEverUsedVR",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsUsingCameraYInverted",
                    RbxInstanceProperty {
                        name: "IsUsingCameraYInverted",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "IsUsingGamepadCameraSensitivity",
                    RbxInstanceProperty {
                        name: "IsUsingGamepadCameraSensitivity",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MasterVolume",
                    RbxInstanceProperty {
                        name: "MasterVolume",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MicroProfilerWebServerEnabled",
                    RbxInstanceProperty {
                        name: "MicroProfilerWebServerEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MicroProfilerWebServerIP",
                    RbxInstanceProperty {
                        name: "MicroProfilerWebServerIP",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "MicroProfilerWebServerPort",
                    RbxInstanceProperty {
                        name: "MicroProfilerWebServerPort",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "MouseSensitivity",
                    RbxInstanceProperty {
                        name: "MouseSensitivity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MouseSensitivityFirstPerson",
                    RbxInstanceProperty {
                        name: "MouseSensitivityFirstPerson",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "MouseSensitivityThirdPerson",
                    RbxInstanceProperty {
                        name: "MouseSensitivityThirdPerson",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "OnScreenProfilerEnabled",
                    RbxInstanceProperty {
                        name: "OnScreenProfilerEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "OnboardingsCompleted",
                    RbxInstanceProperty {
                        name: "OnboardingsCompleted",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "PerformanceStatsVisible",
                    RbxInstanceProperty {
                        name: "PerformanceStatsVisible",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "RCCProfilerRecordFrameRate",
                    RbxInstanceProperty {
                        name: "RCCProfilerRecordFrameRate",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "RCCProfilerRecordTimeFrame",
                    RbxInstanceProperty {
                        name: "RCCProfilerRecordTimeFrame",
                        value_type: "int",
                    },
                );
                properties.insert(
                    "RotationType",
                    RbxInstanceProperty {
                        name: "RotationType",
                        value_type: "RotationType",
                    },
                );
                properties.insert(
                    "SavedQualityLevel",
                    RbxInstanceProperty {
                        name: "SavedQualityLevel",
                        value_type: "SavedQualitySetting",
                    },
                );
                properties.insert(
                    "StartMaximized",
                    RbxInstanceProperty {
                        name: "StartMaximized",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "StartScreenPosition",
                    RbxInstanceProperty {
                        name: "StartScreenPosition",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "StartScreenSize",
                    RbxInstanceProperty {
                        name: "StartScreenSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "TouchCameraMovementMode",
                    RbxInstanceProperty {
                        name: "TouchCameraMovementMode",
                        value_type: "TouchCameraMovementMode",
                    },
                );
                properties.insert(
                    "TouchMovementMode",
                    RbxInstanceProperty {
                        name: "TouchMovementMode",
                        value_type: "TouchMovementMode",
                    },
                );
                properties.insert(
                    "UsedCoreGuiIsVisibleToggle",
                    RbxInstanceProperty {
                        name: "UsedCoreGuiIsVisibleToggle",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "UsedCustomGuiIsVisibleToggle",
                    RbxInstanceProperty {
                        name: "UsedCustomGuiIsVisibleToggle",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "UsedHideHudShortcut",
                    RbxInstanceProperty {
                        name: "UsedHideHudShortcut",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "VREnabled",
                    RbxInstanceProperty {
                        name: "VREnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "VRRotationIntensity",
                    RbxInstanceProperty {
                        name: "VRRotationIntensity",
                        value_type: "int",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "UserInputService",
        RbxInstanceClass {
            name: "UserInputService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AccelerometerEnabled",
                    RbxInstanceProperty {
                        name: "AccelerometerEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "BottomBarSize",
                    RbxInstanceProperty {
                        name: "BottomBarSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "GamepadEnabled",
                    RbxInstanceProperty {
                        name: "GamepadEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "GazeSelectionEnabled",
                    RbxInstanceProperty {
                        name: "GazeSelectionEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "GyroscopeEnabled",
                    RbxInstanceProperty {
                        name: "GyroscopeEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "KeyboardEnabled",
                    RbxInstanceProperty {
                        name: "KeyboardEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "LegacyInputEventsEnabled",
                    RbxInstanceProperty {
                        name: "LegacyInputEventsEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "ModalEnabled",
                    RbxInstanceProperty {
                        name: "ModalEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MouseBehavior",
                    RbxInstanceProperty {
                        name: "MouseBehavior",
                        value_type: "MouseBehavior",
                    },
                );
                properties.insert(
                    "MouseDeltaSensitivity",
                    RbxInstanceProperty {
                        name: "MouseDeltaSensitivity",
                        value_type: "float",
                    },
                );
                properties.insert(
                    "MouseEnabled",
                    RbxInstanceProperty {
                        name: "MouseEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "MouseIconEnabled",
                    RbxInstanceProperty {
                        name: "MouseIconEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "NavBarSize",
                    RbxInstanceProperty {
                        name: "NavBarSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "OnScreenKeyboardAnimationDuration",
                    RbxInstanceProperty {
                        name: "OnScreenKeyboardAnimationDuration",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "OnScreenKeyboardPosition",
                    RbxInstanceProperty {
                        name: "OnScreenKeyboardPosition",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "OnScreenKeyboardSize",
                    RbxInstanceProperty {
                        name: "OnScreenKeyboardSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "OnScreenKeyboardVisible",
                    RbxInstanceProperty {
                        name: "OnScreenKeyboardVisible",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "OverrideMouseIconBehavior",
                    RbxInstanceProperty {
                        name: "OverrideMouseIconBehavior",
                        value_type: "OverrideMouseIconBehavior",
                    },
                );
                properties.insert(
                    "RightBarSize",
                    RbxInstanceProperty {
                        name: "RightBarSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "StatusBarSize",
                    RbxInstanceProperty {
                        name: "StatusBarSize",
                        value_type: "Vector2",
                    },
                );
                properties.insert(
                    "TouchEnabled",
                    RbxInstanceProperty {
                        name: "TouchEnabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "UserHeadCFrame",
                    RbxInstanceProperty {
                        name: "UserHeadCFrame",
                        value_type: "CFrame",
                    },
                );
                properties.insert(
                    "VREnabled",
                    RbxInstanceProperty {
                        name: "VREnabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "VRService",
        RbxInstanceClass {
            name: "VRService",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "GuiInputUserCFrame",
                    RbxInstanceProperty {
                        name: "GuiInputUserCFrame",
                        value_type: "UserCFrame",
                    },
                );
                properties.insert(
                    "VRDeviceName",
                    RbxInstanceProperty {
                        name: "VRDeviceName",
                        value_type: "string",
                    },
                );
                properties.insert(
                    "VREnabled",
                    RbxInstanceProperty {
                        name: "VREnabled",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ValueBase",
        RbxInstanceClass {
            name: "ValueBase",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BinaryStringValue",
        RbxInstanceClass {
            name: "BinaryStringValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "BoolValue",
        RbxInstanceClass {
            name: "BoolValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "bool",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "BrickColorValue",
        RbxInstanceClass {
            name: "BrickColorValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "BrickColor",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "CFrameValue",
        RbxInstanceClass {
            name: "CFrameValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "CFrame",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Color3Value",
        RbxInstanceClass {
            name: "Color3Value",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "Color3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "DoubleConstrainedValue",
        RbxInstanceClass {
            name: "DoubleConstrainedValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ConstrainedValue",
                    RbxInstanceProperty {
                        name: "ConstrainedValue",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "MaxValue",
                    RbxInstanceProperty {
                        name: "MaxValue",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "MinValue",
                    RbxInstanceProperty {
                        name: "MinValue",
                        value_type: "double",
                    },
                );
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "double",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "IntConstrainedValue",
        RbxInstanceClass {
            name: "IntConstrainedValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "ConstrainedValue",
                    RbxInstanceProperty {
                        name: "ConstrainedValue",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "MaxValue",
                    RbxInstanceProperty {
                        name: "MaxValue",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "MinValue",
                    RbxInstanceProperty {
                        name: "MinValue",
                        value_type: "int64",
                    },
                );
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "int64",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "IntValue",
        RbxInstanceClass {
            name: "IntValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "int64",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "NumberValue",
        RbxInstanceClass {
            name: "NumberValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "double",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "ObjectValue",
        RbxInstanceClass {
            name: "ObjectValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "Instance",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "RayValue",
        RbxInstanceClass {
            name: "RayValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "Ray",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "StringValue",
        RbxInstanceClass {
            name: "StringValue",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "Vector3Value",
        RbxInstanceClass {
            name: "Vector3Value",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Value",
                    RbxInstanceProperty {
                        name: "Value",
                        value_type: "Vector3",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "VirtualInputManager",
        RbxInstanceClass {
            name: "VirtualInputManager",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "AdditionalLuaState",
                    RbxInstanceProperty {
                        name: "AdditionalLuaState",
                        value_type: "string",
                    },
                );
                properties
            },
        },
    );
    output.insert(
        "VirtualUser",
        RbxInstanceClass {
            name: "VirtualUser",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "Visit",
        RbxInstanceClass {
            name: "Visit",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties
            },
        },
    );
    output.insert(
        "WeldConstraint",
        RbxInstanceClass {
            name: "WeldConstraint",
            properties: {
                #[allow(unused_mut)]
                let mut properties = HashMap::new();
                properties.insert(
                    "Active",
                    RbxInstanceProperty {
                        name: "Active",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Enabled",
                    RbxInstanceProperty {
                        name: "Enabled",
                        value_type: "bool",
                    },
                );
                properties.insert(
                    "Part0",
                    RbxInstanceProperty {
                        name: "Part0",
                        value_type: "BasePart",
                    },
                );
                properties.insert(
                    "Part1",
                    RbxInstanceProperty {
                        name: "Part1",
                        value_type: "BasePart",
                    },
                );
                properties
            },
        },
    );
    output
}
pub fn generate_enums() -> HashMap<&'static str, RbxEnum> {
    let mut output = HashMap::new();
    output.insert(
        "ActionType",
        RbxEnum {
            name: "ActionType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Nothing", 0);
                items.insert("Pause", 1);
                items.insert("Lose", 2);
                items.insert("Draw", 3);
                items.insert("Win", 4);
                items
            },
        },
    );
    output.insert(
        "ActuatorRelativeTo",
        RbxEnum {
            name: "ActuatorRelativeTo",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Attachment0", 0);
                items.insert("Attachment1", 1);
                items.insert("World", 2);
                items
            },
        },
    );
    output.insert(
        "ActuatorType",
        RbxEnum {
            name: "ActuatorType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 0);
                items.insert("Motor", 1);
                items.insert("Servo", 2);
                items
            },
        },
    );
    output.insert(
        "AlignType",
        RbxEnum {
            name: "AlignType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Parallel", 0);
                items.insert("Perpendicular", 1);
                items
            },
        },
    );
    output.insert(
        "AnimationPriority",
        RbxEnum {
            name: "AnimationPriority",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Idle", 0);
                items.insert("Movement", 1);
                items.insert("Action", 2);
                items.insert("Core", 1000);
                items
            },
        },
    );
    output.insert(
        "AppShellActionType",
        RbxEnum {
            name: "AppShellActionType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 0);
                items.insert("OpenApp", 1);
                items.insert("TapChatTab", 2);
                items.insert("TapConversationEntry", 3);
                items.insert("TapAvatarTab", 4);
                items.insert("ReadConversation", 5);
                items.insert("TapGamePageTab", 6);
                items.insert("TapHomePageTab", 7);
                items.insert("GamePageLoaded", 8);
                items.insert("HomePageLoaded", 9);
                items.insert("AvatarEditorPageLoaded", 10);
                items
            },
        },
    );
    output.insert(
        "AspectType",
        RbxEnum {
            name: "AspectType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("FitWithinMaxSize", 0);
                items.insert("ScaleWithParentSize", 1);
                items
            },
        },
    );
    output.insert(
        "AssetType",
        RbxEnum {
            name: "AssetType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Image", 1);
                items.insert("TeeShirt", 2);
                items.insert("Audio", 3);
                items.insert("Mesh", 4);
                items.insert("Lua", 5);
                items.insert("Hat", 8);
                items.insert("Place", 9);
                items.insert("Model", 10);
                items.insert("Shirt", 11);
                items.insert("Pants", 12);
                items.insert("Decal", 13);
                items.insert("Head", 17);
                items.insert("Face", 18);
                items.insert("Gear", 19);
                items.insert("Badge", 21);
                items.insert("Animation", 24);
                items.insert("Torso", 27);
                items.insert("RightArm", 28);
                items.insert("LeftArm", 29);
                items.insert("LeftLeg", 30);
                items.insert("RightLeg", 31);
                items.insert("Package", 32);
                items.insert("GamePass", 34);
                items.insert("Plugin", 38);
                items.insert("MeshPart", 40);
                items.insert("HairAccessory", 41);
                items.insert("FaceAccessory", 42);
                items.insert("NeckAccessory", 43);
                items.insert("ShoulderAccessory", 44);
                items.insert("FrontAccessory", 45);
                items.insert("BackAccessory", 46);
                items.insert("WaistAccessory", 47);
                items.insert("ClimbAnimation", 48);
                items.insert("DeathAnimation", 49);
                items.insert("FallAnimation", 50);
                items.insert("IdleAnimation", 51);
                items.insert("JumpAnimation", 52);
                items.insert("RunAnimation", 53);
                items.insert("SwimAnimation", 54);
                items.insert("WalkAnimation", 55);
                items.insert("PoseAnimation", 56);
                items.insert("EarAccessory", 57);
                items.insert("EyeAccessory", 58);
                items
            },
        },
    );
    output.insert(
        "AutoJointsMode",
        RbxEnum {
            name: "AutoJointsMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("Explicit", 1);
                items.insert("LegacyImplicit", 2);
                items
            },
        },
    );
    output.insert(
        "AvatarContextMenuOption",
        RbxEnum {
            name: "AvatarContextMenuOption",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Friend", 0);
                items.insert("Chat", 1);
                items.insert("Emote", 2);
                items
            },
        },
    );
    output.insert(
        "AvatarJointPositionType",
        RbxEnum {
            name: "AvatarJointPositionType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Fixed", 0);
                items.insert("ArtistIntent", 1);
                items
            },
        },
    );
    output.insert(
        "Axis",
        RbxEnum {
            name: "Axis",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("X", 0);
                items.insert("Y", 1);
                items.insert("Z", 2);
                items
            },
        },
    );
    output.insert(
        "BinType",
        RbxEnum {
            name: "BinType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Script", 0);
                items.insert("GameTool", 1);
                items.insert("Grab", 2);
                items.insert("Clone", 3);
                items.insert("Hammer", 4);
                items
            },
        },
    );
    output.insert(
        "BodyPart",
        RbxEnum {
            name: "BodyPart",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Head", 0);
                items.insert("Torso", 1);
                items.insert("LeftArm", 2);
                items.insert("RightArm", 3);
                items.insert("LeftLeg", 4);
                items.insert("RightLeg", 5);
                items
            },
        },
    );
    output.insert(
        "BodyPartR15",
        RbxEnum {
            name: "BodyPartR15",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Head", 0);
                items.insert("UpperTorso", 1);
                items.insert("LowerTorso", 2);
                items.insert("LeftFoot", 3);
                items.insert("LeftLowerLeg", 4);
                items.insert("LeftUpperLeg", 5);
                items.insert("RightFoot", 6);
                items.insert("RightLowerLeg", 7);
                items.insert("RightUpperLeg", 8);
                items.insert("LeftHand", 9);
                items.insert("LeftLowerArm", 10);
                items.insert("LeftUpperArm", 11);
                items.insert("RightHand", 12);
                items.insert("RightLowerArm", 13);
                items.insert("RightUpperArm", 14);
                items.insert("RootPart", 15);
                items.insert("Unknown", 17);
                items
            },
        },
    );
    output.insert(
        "Button",
        RbxEnum {
            name: "Button",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Jump", 32);
                items.insert("Dismount", 8);
                items
            },
        },
    );
    output.insert(
        "ButtonStyle",
        RbxEnum {
            name: "ButtonStyle",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Custom", 0);
                items.insert("RobloxButtonDefault", 1);
                items.insert("RobloxButton", 2);
                items.insert("RobloxRoundButton", 3);
                items.insert("RobloxRoundDefaultButton", 4);
                items.insert("RobloxRoundDropdownButton", 5);
                items
            },
        },
    );
    output.insert(
        "CameraMode",
        RbxEnum {
            name: "CameraMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Classic", 0);
                items.insert("LockFirstPerson", 1);
                items
            },
        },
    );
    output.insert(
        "CameraPanMode",
        RbxEnum {
            name: "CameraPanMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Classic", 0);
                items.insert("EdgeBump", 1);
                items
            },
        },
    );
    output.insert(
        "CameraType",
        RbxEnum {
            name: "CameraType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Fixed", 0);
                items.insert("Watch", 2);
                items.insert("Attach", 1);
                items.insert("Track", 3);
                items.insert("Follow", 4);
                items.insert("Custom", 5);
                items.insert("Scriptable", 6);
                items.insert("Orbital", 7);
                items
            },
        },
    );
    output.insert(
        "CellBlock",
        RbxEnum {
            name: "CellBlock",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Solid", 0);
                items.insert("VerticalWedge", 1);
                items.insert("CornerWedge", 2);
                items.insert("InverseCornerWedge", 3);
                items.insert("HorizontalWedge", 4);
                items
            },
        },
    );
    output.insert(
        "CellMaterial",
        RbxEnum {
            name: "CellMaterial",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Empty", 0);
                items.insert("Grass", 1);
                items.insert("Sand", 2);
                items.insert("Brick", 3);
                items.insert("Granite", 4);
                items.insert("Asphalt", 5);
                items.insert("Iron", 6);
                items.insert("Aluminum", 7);
                items.insert("Gold", 8);
                items.insert("WoodPlank", 9);
                items.insert("WoodLog", 10);
                items.insert("Gravel", 11);
                items.insert("CinderBlock", 12);
                items.insert("MossyStone", 13);
                items.insert("Cement", 14);
                items.insert("RedPlastic", 15);
                items.insert("BluePlastic", 16);
                items.insert("Water", 17);
                items
            },
        },
    );
    output.insert(
        "CellOrientation",
        RbxEnum {
            name: "CellOrientation",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("NegZ", 0);
                items.insert("X", 1);
                items.insert("Z", 2);
                items.insert("NegX", 3);
                items
            },
        },
    );
    output.insert(
        "CenterDialogType",
        RbxEnum {
            name: "CenterDialogType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("UnsolicitedDialog", 1);
                items.insert("PlayerInitiatedDialog", 2);
                items.insert("ModalDialog", 3);
                items.insert("QuitDialog", 4);
                items
            },
        },
    );
    output.insert(
        "ChatCallbackType",
        RbxEnum {
            name: "ChatCallbackType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("OnCreatingChatWindow", 1);
                items.insert("OnClientSendingMessage", 2);
                items.insert("OnClientFormattingMessage", 3);
                items.insert("OnServerReceivingMessage", 17);
                items
            },
        },
    );
    output.insert(
        "ChatColor",
        RbxEnum {
            name: "ChatColor",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Blue", 0);
                items.insert("Green", 1);
                items.insert("Red", 2);
                items.insert("White", 3);
                items
            },
        },
    );
    output.insert(
        "ChatMode",
        RbxEnum {
            name: "ChatMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Menu", 0);
                items.insert("TextAndMenu", 1);
                items
            },
        },
    );
    output.insert(
        "ChatPrivacyMode",
        RbxEnum {
            name: "ChatPrivacyMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("AllUsers", 0);
                items.insert("NoOne", 1);
                items.insert("Friends", 2);
                items
            },
        },
    );
    output.insert(
        "ChatStyle",
        RbxEnum {
            name: "ChatStyle",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Classic", 0);
                items.insert("Bubble", 1);
                items.insert("ClassicAndBubble", 2);
                items
            },
        },
    );
    output.insert(
        "CollisionFidelity",
        RbxEnum {
            name: "CollisionFidelity",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("Hull", 1);
                items.insert("Box", 2);
                items
            },
        },
    );
    output.insert(
        "ComputerCameraMovementMode",
        RbxEnum {
            name: "ComputerCameraMovementMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("Follow", 2);
                items.insert("Classic", 1);
                items.insert("Orbital", 3);
                items
            },
        },
    );
    output.insert(
        "ComputerMovementMode",
        RbxEnum {
            name: "ComputerMovementMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("KeyboardMouse", 1);
                items.insert("ClickToMove", 2);
                items
            },
        },
    );
    output.insert(
        "ConnectionError",
        RbxEnum {
            name: "ConnectionError",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("OK", 0);
                items.insert("DisconnectErrors", 256);
                items.insert("DisconnectBadhash", 257);
                items.insert("DisconnectSecurityKeyMismatch", 258);
                items.insert("DisconnectNewSecurityKeyMismatch", 272);
                items.insert("DisconnectProtocolMismatch", 259);
                items.insert("DisconnectReceivePacketError", 260);
                items.insert("DisconnectReceivePacketStreamError", 261);
                items.insert("DisconnectSendPacketError", 262);
                items.insert("DisconnectIllegalTeleport", 263);
                items.insert("DisconnectDuplicatePlayer", 264);
                items.insert("DisconnectDuplicateTicket", 265);
                items.insert("DisconnectTimeout", 266);
                items.insert("DisconnectLuaKick", 267);
                items.insert("DisconnectOnRemoteSysStats", 268);
                items.insert("DisconnectHashTimeout", 269);
                items.insert("DisconnectCloudEditKick", 270);
                items.insert("DisconnectPlayerless", 271);
                items.insert("DisconnectEvicted", 273);
                items.insert("DisconnectDevMaintenance", 274);
                items.insert("DisconnectRobloxMaintenance", 275);
                items.insert("DisconnectRejoin", 276);
                items.insert("DisconnectConnectionLost", 277);
                items.insert("DisconnectIdle", 278);
                items.insert("DisconnectRaknetErrors", 279);
                items.insert("DisconnectWrongVersion", 280);
                items.insert("PlacelaunchErrors", 512);
                items.insert("PlacelaunchDisabled", 515);
                items.insert("PlacelaunchError", 516);
                items.insert("PlacelaunchGameEnded", 517);
                items.insert("PlacelaunchGameFull", 518);
                items.insert("PlacelaunchUserLeft", 522);
                items.insert("PlacelaunchRestricted", 523);
                items.insert("PlacelaunchUnauthorized", 524);
                items.insert("PlacelaunchFlooded", 525);
                items.insert("PlacelaunchHashExpired", 526);
                items.insert("PlacelaunchHashException", 527);
                items.insert("PlacelaunchPartyCannotFit", 528);
                items.insert("PlacelaunchHttpError", 529);
                items.insert("PlacelaunchCustomMessage", 610);
                items.insert("PlacelaunchOtherError", 611);
                items.insert("TeleportErrors", 768);
                items.insert("TeleportFailure", 769);
                items.insert("TeleportGameNotFound", 770);
                items.insert("TeleportGameEnded", 771);
                items.insert("TeleportGameFull", 772);
                items.insert("TeleportUnauthorized", 773);
                items.insert("TeleportFlooded", 774);
                items.insert("TeleportIsTeleporting", 775);
                items
            },
        },
    );
    output.insert(
        "ConnectionState",
        RbxEnum {
            name: "ConnectionState",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Connected", 0);
                items.insert("Disconnected", 1);
                items
            },
        },
    );
    output.insert(
        "ContextActionPriority",
        RbxEnum {
            name: "ContextActionPriority",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Low", 1000);
                items.insert("Medium", 2000);
                items.insert("Default", 2000);
                items.insert("High", 3000);
                items
            },
        },
    );
    output.insert(
        "ContextActionResult",
        RbxEnum {
            name: "ContextActionResult",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Pass", 1);
                items.insert("Sink", 0);
                items
            },
        },
    );
    output.insert(
        "ControlMode",
        RbxEnum {
            name: "ControlMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("MouseLockSwitch", 1);
                items.insert("Classic", 0);
                items
            },
        },
    );
    output.insert(
        "CoreGuiType",
        RbxEnum {
            name: "CoreGuiType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("PlayerList", 0);
                items.insert("Health", 1);
                items.insert("Backpack", 2);
                items.insert("Chat", 3);
                items.insert("All", 4);
                items
            },
        },
    );
    output.insert(
        "CreatorType",
        RbxEnum {
            name: "CreatorType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("User", 0);
                items.insert("Group", 1);
                items
            },
        },
    );
    output.insert(
        "CurrencyType",
        RbxEnum {
            name: "CurrencyType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("Robux", 1);
                items.insert("Tix", 2);
                items
            },
        },
    );
    output.insert(
        "CustomCameraMode",
        RbxEnum {
            name: "CustomCameraMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("Follow", 2);
                items.insert("Classic", 1);
                items
            },
        },
    );
    output.insert(
        "DEPRECATED_DebuggerDataModelPreference",
        RbxEnum {
            name: "DEPRECATED_DebuggerDataModelPreference",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Server", 0);
                items.insert("Client", 1);
                items
            },
        },
    );
    output.insert(
        "DataStoreRequestType",
        RbxEnum {
            name: "DataStoreRequestType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("GetAsync", 0);
                items.insert("SetIncrementAsync", 1);
                items.insert("UpdateAsync", 2);
                items.insert("GetSortedAsync", 3);
                items.insert("SetIncrementSortedAsync", 4);
                items.insert("OnUpdate", 5);
                items
            },
        },
    );
    output.insert(
        "DevCameraOcclusionMode",
        RbxEnum {
            name: "DevCameraOcclusionMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Zoom", 0);
                items.insert("Invisicam", 1);
                items
            },
        },
    );
    output.insert(
        "DevComputerCameraMovementMode",
        RbxEnum {
            name: "DevComputerCameraMovementMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("UserChoice", 0);
                items.insert("Classic", 1);
                items.insert("Follow", 2);
                items.insert("Orbital", 3);
                items
            },
        },
    );
    output.insert(
        "DevComputerMovementMode",
        RbxEnum {
            name: "DevComputerMovementMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("UserChoice", 0);
                items.insert("KeyboardMouse", 1);
                items.insert("ClickToMove", 2);
                items.insert("Scriptable", 3);
                items
            },
        },
    );
    output.insert(
        "DevTouchCameraMovementMode",
        RbxEnum {
            name: "DevTouchCameraMovementMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("UserChoice", 0);
                items.insert("Classic", 1);
                items.insert("Follow", 2);
                items.insert("Orbital", 3);
                items
            },
        },
    );
    output.insert(
        "DevTouchMovementMode",
        RbxEnum {
            name: "DevTouchMovementMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("UserChoice", 0);
                items.insert("Thumbstick", 1);
                items.insert("DPad", 2);
                items.insert("Thumbpad", 3);
                items.insert("ClickToMove", 4);
                items.insert("Scriptable", 5);
                items.insert("DynamicThumbstick", 6);
                items
            },
        },
    );
    output.insert(
        "DeveloperMemoryTag",
        RbxEnum {
            name: "DeveloperMemoryTag",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Internal", 0);
                items.insert("HttpCache", 1);
                items.insert("Instances", 2);
                items.insert("Signals", 3);
                items.insert("LuaHeap", 4);
                items.insert("Script", 5);
                items.insert("PhysicsCollision", 6);
                items.insert("PhysicsParts", 7);
                items.insert("GraphicsSolidModels", 8);
                items.insert("GraphicsMeshParts", 9);
                items.insert("GraphicsParticles", 10);
                items.insert("GraphicsParts", 11);
                items.insert("GraphicsSpatialHash", 12);
                items.insert("GraphicsTerrain", 13);
                items.insert("GraphicsTexture", 14);
                items.insert("GraphicsTextureCharacter", 15);
                items.insert("Sounds", 16);
                items.insert("StreamingSounds", 17);
                items.insert("TerrainVoxels", 18);
                items.insert("Gui", 20);
                items.insert("Animation", 21);
                items.insert("Navigation", 22);
                items
            },
        },
    );
    output.insert(
        "DialogBehaviorType",
        RbxEnum {
            name: "DialogBehaviorType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("SinglePlayer", 0);
                items.insert("MultiplePlayers", 1);
                items
            },
        },
    );
    output.insert(
        "DialogPurpose",
        RbxEnum {
            name: "DialogPurpose",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Quest", 0);
                items.insert("Help", 1);
                items.insert("Shop", 2);
                items
            },
        },
    );
    output.insert(
        "DialogTone",
        RbxEnum {
            name: "DialogTone",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Neutral", 0);
                items.insert("Friendly", 1);
                items.insert("Enemy", 2);
                items
            },
        },
    );
    output.insert(
        "DominantAxis",
        RbxEnum {
            name: "DominantAxis",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Width", 0);
                items.insert("Height", 1);
                items
            },
        },
    );
    output.insert(
        "EasingDirection",
        RbxEnum {
            name: "EasingDirection",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("In", 0);
                items.insert("Out", 1);
                items.insert("InOut", 2);
                items
            },
        },
    );
    output.insert(
        "EasingStyle",
        RbxEnum {
            name: "EasingStyle",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Linear", 0);
                items.insert("Sine", 1);
                items.insert("Back", 2);
                items.insert("Quad", 3);
                items.insert("Quart", 4);
                items.insert("Quint", 5);
                items.insert("Bounce", 6);
                items.insert("Elastic", 7);
                items
            },
        },
    );
    output.insert(
        "ElasticBehavior",
        RbxEnum {
            name: "ElasticBehavior",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("WhenScrollable", 0);
                items.insert("Always", 1);
                items.insert("Never", 2);
                items
            },
        },
    );
    output.insert(
        "EnviromentalPhysicsThrottle",
        RbxEnum {
            name: "EnviromentalPhysicsThrottle",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("DefaultAuto", 0);
                items.insert("Disabled", 1);
                items.insert("Always", 2);
                items.insert("Skip2", 3);
                items.insert("Skip4", 4);
                items.insert("Skip8", 5);
                items.insert("Skip16", 6);
                items
            },
        },
    );
    output.insert(
        "ErrorReporting",
        RbxEnum {
            name: "ErrorReporting",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("DontReport", 0);
                items.insert("Prompt", 1);
                items.insert("Report", 2);
                items
            },
        },
    );
    output.insert(
        "ExplosionType",
        RbxEnum {
            name: "ExplosionType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("NoCraters", 0);
                items.insert("Craters", 1);
                items.insert("CratersAndDebris", 2);
                items
            },
        },
    );
    output.insert(
        "FillDirection",
        RbxEnum {
            name: "FillDirection",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Horizontal", 0);
                items.insert("Vertical", 1);
                items
            },
        },
    );
    output.insert(
        "FilterResult",
        RbxEnum {
            name: "FilterResult",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Rejected", 1);
                items.insert("Accepted", 0);
                items
            },
        },
    );
    output.insert(
        "Font",
        RbxEnum {
            name: "Font",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Legacy", 0);
                items.insert("Arial", 1);
                items.insert("ArialBold", 2);
                items.insert("SourceSans", 3);
                items.insert("SourceSansBold", 4);
                items.insert("SourceSansSemibold", 16);
                items.insert("SourceSansLight", 5);
                items.insert("SourceSansItalic", 6);
                items.insert("Bodoni", 7);
                items.insert("Garamond", 8);
                items.insert("Cartoon", 9);
                items.insert("Code", 10);
                items.insert("Highway", 11);
                items.insert("SciFi", 12);
                items.insert("Arcade", 13);
                items.insert("Fantasy", 14);
                items.insert("Antique", 15);
                items.insert("Gotham", 17);
                items.insert("GothamSemibold", 18);
                items.insert("GothamBold", 19);
                items.insert("GothamBlack", 20);
                items
            },
        },
    );
    output.insert(
        "FontSize",
        RbxEnum {
            name: "FontSize",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Size8", 0);
                items.insert("Size9", 1);
                items.insert("Size10", 2);
                items.insert("Size11", 3);
                items.insert("Size12", 4);
                items.insert("Size14", 5);
                items.insert("Size18", 6);
                items.insert("Size24", 7);
                items.insert("Size36", 8);
                items.insert("Size48", 9);
                items.insert("Size28", 10);
                items.insert("Size32", 11);
                items.insert("Size42", 12);
                items.insert("Size60", 13);
                items.insert("Size96", 14);
                items
            },
        },
    );
    output.insert(
        "FormFactor",
        RbxEnum {
            name: "FormFactor",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Symmetric", 0);
                items.insert("Brick", 1);
                items.insert("Plate", 2);
                items.insert("Custom", 3);
                items
            },
        },
    );
    output.insert(
        "FrameStyle",
        RbxEnum {
            name: "FrameStyle",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Custom", 0);
                items.insert("ChatBlue", 1);
                items.insert("RobloxSquare", 2);
                items.insert("RobloxRound", 3);
                items.insert("ChatGreen", 4);
                items.insert("ChatRed", 5);
                items.insert("DropShadow", 6);
                items
            },
        },
    );
    output.insert(
        "FramerateManagerMode",
        RbxEnum {
            name: "FramerateManagerMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Automatic", 0);
                items.insert("On", 1);
                items.insert("Off", 2);
                items
            },
        },
    );
    output.insert(
        "FriendRequestEvent",
        RbxEnum {
            name: "FriendRequestEvent",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Issue", 0);
                items.insert("Revoke", 1);
                items.insert("Accept", 2);
                items.insert("Deny", 3);
                items
            },
        },
    );
    output.insert(
        "FriendStatus",
        RbxEnum {
            name: "FriendStatus",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Unknown", 0);
                items.insert("NotFriend", 1);
                items.insert("Friend", 2);
                items.insert("FriendRequestSent", 3);
                items.insert("FriendRequestReceived", 4);
                items
            },
        },
    );
    output.insert(
        "FunctionalTestResult",
        RbxEnum {
            name: "FunctionalTestResult",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Passed", 0);
                items.insert("Warning", 1);
                items.insert("Error", 2);
                items
            },
        },
    );
    output.insert(
        "GameAvatarType",
        RbxEnum {
            name: "GameAvatarType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("R6", 0);
                items.insert("R15", 1);
                items.insert("PlayerChoice", 2);
                items
            },
        },
    );
    output.insert(
        "GearGenreSetting",
        RbxEnum {
            name: "GearGenreSetting",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("AllGenres", 0);
                items.insert("MatchingGenreOnly", 1);
                items
            },
        },
    );
    output.insert(
        "GearType",
        RbxEnum {
            name: "GearType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("MeleeWeapons", 0);
                items.insert("RangedWeapons", 1);
                items.insert("Explosives", 2);
                items.insert("PowerUps", 3);
                items.insert("NavigationEnhancers", 4);
                items.insert("MusicalInstruments", 5);
                items.insert("SocialItems", 6);
                items.insert("BuildingTools", 7);
                items.insert("Transport", 8);
                items
            },
        },
    );
    output.insert(
        "Genre",
        RbxEnum {
            name: "Genre",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("All", 0);
                items.insert("TownAndCity", 1);
                items.insert("Fantasy", 2);
                items.insert("SciFi", 3);
                items.insert("Ninja", 4);
                items.insert("Scary", 5);
                items.insert("Pirate", 6);
                items.insert("Adventure", 7);
                items.insert("Sports", 8);
                items.insert("Funny", 9);
                items.insert("WildWest", 10);
                items.insert("War", 11);
                items.insert("SkatePark", 12);
                items.insert("Tutorial", 13);
                items
            },
        },
    );
    output.insert(
        "GraphicsMode",
        RbxEnum {
            name: "GraphicsMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Automatic", 1);
                items.insert("Direct3D9", 3);
                items.insert("Direct3D11", 2);
                items.insert("OpenGL", 4);
                items.insert("Metal", 5);
                items.insert("Vulkan", 6);
                items.insert("NoGraphics", 7);
                items
            },
        },
    );
    output.insert(
        "HandlesStyle",
        RbxEnum {
            name: "HandlesStyle",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Resize", 0);
                items.insert("Movement", 1);
                items
            },
        },
    );
    output.insert(
        "HorizontalAlignment",
        RbxEnum {
            name: "HorizontalAlignment",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Center", 0);
                items.insert("Left", 1);
                items.insert("Right", 2);
                items
            },
        },
    );
    output.insert(
        "HoverAnimateSpeed",
        RbxEnum {
            name: "HoverAnimateSpeed",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("VerySlow", 0);
                items.insert("Slow", 1);
                items.insert("Medium", 2);
                items.insert("Fast", 3);
                items.insert("VeryFast", 4);
                items
            },
        },
    );
    output.insert(
        "HttpCachePolicy",
        RbxEnum {
            name: "HttpCachePolicy",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 0);
                items.insert("Full", 1);
                items.insert("DataOnly", 2);
                items.insert("Default", 3);
                items.insert("InternalRedirectRefresh", 4);
                items
            },
        },
    );
    output.insert(
        "HttpContentType",
        RbxEnum {
            name: "HttpContentType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("ApplicationJson", 0);
                items.insert("ApplicationXml", 1);
                items.insert("ApplicationUrlEncoded", 2);
                items.insert("TextPlain", 3);
                items.insert("TextXml", 4);
                items
            },
        },
    );
    output.insert(
        "HttpError",
        RbxEnum {
            name: "HttpError",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("OK", 0);
                items.insert("InvalidUrl", 1);
                items.insert("DnsResolve", 2);
                items.insert("ConnectFail", 3);
                items.insert("OutOfMemory", 4);
                items.insert("TimedOut", 5);
                items.insert("TooManyRedirects", 6);
                items.insert("InvalidRedirect", 7);
                items.insert("NetFail", 8);
                items.insert("Aborted", 9);
                items.insert("SslConnectFail", 10);
                items.insert("Unknown", 11);
                items
            },
        },
    );
    output.insert(
        "HttpRequestType",
        RbxEnum {
            name: "HttpRequestType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("MarketplaceService", 2);
                items.insert("Players", 7);
                items.insert("Chat", 15);
                items.insert("Avatar", 16);
                items.insert("Analytics", 22);
                items.insert("Localization", 24);
                items
            },
        },
    );
    output.insert(
        "HumanoidDisplayDistanceType",
        RbxEnum {
            name: "HumanoidDisplayDistanceType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Viewer", 0);
                items.insert("Subject", 1);
                items.insert("None", 2);
                items
            },
        },
    );
    output.insert(
        "HumanoidHealthDisplayType",
        RbxEnum {
            name: "HumanoidHealthDisplayType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("DisplayWhenDamaged", 0);
                items.insert("AlwaysOn", 1);
                items.insert("AlwaysOff", 2);
                items
            },
        },
    );
    output.insert(
        "HumanoidRigType",
        RbxEnum {
            name: "HumanoidRigType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("R6", 0);
                items.insert("R15", 1);
                items
            },
        },
    );
    output.insert(
        "HumanoidStateType",
        RbxEnum {
            name: "HumanoidStateType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("FallingDown", 0);
                items.insert("Running", 8);
                items.insert("RunningNoPhysics", 10);
                items.insert("Climbing", 12);
                items.insert("StrafingNoPhysics", 11);
                items.insert("Ragdoll", 1);
                items.insert("GettingUp", 2);
                items.insert("Jumping", 3);
                items.insert("Landed", 7);
                items.insert("Flying", 6);
                items.insert("Freefall", 5);
                items.insert("Seated", 13);
                items.insert("PlatformStanding", 14);
                items.insert("Dead", 15);
                items.insert("Swimming", 4);
                items.insert("Physics", 16);
                items.insert("None", 18);
                items
            },
        },
    );
    output.insert(
        "InOut",
        RbxEnum {
            name: "InOut",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Edge", 0);
                items.insert("Inset", 1);
                items.insert("Center", 2);
                items
            },
        },
    );
    output.insert(
        "InfoType",
        RbxEnum {
            name: "InfoType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Asset", 0);
                items.insert("Product", 1);
                items.insert("GamePass", 2);
                items
            },
        },
    );
    output.insert(
        "InitialDockState",
        RbxEnum {
            name: "InitialDockState",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Top", 0);
                items.insert("Bottom", 1);
                items.insert("Left", 2);
                items.insert("Right", 3);
                items.insert("Float", 4);
                items
            },
        },
    );
    output.insert(
        "InputType",
        RbxEnum {
            name: "InputType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("NoInput", 0);
                items.insert("Constant", 12);
                items.insert("Sin", 13);
                items
            },
        },
    );
    output.insert(
        "JointCreationMode",
        RbxEnum {
            name: "JointCreationMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("All", 0);
                items.insert("Surface", 1);
                items.insert("None", 2);
                items
            },
        },
    );
    output.insert(
        "JointType",
        RbxEnum {
            name: "JointType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 28);
                items.insert("Rotate", 7);
                items.insert("RotateP", 8);
                items.insert("RotateV", 9);
                items.insert("Glue", 10);
                items.insert("Weld", 1);
                items.insert("Snap", 3);
                items
            },
        },
    );
    output.insert(
        "KeyCode",
        RbxEnum {
            name: "KeyCode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Unknown", 0);
                items.insert("Backspace", 8);
                items.insert("Tab", 9);
                items.insert("Clear", 12);
                items.insert("Return", 13);
                items.insert("Pause", 19);
                items.insert("Escape", 27);
                items.insert("Space", 32);
                items.insert("QuotedDouble", 34);
                items.insert("Hash", 35);
                items.insert("Dollar", 36);
                items.insert("Percent", 37);
                items.insert("Ampersand", 38);
                items.insert("Quote", 39);
                items.insert("LeftParenthesis", 40);
                items.insert("RightParenthesis", 41);
                items.insert("Asterisk", 42);
                items.insert("Plus", 43);
                items.insert("Comma", 44);
                items.insert("Minus", 45);
                items.insert("Period", 46);
                items.insert("Slash", 47);
                items.insert("Zero", 48);
                items.insert("One", 49);
                items.insert("Two", 50);
                items.insert("Three", 51);
                items.insert("Four", 52);
                items.insert("Five", 53);
                items.insert("Six", 54);
                items.insert("Seven", 55);
                items.insert("Eight", 56);
                items.insert("Nine", 57);
                items.insert("Colon", 58);
                items.insert("Semicolon", 59);
                items.insert("LessThan", 60);
                items.insert("Equals", 61);
                items.insert("GreaterThan", 62);
                items.insert("Question", 63);
                items.insert("At", 64);
                items.insert("LeftBracket", 91);
                items.insert("BackSlash", 92);
                items.insert("RightBracket", 93);
                items.insert("Caret", 94);
                items.insert("Underscore", 95);
                items.insert("Backquote", 96);
                items.insert("A", 97);
                items.insert("B", 98);
                items.insert("C", 99);
                items.insert("D", 100);
                items.insert("E", 101);
                items.insert("F", 102);
                items.insert("G", 103);
                items.insert("H", 104);
                items.insert("I", 105);
                items.insert("J", 106);
                items.insert("K", 107);
                items.insert("L", 108);
                items.insert("M", 109);
                items.insert("N", 110);
                items.insert("O", 111);
                items.insert("P", 112);
                items.insert("Q", 113);
                items.insert("R", 114);
                items.insert("S", 115);
                items.insert("T", 116);
                items.insert("U", 117);
                items.insert("V", 118);
                items.insert("W", 119);
                items.insert("X", 120);
                items.insert("Y", 121);
                items.insert("Z", 122);
                items.insert("LeftCurly", 123);
                items.insert("Pipe", 124);
                items.insert("RightCurly", 125);
                items.insert("Tilde", 126);
                items.insert("Delete", 127);
                items.insert("KeypadZero", 256);
                items.insert("KeypadOne", 257);
                items.insert("KeypadTwo", 258);
                items.insert("KeypadThree", 259);
                items.insert("KeypadFour", 260);
                items.insert("KeypadFive", 261);
                items.insert("KeypadSix", 262);
                items.insert("KeypadSeven", 263);
                items.insert("KeypadEight", 264);
                items.insert("KeypadNine", 265);
                items.insert("KeypadPeriod", 266);
                items.insert("KeypadDivide", 267);
                items.insert("KeypadMultiply", 268);
                items.insert("KeypadMinus", 269);
                items.insert("KeypadPlus", 270);
                items.insert("KeypadEnter", 271);
                items.insert("KeypadEquals", 272);
                items.insert("Up", 273);
                items.insert("Down", 274);
                items.insert("Right", 275);
                items.insert("Left", 276);
                items.insert("Insert", 277);
                items.insert("Home", 278);
                items.insert("End", 279);
                items.insert("PageUp", 280);
                items.insert("PageDown", 281);
                items.insert("LeftShift", 304);
                items.insert("RightShift", 303);
                items.insert("LeftMeta", 310);
                items.insert("RightMeta", 309);
                items.insert("LeftAlt", 308);
                items.insert("RightAlt", 307);
                items.insert("LeftControl", 306);
                items.insert("RightControl", 305);
                items.insert("CapsLock", 301);
                items.insert("NumLock", 300);
                items.insert("ScrollLock", 302);
                items.insert("LeftSuper", 311);
                items.insert("RightSuper", 312);
                items.insert("Mode", 313);
                items.insert("Compose", 314);
                items.insert("Help", 315);
                items.insert("Print", 316);
                items.insert("SysReq", 317);
                items.insert("Break", 318);
                items.insert("Menu", 319);
                items.insert("Power", 320);
                items.insert("Euro", 321);
                items.insert("Undo", 322);
                items.insert("F1", 282);
                items.insert("F2", 283);
                items.insert("F3", 284);
                items.insert("F4", 285);
                items.insert("F5", 286);
                items.insert("F6", 287);
                items.insert("F7", 288);
                items.insert("F8", 289);
                items.insert("F9", 290);
                items.insert("F10", 291);
                items.insert("F11", 292);
                items.insert("F12", 293);
                items.insert("F13", 294);
                items.insert("F14", 295);
                items.insert("F15", 296);
                items.insert("World0", 160);
                items.insert("World1", 161);
                items.insert("World2", 162);
                items.insert("World3", 163);
                items.insert("World4", 164);
                items.insert("World5", 165);
                items.insert("World6", 166);
                items.insert("World7", 167);
                items.insert("World8", 168);
                items.insert("World9", 169);
                items.insert("World10", 170);
                items.insert("World11", 171);
                items.insert("World12", 172);
                items.insert("World13", 173);
                items.insert("World14", 174);
                items.insert("World15", 175);
                items.insert("World16", 176);
                items.insert("World17", 177);
                items.insert("World18", 178);
                items.insert("World19", 179);
                items.insert("World20", 180);
                items.insert("World21", 181);
                items.insert("World22", 182);
                items.insert("World23", 183);
                items.insert("World24", 184);
                items.insert("World25", 185);
                items.insert("World26", 186);
                items.insert("World27", 187);
                items.insert("World28", 188);
                items.insert("World29", 189);
                items.insert("World30", 190);
                items.insert("World31", 191);
                items.insert("World32", 192);
                items.insert("World33", 193);
                items.insert("World34", 194);
                items.insert("World35", 195);
                items.insert("World36", 196);
                items.insert("World37", 197);
                items.insert("World38", 198);
                items.insert("World39", 199);
                items.insert("World40", 200);
                items.insert("World41", 201);
                items.insert("World42", 202);
                items.insert("World43", 203);
                items.insert("World44", 204);
                items.insert("World45", 205);
                items.insert("World46", 206);
                items.insert("World47", 207);
                items.insert("World48", 208);
                items.insert("World49", 209);
                items.insert("World50", 210);
                items.insert("World51", 211);
                items.insert("World52", 212);
                items.insert("World53", 213);
                items.insert("World54", 214);
                items.insert("World55", 215);
                items.insert("World56", 216);
                items.insert("World57", 217);
                items.insert("World58", 218);
                items.insert("World59", 219);
                items.insert("World60", 220);
                items.insert("World61", 221);
                items.insert("World62", 222);
                items.insert("World63", 223);
                items.insert("World64", 224);
                items.insert("World65", 225);
                items.insert("World66", 226);
                items.insert("World67", 227);
                items.insert("World68", 228);
                items.insert("World69", 229);
                items.insert("World70", 230);
                items.insert("World71", 231);
                items.insert("World72", 232);
                items.insert("World73", 233);
                items.insert("World74", 234);
                items.insert("World75", 235);
                items.insert("World76", 236);
                items.insert("World77", 237);
                items.insert("World78", 238);
                items.insert("World79", 239);
                items.insert("World80", 240);
                items.insert("World81", 241);
                items.insert("World82", 242);
                items.insert("World83", 243);
                items.insert("World84", 244);
                items.insert("World85", 245);
                items.insert("World86", 246);
                items.insert("World87", 247);
                items.insert("World88", 248);
                items.insert("World89", 249);
                items.insert("World90", 250);
                items.insert("World91", 251);
                items.insert("World92", 252);
                items.insert("World93", 253);
                items.insert("World94", 254);
                items.insert("World95", 255);
                items.insert("ButtonX", 1000);
                items.insert("ButtonY", 1001);
                items.insert("ButtonA", 1002);
                items.insert("ButtonB", 1003);
                items.insert("ButtonR1", 1004);
                items.insert("ButtonL1", 1005);
                items.insert("ButtonR2", 1006);
                items.insert("ButtonL2", 1007);
                items.insert("ButtonR3", 1008);
                items.insert("ButtonL3", 1009);
                items.insert("ButtonStart", 1010);
                items.insert("ButtonSelect", 1011);
                items.insert("DPadLeft", 1012);
                items.insert("DPadRight", 1013);
                items.insert("DPadUp", 1014);
                items.insert("DPadDown", 1015);
                items.insert("Thumbstick1", 1016);
                items.insert("Thumbstick2", 1017);
                items
            },
        },
    );
    output.insert(
        "KeywordFilterType",
        RbxEnum {
            name: "KeywordFilterType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Include", 0);
                items.insert("Exclude", 1);
                items
            },
        },
    );
    output.insert(
        "Language",
        RbxEnum {
            name: "Language",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items
            },
        },
    );
    output.insert(
        "LeftRight",
        RbxEnum {
            name: "LeftRight",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Left", 0);
                items.insert("Center", 1);
                items.insert("Right", 2);
                items
            },
        },
    );
    output.insert(
        "LevelOfDetailSetting",
        RbxEnum {
            name: "LevelOfDetailSetting",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("High", 2);
                items.insert("Medium", 1);
                items.insert("Low", 0);
                items
            },
        },
    );
    output.insert(
        "Limb",
        RbxEnum {
            name: "Limb",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Head", 0);
                items.insert("Torso", 1);
                items.insert("LeftArm", 2);
                items.insert("RightArm", 3);
                items.insert("LeftLeg", 4);
                items.insert("RightLeg", 5);
                items.insert("Unknown", 6);
                items
            },
        },
    );
    output.insert(
        "ListDisplayMode",
        RbxEnum {
            name: "ListDisplayMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Horizontal", 0);
                items.insert("Vertical", 1);
                items
            },
        },
    );
    output.insert(
        "ListenerType",
        RbxEnum {
            name: "ListenerType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Camera", 0);
                items.insert("CFrame", 1);
                items.insert("ObjectPosition", 2);
                items.insert("ObjectCFrame", 3);
                items
            },
        },
    );
    output.insert(
        "Material",
        RbxEnum {
            name: "Material",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Plastic", 256);
                items.insert("Wood", 512);
                items.insert("Slate", 800);
                items.insert("Concrete", 816);
                items.insert("CorrodedMetal", 1040);
                items.insert("DiamondPlate", 1056);
                items.insert("Foil", 1072);
                items.insert("Grass", 1280);
                items.insert("Ice", 1536);
                items.insert("Marble", 784);
                items.insert("Granite", 832);
                items.insert("Brick", 848);
                items.insert("Pebble", 864);
                items.insert("Sand", 1296);
                items.insert("Fabric", 1312);
                items.insert("SmoothPlastic", 272);
                items.insert("Metal", 1088);
                items.insert("WoodPlanks", 528);
                items.insert("Cobblestone", 880);
                items.insert("Air", 1792);
                items.insert("Water", 2048);
                items.insert("Rock", 896);
                items.insert("Glacier", 1552);
                items.insert("Snow", 1328);
                items.insert("Sandstone", 912);
                items.insert("Mud", 1344);
                items.insert("Basalt", 788);
                items.insert("Ground", 1360);
                items.insert("CrackedLava", 804);
                items.insert("Neon", 288);
                items.insert("Glass", 1568);
                items.insert("Asphalt", 1376);
                items.insert("LeafyGrass", 1284);
                items.insert("Salt", 1392);
                items.insert("Limestone", 820);
                items.insert("Pavement", 836);
                items
            },
        },
    );
    output.insert(
        "MembershipType",
        RbxEnum {
            name: "MembershipType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 0);
                items.insert("BuildersClub", 1);
                items.insert("TurboBuildersClub", 2);
                items.insert("OutrageousBuildersClub", 3);
                items
            },
        },
    );
    output.insert(
        "MeshType",
        RbxEnum {
            name: "MeshType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Head", 0);
                items.insert("Torso", 1);
                items.insert("Wedge", 2);
                items.insert("Prism", 7);
                items.insert("Pyramid", 8);
                items.insert("ParallelRamp", 9);
                items.insert("RightAngleRamp", 10);
                items.insert("CornerWedge", 11);
                items.insert("Brick", 6);
                items.insert("Sphere", 3);
                items.insert("Cylinder", 4);
                items.insert("FileMesh", 5);
                items
            },
        },
    );
    output.insert(
        "MessageType",
        RbxEnum {
            name: "MessageType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("MessageOutput", 0);
                items.insert("MessageInfo", 1);
                items.insert("MessageWarning", 2);
                items.insert("MessageError", 3);
                items
            },
        },
    );
    output.insert(
        "MouseBehavior",
        RbxEnum {
            name: "MouseBehavior",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("LockCenter", 1);
                items.insert("LockCurrentPosition", 2);
                items
            },
        },
    );
    output.insert(
        "MoveState",
        RbxEnum {
            name: "MoveState",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Stopped", 0);
                items.insert("Coasting", 1);
                items.insert("Pushing", 2);
                items.insert("Stopping", 3);
                items.insert("AirFree", 4);
                items
            },
        },
    );
    output.insert(
        "NameOcclusion",
        RbxEnum {
            name: "NameOcclusion",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("OccludeAll", 2);
                items.insert("EnemyOcclusion", 1);
                items.insert("NoOcclusion", 0);
                items
            },
        },
    );
    output.insert(
        "NetworkOwnership",
        RbxEnum {
            name: "NetworkOwnership",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Automatic", 0);
                items.insert("Manual", 1);
                items.insert("OnContact", 2);
                items
            },
        },
    );
    output.insert(
        "NormalId",
        RbxEnum {
            name: "NormalId",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Top", 1);
                items.insert("Bottom", 4);
                items.insert("Back", 2);
                items.insert("Front", 5);
                items.insert("Right", 0);
                items.insert("Left", 3);
                items
            },
        },
    );
    output.insert(
        "OutputLayoutMode",
        RbxEnum {
            name: "OutputLayoutMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Horizontal", 0);
                items.insert("Vertical", 1);
                items
            },
        },
    );
    output.insert(
        "OverrideMouseIconBehavior",
        RbxEnum {
            name: "OverrideMouseIconBehavior",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 0);
                items.insert("ForceShow", 1);
                items.insert("ForceHide", 2);
                items
            },
        },
    );
    output.insert(
        "PacketPriority",
        RbxEnum {
            name: "PacketPriority",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("IMMEDIATE_PRIORITY", 0);
                items.insert("HIGH_PRIORITY", 1);
                items.insert("MEDIUM_PRIORITY", 2);
                items.insert("LOW_PRIORITY", 3);
                items
            },
        },
    );
    output.insert(
        "PartType",
        RbxEnum {
            name: "PartType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Ball", 0);
                items.insert("Block", 1);
                items.insert("Cylinder", 2);
                items
            },
        },
    );
    output.insert(
        "PathStatus",
        RbxEnum {
            name: "PathStatus",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Success", 0);
                items.insert("ClosestNoPath", 1);
                items.insert("ClosestOutOfRange", 2);
                items.insert("FailStartNotEmpty", 3);
                items.insert("FailFinishNotEmpty", 4);
                items.insert("NoPath", 5);
                items
            },
        },
    );
    output.insert(
        "PathWaypointAction",
        RbxEnum {
            name: "PathWaypointAction",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Walk", 0);
                items.insert("Jump", 1);
                items
            },
        },
    );
    output.insert(
        "PermissionLevelShown",
        RbxEnum {
            name: "PermissionLevelShown",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Game", 0);
                items.insert("RobloxGame", 1);
                items.insert("RobloxScript", 2);
                items.insert("Studio", 3);
                items.insert("Roblox", 4);
                items
            },
        },
    );
    output.insert(
        "Platform",
        RbxEnum {
            name: "Platform",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Windows", 0);
                items.insert("OSX", 1);
                items.insert("IOS", 2);
                items.insert("Android", 3);
                items.insert("XBoxOne", 4);
                items.insert("PS4", 5);
                items.insert("PS3", 6);
                items.insert("XBox360", 7);
                items.insert("WiiU", 8);
                items.insert("NX", 9);
                items.insert("Ouya", 10);
                items.insert("AndroidTV", 11);
                items.insert("Chromecast", 12);
                items.insert("Linux", 13);
                items.insert("SteamOS", 14);
                items.insert("WebOS", 15);
                items.insert("DOS", 16);
                items.insert("BeOS", 17);
                items.insert("UWP", 18);
                items.insert("None", 19);
                items
            },
        },
    );
    output.insert(
        "PlaybackState",
        RbxEnum {
            name: "PlaybackState",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Begin", 0);
                items.insert("Delayed", 1);
                items.insert("Playing", 2);
                items.insert("Paused", 3);
                items.insert("Completed", 4);
                items.insert("Cancelled", 5);
                items
            },
        },
    );
    output.insert(
        "PlayerActions",
        RbxEnum {
            name: "PlayerActions",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("CharacterForward", 0);
                items.insert("CharacterBackward", 1);
                items.insert("CharacterLeft", 2);
                items.insert("CharacterRight", 3);
                items.insert("CharacterJump", 4);
                items
            },
        },
    );
    output.insert(
        "PlayerChatType",
        RbxEnum {
            name: "PlayerChatType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("All", 0);
                items.insert("Team", 1);
                items.insert("Whisper", 2);
                items
            },
        },
    );
    output.insert(
        "PoseEasingDirection",
        RbxEnum {
            name: "PoseEasingDirection",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Out", 1);
                items.insert("InOut", 2);
                items.insert("In", 0);
                items
            },
        },
    );
    output.insert(
        "PoseEasingStyle",
        RbxEnum {
            name: "PoseEasingStyle",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Linear", 0);
                items.insert("Constant", 1);
                items.insert("Elastic", 2);
                items.insert("Cubic", 3);
                items.insert("Bounce", 4);
                items
            },
        },
    );
    output.insert(
        "PrivilegeType",
        RbxEnum {
            name: "PrivilegeType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Owner", 255);
                items.insert("Admin", 240);
                items.insert("Member", 128);
                items.insert("Visitor", 10);
                items.insert("Banned", 0);
                items
            },
        },
    );
    output.insert(
        "ProductPurchaseDecision",
        RbxEnum {
            name: "ProductPurchaseDecision",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("NotProcessedYet", 0);
                items.insert("PurchaseGranted", 1);
                items
            },
        },
    );
    output.insert(
        "QualityLevel",
        RbxEnum {
            name: "QualityLevel",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Automatic", 0);
                items.insert("Level01", 1);
                items.insert("Level02", 2);
                items.insert("Level03", 3);
                items.insert("Level04", 4);
                items.insert("Level05", 5);
                items.insert("Level06", 6);
                items.insert("Level07", 7);
                items.insert("Level08", 8);
                items.insert("Level09", 9);
                items.insert("Level10", 10);
                items.insert("Level11", 11);
                items.insert("Level12", 12);
                items.insert("Level13", 13);
                items.insert("Level14", 14);
                items.insert("Level15", 15);
                items.insert("Level16", 16);
                items.insert("Level17", 17);
                items.insert("Level18", 18);
                items.insert("Level19", 19);
                items.insert("Level20", 20);
                items.insert("Level21", 21);
                items
            },
        },
    );
    output.insert(
        "R15CollisionType",
        RbxEnum {
            name: "R15CollisionType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("OuterBox", 0);
                items.insert("InnerBox", 1);
                items
            },
        },
    );
    output.insert(
        "RenderFidelity",
        RbxEnum {
            name: "RenderFidelity",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Automatic", 0);
                items.insert("Precise", 1);
                items
            },
        },
    );
    output.insert(
        "RenderPriority",
        RbxEnum {
            name: "RenderPriority",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("First", 0);
                items.insert("Input", 100);
                items.insert("Camera", 200);
                items.insert("Character", 300);
                items.insert("Last", 2000);
                items
            },
        },
    );
    output.insert(
        "RenderingTestComparisonMethod",
        RbxEnum {
            name: "RenderingTestComparisonMethod",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("psnr", 0);
                items.insert("diff", 1);
                items
            },
        },
    );
    output.insert(
        "ReverbType",
        RbxEnum {
            name: "ReverbType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("NoReverb", 0);
                items.insert("GenericReverb", 1);
                items.insert("PaddedCell", 2);
                items.insert("Room", 3);
                items.insert("Bathroom", 4);
                items.insert("LivingRoom", 5);
                items.insert("StoneRoom", 6);
                items.insert("Auditorium", 7);
                items.insert("ConcertHall", 8);
                items.insert("Cave", 9);
                items.insert("Arena", 10);
                items.insert("Hangar", 11);
                items.insert("CarpettedHallway", 12);
                items.insert("Hallway", 13);
                items.insert("StoneCorridor", 14);
                items.insert("Alley", 15);
                items.insert("Forest", 16);
                items.insert("City", 17);
                items.insert("Mountains", 18);
                items.insert("Quarry", 19);
                items.insert("Plain", 20);
                items.insert("ParkingLot", 21);
                items.insert("SewerPipe", 22);
                items.insert("UnderWater", 23);
                items
            },
        },
    );
    output.insert(
        "RibbonTool",
        RbxEnum {
            name: "RibbonTool",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Select", 0);
                items.insert("Scale", 1);
                items.insert("Rotate", 2);
                items.insert("Move", 3);
                items.insert("Transform", 4);
                items.insert("ColorPicker", 5);
                items.insert("MaterialPicker", 6);
                items.insert("Group", 7);
                items.insert("Ungroup", 8);
                items.insert("None", 9);
                items
            },
        },
    );
    output.insert(
        "RollOffMode",
        RbxEnum {
            name: "RollOffMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Inverse", 0);
                items.insert("Linear", 1);
                items.insert("InverseTapered", 3);
                items.insert("LinearSquare", 2);
                items
            },
        },
    );
    output.insert(
        "RotationType",
        RbxEnum {
            name: "RotationType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("MovementRelative", 0);
                items.insert("CameraRelative", 1);
                items
            },
        },
    );
    output.insert(
        "RuntimeUndoBehavior",
        RbxEnum {
            name: "RuntimeUndoBehavior",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Aggregate", 0);
                items.insert("Snapshot", 1);
                items.insert("Hybrid", 2);
                items
            },
        },
    );
    output.insert(
        "SaveFilter",
        RbxEnum {
            name: "SaveFilter",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("SaveAll", 2);
                items.insert("SaveWorld", 0);
                items.insert("SaveGame", 1);
                items
            },
        },
    );
    output.insert(
        "SavedQualitySetting",
        RbxEnum {
            name: "SavedQualitySetting",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Automatic", 0);
                items.insert("QualityLevel1", 1);
                items.insert("QualityLevel2", 2);
                items.insert("QualityLevel3", 3);
                items.insert("QualityLevel4", 4);
                items.insert("QualityLevel5", 5);
                items.insert("QualityLevel6", 6);
                items.insert("QualityLevel7", 7);
                items.insert("QualityLevel8", 8);
                items.insert("QualityLevel9", 9);
                items.insert("QualityLevel10", 10);
                items
            },
        },
    );
    output.insert(
        "ScaleType",
        RbxEnum {
            name: "ScaleType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Stretch", 0);
                items.insert("Slice", 1);
                items.insert("Tile", 2);
                items.insert("Fit", 3);
                items.insert("Crop", 4);
                items
            },
        },
    );
    output.insert(
        "ScreenOrientation",
        RbxEnum {
            name: "ScreenOrientation",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("LandscapeLeft", 0);
                items.insert("LandscapeRight", 1);
                items.insert("LandscapeSensor", 2);
                items.insert("Portrait", 3);
                items.insert("Sensor", 4);
                items
            },
        },
    );
    output.insert(
        "ScrollBarInset",
        RbxEnum {
            name: "ScrollBarInset",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 0);
                items.insert("ScrollBar", 1);
                items.insert("Always", 2);
                items
            },
        },
    );
    output.insert(
        "ScrollingDirection",
        RbxEnum {
            name: "ScrollingDirection",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("X", 1);
                items.insert("Y", 2);
                items.insert("XY", 4);
                items
            },
        },
    );
    output.insert(
        "ServerAudioBehavior",
        RbxEnum {
            name: "ServerAudioBehavior",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Enabled", 0);
                items.insert("Muted", 1);
                items.insert("OnlineGame", 2);
                items
            },
        },
    );
    output.insert(
        "SizeConstraint",
        RbxEnum {
            name: "SizeConstraint",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("RelativeXY", 0);
                items.insert("RelativeXX", 1);
                items.insert("RelativeYY", 2);
                items
            },
        },
    );
    output.insert(
        "SortOrder",
        RbxEnum {
            name: "SortOrder",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("LayoutOrder", 2);
                items.insert("Name", 0);
                items.insert("Custom", 1);
                items
            },
        },
    );
    output.insert(
        "SoundType",
        RbxEnum {
            name: "SoundType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("NoSound", 0);
                items.insert("Boing", 1);
                items.insert("Bomb", 2);
                items.insert("Break", 3);
                items.insert("Click", 4);
                items.insert("Clock", 5);
                items.insert("Slingshot", 6);
                items.insert("Page", 7);
                items.insert("Ping", 8);
                items.insert("Snap", 9);
                items.insert("Splat", 10);
                items.insert("Step", 11);
                items.insert("StepOn", 12);
                items.insert("Swoosh", 13);
                items.insert("Victory", 14);
                items
            },
        },
    );
    output.insert(
        "SpecialKey",
        RbxEnum {
            name: "SpecialKey",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Insert", 0);
                items.insert("Home", 1);
                items.insert("End", 2);
                items.insert("PageUp", 3);
                items.insert("PageDown", 4);
                items.insert("ChatHotkey", 5);
                items
            },
        },
    );
    output.insert(
        "StartCorner",
        RbxEnum {
            name: "StartCorner",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("TopLeft", 0);
                items.insert("TopRight", 1);
                items.insert("BottomLeft", 2);
                items.insert("BottomRight", 3);
                items
            },
        },
    );
    output.insert(
        "Status",
        RbxEnum {
            name: "Status",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Poison", 0);
                items.insert("Confusion", 1);
                items
            },
        },
    );
    output.insert(
        "StudioStyleGuideColor",
        RbxEnum {
            name: "StudioStyleGuideColor",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("MainBackground", 0);
                items.insert("Titlebar", 1);
                items.insert("Dropdown", 2);
                items.insert("Tooltip", 3);
                items.insert("Notification", 4);
                items.insert("ScrollBar", 5);
                items.insert("ScrollBarBackground", 6);
                items.insert("TabBar", 7);
                items.insert("Tab", 8);
                items.insert("RibbonTab", 9);
                items.insert("RibbonTabTopBar", 10);
                items.insert("Button", 11);
                items.insert("MainButton", 12);
                items.insert("RibbonButton", 13);
                items.insert("ViewPortBackground", 14);
                items.insert("InputFieldBackground", 15);
                items.insert("Item", 16);
                items.insert("TableItem", 17);
                items.insert("CategoryItem", 18);
                items.insert("GameSettingsTableItem", 19);
                items.insert("GameSettingsTooltip", 20);
                items.insert("EmulatorBar", 21);
                items.insert("EmulatorDropDown", 22);
                items.insert("ColorPickerFrame", 23);
                items.insert("CurrentMarker", 24);
                items.insert("Border", 25);
                items.insert("Shadow", 26);
                items.insert("Light", 27);
                items.insert("Dark", 28);
                items.insert("Mid", 29);
                items.insert("MainText", 30);
                items.insert("SubText", 31);
                items.insert("TitlebarText", 32);
                items.insert("BrightText", 33);
                items.insert("DimmedText", 34);
                items.insert("LinkText", 35);
                items.insert("WarningText", 36);
                items.insert("ErrorText", 37);
                items.insert("InfoText", 38);
                items.insert("SensitiveText", 39);
                items.insert("ScriptSideWidget", 40);
                items.insert("ScriptBackground", 41);
                items.insert("ScriptText", 42);
                items.insert("ScriptSelectionText", 43);
                items.insert("ScriptSelectionBackground", 44);
                items.insert("ScriptFindSelectionBackground", 45);
                items.insert("ScriptMatchingWordSelectionBackground", 46);
                items.insert("ScriptOperator", 47);
                items.insert("ScriptNumber", 48);
                items.insert("ScriptString", 49);
                items.insert("ScriptComment", 50);
                items.insert("ScriptPreprocessor", 51);
                items.insert("ScriptKeyword", 52);
                items.insert("ScriptBuiltInFunction", 53);
                items.insert("ScriptWarning", 54);
                items.insert("ScriptError", 55);
                items.insert("DebuggerCurrentLine", 56);
                items.insert("DebuggerErrorLine", 57);
                items.insert("DiffFilePathText", 58);
                items.insert("DiffTextHunkInfo", 59);
                items.insert("DiffTextNoChange", 60);
                items.insert("DiffTextAddition", 61);
                items.insert("DiffTextDeletion", 62);
                items.insert("DiffTextSeparatorBackground", 63);
                items.insert("DiffTextNoChangeBackground", 64);
                items.insert("DiffTextAdditionBackground", 65);
                items.insert("DiffTextDeletionBackground", 66);
                items.insert("DiffLineNum", 67);
                items.insert("DiffLineNumSeparatorBackground", 68);
                items.insert("DiffLineNumNoChangeBackground", 69);
                items.insert("DiffLineNumAdditionBackground", 70);
                items.insert("DiffLineNumDeletionBackground", 71);
                items.insert("DiffFilePathBackground", 72);
                items.insert("DiffFilePathBorder", 73);
                items.insert("Separator", 74);
                items.insert("ButtonBorder", 75);
                items.insert("ButtonText", 76);
                items.insert("InputFieldBorder", 77);
                items.insert("CheckedFieldBackground", 78);
                items.insert("CheckedFieldBorder", 79);
                items.insert("CheckedFieldIndicator", 80);
                items.insert("HeaderSection", 81);
                items.insert("Midlight", 82);
                items.insert("StatusBar", 83);
                items
            },
        },
    );
    output.insert(
        "StudioStyleGuideModifier",
        RbxEnum {
            name: "StudioStyleGuideModifier",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("Selected", 1);
                items.insert("Pressed", 2);
                items.insert("Disabled", 3);
                items.insert("Hover", 4);
                items
            },
        },
    );
    output.insert(
        "Style",
        RbxEnum {
            name: "Style",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("AlternatingSupports", 0);
                items.insert("BridgeStyleSupports", 1);
                items.insert("NoSupports", 2);
                items
            },
        },
    );
    output.insert(
        "SurfaceConstraint",
        RbxEnum {
            name: "SurfaceConstraint",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 0);
                items.insert("Hinge", 1);
                items.insert("SteppingMotor", 2);
                items.insert("Motor", 3);
                items
            },
        },
    );
    output.insert(
        "SurfaceType",
        RbxEnum {
            name: "SurfaceType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Smooth", 0);
                items.insert("Glue", 1);
                items.insert("Weld", 2);
                items.insert("Studs", 3);
                items.insert("Inlet", 4);
                items.insert("Universal", 5);
                items.insert("Hinge", 6);
                items.insert("Motor", 7);
                items.insert("SteppingMotor", 8);
                items.insert("SmoothNoOutlines", 10);
                items
            },
        },
    );
    output.insert(
        "SwipeDirection",
        RbxEnum {
            name: "SwipeDirection",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Right", 0);
                items.insert("Left", 1);
                items.insert("Up", 2);
                items.insert("Down", 3);
                items.insert("None", 4);
                items
            },
        },
    );
    output.insert(
        "TableMajorAxis",
        RbxEnum {
            name: "TableMajorAxis",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("RowMajor", 0);
                items.insert("ColumnMajor", 1);
                items
            },
        },
    );
    output.insert(
        "Technology",
        RbxEnum {
            name: "Technology",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Legacy", 0);
                items.insert("Voxel", 1);
                items
            },
        },
    );
    output.insert(
        "TeleportResult",
        RbxEnum {
            name: "TeleportResult",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Success", 0);
                items.insert("Failure", 1);
                items.insert("GameNotFound", 2);
                items.insert("GameEnded", 3);
                items.insert("GameFull", 4);
                items.insert("Unauthorized", 5);
                items.insert("Flooded", 6);
                items.insert("IsTeleporting", 7);
                items
            },
        },
    );
    output.insert(
        "TeleportState",
        RbxEnum {
            name: "TeleportState",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("RequestedFromServer", 0);
                items.insert("Started", 1);
                items.insert("WaitingForServer", 2);
                items.insert("Failed", 3);
                items.insert("InProgress", 4);
                items
            },
        },
    );
    output.insert(
        "TeleportType",
        RbxEnum {
            name: "TeleportType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("ToPlace", 0);
                items.insert("ToInstance", 1);
                items.insert("ToReservedServer", 2);
                items
            },
        },
    );
    output.insert(
        "TextFilterContext",
        RbxEnum {
            name: "TextFilterContext",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("PublicChat", 1);
                items.insert("PrivateChat", 2);
                items
            },
        },
    );
    output.insert(
        "TextTruncate",
        RbxEnum {
            name: "TextTruncate",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 0);
                items.insert("AtEnd", 1);
                items
            },
        },
    );
    output.insert(
        "TextXAlignment",
        RbxEnum {
            name: "TextXAlignment",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Left", 0);
                items.insert("Center", 2);
                items.insert("Right", 1);
                items
            },
        },
    );
    output.insert(
        "TextYAlignment",
        RbxEnum {
            name: "TextYAlignment",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Top", 0);
                items.insert("Center", 1);
                items.insert("Bottom", 2);
                items
            },
        },
    );
    output.insert(
        "TextureMode",
        RbxEnum {
            name: "TextureMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Stretch", 0);
                items.insert("Wrap", 1);
                items.insert("Static", 2);
                items
            },
        },
    );
    output.insert(
        "TextureQueryType",
        RbxEnum {
            name: "TextureQueryType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("NonHumanoid", 0);
                items.insert("NonHumanoidOrphaned", 1);
                items.insert("Humanoid", 2);
                items.insert("HumanoidOrphaned", 3);
                items
            },
        },
    );
    output.insert(
        "ThreadPoolConfig",
        RbxEnum {
            name: "ThreadPoolConfig",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Auto", 0);
                items.insert("PerCore1", 101);
                items.insert("PerCore2", 102);
                items.insert("PerCore3", 103);
                items.insert("PerCore4", 104);
                items.insert("Threads1", 1);
                items.insert("Threads2", 2);
                items.insert("Threads3", 3);
                items.insert("Threads4", 4);
                items.insert("Threads8", 8);
                items.insert("Threads16", 16);
                items
            },
        },
    );
    output.insert(
        "ThrottlingPriority",
        RbxEnum {
            name: "ThrottlingPriority",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Extreme", 2);
                items.insert("ElevatedOnServer", 1);
                items.insert("Default", 0);
                items
            },
        },
    );
    output.insert(
        "ThumbnailSize",
        RbxEnum {
            name: "ThumbnailSize",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Size48x48", 0);
                items.insert("Size180x180", 1);
                items.insert("Size420x420", 2);
                items.insert("Size60x60", 3);
                items.insert("Size100x100", 4);
                items.insert("Size150x150", 5);
                items.insert("Size352x352", 6);
                items
            },
        },
    );
    output.insert(
        "ThumbnailType",
        RbxEnum {
            name: "ThumbnailType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("HeadShot", 0);
                items.insert("AvatarBust", 1);
                items.insert("AvatarThumbnail", 2);
                items
            },
        },
    );
    output.insert(
        "TickCountSampleMethod",
        RbxEnum {
            name: "TickCountSampleMethod",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Fast", 0);
                items.insert("Benchmark", 1);
                items.insert("Precise", 2);
                items
            },
        },
    );
    output.insert(
        "TopBottom",
        RbxEnum {
            name: "TopBottom",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Top", 0);
                items.insert("Center", 1);
                items.insert("Bottom", 2);
                items
            },
        },
    );
    output.insert(
        "TouchCameraMovementMode",
        RbxEnum {
            name: "TouchCameraMovementMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("Follow", 2);
                items.insert("Classic", 1);
                items.insert("Orbital", 3);
                items
            },
        },
    );
    output.insert(
        "TouchMovementMode",
        RbxEnum {
            name: "TouchMovementMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Default", 0);
                items.insert("Thumbstick", 1);
                items.insert("DPad", 2);
                items.insert("Thumbpad", 3);
                items.insert("ClickToMove", 4);
                items.insert("DynamicThumbstick", 5);
                items
            },
        },
    );
    output.insert(
        "TweenStatus",
        RbxEnum {
            name: "TweenStatus",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Canceled", 0);
                items.insert("Completed", 1);
                items
            },
        },
    );
    output.insert(
        "UITheme",
        RbxEnum {
            name: "UITheme",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Light", 0);
                items.insert("Dark", 1);
                items
            },
        },
    );
    output.insert(
        "UiMessageType",
        RbxEnum {
            name: "UiMessageType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("UiMessageError", 0);
                items.insert("UiMessageInfo", 1);
                items
            },
        },
    );
    output.insert(
        "UploadSetting",
        RbxEnum {
            name: "UploadSetting",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Never", 0);
                items.insert("Ask", 1);
                items.insert("Always", 2);
                items
            },
        },
    );
    output.insert(
        "UserCFrame",
        RbxEnum {
            name: "UserCFrame",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Head", 0);
                items.insert("LeftHand", 1);
                items.insert("RightHand", 2);
                items
            },
        },
    );
    output.insert(
        "UserInputState",
        RbxEnum {
            name: "UserInputState",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Begin", 0);
                items.insert("Change", 1);
                items.insert("End", 2);
                items.insert("Cancel", 3);
                items.insert("None", 4);
                items
            },
        },
    );
    output.insert(
        "UserInputType",
        RbxEnum {
            name: "UserInputType",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("MouseButton1", 0);
                items.insert("MouseButton2", 1);
                items.insert("MouseButton3", 2);
                items.insert("MouseWheel", 3);
                items.insert("MouseMovement", 4);
                items.insert("Touch", 7);
                items.insert("Keyboard", 8);
                items.insert("Focus", 9);
                items.insert("Accelerometer", 10);
                items.insert("Gyro", 11);
                items.insert("Gamepad1", 12);
                items.insert("Gamepad2", 13);
                items.insert("Gamepad3", 14);
                items.insert("Gamepad4", 15);
                items.insert("Gamepad5", 16);
                items.insert("Gamepad6", 17);
                items.insert("Gamepad7", 18);
                items.insert("Gamepad8", 19);
                items.insert("TextInput", 20);
                items.insert("None", 21);
                items
            },
        },
    );
    output.insert(
        "VRTouchpad",
        RbxEnum {
            name: "VRTouchpad",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Left", 0);
                items.insert("Right", 1);
                items
            },
        },
    );
    output.insert(
        "VRTouchpadMode",
        RbxEnum {
            name: "VRTouchpadMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Touch", 0);
                items.insert("VirtualThumbstick", 1);
                items.insert("ABXY", 2);
                items
            },
        },
    );
    output.insert(
        "VerticalAlignment",
        RbxEnum {
            name: "VerticalAlignment",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Center", 0);
                items.insert("Top", 1);
                items.insert("Bottom", 2);
                items
            },
        },
    );
    output.insert(
        "VerticalScrollBarPosition",
        RbxEnum {
            name: "VerticalScrollBarPosition",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Left", 1);
                items.insert("Right", 0);
                items
            },
        },
    );
    output.insert(
        "VibrationMotor",
        RbxEnum {
            name: "VibrationMotor",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Large", 0);
                items.insert("Small", 1);
                items.insert("LeftTrigger", 2);
                items.insert("RightTrigger", 3);
                items.insert("LeftHand", 4);
                items.insert("RightHand", 5);
                items
            },
        },
    );
    output.insert(
        "VideoQualitySettings",
        RbxEnum {
            name: "VideoQualitySettings",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("LowResolution", 0);
                items.insert("MediumResolution", 1);
                items.insert("HighResolution", 2);
                items
            },
        },
    );
    output.insert(
        "VirtualInputMode",
        RbxEnum {
            name: "VirtualInputMode",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Recording", 1);
                items.insert("Playing", 2);
                items.insert("None", 0);
                items
            },
        },
    );
    output.insert(
        "WaterDirection",
        RbxEnum {
            name: "WaterDirection",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("NegX", 0);
                items.insert("X", 1);
                items.insert("NegY", 2);
                items.insert("Y", 3);
                items.insert("NegZ", 4);
                items.insert("Z", 5);
                items
            },
        },
    );
    output.insert(
        "WaterForce",
        RbxEnum {
            name: "WaterForce",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("None", 0);
                items.insert("Small", 1);
                items.insert("Medium", 2);
                items.insert("Strong", 3);
                items.insert("Max", 4);
                items
            },
        },
    );
    output.insert(
        "ZIndexBehavior",
        RbxEnum {
            name: "ZIndexBehavior",
            items: {
                #[allow(unused_mut)]
                let mut items = HashMap::new();
                items.insert("Global", 0);
                items.insert("Sibling", 1);
                items
            },
        },
    );
    output
}
