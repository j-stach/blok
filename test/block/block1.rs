
use blok::Block;

#[derive(Debug, Default, Clone)]
pub struct TestBlock {
    id: String,
    connections: Vec<String>,
}

impl Block for TestBlock {
    type CreationInstructions = String;
    type ConnectionInstructions = u32;

    fn create(id: &String) -> Self {
        TestBlock {
            id: id.to_owned(),
            connections: Vec::new(),
        }
    }

    fn connect(
        &mut self, 
        other: &mut Self, 
        times: &u32
    ) {
        for _ in 0..*times {
            self.connections.push(other.id.clone())
        }
    }

    fn void() -> Self {
        Self::default()
    }

    fn is_void(&self) -> bool {
        self.id.is_empty()
    }
}

#[test] fn new_block_test() {
    let block1 = TestBlock::create(&"block1".to_string());
}


