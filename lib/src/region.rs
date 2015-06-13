type Cell = Option<u8>;

struct Slab;

impl Iterator for Slab {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        None
    }

    fn count(&self) -> usize {
        0
    }

    fn idx(&mut self, index: usize) -> Option<Cell> {
        None
    }
}

enum Layer {
    Raw{ name: String, data: Vec<u8> },
    Undefined{ name: String, data: u64 },
    Sparse{ name: String, data: HashMap<u64,Cell> }
}

impl Layer {
    pub fn wrap(s: String, d: Vec<u8>) -> Layer {
        Layer::Raw{
            name: s,
            data: d
        }
    }

    pub fn undefined(s: String, l: u64) -> Layer {
        Layer::Undefined{
            name: s,
            data: l
        }
    }

    pub fn writable(s: String) -> Layer {
        Layer::Sparse{
            name: s,
            data: HashMap::new()
        }
    }

    pub fn filter(&self, s: &Slab) -> Slab {
        unimplemented!();
    }

    pub fn name(&self) -> &String {
        unimplemented!();
    }

    pub fn write(&self, p: u64, c: Cell) -> bool {
        unimplemented!();
    }

    pub fn is_undefined(&self) -> bool {
        unimplemented!();
    }

    pub fn is_writeable(&self) -> bool {
        unimplemented!();
    }
}

struct Region {
    base: Layer,
    stack: Vec<Layer>,
    name: String,
    size: u64,
}

impl Region {
    pub fn open(s: String, p: Path) -> Region {
    }

    pub fn wrap(s: String, d: Vec<u8>) -> Region {
        Layer::Raw{
            name: s,
            data: d
        }
    }

    pub fn undefined(s: String, l: u64) -> Region {
        Layer::Undefined{
            name: s,
            data: l
        }
    }

    pub fn new(s: String, r: Layer) -> Region {
        Region{
            base: r,
            stack: vec!(r),
            name: s,
            size: r.size()
        }
    }

    pub fn push(&mut self, l: Layer) {
        unimplemented!();
    }

    pub fn read(&self) -> Slab {
        unimplemented!();
    }

    pub fn flatten(&self) -> Vec<(Bound,&Layer)> {
        unimplemented!();
    }

    pub fn stack(&self) -> Vec<(Bound,&Layer)> {
        unimplemented!();
    }

    pub fn size(&self) -> usize {
        unimplemented!();
    }

    pub fn name(&self) -> &String {
        unimplemented!();
    }
}
