use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::{Cell, RefCell},
    convert::TryFrom,
    io::{self, Cursor, Read},
    rc::Rc,
};

use behave::prelude::*;
use rbx_binary::{DecodeError, Deserializer};
use rbx_dom_weak::WeakDom;

#[path = "support/native_model.rs"]
mod native_model;

use native_model::{strings, Model};

// Native Chunk::decode allocates exactly the uncompressed payload length.
// Fixture storage is built before observation, and its aggregate allocation
// size differs from this marker. Reader bookkeeping contains only five counts.
const PAYLOAD_LENGTH: usize = 65_537;

thread_local! {
    static LIVE_PAYLOADS: Cell<Option<isize>> = const { Cell::new(None) };
}

struct PayloadAllocator;

enum CounterAccess {
    Available,
    ThreadFinished,
}

fn account_payload(change: isize) -> CounterAccess {
    match LIVE_PAYLOADS.try_with(|counter| {
        if let Some(current) = counter.get() {
            counter.set(Some(current + change));
        }
    }) {
        Ok(()) => CounterAccess::Available,
        // Allocator calls during TLS teardown cannot belong to an observation.
        Err(_) => CounterAccess::ThreadFinished,
    }
}

// SAFETY: Every pointer and layout is forwarded unchanged to System. The
// nonallocating thread-local counter observes ownership without changing it.
unsafe impl GlobalAlloc for PayloadAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: GlobalAlloc's caller supplies System's required layout.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() && layout.size() == PAYLOAD_LENGTH {
            account_payload(1);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if layout.size() == PAYLOAD_LENGTH {
            account_payload(-1);
        }
        // SAFETY: The pointer and its original layout come from GlobalAlloc.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: The original allocation and requested size are forwarded.
        let resized = unsafe { System.realloc(pointer, layout, new_size) };
        if !resized.is_null() {
            if layout.size() == PAYLOAD_LENGTH {
                account_payload(-1);
            }
            if new_size == PAYLOAD_LENGTH {
                account_payload(1);
            }
        }
        resized
    }
}

#[global_allocator]
static ALLOCATOR: PayloadAllocator = PayloadAllocator;

struct BoundaryReader {
    input: Cursor<Vec<u8>>,
    boundaries: Vec<u64>,
    observations: Rc<RefCell<Vec<isize>>>,
}

impl BoundaryReader {
    fn new(observations: Rc<RefCell<Vec<isize>>>) -> io::Result<Self> {
        let mut model = Model::new("Folder")?;
        model.property("Name", &strings(b"first", b"second")?)?;
        let payload = vec![b'x'; PAYLOAD_LENGTH];
        let mut boundaries = Vec::with_capacity(5);
        for _ in 0..4 {
            boundaries.push(u64::try_from(model.bytes.len()).map_err(io::Error::other)?);
            model.chunk(*b"FUTR", &payload)?;
        }
        boundaries.push(u64::try_from(model.bytes.len()).map_err(io::Error::other)?);
        let bytes = model.finish()?;
        Ok(Self {
            input: Cursor::new(bytes),
            boundaries,
            observations,
        })
    }
}

impl Read for BoundaryReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.boundaries.contains(&self.input.position()) {
            LIVE_PAYLOADS.with(|counter| {
                if let Some(live) = counter.get() {
                    self.observations.borrow_mut().push(live);
                }
            });
        }
        self.input.read(buffer)
    }
}

fn legacy_deserializer(reader: BoundaryReader) -> Result<WeakDom, DecodeError> {
    Deserializer::new().deserialize(reader)
}

fn legacy_from_reader(reader: BoundaryReader) -> Result<WeakDom, DecodeError> {
    rbx_binary::from_reader(reader)
}

behave! {
    "Reading ignored native chunks" {
        "legacy entries release ignored payloads before reading the next chunk while audited decoding retains them" {
            // @behavior native-legacy-ignored-payload-retention
            // @boundary before=repeated-unknown-native-chunks at=public-reader-chunk-boundaries after=legacy-releases-and-audited-retains
            // @red Both legacy entries retain all four payload allocations until END.
            // @green Only audited decoding retains payloads for its returned evidence.
            let deserializer = Deserializer::new();
            let audited_observations = Rc::new(RefCell::new(Vec::with_capacity(5)));
            let audited_reader = BoundaryReader::new(Rc::clone(&audited_observations))?;
            LIVE_PAYLOADS.with(|counter| counter.set(Some(0)));
            let report = deserializer.deserialize_with_diagnostics(audited_reader)?;
            expect!(audited_observations.borrow().as_slice()).to_equal(&[0, 1, 2, 3, 4][..])?;
            expect!(report.diagnostics().len()).to_equal(4)?;
            drop(report);
            let remaining = LIVE_PAYLOADS.with(|counter| counter.replace(None));
            expect!(remaining).to_equal(Some(0))?;

            for decode in [legacy_deserializer, legacy_from_reader] {
                let observations = Rc::new(RefCell::new(Vec::with_capacity(5)));
                let reader = BoundaryReader::new(Rc::clone(&observations))?;
                LIVE_PAYLOADS.with(|counter| counter.set(Some(0)));
                let dom = decode(reader)?;
                let remaining = LIVE_PAYLOADS.with(|counter| counter.replace(None));
                expect!(observations.borrow().as_slice()).to_equal(&[0, 0, 0, 0, 0][..])?;
                expect!(remaining).to_equal(Some(0))?;
                expect!(dom.root().children().len()).to_equal(2)?;
            }
        }
    }
}
