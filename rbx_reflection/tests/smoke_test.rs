#[test]
fn services() {
    let workspace = rbx_reflection::get_class_descriptor("Workspace").unwrap();
    assert!(workspace.is_service());

    let starter_player = rbx_reflection::get_class_descriptor("StarterPlayer").unwrap();
    assert!(starter_player.is_service());

    let folder = rbx_reflection::get_class_descriptor("Folder").unwrap();
    assert!(!folder.is_service());

    let starter_player_scripts = rbx_reflection::get_class_descriptor("StarterPlayerScripts").unwrap();
    assert!(!starter_player_scripts.is_service());
}