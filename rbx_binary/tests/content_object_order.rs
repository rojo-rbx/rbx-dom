use behave::prelude::*;
use rbx_dom_weak::{
    types::{Content, Variant},
    InstanceBuilder, WeakDom,
};

#[path = "support/native_model.rs"]
mod native_model;

use native_model::{interleaved, strings, Model};

behave! {
    "Reading native object content" {
        "two instances keep their own distinct object targets after serialization" {
            // @behavior native-content-object-target-order
            // @boundary before=two-distinct-object-targets at=public-native-roundtrip after=each-target-stays-with-its-instance
            // @red The decoder reverses the two targets by consuming the object array from the back.
            // @green Each decoded instance still targets itself.
            let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));
            for _ in 0..2 {
                let mut instance = InstanceBuilder::new("Folder").with_name("same");
                instance.add_property("Target", Content::from_referent(instance.referent()));
                dom.insert(dom.root_ref(), instance);
            }
            let mut bytes = Vec::new();
            rbx_binary::to_writer(&mut bytes, &dom, dom.root().children())?;
            let decoded = rbx_binary::from_reader(bytes.as_slice())?;
            expect!(decoded.root().children().len()).to_equal(2)?;
            for referent in decoded.root().children() {
                let expected = Variant::Content(Content::from_referent(*referent));
                expect!(decoded.get_by_ref(*referent).and_then(|node| node.properties.get(&"Target".into())))
                    .to_equal(Some(&expected))?;
            }
        }

        "missing URI or object entries and a missing external count return decode errors" {
            // @behavior native-content-missing-data
            // @boundary before=content-source-without-required-data at=public-native-decode after=decode-error
            // @red The decoder panics while taking a missing URI or object entry.
            // @green Malformed input returns the existing property-data error.
            for (sources, counts) in [([2, 2], 12), ([1, 1], 12), ([0, 0], 8)] {
                let mut model = Model::new("Folder")?;
                model.property("Name", &strings(b"same", b"same")?)?;
                let mut content = vec![0x22];
                content.extend(interleaved(&sources));
                content.extend(vec![0; counts]);
                model.property("Target", &content)?;
                let bytes = model.finish()?;
                expect!(rbx_binary::from_reader(bytes.as_slice()).is_err()).to_equal(true)?;
            }
        }
    }
}
