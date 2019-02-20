use std::{
    collections::HashMap,
    fs::{self, File},
    process::{self, Command},
    str,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use futures::{
    future,
    stream::Stream,
    Future,
};
use hyper::{
    service::service_fn,
    Method,
    Body,
    Server,
};
use rbx_dom_weak::{RbxValue, RbxTree, RbxInstanceProperties};
use tempfile::tempdir;

use crate::roblox_install::RobloxStudio;

const PORT: u16 = 54023;

static PLUGIN_TEMPLATE: &'static str = include_str!("roblox_plugin_template.lua");

#[derive(Debug, Clone)]
enum Message {
    Start,
    Finish,
    Message(Vec<u8>),
}

/// A wrapper for process::Child that force-kills the process on drop.
struct KillOnDrop(process::Child);

impl Drop for KillOnDrop {
    fn drop(&mut self) {
        let _dont_care = self.0.kill();
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

pub fn inject_plugin_main(tree: &mut RbxTree) {
    let complete_source = PLUGIN_TEMPLATE
        .replace("{{PORT}}", &PORT.to_string());

    let entry_point = RbxInstanceProperties {
        name: String::from("generate_rbx_reflection main"),
        class_name: String::from("Script"),
        properties: {
            let mut properties = HashMap::new();

            properties.insert(
                String::from("Source"),
                RbxValue::String { value: complete_source },
            );

            properties
        },
    };

    let root_id = tree.get_root_id();
    tree.insert_instance(entry_point, root_id);
}

pub fn run_in_roblox(plugin: &RbxTree) -> Vec<Vec<u8>> {
    let studio_install = RobloxStudio::locate()
        .expect("Could not find Roblox Studio installation");

    let work_dir = tempdir()
        .expect("Could not create temporary directory");

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
        let mut plugin_file = File::create(&plugin_file_path)
            .expect("Could not create temporary plugin file");

        let root_id = plugin.get_root_id();

        rbx_xml::encode(&plugin, &[root_id], &mut plugin_file)
            .expect("Could not serialize plugin file to disk");
    }

    let (sender, receiver) = mpsc::channel();
    let (shutdown_tx, shutdown_rx) = futures::sync::oneshot::channel::<()>();

    let sender = Arc::new(Mutex::new(sender));

    thread::spawn(move || {
        let service = move || {
            let sender = Arc::clone(&sender);

            service_fn(move |request: hyper::Request<Body>| -> Box<Future<Item = hyper::Response<Body>, Error = hyper::Error> + Send> {
                let mut response = hyper::Response::new(Body::empty());

                match (request.method(), request.uri().path()) {
                    (&Method::GET, "/") => {
                        *response.body_mut() = Body::from("Hey there!");
                    },
                    (&Method::POST, "/start") => {
                        let sender = sender.lock().unwrap();
                        sender.send(Message::Start).unwrap();
                        *response.body_mut() = Body::from("Started");
                    },
                    (&Method::POST, "/finish") => {
                        let sender = sender.lock().unwrap();
                        sender.send(Message::Finish).unwrap();
                        *response.body_mut() = Body::from("Finished");
                    },
                    (&Method::POST, "/message") => {
                        let sender = Arc::clone(&sender);

                        let future = request
                            .into_body()
                            .concat2()
                            .map(move |chunk| {
                                let sender = sender.lock().unwrap();
                                sender.send(Message::Message(chunk.to_vec())).unwrap();

                                *response.body_mut() = Body::from("Got it!");
                                response
                            });

                        return Box::new(future);
                    },
                    _ => {
                        *response.status_mut() = hyper::StatusCode::NOT_FOUND;
                    },
                }

                Box::new(future::ok(response))
            })
        };

        let addr = ([127, 0, 0, 1], PORT).into();
        let server = Server::bind(&addr)
            .serve(service)
            .with_graceful_shutdown(shutdown_rx)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    });

    let _studio_process = KillOnDrop(Command::new(studio_install.exe_path())
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
            Message::Message(message) => messages.push(message),
        }
    }

    let _dont_care = shutdown_tx.send(());
    let _dont_care = fs::remove_file(&plugin_file_path);

    messages
}
