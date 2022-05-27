use rbx_dom_weak::Instance;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FastRef(u32);

pub struct FastDom<'a> {
    instances: Vec<&'a Instance>,
}

impl<'a> FastDom<'a> {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
        }
    }

    pub fn insert(&mut self, instance: &'a Instance) -> FastRef {
        let id = FastRef(self.instances.len() as u32);
        self.instances.push(instance);
        id
    }

    pub fn get(&self, id: FastRef) -> Option<&'a Instance> {
        self.instances.get(id.0 as usize).map(|&i| i)
    }
}
