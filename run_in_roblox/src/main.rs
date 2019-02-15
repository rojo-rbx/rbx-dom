mod roblox_install;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    process::{self, Command},
    str,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use rbx_dom_weak::{RbxValue, RbxTree, RbxInstanceProperties};
use tempfile::tempdir;
use rouille::{router, Response};

use crate::roblox_install::RobloxStudio;

const PORT: u16 = 54023;

static PLUGIN_SOURCE: &'static str = include_str!("plugin.lua");

#[derive(Debug, Clone)]
enum Message {
    Start,
    Finish,
    Message(Vec<u8>),
}

/// A wrapper for process::Child that force-kills the process on drop.
struct ChildMurderer(process::Child);

impl Drop for ChildMurderer {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

fn create_place() -> RbxTree {
    let mut tree = RbxTree::new(RbxInstanceProperties {
        name: String::from("run_in_roblox Place"),
        class_name: String::from("DataModel"),
        properties: HashMap::new(),
    });

    let root_id = tree.get_root_id();

    let http_service = RbxInstanceProperties {
        name: String::from("HttpService"),
        class_name: String::from("HttpService"),
        properties: {
            let mut properties = HashMap::new();

            properties.insert(
                String::from("HttpEnabled"),
                RbxValue::Bool { value: true },
            );

            properties
        },
    };
    tree.insert_instance(http_service, root_id);

    let marker = RbxInstanceProperties {
        name: String::from("RUN_IN_ROBLOX_MARKER"),
        class_name: String::from("StringValue"),
        properties: HashMap::new(),
    };
    tree.insert_instance(marker, root_id);

    tree
}

fn create_plugin(port: u16, injected_source: &str) -> RbxTree {
    let complete_source = PLUGIN_SOURCE
        .replace("{{PORT}}", &port.to_string())
        .replace("{{BODY}}", injected_source);

    RbxTree::new(RbxInstanceProperties {
        name: String::from("run_in_roblox Plugin"),
        class_name: String::from("Script"),
        properties: {
            let mut properties = HashMap::new();

            properties.insert(
                String::from("Source"),
                RbxValue::String { value: complete_source },
            );

            properties
        },
    })
}

fn main() {
    let studio_install = RobloxStudio::locate()
        .expect("Could not find Roblox Studio installation");

    let work_dir = tempdir()
        .expect("Could not create temporary directory");

    let injected_source = r#"
        POST_MESSAGE("HEY")
        POST_MESSAGE("ARE YOU LISTENING?")
    "#;

    let place_file_path = work_dir.path().join("place.rbxlx");
    let plugin_file_path = studio_install.built_in_plugins_path().join("run_in_roblox.rbxmx");

    {
        let place = create_place();
        let mut place_file = File::create(&place_file_path)
            .expect("Could not create temporary place file");

        let root_id = place.get_root_id();
        let top_level_ids = place.get_instance(root_id).unwrap().get_children_ids();

        rbx_xml::encode(&place, top_level_ids, &mut place_file)
            .expect("Could not serialize temporary place file to disk");
    }

    {
        let plugin = create_plugin(PORT, injected_source);
        let mut plugin_file = File::create(&plugin_file_path)
            .expect("Could not create temporary plugin file");

        let root_id = plugin.get_root_id();

        rbx_xml::encode(&plugin, &[root_id], &mut plugin_file)
            .expect("Could not serialize plugin file to disk");
    }

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let sender = Arc::new(Mutex::new(sender));

        rouille::start_server(format!("localhost:{}", PORT), move |request| {
            router!(request,
                (POST) (/start) => {
                    let sender = sender.lock().unwrap();
                    sender.send(Message::Start).unwrap();
                    Response::text("Started")
                },

                (POST) (/finish) => {
                    let sender = sender.lock().unwrap();
                    sender.send(Message::Finish).unwrap();
                    Response::text("Finished")
                },

                (POST) (/message) => {
                    let mut message = Vec::new();
                    let mut body = request.data().unwrap();
                    body.read_to_end(&mut message).unwrap();

                    let sender = sender.lock().unwrap();
                    sender.send(Message::Message(message)).unwrap();

                    Response::text("Logged")
                },

                _ => Response::empty_404()
            )
        });
    });

    let _studio_process = ChildMurderer(Command::new(studio_install.exe_path())
        .arg(format!("{}", place_file_path.display()))
        .spawn()
        .expect("Couldn't start Roblox Studio"));

    match receiver.recv_timeout(Duration::from_secs(10)).unwrap() {
        Message::Start => {},
        _ => panic!("Invalid first message received"),
    }

    let mut messages = Vec::new();

    loop {
        let message = receiver.recv().unwrap();

        match message {
            Message::Start => {},
            Message::Finish => break,
            Message::Message(message) => {
                if let Ok(message_str) = str::from_utf8(&message) {
                    println!("{}", message_str);
                }

                messages.push(message);
            },
        }
    }

    let _dont_care = fs::remove_file(&plugin_file_path);
}
