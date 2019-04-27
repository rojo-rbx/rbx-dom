// This file is automatically @generated.
use crate::reflection_types::*;
use std::{borrow::Cow, collections::HashMap};
pub fn generate_enums() -> HashMap<Cow<'static, str>, RbxEnumDescriptor> {
    let mut output = HashMap::with_capacity(191);
    output.insert(
        Cow::Borrowed("ActionType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ActionType"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Nothing"), 0);
                items.insert(Cow::Borrowed("Pause"), 1);
                items.insert(Cow::Borrowed("Lose"), 2);
                items.insert(Cow::Borrowed("Draw"), 3);
                items.insert(Cow::Borrowed("Win"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ActuatorRelativeTo"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ActuatorRelativeTo"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Attachment0"), 0);
                items.insert(Cow::Borrowed("Attachment1"), 1);
                items.insert(Cow::Borrowed("World"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ActuatorType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ActuatorType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("None"), 0);
                items.insert(Cow::Borrowed("Motor"), 1);
                items.insert(Cow::Borrowed("Servo"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("AlignType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("AlignType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Parallel"), 0);
                items.insert(Cow::Borrowed("Perpendicular"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("AnimationPriority"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("AnimationPriority"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Idle"), 0);
                items.insert(Cow::Borrowed("Movement"), 1);
                items.insert(Cow::Borrowed("Action"), 2);
                items.insert(Cow::Borrowed("Core"), 1000);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("AppShellActionType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("AppShellActionType"),
            items: {
                let mut items = HashMap::with_capacity(11);
                items.insert(Cow::Borrowed("None"), 0);
                items.insert(Cow::Borrowed("OpenApp"), 1);
                items.insert(Cow::Borrowed("TapChatTab"), 2);
                items.insert(Cow::Borrowed("TapConversationEntry"), 3);
                items.insert(Cow::Borrowed("TapAvatarTab"), 4);
                items.insert(Cow::Borrowed("ReadConversation"), 5);
                items.insert(Cow::Borrowed("TapGamePageTab"), 6);
                items.insert(Cow::Borrowed("TapHomePageTab"), 7);
                items.insert(Cow::Borrowed("GamePageLoaded"), 8);
                items.insert(Cow::Borrowed("HomePageLoaded"), 9);
                items.insert(Cow::Borrowed("AvatarEditorPageLoaded"), 10);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("AspectType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("AspectType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("FitWithinMaxSize"), 0);
                items.insert(Cow::Borrowed("ScaleWithParentSize"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("AssetType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("AssetType"),
            items: {
                let mut items = HashMap::with_capacity(43);
                items.insert(Cow::Borrowed("Image"), 1);
                items.insert(Cow::Borrowed("TeeShirt"), 2);
                items.insert(Cow::Borrowed("Audio"), 3);
                items.insert(Cow::Borrowed("Mesh"), 4);
                items.insert(Cow::Borrowed("Lua"), 5);
                items.insert(Cow::Borrowed("Hat"), 8);
                items.insert(Cow::Borrowed("Place"), 9);
                items.insert(Cow::Borrowed("Model"), 10);
                items.insert(Cow::Borrowed("Shirt"), 11);
                items.insert(Cow::Borrowed("Pants"), 12);
                items.insert(Cow::Borrowed("Decal"), 13);
                items.insert(Cow::Borrowed("Head"), 17);
                items.insert(Cow::Borrowed("Face"), 18);
                items.insert(Cow::Borrowed("Gear"), 19);
                items.insert(Cow::Borrowed("Badge"), 21);
                items.insert(Cow::Borrowed("Animation"), 24);
                items.insert(Cow::Borrowed("Torso"), 27);
                items.insert(Cow::Borrowed("RightArm"), 28);
                items.insert(Cow::Borrowed("LeftArm"), 29);
                items.insert(Cow::Borrowed("LeftLeg"), 30);
                items.insert(Cow::Borrowed("RightLeg"), 31);
                items.insert(Cow::Borrowed("Package"), 32);
                items.insert(Cow::Borrowed("GamePass"), 34);
                items.insert(Cow::Borrowed("Plugin"), 38);
                items.insert(Cow::Borrowed("MeshPart"), 40);
                items.insert(Cow::Borrowed("HairAccessory"), 41);
                items.insert(Cow::Borrowed("FaceAccessory"), 42);
                items.insert(Cow::Borrowed("NeckAccessory"), 43);
                items.insert(Cow::Borrowed("ShoulderAccessory"), 44);
                items.insert(Cow::Borrowed("FrontAccessory"), 45);
                items.insert(Cow::Borrowed("BackAccessory"), 46);
                items.insert(Cow::Borrowed("WaistAccessory"), 47);
                items.insert(Cow::Borrowed("ClimbAnimation"), 48);
                items.insert(Cow::Borrowed("DeathAnimation"), 49);
                items.insert(Cow::Borrowed("FallAnimation"), 50);
                items.insert(Cow::Borrowed("IdleAnimation"), 51);
                items.insert(Cow::Borrowed("JumpAnimation"), 52);
                items.insert(Cow::Borrowed("RunAnimation"), 53);
                items.insert(Cow::Borrowed("SwimAnimation"), 54);
                items.insert(Cow::Borrowed("WalkAnimation"), 55);
                items.insert(Cow::Borrowed("PoseAnimation"), 56);
                items.insert(Cow::Borrowed("EarAccessory"), 57);
                items.insert(Cow::Borrowed("EyeAccessory"), 58);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("AutoJointsMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("AutoJointsMode"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("Explicit"), 1);
                items.insert(Cow::Borrowed("LegacyImplicit"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("AvatarContextMenuOption"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("AvatarContextMenuOption"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Friend"), 0);
                items.insert(Cow::Borrowed("Chat"), 1);
                items.insert(Cow::Borrowed("Emote"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("AvatarJointPositionType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("AvatarJointPositionType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Fixed"), 0);
                items.insert(Cow::Borrowed("ArtistIntent"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Axis"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Axis"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("X"), 0);
                items.insert(Cow::Borrowed("Y"), 1);
                items.insert(Cow::Borrowed("Z"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("BinType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("BinType"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Script"), 0);
                items.insert(Cow::Borrowed("GameTool"), 1);
                items.insert(Cow::Borrowed("Grab"), 2);
                items.insert(Cow::Borrowed("Clone"), 3);
                items.insert(Cow::Borrowed("Hammer"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("BodyPart"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("BodyPart"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("Head"), 0);
                items.insert(Cow::Borrowed("Torso"), 1);
                items.insert(Cow::Borrowed("LeftArm"), 2);
                items.insert(Cow::Borrowed("RightArm"), 3);
                items.insert(Cow::Borrowed("LeftLeg"), 4);
                items.insert(Cow::Borrowed("RightLeg"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("BodyPartR15"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("BodyPartR15"),
            items: {
                let mut items = HashMap::with_capacity(17);
                items.insert(Cow::Borrowed("Head"), 0);
                items.insert(Cow::Borrowed("UpperTorso"), 1);
                items.insert(Cow::Borrowed("LowerTorso"), 2);
                items.insert(Cow::Borrowed("LeftFoot"), 3);
                items.insert(Cow::Borrowed("LeftLowerLeg"), 4);
                items.insert(Cow::Borrowed("LeftUpperLeg"), 5);
                items.insert(Cow::Borrowed("RightFoot"), 6);
                items.insert(Cow::Borrowed("RightLowerLeg"), 7);
                items.insert(Cow::Borrowed("RightUpperLeg"), 8);
                items.insert(Cow::Borrowed("LeftHand"), 9);
                items.insert(Cow::Borrowed("LeftLowerArm"), 10);
                items.insert(Cow::Borrowed("LeftUpperArm"), 11);
                items.insert(Cow::Borrowed("RightHand"), 12);
                items.insert(Cow::Borrowed("RightLowerArm"), 13);
                items.insert(Cow::Borrowed("RightUpperArm"), 14);
                items.insert(Cow::Borrowed("RootPart"), 15);
                items.insert(Cow::Borrowed("Unknown"), 17);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Button"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Button"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Jump"), 32);
                items.insert(Cow::Borrowed("Dismount"), 8);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ButtonStyle"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ButtonStyle"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("Custom"), 0);
                items.insert(Cow::Borrowed("RobloxButtonDefault"), 1);
                items.insert(Cow::Borrowed("RobloxButton"), 2);
                items.insert(Cow::Borrowed("RobloxRoundButton"), 3);
                items.insert(Cow::Borrowed("RobloxRoundDefaultButton"), 4);
                items.insert(Cow::Borrowed("RobloxRoundDropdownButton"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CameraMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CameraMode"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Classic"), 0);
                items.insert(Cow::Borrowed("LockFirstPerson"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CameraPanMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CameraPanMode"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Classic"), 0);
                items.insert(Cow::Borrowed("EdgeBump"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CameraType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CameraType"),
            items: {
                let mut items = HashMap::with_capacity(8);
                items.insert(Cow::Borrowed("Fixed"), 0);
                items.insert(Cow::Borrowed("Watch"), 2);
                items.insert(Cow::Borrowed("Attach"), 1);
                items.insert(Cow::Borrowed("Track"), 3);
                items.insert(Cow::Borrowed("Follow"), 4);
                items.insert(Cow::Borrowed("Custom"), 5);
                items.insert(Cow::Borrowed("Scriptable"), 6);
                items.insert(Cow::Borrowed("Orbital"), 7);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CellBlock"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CellBlock"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Solid"), 0);
                items.insert(Cow::Borrowed("VerticalWedge"), 1);
                items.insert(Cow::Borrowed("CornerWedge"), 2);
                items.insert(Cow::Borrowed("InverseCornerWedge"), 3);
                items.insert(Cow::Borrowed("HorizontalWedge"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CellMaterial"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CellMaterial"),
            items: {
                let mut items = HashMap::with_capacity(18);
                items.insert(Cow::Borrowed("Empty"), 0);
                items.insert(Cow::Borrowed("Grass"), 1);
                items.insert(Cow::Borrowed("Sand"), 2);
                items.insert(Cow::Borrowed("Brick"), 3);
                items.insert(Cow::Borrowed("Granite"), 4);
                items.insert(Cow::Borrowed("Asphalt"), 5);
                items.insert(Cow::Borrowed("Iron"), 6);
                items.insert(Cow::Borrowed("Aluminum"), 7);
                items.insert(Cow::Borrowed("Gold"), 8);
                items.insert(Cow::Borrowed("WoodPlank"), 9);
                items.insert(Cow::Borrowed("WoodLog"), 10);
                items.insert(Cow::Borrowed("Gravel"), 11);
                items.insert(Cow::Borrowed("CinderBlock"), 12);
                items.insert(Cow::Borrowed("MossyStone"), 13);
                items.insert(Cow::Borrowed("Cement"), 14);
                items.insert(Cow::Borrowed("RedPlastic"), 15);
                items.insert(Cow::Borrowed("BluePlastic"), 16);
                items.insert(Cow::Borrowed("Water"), 17);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CellOrientation"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CellOrientation"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("NegZ"), 0);
                items.insert(Cow::Borrowed("X"), 1);
                items.insert(Cow::Borrowed("Z"), 2);
                items.insert(Cow::Borrowed("NegX"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CenterDialogType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CenterDialogType"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("UnsolicitedDialog"), 1);
                items.insert(Cow::Borrowed("PlayerInitiatedDialog"), 2);
                items.insert(Cow::Borrowed("ModalDialog"), 3);
                items.insert(Cow::Borrowed("QuitDialog"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ChatCallbackType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ChatCallbackType"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("OnCreatingChatWindow"), 1);
                items.insert(Cow::Borrowed("OnClientSendingMessage"), 2);
                items.insert(Cow::Borrowed("OnClientFormattingMessage"), 3);
                items.insert(Cow::Borrowed("OnServerReceivingMessage"), 17);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ChatColor"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ChatColor"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Blue"), 0);
                items.insert(Cow::Borrowed("Green"), 1);
                items.insert(Cow::Borrowed("Red"), 2);
                items.insert(Cow::Borrowed("White"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ChatMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ChatMode"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Menu"), 0);
                items.insert(Cow::Borrowed("TextAndMenu"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ChatPrivacyMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ChatPrivacyMode"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("AllUsers"), 0);
                items.insert(Cow::Borrowed("NoOne"), 1);
                items.insert(Cow::Borrowed("Friends"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ChatStyle"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ChatStyle"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Classic"), 0);
                items.insert(Cow::Borrowed("Bubble"), 1);
                items.insert(Cow::Borrowed("ClassicAndBubble"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CollisionFidelity"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CollisionFidelity"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("Hull"), 1);
                items.insert(Cow::Borrowed("Box"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ComputerCameraMovementMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ComputerCameraMovementMode"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("Follow"), 2);
                items.insert(Cow::Borrowed("Classic"), 1);
                items.insert(Cow::Borrowed("Orbital"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ComputerMovementMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ComputerMovementMode"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("KeyboardMouse"), 1);
                items.insert(Cow::Borrowed("ClickToMove"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ConnectionError"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ConnectionError"),
            items: {
                let mut items = HashMap::with_capacity(49);
                items.insert(Cow::Borrowed("OK"), 0);
                items.insert(Cow::Borrowed("DisconnectErrors"), 256);
                items.insert(Cow::Borrowed("DisconnectBadhash"), 257);
                items.insert(Cow::Borrowed("DisconnectSecurityKeyMismatch"), 258);
                items.insert(Cow::Borrowed("DisconnectNewSecurityKeyMismatch"), 272);
                items.insert(Cow::Borrowed("DisconnectProtocolMismatch"), 259);
                items.insert(Cow::Borrowed("DisconnectReceivePacketError"), 260);
                items.insert(Cow::Borrowed("DisconnectReceivePacketStreamError"), 261);
                items.insert(Cow::Borrowed("DisconnectSendPacketError"), 262);
                items.insert(Cow::Borrowed("DisconnectIllegalTeleport"), 263);
                items.insert(Cow::Borrowed("DisconnectDuplicatePlayer"), 264);
                items.insert(Cow::Borrowed("DisconnectDuplicateTicket"), 265);
                items.insert(Cow::Borrowed("DisconnectTimeout"), 266);
                items.insert(Cow::Borrowed("DisconnectLuaKick"), 267);
                items.insert(Cow::Borrowed("DisconnectOnRemoteSysStats"), 268);
                items.insert(Cow::Borrowed("DisconnectHashTimeout"), 269);
                items.insert(Cow::Borrowed("DisconnectCloudEditKick"), 270);
                items.insert(Cow::Borrowed("DisconnectPlayerless"), 271);
                items.insert(Cow::Borrowed("DisconnectEvicted"), 273);
                items.insert(Cow::Borrowed("DisconnectDevMaintenance"), 274);
                items.insert(Cow::Borrowed("DisconnectRobloxMaintenance"), 275);
                items.insert(Cow::Borrowed("DisconnectRejoin"), 276);
                items.insert(Cow::Borrowed("DisconnectConnectionLost"), 277);
                items.insert(Cow::Borrowed("DisconnectIdle"), 278);
                items.insert(Cow::Borrowed("DisconnectRaknetErrors"), 279);
                items.insert(Cow::Borrowed("DisconnectWrongVersion"), 280);
                items.insert(Cow::Borrowed("PlacelaunchErrors"), 512);
                items.insert(Cow::Borrowed("PlacelaunchDisabled"), 515);
                items.insert(Cow::Borrowed("PlacelaunchError"), 516);
                items.insert(Cow::Borrowed("PlacelaunchGameEnded"), 517);
                items.insert(Cow::Borrowed("PlacelaunchGameFull"), 518);
                items.insert(Cow::Borrowed("PlacelaunchUserLeft"), 522);
                items.insert(Cow::Borrowed("PlacelaunchRestricted"), 523);
                items.insert(Cow::Borrowed("PlacelaunchUnauthorized"), 524);
                items.insert(Cow::Borrowed("PlacelaunchFlooded"), 525);
                items.insert(Cow::Borrowed("PlacelaunchHashExpired"), 526);
                items.insert(Cow::Borrowed("PlacelaunchHashException"), 527);
                items.insert(Cow::Borrowed("PlacelaunchPartyCannotFit"), 528);
                items.insert(Cow::Borrowed("PlacelaunchHttpError"), 529);
                items.insert(Cow::Borrowed("PlacelaunchCustomMessage"), 610);
                items.insert(Cow::Borrowed("PlacelaunchOtherError"), 611);
                items.insert(Cow::Borrowed("TeleportErrors"), 768);
                items.insert(Cow::Borrowed("TeleportFailure"), 769);
                items.insert(Cow::Borrowed("TeleportGameNotFound"), 770);
                items.insert(Cow::Borrowed("TeleportGameEnded"), 771);
                items.insert(Cow::Borrowed("TeleportGameFull"), 772);
                items.insert(Cow::Borrowed("TeleportUnauthorized"), 773);
                items.insert(Cow::Borrowed("TeleportFlooded"), 774);
                items.insert(Cow::Borrowed("TeleportIsTeleporting"), 775);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ConnectionState"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ConnectionState"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Connected"), 0);
                items.insert(Cow::Borrowed("Disconnected"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ContextActionPriority"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ContextActionPriority"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Low"), 1000);
                items.insert(Cow::Borrowed("Medium"), 2000);
                items.insert(Cow::Borrowed("Default"), 2000);
                items.insert(Cow::Borrowed("High"), 3000);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ContextActionResult"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ContextActionResult"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Pass"), 1);
                items.insert(Cow::Borrowed("Sink"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ControlMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ControlMode"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("MouseLockSwitch"), 1);
                items.insert(Cow::Borrowed("Classic"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CoreGuiType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CoreGuiType"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("PlayerList"), 0);
                items.insert(Cow::Borrowed("Health"), 1);
                items.insert(Cow::Borrowed("Backpack"), 2);
                items.insert(Cow::Borrowed("Chat"), 3);
                items.insert(Cow::Borrowed("All"), 4);
                items.insert(Cow::Borrowed("EmotesMenu"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CreatorType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CreatorType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("User"), 0);
                items.insert(Cow::Borrowed("Group"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CurrencyType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CurrencyType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("Robux"), 1);
                items.insert(Cow::Borrowed("Tix"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("CustomCameraMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("CustomCameraMode"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("Follow"), 2);
                items.insert(Cow::Borrowed("Classic"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DEPRECATED_DebuggerDataModelPreference"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DEPRECATED_DebuggerDataModelPreference"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Server"), 0);
                items.insert(Cow::Borrowed("Client"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DataStoreRequestType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DataStoreRequestType"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("GetAsync"), 0);
                items.insert(Cow::Borrowed("SetIncrementAsync"), 1);
                items.insert(Cow::Borrowed("UpdateAsync"), 2);
                items.insert(Cow::Borrowed("GetSortedAsync"), 3);
                items.insert(Cow::Borrowed("SetIncrementSortedAsync"), 4);
                items.insert(Cow::Borrowed("OnUpdate"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DevCameraOcclusionMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DevCameraOcclusionMode"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Zoom"), 0);
                items.insert(Cow::Borrowed("Invisicam"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DevComputerCameraMovementMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DevComputerCameraMovementMode"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("UserChoice"), 0);
                items.insert(Cow::Borrowed("Classic"), 1);
                items.insert(Cow::Borrowed("Follow"), 2);
                items.insert(Cow::Borrowed("Orbital"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DevComputerMovementMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DevComputerMovementMode"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("UserChoice"), 0);
                items.insert(Cow::Borrowed("KeyboardMouse"), 1);
                items.insert(Cow::Borrowed("ClickToMove"), 2);
                items.insert(Cow::Borrowed("Scriptable"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DevTouchCameraMovementMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DevTouchCameraMovementMode"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("UserChoice"), 0);
                items.insert(Cow::Borrowed("Classic"), 1);
                items.insert(Cow::Borrowed("Follow"), 2);
                items.insert(Cow::Borrowed("Orbital"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DevTouchMovementMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DevTouchMovementMode"),
            items: {
                let mut items = HashMap::with_capacity(7);
                items.insert(Cow::Borrowed("UserChoice"), 0);
                items.insert(Cow::Borrowed("Thumbstick"), 1);
                items.insert(Cow::Borrowed("DPad"), 2);
                items.insert(Cow::Borrowed("Thumbpad"), 3);
                items.insert(Cow::Borrowed("ClickToMove"), 4);
                items.insert(Cow::Borrowed("Scriptable"), 5);
                items.insert(Cow::Borrowed("DynamicThumbstick"), 6);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DeveloperMemoryTag"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DeveloperMemoryTag"),
            items: {
                let mut items = HashMap::with_capacity(22);
                items.insert(Cow::Borrowed("Internal"), 0);
                items.insert(Cow::Borrowed("HttpCache"), 1);
                items.insert(Cow::Borrowed("Instances"), 2);
                items.insert(Cow::Borrowed("Signals"), 3);
                items.insert(Cow::Borrowed("LuaHeap"), 4);
                items.insert(Cow::Borrowed("Script"), 5);
                items.insert(Cow::Borrowed("PhysicsCollision"), 6);
                items.insert(Cow::Borrowed("PhysicsParts"), 7);
                items.insert(Cow::Borrowed("GraphicsSolidModels"), 8);
                items.insert(Cow::Borrowed("GraphicsMeshParts"), 9);
                items.insert(Cow::Borrowed("GraphicsParticles"), 10);
                items.insert(Cow::Borrowed("GraphicsParts"), 11);
                items.insert(Cow::Borrowed("GraphicsSpatialHash"), 12);
                items.insert(Cow::Borrowed("GraphicsTerrain"), 13);
                items.insert(Cow::Borrowed("GraphicsTexture"), 14);
                items.insert(Cow::Borrowed("GraphicsTextureCharacter"), 15);
                items.insert(Cow::Borrowed("Sounds"), 16);
                items.insert(Cow::Borrowed("StreamingSounds"), 17);
                items.insert(Cow::Borrowed("TerrainVoxels"), 18);
                items.insert(Cow::Borrowed("Gui"), 20);
                items.insert(Cow::Borrowed("Animation"), 21);
                items.insert(Cow::Borrowed("Navigation"), 22);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DeviceType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DeviceType"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Unknown"), 0);
                items.insert(Cow::Borrowed("Desktop"), 1);
                items.insert(Cow::Borrowed("Tablet"), 2);
                items.insert(Cow::Borrowed("Phone"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DialogBehaviorType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DialogBehaviorType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("SinglePlayer"), 0);
                items.insert(Cow::Borrowed("MultiplePlayers"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DialogPurpose"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DialogPurpose"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Quest"), 0);
                items.insert(Cow::Borrowed("Help"), 1);
                items.insert(Cow::Borrowed("Shop"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DialogTone"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DialogTone"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Neutral"), 0);
                items.insert(Cow::Borrowed("Friendly"), 1);
                items.insert(Cow::Borrowed("Enemy"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("DominantAxis"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("DominantAxis"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Width"), 0);
                items.insert(Cow::Borrowed("Height"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("EasingDirection"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("EasingDirection"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("In"), 0);
                items.insert(Cow::Borrowed("Out"), 1);
                items.insert(Cow::Borrowed("InOut"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("EasingStyle"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("EasingStyle"),
            items: {
                let mut items = HashMap::with_capacity(8);
                items.insert(Cow::Borrowed("Linear"), 0);
                items.insert(Cow::Borrowed("Sine"), 1);
                items.insert(Cow::Borrowed("Back"), 2);
                items.insert(Cow::Borrowed("Quad"), 3);
                items.insert(Cow::Borrowed("Quart"), 4);
                items.insert(Cow::Borrowed("Quint"), 5);
                items.insert(Cow::Borrowed("Bounce"), 6);
                items.insert(Cow::Borrowed("Elastic"), 7);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ElasticBehavior"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ElasticBehavior"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("WhenScrollable"), 0);
                items.insert(Cow::Borrowed("Always"), 1);
                items.insert(Cow::Borrowed("Never"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("EnviromentalPhysicsThrottle"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("EnviromentalPhysicsThrottle"),
            items: {
                let mut items = HashMap::with_capacity(7);
                items.insert(Cow::Borrowed("DefaultAuto"), 0);
                items.insert(Cow::Borrowed("Disabled"), 1);
                items.insert(Cow::Borrowed("Always"), 2);
                items.insert(Cow::Borrowed("Skip2"), 3);
                items.insert(Cow::Borrowed("Skip4"), 4);
                items.insert(Cow::Borrowed("Skip8"), 5);
                items.insert(Cow::Borrowed("Skip16"), 6);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ErrorReporting"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ErrorReporting"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("DontReport"), 0);
                items.insert(Cow::Borrowed("Prompt"), 1);
                items.insert(Cow::Borrowed("Report"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ExplosionType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ExplosionType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("NoCraters"), 0);
                items.insert(Cow::Borrowed("Craters"), 1);
                items.insert(Cow::Borrowed("CratersAndDebris"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("FillDirection"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("FillDirection"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Horizontal"), 0);
                items.insert(Cow::Borrowed("Vertical"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("FilterResult"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("FilterResult"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Rejected"), 1);
                items.insert(Cow::Borrowed("Accepted"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Font"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Font"),
            items: {
                let mut items = HashMap::with_capacity(21);
                items.insert(Cow::Borrowed("Legacy"), 0);
                items.insert(Cow::Borrowed("Arial"), 1);
                items.insert(Cow::Borrowed("ArialBold"), 2);
                items.insert(Cow::Borrowed("SourceSans"), 3);
                items.insert(Cow::Borrowed("SourceSansBold"), 4);
                items.insert(Cow::Borrowed("SourceSansSemibold"), 16);
                items.insert(Cow::Borrowed("SourceSansLight"), 5);
                items.insert(Cow::Borrowed("SourceSansItalic"), 6);
                items.insert(Cow::Borrowed("Bodoni"), 7);
                items.insert(Cow::Borrowed("Garamond"), 8);
                items.insert(Cow::Borrowed("Cartoon"), 9);
                items.insert(Cow::Borrowed("Code"), 10);
                items.insert(Cow::Borrowed("Highway"), 11);
                items.insert(Cow::Borrowed("SciFi"), 12);
                items.insert(Cow::Borrowed("Arcade"), 13);
                items.insert(Cow::Borrowed("Fantasy"), 14);
                items.insert(Cow::Borrowed("Antique"), 15);
                items.insert(Cow::Borrowed("Gotham"), 17);
                items.insert(Cow::Borrowed("GothamSemibold"), 18);
                items.insert(Cow::Borrowed("GothamBold"), 19);
                items.insert(Cow::Borrowed("GothamBlack"), 20);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("FontSize"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("FontSize"),
            items: {
                let mut items = HashMap::with_capacity(15);
                items.insert(Cow::Borrowed("Size8"), 0);
                items.insert(Cow::Borrowed("Size9"), 1);
                items.insert(Cow::Borrowed("Size10"), 2);
                items.insert(Cow::Borrowed("Size11"), 3);
                items.insert(Cow::Borrowed("Size12"), 4);
                items.insert(Cow::Borrowed("Size14"), 5);
                items.insert(Cow::Borrowed("Size18"), 6);
                items.insert(Cow::Borrowed("Size24"), 7);
                items.insert(Cow::Borrowed("Size36"), 8);
                items.insert(Cow::Borrowed("Size48"), 9);
                items.insert(Cow::Borrowed("Size28"), 10);
                items.insert(Cow::Borrowed("Size32"), 11);
                items.insert(Cow::Borrowed("Size42"), 12);
                items.insert(Cow::Borrowed("Size60"), 13);
                items.insert(Cow::Borrowed("Size96"), 14);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("FormFactor"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("FormFactor"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Symmetric"), 0);
                items.insert(Cow::Borrowed("Brick"), 1);
                items.insert(Cow::Borrowed("Plate"), 2);
                items.insert(Cow::Borrowed("Custom"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("FrameStyle"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("FrameStyle"),
            items: {
                let mut items = HashMap::with_capacity(7);
                items.insert(Cow::Borrowed("Custom"), 0);
                items.insert(Cow::Borrowed("ChatBlue"), 1);
                items.insert(Cow::Borrowed("RobloxSquare"), 2);
                items.insert(Cow::Borrowed("RobloxRound"), 3);
                items.insert(Cow::Borrowed("ChatGreen"), 4);
                items.insert(Cow::Borrowed("ChatRed"), 5);
                items.insert(Cow::Borrowed("DropShadow"), 6);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("FramerateManagerMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("FramerateManagerMode"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Automatic"), 0);
                items.insert(Cow::Borrowed("On"), 1);
                items.insert(Cow::Borrowed("Off"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("FriendRequestEvent"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("FriendRequestEvent"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Issue"), 0);
                items.insert(Cow::Borrowed("Revoke"), 1);
                items.insert(Cow::Borrowed("Accept"), 2);
                items.insert(Cow::Borrowed("Deny"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("FriendStatus"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("FriendStatus"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Unknown"), 0);
                items.insert(Cow::Borrowed("NotFriend"), 1);
                items.insert(Cow::Borrowed("Friend"), 2);
                items.insert(Cow::Borrowed("FriendRequestSent"), 3);
                items.insert(Cow::Borrowed("FriendRequestReceived"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("FunctionalTestResult"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("FunctionalTestResult"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Passed"), 0);
                items.insert(Cow::Borrowed("Warning"), 1);
                items.insert(Cow::Borrowed("Error"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("GameAvatarType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("GameAvatarType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("R6"), 0);
                items.insert(Cow::Borrowed("R15"), 1);
                items.insert(Cow::Borrowed("PlayerChoice"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("GearGenreSetting"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("GearGenreSetting"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("AllGenres"), 0);
                items.insert(Cow::Borrowed("MatchingGenreOnly"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("GearType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("GearType"),
            items: {
                let mut items = HashMap::with_capacity(9);
                items.insert(Cow::Borrowed("MeleeWeapons"), 0);
                items.insert(Cow::Borrowed("RangedWeapons"), 1);
                items.insert(Cow::Borrowed("Explosives"), 2);
                items.insert(Cow::Borrowed("PowerUps"), 3);
                items.insert(Cow::Borrowed("NavigationEnhancers"), 4);
                items.insert(Cow::Borrowed("MusicalInstruments"), 5);
                items.insert(Cow::Borrowed("SocialItems"), 6);
                items.insert(Cow::Borrowed("BuildingTools"), 7);
                items.insert(Cow::Borrowed("Transport"), 8);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Genre"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Genre"),
            items: {
                let mut items = HashMap::with_capacity(14);
                items.insert(Cow::Borrowed("All"), 0);
                items.insert(Cow::Borrowed("TownAndCity"), 1);
                items.insert(Cow::Borrowed("Fantasy"), 2);
                items.insert(Cow::Borrowed("SciFi"), 3);
                items.insert(Cow::Borrowed("Ninja"), 4);
                items.insert(Cow::Borrowed("Scary"), 5);
                items.insert(Cow::Borrowed("Pirate"), 6);
                items.insert(Cow::Borrowed("Adventure"), 7);
                items.insert(Cow::Borrowed("Sports"), 8);
                items.insert(Cow::Borrowed("Funny"), 9);
                items.insert(Cow::Borrowed("WildWest"), 10);
                items.insert(Cow::Borrowed("War"), 11);
                items.insert(Cow::Borrowed("SkatePark"), 12);
                items.insert(Cow::Borrowed("Tutorial"), 13);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("GraphicsMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("GraphicsMode"),
            items: {
                let mut items = HashMap::with_capacity(7);
                items.insert(Cow::Borrowed("Automatic"), 1);
                items.insert(Cow::Borrowed("Direct3D9"), 3);
                items.insert(Cow::Borrowed("Direct3D11"), 2);
                items.insert(Cow::Borrowed("OpenGL"), 4);
                items.insert(Cow::Borrowed("Metal"), 5);
                items.insert(Cow::Borrowed("Vulkan"), 6);
                items.insert(Cow::Borrowed("NoGraphics"), 7);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HandlesStyle"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HandlesStyle"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Resize"), 0);
                items.insert(Cow::Borrowed("Movement"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HorizontalAlignment"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HorizontalAlignment"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Center"), 0);
                items.insert(Cow::Borrowed("Left"), 1);
                items.insert(Cow::Borrowed("Right"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HoverAnimateSpeed"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HoverAnimateSpeed"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("VerySlow"), 0);
                items.insert(Cow::Borrowed("Slow"), 1);
                items.insert(Cow::Borrowed("Medium"), 2);
                items.insert(Cow::Borrowed("Fast"), 3);
                items.insert(Cow::Borrowed("VeryFast"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HttpCachePolicy"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HttpCachePolicy"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("None"), 0);
                items.insert(Cow::Borrowed("Full"), 1);
                items.insert(Cow::Borrowed("DataOnly"), 2);
                items.insert(Cow::Borrowed("Default"), 3);
                items.insert(Cow::Borrowed("InternalRedirectRefresh"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HttpContentType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HttpContentType"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("ApplicationJson"), 0);
                items.insert(Cow::Borrowed("ApplicationXml"), 1);
                items.insert(Cow::Borrowed("ApplicationUrlEncoded"), 2);
                items.insert(Cow::Borrowed("TextPlain"), 3);
                items.insert(Cow::Borrowed("TextXml"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HttpError"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HttpError"),
            items: {
                let mut items = HashMap::with_capacity(12);
                items.insert(Cow::Borrowed("OK"), 0);
                items.insert(Cow::Borrowed("InvalidUrl"), 1);
                items.insert(Cow::Borrowed("DnsResolve"), 2);
                items.insert(Cow::Borrowed("ConnectFail"), 3);
                items.insert(Cow::Borrowed("OutOfMemory"), 4);
                items.insert(Cow::Borrowed("TimedOut"), 5);
                items.insert(Cow::Borrowed("TooManyRedirects"), 6);
                items.insert(Cow::Borrowed("InvalidRedirect"), 7);
                items.insert(Cow::Borrowed("NetFail"), 8);
                items.insert(Cow::Borrowed("Aborted"), 9);
                items.insert(Cow::Borrowed("SslConnectFail"), 10);
                items.insert(Cow::Borrowed("Unknown"), 11);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HttpRequestType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HttpRequestType"),
            items: {
                let mut items = HashMap::with_capacity(7);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("MarketplaceService"), 2);
                items.insert(Cow::Borrowed("Players"), 7);
                items.insert(Cow::Borrowed("Chat"), 15);
                items.insert(Cow::Borrowed("Avatar"), 16);
                items.insert(Cow::Borrowed("Analytics"), 22);
                items.insert(Cow::Borrowed("Localization"), 24);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HumanoidCollisionType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HumanoidCollisionType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("OuterBox"), 0);
                items.insert(Cow::Borrowed("InnerBox"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HumanoidDisplayDistanceType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HumanoidDisplayDistanceType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Viewer"), 0);
                items.insert(Cow::Borrowed("Subject"), 1);
                items.insert(Cow::Borrowed("None"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HumanoidHealthDisplayType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HumanoidHealthDisplayType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("DisplayWhenDamaged"), 0);
                items.insert(Cow::Borrowed("AlwaysOn"), 1);
                items.insert(Cow::Borrowed("AlwaysOff"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HumanoidRigType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HumanoidRigType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("R6"), 0);
                items.insert(Cow::Borrowed("R15"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("HumanoidStateType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("HumanoidStateType"),
            items: {
                let mut items = HashMap::with_capacity(17);
                items.insert(Cow::Borrowed("FallingDown"), 0);
                items.insert(Cow::Borrowed("Running"), 8);
                items.insert(Cow::Borrowed("RunningNoPhysics"), 10);
                items.insert(Cow::Borrowed("Climbing"), 12);
                items.insert(Cow::Borrowed("StrafingNoPhysics"), 11);
                items.insert(Cow::Borrowed("Ragdoll"), 1);
                items.insert(Cow::Borrowed("GettingUp"), 2);
                items.insert(Cow::Borrowed("Jumping"), 3);
                items.insert(Cow::Borrowed("Landed"), 7);
                items.insert(Cow::Borrowed("Flying"), 6);
                items.insert(Cow::Borrowed("Freefall"), 5);
                items.insert(Cow::Borrowed("Seated"), 13);
                items.insert(Cow::Borrowed("PlatformStanding"), 14);
                items.insert(Cow::Borrowed("Dead"), 15);
                items.insert(Cow::Borrowed("Swimming"), 4);
                items.insert(Cow::Borrowed("Physics"), 16);
                items.insert(Cow::Borrowed("None"), 18);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("InOut"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("InOut"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Edge"), 0);
                items.insert(Cow::Borrowed("Inset"), 1);
                items.insert(Cow::Borrowed("Center"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("InfoType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("InfoType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Asset"), 0);
                items.insert(Cow::Borrowed("Product"), 1);
                items.insert(Cow::Borrowed("GamePass"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("InitialDockState"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("InitialDockState"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Top"), 0);
                items.insert(Cow::Borrowed("Bottom"), 1);
                items.insert(Cow::Borrowed("Left"), 2);
                items.insert(Cow::Borrowed("Right"), 3);
                items.insert(Cow::Borrowed("Float"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("InputType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("InputType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("NoInput"), 0);
                items.insert(Cow::Borrowed("Constant"), 12);
                items.insert(Cow::Borrowed("Sin"), 13);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("JointCreationMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("JointCreationMode"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("All"), 0);
                items.insert(Cow::Borrowed("Surface"), 1);
                items.insert(Cow::Borrowed("None"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("JointType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("JointType"),
            items: {
                let mut items = HashMap::with_capacity(7);
                items.insert(Cow::Borrowed("None"), 28);
                items.insert(Cow::Borrowed("Rotate"), 7);
                items.insert(Cow::Borrowed("RotateP"), 8);
                items.insert(Cow::Borrowed("RotateV"), 9);
                items.insert(Cow::Borrowed("Glue"), 10);
                items.insert(Cow::Borrowed("Weld"), 1);
                items.insert(Cow::Borrowed("Snap"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("KeyCode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("KeyCode"),
            items: {
                let mut items = HashMap::with_capacity(254);
                items.insert(Cow::Borrowed("Unknown"), 0);
                items.insert(Cow::Borrowed("Backspace"), 8);
                items.insert(Cow::Borrowed("Tab"), 9);
                items.insert(Cow::Borrowed("Clear"), 12);
                items.insert(Cow::Borrowed("Return"), 13);
                items.insert(Cow::Borrowed("Pause"), 19);
                items.insert(Cow::Borrowed("Escape"), 27);
                items.insert(Cow::Borrowed("Space"), 32);
                items.insert(Cow::Borrowed("QuotedDouble"), 34);
                items.insert(Cow::Borrowed("Hash"), 35);
                items.insert(Cow::Borrowed("Dollar"), 36);
                items.insert(Cow::Borrowed("Percent"), 37);
                items.insert(Cow::Borrowed("Ampersand"), 38);
                items.insert(Cow::Borrowed("Quote"), 39);
                items.insert(Cow::Borrowed("LeftParenthesis"), 40);
                items.insert(Cow::Borrowed("RightParenthesis"), 41);
                items.insert(Cow::Borrowed("Asterisk"), 42);
                items.insert(Cow::Borrowed("Plus"), 43);
                items.insert(Cow::Borrowed("Comma"), 44);
                items.insert(Cow::Borrowed("Minus"), 45);
                items.insert(Cow::Borrowed("Period"), 46);
                items.insert(Cow::Borrowed("Slash"), 47);
                items.insert(Cow::Borrowed("Zero"), 48);
                items.insert(Cow::Borrowed("One"), 49);
                items.insert(Cow::Borrowed("Two"), 50);
                items.insert(Cow::Borrowed("Three"), 51);
                items.insert(Cow::Borrowed("Four"), 52);
                items.insert(Cow::Borrowed("Five"), 53);
                items.insert(Cow::Borrowed("Six"), 54);
                items.insert(Cow::Borrowed("Seven"), 55);
                items.insert(Cow::Borrowed("Eight"), 56);
                items.insert(Cow::Borrowed("Nine"), 57);
                items.insert(Cow::Borrowed("Colon"), 58);
                items.insert(Cow::Borrowed("Semicolon"), 59);
                items.insert(Cow::Borrowed("LessThan"), 60);
                items.insert(Cow::Borrowed("Equals"), 61);
                items.insert(Cow::Borrowed("GreaterThan"), 62);
                items.insert(Cow::Borrowed("Question"), 63);
                items.insert(Cow::Borrowed("At"), 64);
                items.insert(Cow::Borrowed("LeftBracket"), 91);
                items.insert(Cow::Borrowed("BackSlash"), 92);
                items.insert(Cow::Borrowed("RightBracket"), 93);
                items.insert(Cow::Borrowed("Caret"), 94);
                items.insert(Cow::Borrowed("Underscore"), 95);
                items.insert(Cow::Borrowed("Backquote"), 96);
                items.insert(Cow::Borrowed("A"), 97);
                items.insert(Cow::Borrowed("B"), 98);
                items.insert(Cow::Borrowed("C"), 99);
                items.insert(Cow::Borrowed("D"), 100);
                items.insert(Cow::Borrowed("E"), 101);
                items.insert(Cow::Borrowed("F"), 102);
                items.insert(Cow::Borrowed("G"), 103);
                items.insert(Cow::Borrowed("H"), 104);
                items.insert(Cow::Borrowed("I"), 105);
                items.insert(Cow::Borrowed("J"), 106);
                items.insert(Cow::Borrowed("K"), 107);
                items.insert(Cow::Borrowed("L"), 108);
                items.insert(Cow::Borrowed("M"), 109);
                items.insert(Cow::Borrowed("N"), 110);
                items.insert(Cow::Borrowed("O"), 111);
                items.insert(Cow::Borrowed("P"), 112);
                items.insert(Cow::Borrowed("Q"), 113);
                items.insert(Cow::Borrowed("R"), 114);
                items.insert(Cow::Borrowed("S"), 115);
                items.insert(Cow::Borrowed("T"), 116);
                items.insert(Cow::Borrowed("U"), 117);
                items.insert(Cow::Borrowed("V"), 118);
                items.insert(Cow::Borrowed("W"), 119);
                items.insert(Cow::Borrowed("X"), 120);
                items.insert(Cow::Borrowed("Y"), 121);
                items.insert(Cow::Borrowed("Z"), 122);
                items.insert(Cow::Borrowed("LeftCurly"), 123);
                items.insert(Cow::Borrowed("Pipe"), 124);
                items.insert(Cow::Borrowed("RightCurly"), 125);
                items.insert(Cow::Borrowed("Tilde"), 126);
                items.insert(Cow::Borrowed("Delete"), 127);
                items.insert(Cow::Borrowed("KeypadZero"), 256);
                items.insert(Cow::Borrowed("KeypadOne"), 257);
                items.insert(Cow::Borrowed("KeypadTwo"), 258);
                items.insert(Cow::Borrowed("KeypadThree"), 259);
                items.insert(Cow::Borrowed("KeypadFour"), 260);
                items.insert(Cow::Borrowed("KeypadFive"), 261);
                items.insert(Cow::Borrowed("KeypadSix"), 262);
                items.insert(Cow::Borrowed("KeypadSeven"), 263);
                items.insert(Cow::Borrowed("KeypadEight"), 264);
                items.insert(Cow::Borrowed("KeypadNine"), 265);
                items.insert(Cow::Borrowed("KeypadPeriod"), 266);
                items.insert(Cow::Borrowed("KeypadDivide"), 267);
                items.insert(Cow::Borrowed("KeypadMultiply"), 268);
                items.insert(Cow::Borrowed("KeypadMinus"), 269);
                items.insert(Cow::Borrowed("KeypadPlus"), 270);
                items.insert(Cow::Borrowed("KeypadEnter"), 271);
                items.insert(Cow::Borrowed("KeypadEquals"), 272);
                items.insert(Cow::Borrowed("Up"), 273);
                items.insert(Cow::Borrowed("Down"), 274);
                items.insert(Cow::Borrowed("Right"), 275);
                items.insert(Cow::Borrowed("Left"), 276);
                items.insert(Cow::Borrowed("Insert"), 277);
                items.insert(Cow::Borrowed("Home"), 278);
                items.insert(Cow::Borrowed("End"), 279);
                items.insert(Cow::Borrowed("PageUp"), 280);
                items.insert(Cow::Borrowed("PageDown"), 281);
                items.insert(Cow::Borrowed("LeftShift"), 304);
                items.insert(Cow::Borrowed("RightShift"), 303);
                items.insert(Cow::Borrowed("LeftMeta"), 310);
                items.insert(Cow::Borrowed("RightMeta"), 309);
                items.insert(Cow::Borrowed("LeftAlt"), 308);
                items.insert(Cow::Borrowed("RightAlt"), 307);
                items.insert(Cow::Borrowed("LeftControl"), 306);
                items.insert(Cow::Borrowed("RightControl"), 305);
                items.insert(Cow::Borrowed("CapsLock"), 301);
                items.insert(Cow::Borrowed("NumLock"), 300);
                items.insert(Cow::Borrowed("ScrollLock"), 302);
                items.insert(Cow::Borrowed("LeftSuper"), 311);
                items.insert(Cow::Borrowed("RightSuper"), 312);
                items.insert(Cow::Borrowed("Mode"), 313);
                items.insert(Cow::Borrowed("Compose"), 314);
                items.insert(Cow::Borrowed("Help"), 315);
                items.insert(Cow::Borrowed("Print"), 316);
                items.insert(Cow::Borrowed("SysReq"), 317);
                items.insert(Cow::Borrowed("Break"), 318);
                items.insert(Cow::Borrowed("Menu"), 319);
                items.insert(Cow::Borrowed("Power"), 320);
                items.insert(Cow::Borrowed("Euro"), 321);
                items.insert(Cow::Borrowed("Undo"), 322);
                items.insert(Cow::Borrowed("F1"), 282);
                items.insert(Cow::Borrowed("F2"), 283);
                items.insert(Cow::Borrowed("F3"), 284);
                items.insert(Cow::Borrowed("F4"), 285);
                items.insert(Cow::Borrowed("F5"), 286);
                items.insert(Cow::Borrowed("F6"), 287);
                items.insert(Cow::Borrowed("F7"), 288);
                items.insert(Cow::Borrowed("F8"), 289);
                items.insert(Cow::Borrowed("F9"), 290);
                items.insert(Cow::Borrowed("F10"), 291);
                items.insert(Cow::Borrowed("F11"), 292);
                items.insert(Cow::Borrowed("F12"), 293);
                items.insert(Cow::Borrowed("F13"), 294);
                items.insert(Cow::Borrowed("F14"), 295);
                items.insert(Cow::Borrowed("F15"), 296);
                items.insert(Cow::Borrowed("World0"), 160);
                items.insert(Cow::Borrowed("World1"), 161);
                items.insert(Cow::Borrowed("World2"), 162);
                items.insert(Cow::Borrowed("World3"), 163);
                items.insert(Cow::Borrowed("World4"), 164);
                items.insert(Cow::Borrowed("World5"), 165);
                items.insert(Cow::Borrowed("World6"), 166);
                items.insert(Cow::Borrowed("World7"), 167);
                items.insert(Cow::Borrowed("World8"), 168);
                items.insert(Cow::Borrowed("World9"), 169);
                items.insert(Cow::Borrowed("World10"), 170);
                items.insert(Cow::Borrowed("World11"), 171);
                items.insert(Cow::Borrowed("World12"), 172);
                items.insert(Cow::Borrowed("World13"), 173);
                items.insert(Cow::Borrowed("World14"), 174);
                items.insert(Cow::Borrowed("World15"), 175);
                items.insert(Cow::Borrowed("World16"), 176);
                items.insert(Cow::Borrowed("World17"), 177);
                items.insert(Cow::Borrowed("World18"), 178);
                items.insert(Cow::Borrowed("World19"), 179);
                items.insert(Cow::Borrowed("World20"), 180);
                items.insert(Cow::Borrowed("World21"), 181);
                items.insert(Cow::Borrowed("World22"), 182);
                items.insert(Cow::Borrowed("World23"), 183);
                items.insert(Cow::Borrowed("World24"), 184);
                items.insert(Cow::Borrowed("World25"), 185);
                items.insert(Cow::Borrowed("World26"), 186);
                items.insert(Cow::Borrowed("World27"), 187);
                items.insert(Cow::Borrowed("World28"), 188);
                items.insert(Cow::Borrowed("World29"), 189);
                items.insert(Cow::Borrowed("World30"), 190);
                items.insert(Cow::Borrowed("World31"), 191);
                items.insert(Cow::Borrowed("World32"), 192);
                items.insert(Cow::Borrowed("World33"), 193);
                items.insert(Cow::Borrowed("World34"), 194);
                items.insert(Cow::Borrowed("World35"), 195);
                items.insert(Cow::Borrowed("World36"), 196);
                items.insert(Cow::Borrowed("World37"), 197);
                items.insert(Cow::Borrowed("World38"), 198);
                items.insert(Cow::Borrowed("World39"), 199);
                items.insert(Cow::Borrowed("World40"), 200);
                items.insert(Cow::Borrowed("World41"), 201);
                items.insert(Cow::Borrowed("World42"), 202);
                items.insert(Cow::Borrowed("World43"), 203);
                items.insert(Cow::Borrowed("World44"), 204);
                items.insert(Cow::Borrowed("World45"), 205);
                items.insert(Cow::Borrowed("World46"), 206);
                items.insert(Cow::Borrowed("World47"), 207);
                items.insert(Cow::Borrowed("World48"), 208);
                items.insert(Cow::Borrowed("World49"), 209);
                items.insert(Cow::Borrowed("World50"), 210);
                items.insert(Cow::Borrowed("World51"), 211);
                items.insert(Cow::Borrowed("World52"), 212);
                items.insert(Cow::Borrowed("World53"), 213);
                items.insert(Cow::Borrowed("World54"), 214);
                items.insert(Cow::Borrowed("World55"), 215);
                items.insert(Cow::Borrowed("World56"), 216);
                items.insert(Cow::Borrowed("World57"), 217);
                items.insert(Cow::Borrowed("World58"), 218);
                items.insert(Cow::Borrowed("World59"), 219);
                items.insert(Cow::Borrowed("World60"), 220);
                items.insert(Cow::Borrowed("World61"), 221);
                items.insert(Cow::Borrowed("World62"), 222);
                items.insert(Cow::Borrowed("World63"), 223);
                items.insert(Cow::Borrowed("World64"), 224);
                items.insert(Cow::Borrowed("World65"), 225);
                items.insert(Cow::Borrowed("World66"), 226);
                items.insert(Cow::Borrowed("World67"), 227);
                items.insert(Cow::Borrowed("World68"), 228);
                items.insert(Cow::Borrowed("World69"), 229);
                items.insert(Cow::Borrowed("World70"), 230);
                items.insert(Cow::Borrowed("World71"), 231);
                items.insert(Cow::Borrowed("World72"), 232);
                items.insert(Cow::Borrowed("World73"), 233);
                items.insert(Cow::Borrowed("World74"), 234);
                items.insert(Cow::Borrowed("World75"), 235);
                items.insert(Cow::Borrowed("World76"), 236);
                items.insert(Cow::Borrowed("World77"), 237);
                items.insert(Cow::Borrowed("World78"), 238);
                items.insert(Cow::Borrowed("World79"), 239);
                items.insert(Cow::Borrowed("World80"), 240);
                items.insert(Cow::Borrowed("World81"), 241);
                items.insert(Cow::Borrowed("World82"), 242);
                items.insert(Cow::Borrowed("World83"), 243);
                items.insert(Cow::Borrowed("World84"), 244);
                items.insert(Cow::Borrowed("World85"), 245);
                items.insert(Cow::Borrowed("World86"), 246);
                items.insert(Cow::Borrowed("World87"), 247);
                items.insert(Cow::Borrowed("World88"), 248);
                items.insert(Cow::Borrowed("World89"), 249);
                items.insert(Cow::Borrowed("World90"), 250);
                items.insert(Cow::Borrowed("World91"), 251);
                items.insert(Cow::Borrowed("World92"), 252);
                items.insert(Cow::Borrowed("World93"), 253);
                items.insert(Cow::Borrowed("World94"), 254);
                items.insert(Cow::Borrowed("World95"), 255);
                items.insert(Cow::Borrowed("ButtonX"), 1000);
                items.insert(Cow::Borrowed("ButtonY"), 1001);
                items.insert(Cow::Borrowed("ButtonA"), 1002);
                items.insert(Cow::Borrowed("ButtonB"), 1003);
                items.insert(Cow::Borrowed("ButtonR1"), 1004);
                items.insert(Cow::Borrowed("ButtonL1"), 1005);
                items.insert(Cow::Borrowed("ButtonR2"), 1006);
                items.insert(Cow::Borrowed("ButtonL2"), 1007);
                items.insert(Cow::Borrowed("ButtonR3"), 1008);
                items.insert(Cow::Borrowed("ButtonL3"), 1009);
                items.insert(Cow::Borrowed("ButtonStart"), 1010);
                items.insert(Cow::Borrowed("ButtonSelect"), 1011);
                items.insert(Cow::Borrowed("DPadLeft"), 1012);
                items.insert(Cow::Borrowed("DPadRight"), 1013);
                items.insert(Cow::Borrowed("DPadUp"), 1014);
                items.insert(Cow::Borrowed("DPadDown"), 1015);
                items.insert(Cow::Borrowed("Thumbstick1"), 1016);
                items.insert(Cow::Borrowed("Thumbstick2"), 1017);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("KeywordFilterType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("KeywordFilterType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Include"), 0);
                items.insert(Cow::Borrowed("Exclude"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Language"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Language"),
            items: {
                let mut items = HashMap::with_capacity(1);
                items.insert(Cow::Borrowed("Default"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("LanguagePreference"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("LanguagePreference"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("SystemDefault"), 0);
                items.insert(Cow::Borrowed("English"), 1);
                items.insert(Cow::Borrowed("SimplifiedChinese"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("LeftRight"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("LeftRight"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Left"), 0);
                items.insert(Cow::Borrowed("Center"), 1);
                items.insert(Cow::Borrowed("Right"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("LevelOfDetailSetting"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("LevelOfDetailSetting"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("High"), 2);
                items.insert(Cow::Borrowed("Medium"), 1);
                items.insert(Cow::Borrowed("Low"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Limb"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Limb"),
            items: {
                let mut items = HashMap::with_capacity(7);
                items.insert(Cow::Borrowed("Head"), 0);
                items.insert(Cow::Borrowed("Torso"), 1);
                items.insert(Cow::Borrowed("LeftArm"), 2);
                items.insert(Cow::Borrowed("RightArm"), 3);
                items.insert(Cow::Borrowed("LeftLeg"), 4);
                items.insert(Cow::Borrowed("RightLeg"), 5);
                items.insert(Cow::Borrowed("Unknown"), 6);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ListDisplayMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ListDisplayMode"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Horizontal"), 0);
                items.insert(Cow::Borrowed("Vertical"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ListenerType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ListenerType"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Camera"), 0);
                items.insert(Cow::Borrowed("CFrame"), 1);
                items.insert(Cow::Borrowed("ObjectPosition"), 2);
                items.insert(Cow::Borrowed("ObjectCFrame"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Material"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Material"),
            items: {
                let mut items = HashMap::with_capacity(37);
                items.insert(Cow::Borrowed("Plastic"), 256);
                items.insert(Cow::Borrowed("Wood"), 512);
                items.insert(Cow::Borrowed("Slate"), 800);
                items.insert(Cow::Borrowed("Concrete"), 816);
                items.insert(Cow::Borrowed("CorrodedMetal"), 1040);
                items.insert(Cow::Borrowed("DiamondPlate"), 1056);
                items.insert(Cow::Borrowed("Foil"), 1072);
                items.insert(Cow::Borrowed("Grass"), 1280);
                items.insert(Cow::Borrowed("Ice"), 1536);
                items.insert(Cow::Borrowed("Marble"), 784);
                items.insert(Cow::Borrowed("Granite"), 832);
                items.insert(Cow::Borrowed("Brick"), 848);
                items.insert(Cow::Borrowed("Pebble"), 864);
                items.insert(Cow::Borrowed("Sand"), 1296);
                items.insert(Cow::Borrowed("Fabric"), 1312);
                items.insert(Cow::Borrowed("SmoothPlastic"), 272);
                items.insert(Cow::Borrowed("Metal"), 1088);
                items.insert(Cow::Borrowed("WoodPlanks"), 528);
                items.insert(Cow::Borrowed("Cobblestone"), 880);
                items.insert(Cow::Borrowed("Air"), 1792);
                items.insert(Cow::Borrowed("Water"), 2048);
                items.insert(Cow::Borrowed("Rock"), 896);
                items.insert(Cow::Borrowed("Glacier"), 1552);
                items.insert(Cow::Borrowed("Snow"), 1328);
                items.insert(Cow::Borrowed("Sandstone"), 912);
                items.insert(Cow::Borrowed("Mud"), 1344);
                items.insert(Cow::Borrowed("Basalt"), 788);
                items.insert(Cow::Borrowed("Ground"), 1360);
                items.insert(Cow::Borrowed("CrackedLava"), 804);
                items.insert(Cow::Borrowed("Neon"), 288);
                items.insert(Cow::Borrowed("Glass"), 1568);
                items.insert(Cow::Borrowed("Asphalt"), 1376);
                items.insert(Cow::Borrowed("LeafyGrass"), 1284);
                items.insert(Cow::Borrowed("Salt"), 1392);
                items.insert(Cow::Borrowed("Limestone"), 820);
                items.insert(Cow::Borrowed("Pavement"), 836);
                items.insert(Cow::Borrowed("ForceField"), 1584);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("MembershipType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("MembershipType"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("None"), 0);
                items.insert(Cow::Borrowed("BuildersClub"), 1);
                items.insert(Cow::Borrowed("TurboBuildersClub"), 2);
                items.insert(Cow::Borrowed("OutrageousBuildersClub"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("MeshType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("MeshType"),
            items: {
                let mut items = HashMap::with_capacity(12);
                items.insert(Cow::Borrowed("Head"), 0);
                items.insert(Cow::Borrowed("Torso"), 1);
                items.insert(Cow::Borrowed("Wedge"), 2);
                items.insert(Cow::Borrowed("Prism"), 7);
                items.insert(Cow::Borrowed("Pyramid"), 8);
                items.insert(Cow::Borrowed("ParallelRamp"), 9);
                items.insert(Cow::Borrowed("RightAngleRamp"), 10);
                items.insert(Cow::Borrowed("CornerWedge"), 11);
                items.insert(Cow::Borrowed("Brick"), 6);
                items.insert(Cow::Borrowed("Sphere"), 3);
                items.insert(Cow::Borrowed("Cylinder"), 4);
                items.insert(Cow::Borrowed("FileMesh"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("MessageType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("MessageType"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("MessageOutput"), 0);
                items.insert(Cow::Borrowed("MessageInfo"), 1);
                items.insert(Cow::Borrowed("MessageWarning"), 2);
                items.insert(Cow::Borrowed("MessageError"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("MouseBehavior"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("MouseBehavior"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("LockCenter"), 1);
                items.insert(Cow::Borrowed("LockCurrentPosition"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("MoveState"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("MoveState"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Stopped"), 0);
                items.insert(Cow::Borrowed("Coasting"), 1);
                items.insert(Cow::Borrowed("Pushing"), 2);
                items.insert(Cow::Borrowed("Stopping"), 3);
                items.insert(Cow::Borrowed("AirFree"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("NameOcclusion"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("NameOcclusion"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("OccludeAll"), 2);
                items.insert(Cow::Borrowed("EnemyOcclusion"), 1);
                items.insert(Cow::Borrowed("NoOcclusion"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("NetworkOwnership"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("NetworkOwnership"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Automatic"), 0);
                items.insert(Cow::Borrowed("Manual"), 1);
                items.insert(Cow::Borrowed("OnContact"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("NormalId"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("NormalId"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("Top"), 1);
                items.insert(Cow::Borrowed("Bottom"), 4);
                items.insert(Cow::Borrowed("Back"), 2);
                items.insert(Cow::Borrowed("Front"), 5);
                items.insert(Cow::Borrowed("Right"), 0);
                items.insert(Cow::Borrowed("Left"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("OutputLayoutMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("OutputLayoutMode"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Horizontal"), 0);
                items.insert(Cow::Borrowed("Vertical"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("OverrideMouseIconBehavior"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("OverrideMouseIconBehavior"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("None"), 0);
                items.insert(Cow::Borrowed("ForceShow"), 1);
                items.insert(Cow::Borrowed("ForceHide"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PacketPriority"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PacketPriority"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("IMMEDIATE_PRIORITY"), 0);
                items.insert(Cow::Borrowed("HIGH_PRIORITY"), 1);
                items.insert(Cow::Borrowed("MEDIUM_PRIORITY"), 2);
                items.insert(Cow::Borrowed("LOW_PRIORITY"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PartType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PartType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Ball"), 0);
                items.insert(Cow::Borrowed("Block"), 1);
                items.insert(Cow::Borrowed("Cylinder"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PathStatus"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PathStatus"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("Success"), 0);
                items.insert(Cow::Borrowed("ClosestNoPath"), 1);
                items.insert(Cow::Borrowed("ClosestOutOfRange"), 2);
                items.insert(Cow::Borrowed("FailStartNotEmpty"), 3);
                items.insert(Cow::Borrowed("FailFinishNotEmpty"), 4);
                items.insert(Cow::Borrowed("NoPath"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PathWaypointAction"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PathWaypointAction"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Walk"), 0);
                items.insert(Cow::Borrowed("Jump"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PermissionLevelShown"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PermissionLevelShown"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Game"), 0);
                items.insert(Cow::Borrowed("RobloxGame"), 1);
                items.insert(Cow::Borrowed("RobloxScript"), 2);
                items.insert(Cow::Borrowed("Studio"), 3);
                items.insert(Cow::Borrowed("Roblox"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Platform"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Platform"),
            items: {
                let mut items = HashMap::with_capacity(20);
                items.insert(Cow::Borrowed("Windows"), 0);
                items.insert(Cow::Borrowed("OSX"), 1);
                items.insert(Cow::Borrowed("IOS"), 2);
                items.insert(Cow::Borrowed("Android"), 3);
                items.insert(Cow::Borrowed("XBoxOne"), 4);
                items.insert(Cow::Borrowed("PS4"), 5);
                items.insert(Cow::Borrowed("PS3"), 6);
                items.insert(Cow::Borrowed("XBox360"), 7);
                items.insert(Cow::Borrowed("WiiU"), 8);
                items.insert(Cow::Borrowed("NX"), 9);
                items.insert(Cow::Borrowed("Ouya"), 10);
                items.insert(Cow::Borrowed("AndroidTV"), 11);
                items.insert(Cow::Borrowed("Chromecast"), 12);
                items.insert(Cow::Borrowed("Linux"), 13);
                items.insert(Cow::Borrowed("SteamOS"), 14);
                items.insert(Cow::Borrowed("WebOS"), 15);
                items.insert(Cow::Borrowed("DOS"), 16);
                items.insert(Cow::Borrowed("BeOS"), 17);
                items.insert(Cow::Borrowed("UWP"), 18);
                items.insert(Cow::Borrowed("None"), 19);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PlaybackState"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PlaybackState"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("Begin"), 0);
                items.insert(Cow::Borrowed("Delayed"), 1);
                items.insert(Cow::Borrowed("Playing"), 2);
                items.insert(Cow::Borrowed("Paused"), 3);
                items.insert(Cow::Borrowed("Completed"), 4);
                items.insert(Cow::Borrowed("Cancelled"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PlayerActions"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PlayerActions"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("CharacterForward"), 0);
                items.insert(Cow::Borrowed("CharacterBackward"), 1);
                items.insert(Cow::Borrowed("CharacterLeft"), 2);
                items.insert(Cow::Borrowed("CharacterRight"), 3);
                items.insert(Cow::Borrowed("CharacterJump"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PlayerChatType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PlayerChatType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("All"), 0);
                items.insert(Cow::Borrowed("Team"), 1);
                items.insert(Cow::Borrowed("Whisper"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PoseEasingDirection"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PoseEasingDirection"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Out"), 1);
                items.insert(Cow::Borrowed("InOut"), 2);
                items.insert(Cow::Borrowed("In"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PoseEasingStyle"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PoseEasingStyle"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Linear"), 0);
                items.insert(Cow::Borrowed("Constant"), 1);
                items.insert(Cow::Borrowed("Elastic"), 2);
                items.insert(Cow::Borrowed("Cubic"), 3);
                items.insert(Cow::Borrowed("Bounce"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("PrivilegeType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("PrivilegeType"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Owner"), 255);
                items.insert(Cow::Borrowed("Admin"), 240);
                items.insert(Cow::Borrowed("Member"), 128);
                items.insert(Cow::Borrowed("Visitor"), 10);
                items.insert(Cow::Borrowed("Banned"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ProductPurchaseDecision"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ProductPurchaseDecision"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("NotProcessedYet"), 0);
                items.insert(Cow::Borrowed("PurchaseGranted"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("QualityLevel"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("QualityLevel"),
            items: {
                let mut items = HashMap::with_capacity(22);
                items.insert(Cow::Borrowed("Automatic"), 0);
                items.insert(Cow::Borrowed("Level01"), 1);
                items.insert(Cow::Borrowed("Level02"), 2);
                items.insert(Cow::Borrowed("Level03"), 3);
                items.insert(Cow::Borrowed("Level04"), 4);
                items.insert(Cow::Borrowed("Level05"), 5);
                items.insert(Cow::Borrowed("Level06"), 6);
                items.insert(Cow::Borrowed("Level07"), 7);
                items.insert(Cow::Borrowed("Level08"), 8);
                items.insert(Cow::Borrowed("Level09"), 9);
                items.insert(Cow::Borrowed("Level10"), 10);
                items.insert(Cow::Borrowed("Level11"), 11);
                items.insert(Cow::Borrowed("Level12"), 12);
                items.insert(Cow::Borrowed("Level13"), 13);
                items.insert(Cow::Borrowed("Level14"), 14);
                items.insert(Cow::Borrowed("Level15"), 15);
                items.insert(Cow::Borrowed("Level16"), 16);
                items.insert(Cow::Borrowed("Level17"), 17);
                items.insert(Cow::Borrowed("Level18"), 18);
                items.insert(Cow::Borrowed("Level19"), 19);
                items.insert(Cow::Borrowed("Level20"), 20);
                items.insert(Cow::Borrowed("Level21"), 21);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("R15CollisionType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("R15CollisionType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("OuterBox"), 0);
                items.insert(Cow::Borrowed("InnerBox"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("RenderFidelity"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("RenderFidelity"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Automatic"), 0);
                items.insert(Cow::Borrowed("Precise"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("RenderPriority"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("RenderPriority"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("First"), 0);
                items.insert(Cow::Borrowed("Input"), 100);
                items.insert(Cow::Borrowed("Camera"), 200);
                items.insert(Cow::Borrowed("Character"), 300);
                items.insert(Cow::Borrowed("Last"), 2000);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("RenderingTestComparisonMethod"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("RenderingTestComparisonMethod"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("psnr"), 0);
                items.insert(Cow::Borrowed("diff"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ReverbType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ReverbType"),
            items: {
                let mut items = HashMap::with_capacity(24);
                items.insert(Cow::Borrowed("NoReverb"), 0);
                items.insert(Cow::Borrowed("GenericReverb"), 1);
                items.insert(Cow::Borrowed("PaddedCell"), 2);
                items.insert(Cow::Borrowed("Room"), 3);
                items.insert(Cow::Borrowed("Bathroom"), 4);
                items.insert(Cow::Borrowed("LivingRoom"), 5);
                items.insert(Cow::Borrowed("StoneRoom"), 6);
                items.insert(Cow::Borrowed("Auditorium"), 7);
                items.insert(Cow::Borrowed("ConcertHall"), 8);
                items.insert(Cow::Borrowed("Cave"), 9);
                items.insert(Cow::Borrowed("Arena"), 10);
                items.insert(Cow::Borrowed("Hangar"), 11);
                items.insert(Cow::Borrowed("CarpettedHallway"), 12);
                items.insert(Cow::Borrowed("Hallway"), 13);
                items.insert(Cow::Borrowed("StoneCorridor"), 14);
                items.insert(Cow::Borrowed("Alley"), 15);
                items.insert(Cow::Borrowed("Forest"), 16);
                items.insert(Cow::Borrowed("City"), 17);
                items.insert(Cow::Borrowed("Mountains"), 18);
                items.insert(Cow::Borrowed("Quarry"), 19);
                items.insert(Cow::Borrowed("Plain"), 20);
                items.insert(Cow::Borrowed("ParkingLot"), 21);
                items.insert(Cow::Borrowed("SewerPipe"), 22);
                items.insert(Cow::Borrowed("UnderWater"), 23);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("RibbonTool"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("RibbonTool"),
            items: {
                let mut items = HashMap::with_capacity(10);
                items.insert(Cow::Borrowed("Select"), 0);
                items.insert(Cow::Borrowed("Scale"), 1);
                items.insert(Cow::Borrowed("Rotate"), 2);
                items.insert(Cow::Borrowed("Move"), 3);
                items.insert(Cow::Borrowed("Transform"), 4);
                items.insert(Cow::Borrowed("ColorPicker"), 5);
                items.insert(Cow::Borrowed("MaterialPicker"), 6);
                items.insert(Cow::Borrowed("Group"), 7);
                items.insert(Cow::Borrowed("Ungroup"), 8);
                items.insert(Cow::Borrowed("None"), 9);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("RollOffMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("RollOffMode"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Inverse"), 0);
                items.insert(Cow::Borrowed("Linear"), 1);
                items.insert(Cow::Borrowed("InverseTapered"), 3);
                items.insert(Cow::Borrowed("LinearSquare"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("RotationType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("RotationType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("MovementRelative"), 0);
                items.insert(Cow::Borrowed("CameraRelative"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("RuntimeUndoBehavior"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("RuntimeUndoBehavior"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Aggregate"), 0);
                items.insert(Cow::Borrowed("Snapshot"), 1);
                items.insert(Cow::Borrowed("Hybrid"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("SaveFilter"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("SaveFilter"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("SaveAll"), 2);
                items.insert(Cow::Borrowed("SaveWorld"), 0);
                items.insert(Cow::Borrowed("SaveGame"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("SavedQualitySetting"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("SavedQualitySetting"),
            items: {
                let mut items = HashMap::with_capacity(11);
                items.insert(Cow::Borrowed("Automatic"), 0);
                items.insert(Cow::Borrowed("QualityLevel1"), 1);
                items.insert(Cow::Borrowed("QualityLevel2"), 2);
                items.insert(Cow::Borrowed("QualityLevel3"), 3);
                items.insert(Cow::Borrowed("QualityLevel4"), 4);
                items.insert(Cow::Borrowed("QualityLevel5"), 5);
                items.insert(Cow::Borrowed("QualityLevel6"), 6);
                items.insert(Cow::Borrowed("QualityLevel7"), 7);
                items.insert(Cow::Borrowed("QualityLevel8"), 8);
                items.insert(Cow::Borrowed("QualityLevel9"), 9);
                items.insert(Cow::Borrowed("QualityLevel10"), 10);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ScaleType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ScaleType"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Stretch"), 0);
                items.insert(Cow::Borrowed("Slice"), 1);
                items.insert(Cow::Borrowed("Tile"), 2);
                items.insert(Cow::Borrowed("Fit"), 3);
                items.insert(Cow::Borrowed("Crop"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ScreenOrientation"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ScreenOrientation"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("LandscapeLeft"), 0);
                items.insert(Cow::Borrowed("LandscapeRight"), 1);
                items.insert(Cow::Borrowed("LandscapeSensor"), 2);
                items.insert(Cow::Borrowed("Portrait"), 3);
                items.insert(Cow::Borrowed("Sensor"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ScrollBarInset"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ScrollBarInset"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("None"), 0);
                items.insert(Cow::Borrowed("ScrollBar"), 1);
                items.insert(Cow::Borrowed("Always"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ScrollingDirection"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ScrollingDirection"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("X"), 1);
                items.insert(Cow::Borrowed("Y"), 2);
                items.insert(Cow::Borrowed("XY"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ServerAudioBehavior"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ServerAudioBehavior"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Enabled"), 0);
                items.insert(Cow::Borrowed("Muted"), 1);
                items.insert(Cow::Borrowed("OnlineGame"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("SizeConstraint"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("SizeConstraint"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("RelativeXY"), 0);
                items.insert(Cow::Borrowed("RelativeXX"), 1);
                items.insert(Cow::Borrowed("RelativeYY"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("SortOrder"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("SortOrder"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("LayoutOrder"), 2);
                items.insert(Cow::Borrowed("Name"), 0);
                items.insert(Cow::Borrowed("Custom"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("SoundType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("SoundType"),
            items: {
                let mut items = HashMap::with_capacity(15);
                items.insert(Cow::Borrowed("NoSound"), 0);
                items.insert(Cow::Borrowed("Boing"), 1);
                items.insert(Cow::Borrowed("Bomb"), 2);
                items.insert(Cow::Borrowed("Break"), 3);
                items.insert(Cow::Borrowed("Click"), 4);
                items.insert(Cow::Borrowed("Clock"), 5);
                items.insert(Cow::Borrowed("Slingshot"), 6);
                items.insert(Cow::Borrowed("Page"), 7);
                items.insert(Cow::Borrowed("Ping"), 8);
                items.insert(Cow::Borrowed("Snap"), 9);
                items.insert(Cow::Borrowed("Splat"), 10);
                items.insert(Cow::Borrowed("Step"), 11);
                items.insert(Cow::Borrowed("StepOn"), 12);
                items.insert(Cow::Borrowed("Swoosh"), 13);
                items.insert(Cow::Borrowed("Victory"), 14);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("SpecialKey"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("SpecialKey"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("Insert"), 0);
                items.insert(Cow::Borrowed("Home"), 1);
                items.insert(Cow::Borrowed("End"), 2);
                items.insert(Cow::Borrowed("PageUp"), 3);
                items.insert(Cow::Borrowed("PageDown"), 4);
                items.insert(Cow::Borrowed("ChatHotkey"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("StartCorner"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("StartCorner"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("TopLeft"), 0);
                items.insert(Cow::Borrowed("TopRight"), 1);
                items.insert(Cow::Borrowed("BottomLeft"), 2);
                items.insert(Cow::Borrowed("BottomRight"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Status"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Status"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Poison"), 0);
                items.insert(Cow::Borrowed("Confusion"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("StudioStyleGuideColor"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("StudioStyleGuideColor"),
            items: {
                let mut items = HashMap::with_capacity(89);
                items.insert(Cow::Borrowed("MainBackground"), 0);
                items.insert(Cow::Borrowed("Titlebar"), 1);
                items.insert(Cow::Borrowed("Dropdown"), 2);
                items.insert(Cow::Borrowed("Tooltip"), 3);
                items.insert(Cow::Borrowed("Notification"), 4);
                items.insert(Cow::Borrowed("ScrollBar"), 5);
                items.insert(Cow::Borrowed("ScrollBarBackground"), 6);
                items.insert(Cow::Borrowed("TabBar"), 7);
                items.insert(Cow::Borrowed("Tab"), 8);
                items.insert(Cow::Borrowed("RibbonTab"), 9);
                items.insert(Cow::Borrowed("RibbonTabTopBar"), 10);
                items.insert(Cow::Borrowed("Button"), 11);
                items.insert(Cow::Borrowed("MainButton"), 12);
                items.insert(Cow::Borrowed("RibbonButton"), 13);
                items.insert(Cow::Borrowed("ViewPortBackground"), 14);
                items.insert(Cow::Borrowed("InputFieldBackground"), 15);
                items.insert(Cow::Borrowed("Item"), 16);
                items.insert(Cow::Borrowed("TableItem"), 17);
                items.insert(Cow::Borrowed("CategoryItem"), 18);
                items.insert(Cow::Borrowed("GameSettingsTableItem"), 19);
                items.insert(Cow::Borrowed("GameSettingsTooltip"), 20);
                items.insert(Cow::Borrowed("EmulatorBar"), 21);
                items.insert(Cow::Borrowed("EmulatorDropDown"), 22);
                items.insert(Cow::Borrowed("ColorPickerFrame"), 23);
                items.insert(Cow::Borrowed("CurrentMarker"), 24);
                items.insert(Cow::Borrowed("Border"), 25);
                items.insert(Cow::Borrowed("Shadow"), 26);
                items.insert(Cow::Borrowed("Light"), 27);
                items.insert(Cow::Borrowed("Dark"), 28);
                items.insert(Cow::Borrowed("Mid"), 29);
                items.insert(Cow::Borrowed("MainText"), 30);
                items.insert(Cow::Borrowed("SubText"), 31);
                items.insert(Cow::Borrowed("TitlebarText"), 32);
                items.insert(Cow::Borrowed("BrightText"), 33);
                items.insert(Cow::Borrowed("DimmedText"), 34);
                items.insert(Cow::Borrowed("LinkText"), 35);
                items.insert(Cow::Borrowed("WarningText"), 36);
                items.insert(Cow::Borrowed("ErrorText"), 37);
                items.insert(Cow::Borrowed("InfoText"), 38);
                items.insert(Cow::Borrowed("SensitiveText"), 39);
                items.insert(Cow::Borrowed("ScriptSideWidget"), 40);
                items.insert(Cow::Borrowed("ScriptBackground"), 41);
                items.insert(Cow::Borrowed("ScriptText"), 42);
                items.insert(Cow::Borrowed("ScriptSelectionText"), 43);
                items.insert(Cow::Borrowed("ScriptSelectionBackground"), 44);
                items.insert(Cow::Borrowed("ScriptFindSelectionBackground"), 45);
                items.insert(Cow::Borrowed("ScriptMatchingWordSelectionBackground"), 46);
                items.insert(Cow::Borrowed("ScriptOperator"), 47);
                items.insert(Cow::Borrowed("ScriptNumber"), 48);
                items.insert(Cow::Borrowed("ScriptString"), 49);
                items.insert(Cow::Borrowed("ScriptComment"), 50);
                items.insert(Cow::Borrowed("ScriptPreprocessor"), 51);
                items.insert(Cow::Borrowed("ScriptKeyword"), 52);
                items.insert(Cow::Borrowed("ScriptBuiltInFunction"), 53);
                items.insert(Cow::Borrowed("ScriptWarning"), 54);
                items.insert(Cow::Borrowed("ScriptError"), 55);
                items.insert(Cow::Borrowed("DebuggerCurrentLine"), 56);
                items.insert(Cow::Borrowed("DebuggerErrorLine"), 57);
                items.insert(Cow::Borrowed("DiffFilePathText"), 58);
                items.insert(Cow::Borrowed("DiffTextHunkInfo"), 59);
                items.insert(Cow::Borrowed("DiffTextNoChange"), 60);
                items.insert(Cow::Borrowed("DiffTextAddition"), 61);
                items.insert(Cow::Borrowed("DiffTextDeletion"), 62);
                items.insert(Cow::Borrowed("DiffTextSeparatorBackground"), 63);
                items.insert(Cow::Borrowed("DiffTextNoChangeBackground"), 64);
                items.insert(Cow::Borrowed("DiffTextAdditionBackground"), 65);
                items.insert(Cow::Borrowed("DiffTextDeletionBackground"), 66);
                items.insert(Cow::Borrowed("DiffLineNum"), 67);
                items.insert(Cow::Borrowed("DiffLineNumSeparatorBackground"), 68);
                items.insert(Cow::Borrowed("DiffLineNumNoChangeBackground"), 69);
                items.insert(Cow::Borrowed("DiffLineNumAdditionBackground"), 70);
                items.insert(Cow::Borrowed("DiffLineNumDeletionBackground"), 71);
                items.insert(Cow::Borrowed("DiffFilePathBackground"), 72);
                items.insert(Cow::Borrowed("DiffFilePathBorder"), 73);
                items.insert(Cow::Borrowed("Separator"), 74);
                items.insert(Cow::Borrowed("ButtonBorder"), 75);
                items.insert(Cow::Borrowed("ButtonText"), 76);
                items.insert(Cow::Borrowed("InputFieldBorder"), 77);
                items.insert(Cow::Borrowed("CheckedFieldBackground"), 78);
                items.insert(Cow::Borrowed("CheckedFieldBorder"), 79);
                items.insert(Cow::Borrowed("CheckedFieldIndicator"), 80);
                items.insert(Cow::Borrowed("HeaderSection"), 81);
                items.insert(Cow::Borrowed("Midlight"), 82);
                items.insert(Cow::Borrowed("StatusBar"), 83);
                items.insert(Cow::Borrowed("DialogButton"), 84);
                items.insert(Cow::Borrowed("DialogButtonText"), 85);
                items.insert(Cow::Borrowed("DialogButtonBorder"), 86);
                items.insert(Cow::Borrowed("DialogMainButton"), 87);
                items.insert(Cow::Borrowed("DialogMainButtonText"), 88);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("StudioStyleGuideModifier"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("StudioStyleGuideModifier"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("Selected"), 1);
                items.insert(Cow::Borrowed("Pressed"), 2);
                items.insert(Cow::Borrowed("Disabled"), 3);
                items.insert(Cow::Borrowed("Hover"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Style"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Style"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("AlternatingSupports"), 0);
                items.insert(Cow::Borrowed("BridgeStyleSupports"), 1);
                items.insert(Cow::Borrowed("NoSupports"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("SurfaceConstraint"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("SurfaceConstraint"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("None"), 0);
                items.insert(Cow::Borrowed("Hinge"), 1);
                items.insert(Cow::Borrowed("SteppingMotor"), 2);
                items.insert(Cow::Borrowed("Motor"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("SurfaceType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("SurfaceType"),
            items: {
                let mut items = HashMap::with_capacity(10);
                items.insert(Cow::Borrowed("Smooth"), 0);
                items.insert(Cow::Borrowed("Glue"), 1);
                items.insert(Cow::Borrowed("Weld"), 2);
                items.insert(Cow::Borrowed("Studs"), 3);
                items.insert(Cow::Borrowed("Inlet"), 4);
                items.insert(Cow::Borrowed("Universal"), 5);
                items.insert(Cow::Borrowed("Hinge"), 6);
                items.insert(Cow::Borrowed("Motor"), 7);
                items.insert(Cow::Borrowed("SteppingMotor"), 8);
                items.insert(Cow::Borrowed("SmoothNoOutlines"), 10);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("SwipeDirection"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("SwipeDirection"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Right"), 0);
                items.insert(Cow::Borrowed("Left"), 1);
                items.insert(Cow::Borrowed("Up"), 2);
                items.insert(Cow::Borrowed("Down"), 3);
                items.insert(Cow::Borrowed("None"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TableMajorAxis"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TableMajorAxis"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("RowMajor"), 0);
                items.insert(Cow::Borrowed("ColumnMajor"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("Technology"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("Technology"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Legacy"), 0);
                items.insert(Cow::Borrowed("Voxel"), 1);
                items.insert(Cow::Borrowed("Compatibility"), 2);
                items.insert(Cow::Borrowed("ShadowMap"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TeleportResult"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TeleportResult"),
            items: {
                let mut items = HashMap::with_capacity(8);
                items.insert(Cow::Borrowed("Success"), 0);
                items.insert(Cow::Borrowed("Failure"), 1);
                items.insert(Cow::Borrowed("GameNotFound"), 2);
                items.insert(Cow::Borrowed("GameEnded"), 3);
                items.insert(Cow::Borrowed("GameFull"), 4);
                items.insert(Cow::Borrowed("Unauthorized"), 5);
                items.insert(Cow::Borrowed("Flooded"), 6);
                items.insert(Cow::Borrowed("IsTeleporting"), 7);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TeleportState"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TeleportState"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("RequestedFromServer"), 0);
                items.insert(Cow::Borrowed("Started"), 1);
                items.insert(Cow::Borrowed("WaitingForServer"), 2);
                items.insert(Cow::Borrowed("Failed"), 3);
                items.insert(Cow::Borrowed("InProgress"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TeleportType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TeleportType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("ToPlace"), 0);
                items.insert(Cow::Borrowed("ToInstance"), 1);
                items.insert(Cow::Borrowed("ToReservedServer"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TextFilterContext"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TextFilterContext"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("PublicChat"), 1);
                items.insert(Cow::Borrowed("PrivateChat"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TextTruncate"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TextTruncate"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("None"), 0);
                items.insert(Cow::Borrowed("AtEnd"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TextXAlignment"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TextXAlignment"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Left"), 0);
                items.insert(Cow::Borrowed("Center"), 2);
                items.insert(Cow::Borrowed("Right"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TextYAlignment"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TextYAlignment"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Top"), 0);
                items.insert(Cow::Borrowed("Center"), 1);
                items.insert(Cow::Borrowed("Bottom"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TextureMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TextureMode"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Stretch"), 0);
                items.insert(Cow::Borrowed("Wrap"), 1);
                items.insert(Cow::Borrowed("Static"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TextureQueryType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TextureQueryType"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("NonHumanoid"), 0);
                items.insert(Cow::Borrowed("NonHumanoidOrphaned"), 1);
                items.insert(Cow::Borrowed("Humanoid"), 2);
                items.insert(Cow::Borrowed("HumanoidOrphaned"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ThreadPoolConfig"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ThreadPoolConfig"),
            items: {
                let mut items = HashMap::with_capacity(11);
                items.insert(Cow::Borrowed("Auto"), 0);
                items.insert(Cow::Borrowed("PerCore1"), 101);
                items.insert(Cow::Borrowed("PerCore2"), 102);
                items.insert(Cow::Borrowed("PerCore3"), 103);
                items.insert(Cow::Borrowed("PerCore4"), 104);
                items.insert(Cow::Borrowed("Threads1"), 1);
                items.insert(Cow::Borrowed("Threads2"), 2);
                items.insert(Cow::Borrowed("Threads3"), 3);
                items.insert(Cow::Borrowed("Threads4"), 4);
                items.insert(Cow::Borrowed("Threads8"), 8);
                items.insert(Cow::Borrowed("Threads16"), 16);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ThrottlingPriority"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ThrottlingPriority"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Extreme"), 2);
                items.insert(Cow::Borrowed("ElevatedOnServer"), 1);
                items.insert(Cow::Borrowed("Default"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ThumbnailSize"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ThumbnailSize"),
            items: {
                let mut items = HashMap::with_capacity(7);
                items.insert(Cow::Borrowed("Size48x48"), 0);
                items.insert(Cow::Borrowed("Size180x180"), 1);
                items.insert(Cow::Borrowed("Size420x420"), 2);
                items.insert(Cow::Borrowed("Size60x60"), 3);
                items.insert(Cow::Borrowed("Size100x100"), 4);
                items.insert(Cow::Borrowed("Size150x150"), 5);
                items.insert(Cow::Borrowed("Size352x352"), 6);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ThumbnailType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ThumbnailType"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("HeadShot"), 0);
                items.insert(Cow::Borrowed("AvatarBust"), 1);
                items.insert(Cow::Borrowed("AvatarThumbnail"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TickCountSampleMethod"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TickCountSampleMethod"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Fast"), 0);
                items.insert(Cow::Borrowed("Benchmark"), 1);
                items.insert(Cow::Borrowed("Precise"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TopBottom"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TopBottom"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Top"), 0);
                items.insert(Cow::Borrowed("Center"), 1);
                items.insert(Cow::Borrowed("Bottom"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TouchCameraMovementMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TouchCameraMovementMode"),
            items: {
                let mut items = HashMap::with_capacity(4);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("Follow"), 2);
                items.insert(Cow::Borrowed("Classic"), 1);
                items.insert(Cow::Borrowed("Orbital"), 3);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TouchMovementMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TouchMovementMode"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("Default"), 0);
                items.insert(Cow::Borrowed("Thumbstick"), 1);
                items.insert(Cow::Borrowed("DPad"), 2);
                items.insert(Cow::Borrowed("Thumbpad"), 3);
                items.insert(Cow::Borrowed("ClickToMove"), 4);
                items.insert(Cow::Borrowed("DynamicThumbstick"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("TweenStatus"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("TweenStatus"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Canceled"), 0);
                items.insert(Cow::Borrowed("Completed"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("UITheme"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("UITheme"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Light"), 0);
                items.insert(Cow::Borrowed("Dark"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("UiMessageType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("UiMessageType"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("UiMessageError"), 0);
                items.insert(Cow::Borrowed("UiMessageInfo"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("UploadSetting"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("UploadSetting"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Never"), 0);
                items.insert(Cow::Borrowed("Ask"), 1);
                items.insert(Cow::Borrowed("Always"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("UserCFrame"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("UserCFrame"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Head"), 0);
                items.insert(Cow::Borrowed("LeftHand"), 1);
                items.insert(Cow::Borrowed("RightHand"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("UserInputState"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("UserInputState"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("Begin"), 0);
                items.insert(Cow::Borrowed("Change"), 1);
                items.insert(Cow::Borrowed("End"), 2);
                items.insert(Cow::Borrowed("Cancel"), 3);
                items.insert(Cow::Borrowed("None"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("UserInputType"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("UserInputType"),
            items: {
                let mut items = HashMap::with_capacity(21);
                items.insert(Cow::Borrowed("MouseButton1"), 0);
                items.insert(Cow::Borrowed("MouseButton2"), 1);
                items.insert(Cow::Borrowed("MouseButton3"), 2);
                items.insert(Cow::Borrowed("MouseWheel"), 3);
                items.insert(Cow::Borrowed("MouseMovement"), 4);
                items.insert(Cow::Borrowed("Touch"), 7);
                items.insert(Cow::Borrowed("Keyboard"), 8);
                items.insert(Cow::Borrowed("Focus"), 9);
                items.insert(Cow::Borrowed("Accelerometer"), 10);
                items.insert(Cow::Borrowed("Gyro"), 11);
                items.insert(Cow::Borrowed("Gamepad1"), 12);
                items.insert(Cow::Borrowed("Gamepad2"), 13);
                items.insert(Cow::Borrowed("Gamepad3"), 14);
                items.insert(Cow::Borrowed("Gamepad4"), 15);
                items.insert(Cow::Borrowed("Gamepad5"), 16);
                items.insert(Cow::Borrowed("Gamepad6"), 17);
                items.insert(Cow::Borrowed("Gamepad7"), 18);
                items.insert(Cow::Borrowed("Gamepad8"), 19);
                items.insert(Cow::Borrowed("TextInput"), 20);
                items.insert(Cow::Borrowed("InputMethod"), 21);
                items.insert(Cow::Borrowed("None"), 22);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("VRTouchpad"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("VRTouchpad"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Left"), 0);
                items.insert(Cow::Borrowed("Right"), 1);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("VRTouchpadMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("VRTouchpadMode"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Touch"), 0);
                items.insert(Cow::Borrowed("VirtualThumbstick"), 1);
                items.insert(Cow::Borrowed("ABXY"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("VerticalAlignment"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("VerticalAlignment"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Center"), 0);
                items.insert(Cow::Borrowed("Top"), 1);
                items.insert(Cow::Borrowed("Bottom"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("VerticalScrollBarPosition"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("VerticalScrollBarPosition"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Left"), 1);
                items.insert(Cow::Borrowed("Right"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("VibrationMotor"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("VibrationMotor"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("Large"), 0);
                items.insert(Cow::Borrowed("Small"), 1);
                items.insert(Cow::Borrowed("LeftTrigger"), 2);
                items.insert(Cow::Borrowed("RightTrigger"), 3);
                items.insert(Cow::Borrowed("LeftHand"), 4);
                items.insert(Cow::Borrowed("RightHand"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("VideoQualitySettings"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("VideoQualitySettings"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("LowResolution"), 0);
                items.insert(Cow::Borrowed("MediumResolution"), 1);
                items.insert(Cow::Borrowed("HighResolution"), 2);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("VirtualInputMode"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("VirtualInputMode"),
            items: {
                let mut items = HashMap::with_capacity(3);
                items.insert(Cow::Borrowed("Recording"), 1);
                items.insert(Cow::Borrowed("Playing"), 2);
                items.insert(Cow::Borrowed("None"), 0);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("WaterDirection"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("WaterDirection"),
            items: {
                let mut items = HashMap::with_capacity(6);
                items.insert(Cow::Borrowed("NegX"), 0);
                items.insert(Cow::Borrowed("X"), 1);
                items.insert(Cow::Borrowed("NegY"), 2);
                items.insert(Cow::Borrowed("Y"), 3);
                items.insert(Cow::Borrowed("NegZ"), 4);
                items.insert(Cow::Borrowed("Z"), 5);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("WaterForce"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("WaterForce"),
            items: {
                let mut items = HashMap::with_capacity(5);
                items.insert(Cow::Borrowed("None"), 0);
                items.insert(Cow::Borrowed("Small"), 1);
                items.insert(Cow::Borrowed("Medium"), 2);
                items.insert(Cow::Borrowed("Strong"), 3);
                items.insert(Cow::Borrowed("Max"), 4);
                items
            },
        },
    );
    output.insert(
        Cow::Borrowed("ZIndexBehavior"),
        RbxEnumDescriptor {
            name: Cow::Borrowed("ZIndexBehavior"),
            items: {
                let mut items = HashMap::with_capacity(2);
                items.insert(Cow::Borrowed("Global"), 0);
                items.insert(Cow::Borrowed("Sibling"), 1);
                items
            },
        },
    );
    output
}
