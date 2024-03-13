use syn_solidity::{Block, Visit};

struct BlockVisitor {
    blocks: Vec<Block>,
}

impl BlockVisitor {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for BlockVisitor {
    fn visit_block(&mut self, i: &Block) {
        self.blocks.push(i.clone());
        syn_solidity::visit::visit_block(self, i);
    }
}

pub fn retrieve_block_nodes(ast: &syn_solidity::File) -> Vec<Block> {
    let mut visitor = BlockVisitor::new();
    visitor.visit_file(ast);
    visitor.blocks
}
