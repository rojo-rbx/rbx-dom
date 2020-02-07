use std::fmt::{self, Write};

use crate::database::ReflectionDatabase;

fn generate_fixture_place(database: &ReflectionDatabase) -> String {
    let mut output = String::new();

    writeln!(&mut output, "<roblox version=\"4\">").unwrap();

    for descriptor in database.0.classes.values() {
        let mut instance = FixtureInstance::named(&descriptor.name);

        match &*descriptor.name {
            // These types can't be put into place files by default.
            "DebuggerWatch" | "DebuggerBreakpoint" | "AdvancedDragger" | "Dragger"
            | "ScriptDebugger" | "PackageLink" => continue,

            // rbx_xml does not currently support Ray values.
            // https://github.com/rojo-rbx/rbx-dom/issues/87
            "RayValue" => continue,

            // rbx_xml does not currently support Faces values.
            // https://github.com/rojo-rbx/rbx-dom/issues/88
            "Handles" => continue,

            // rbx_xml does not currently support Axes values.
            // https://github.com/rojo-rbx/rbx-dom/issues/89
            "ArcHandles" => continue,

            // These types have specific parenting restrictions handled
            // elsewhere.
            "Terrain"
            | "Attachment"
            | "Animator"
            | "StarterPlayerScripts"
            | "StarterCharacterScripts" => continue,

            // WorldModel is not yet enabled.
            "WorldModel" => continue,

            "StarterPlayer" => {
                instance
                    .children
                    .push(FixtureInstance::named("StarterPlayerScripts"));
                instance
                    .children
                    .push(FixtureInstance::named("StarterCharacterScripts"));
            }
            "Workspace" => {
                instance.children.push(FixtureInstance::named("Terrain"));
            }
            "Part" => {
                instance.children.push(FixtureInstance::named("Attachment"));
            }
            "Humanoid" => {
                instance.children.push(FixtureInstance::named("Animator"));
            }
            _ => {}
        }

        write!(output, "{}", instance).unwrap();
    }

    writeln!(&mut output, "</roblox>").unwrap();
    output
}

struct FixtureInstance<'a> {
    name: &'a str,
    children: Vec<FixtureInstance<'a>>,
}

impl<'a> FixtureInstance<'a> {
    fn named(name: &'a str) -> Self {
        Self {
            name,
            children: Vec::new(),
        }
    }
}

impl fmt::Display for FixtureInstance<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            formatter,
            "<Item class=\"{}\" reference=\"{}\">",
            &self.name, &self.name
        )?;

        for child in &self.children {
            write!(formatter, "{}", child)?;
        }

        writeln!(formatter, "</Item>")?;

        Ok(())
    }
}
