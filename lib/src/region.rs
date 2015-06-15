use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::path::Path;
use mnemonic::Bound;
use std::iter::Repeat;
use std::slice::Iter;
pub type Cell = Option<u8>;

pub enum Slab<'a> {
    Undefined(Repeat<Cell>),
    Sparse(Values<'a,u64,Cell>),
    Raw(Iter<'a,u8>),
    Empty,
}

impl<'a> Iterator for Slab<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        match self {
            &mut Slab::Undefined(ref mut a) => a.next(),
            &mut Slab::Sparse(ref mut a) => a.next().cloned(),
            &mut Slab::Raw(ref mut a) => a.next().map(|a| Some(a.clone())),
            &mut Slab::Empty => None,
        }
    }
}

impl<'a> Slab<'a> {
    pub fn empty() -> Slab<'a> {
        Slab::Empty
    }

    pub fn idx(&mut self, index: usize) -> Option<Cell> {
        None
    }

    pub fn length(&self) -> usize {
        0
    }
}

pub enum Layer {
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

pub struct Region {
    base: Layer,
    stack: Vec<Layer>,
    name: String,
    size: u64,
}

impl Region {
    pub fn open(s: String, p: &Path) -> Region {
        unimplemented!();
    }

    pub fn wrap(s: String, d: Vec<u8>) -> Region {
        Region::new(
            s.clone(),
            Layer::Raw{
                name: s,
                data: d
            })
    }

    pub fn undefined(s: String, l: u64) -> Region {
        Region::new(
            s.clone(),
            Layer::Undefined{
                name: s,
                data: l
            })
    }

    pub fn new(s: String, r: Layer) -> Region {
        Region{
            stack: vec!(),
            name: s,
            size: r.filter(&Slab::empty()).length() as u64,
            base: r,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Region,Region,Region) {
        let r1 = Region::undefined("base".to_string(),128);
        let r2 = Region::undefined("zlib".to_string(),64);
        let r3 = Region::undefined("aes".to_string(),48);

        (r1,r2,r3)
    }

    #[test]
    fn too_small_layer_cover() {
        let mut st = Region::undefined("".to_string(),12);

        assert!(st.cover(Bound::new(0,6),Layer::wrap("anon 2".to_string(),vec!(1,2,3,4,5))));
    }

    /*
    struct region : public ::testing::Test
    {
        region(void) : regs(), r1(po::region::undefined("base",128)), r2(po::region::undefined("zlib",64)), r3(po::region::undefined("aes",48)) {}

        void SetUp(void)
        {
            auto vx1 = insert_vertex(r1,regs);
            auto vx2 = insert_vertex(r2,regs);
            auto vx3 = insert_vertex(r3,regs);

            insert_edge(po::bound(32,96),vx1,vx2,regs);
            insert_edge(po::bound(16,32),vx1,vx3,regs);
            insert_edge(po::bound(0,32),vx2,vx3,regs);
        }

        po::regions regs;

        po::region_loc r1;
        po::region_loc r2;
        po::region_loc r3;

        using vx = boost::graph_traits<po::regions>::vertex_descriptor;

        vx vx1;
        vx vx2;
        vx vx3;
    };*/

    /*
    #[test]
    fn tree() {
        let f = fixture();

        assert_eq!(spanning_tree(f.3), vec!((&r2,&r1),(&r3,&r1)));
    }
    */

    /*
    TEST_F(region,proj)
    {
        auto proj = po::projection(regs);
        decltype(proj) expect({
            make_pair(po::bound(0,16),po::region_wloc(r1)),
            make_pair(po::bound(0,48),po::region_wloc(r3)),
            make_pair(po::bound(32,64),po::region_wloc(r2)),
            make_pair(po::bound(96,128),po::region_wloc(r1))
        });

        for(auto i: proj)
        {
            std::cout << i.first << ": " << i.second->name() << std::endl;
        }

        ASSERT_TRUE(proj == expect);
    }
    */

    #[test]
    fn read_undefined() {
        let r1 = Region::undefined("test".to_string(),128);
        let mut s1 = r1.read();

        assert_eq!(s1.length(), 128);
        assert!(s1.all(|x| x.is_none()));
    }

    /*
    TEST_F(region,read_one_layer)
    {
        boost::filesystem::path p1 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");
        po::region_loc r1 = po::region::undefined("test",128);
        std::ofstream s1(p1.string());

        ASSERT_TRUE(s1.is_open());
        s1 << "Hello, World" << std::flush;
        s1.close();

        r1.write().add(po::bound(1,8),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,7})));
        r1.write().add(po::bound(50,62),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,6,5,4,3,2,1})));
        r1.write().add(po::bound(62,63),po::layer_loc(new po::layer("anon 2",{po::byte(1)})));
        r1.write().add(po::bound(70,82),po::layer_loc(new po::layer("anon 2",po::blob(p1))));

        po::slab s = r1->read();
        ASSERT_EQ(s.size(),128u);
        size_t idx = 0;

        for(auto i: s)
        {
            cout << idx << ": " << (i ? to_string((unsigned int)(*i)) : "none") << endl;
            if(idx >= 1 && idx < 8)
                ASSERT_TRUE(i && *i == idx);
            else if(idx >= 50 && idx < 56)
                ASSERT_TRUE(i && *i == idx - 49);
            else if(idx >= 56 && idx < 62)
                ASSERT_TRUE(i && *i == 6 - (idx - 56));
            else if(idx >= 70 && idx < 82)
                EXPECT_TRUE(i && *i == std::string("Hello, World").substr(idx - 70,1)[0]);
            else if(idx == 62)
                ASSERT_TRUE(i && *i == 1);
            else
                ASSERT_TRUE(i == boost::none);
            ++idx;
        }
    }
*/
 }
