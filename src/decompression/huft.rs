use std::collections::HashMap;

// 霍夫曼树的节点结构
#[derive(Debug, Clone)]
pub struct HuffmanNode {
    pub symbol: Option<u8>,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    pub fn new(symbol: Option<u8>) -> Self {
        HuffmanNode {
            symbol,
            left: None,
            right: None,
        }
    }

    pub fn print_tree(&self, prefix: String) {
        if let Some(symbol) = self.symbol {
            println!("{}Symbol: {}", prefix, symbol);
        } else {
            println!("{}[Internal Node]", prefix);
        }

        if let Some(ref left) = self.left {
            left.print_tree(format!("{}0", prefix));
        }

        if let Some(ref right) = self.right {
            right.print_tree(format!("{}1", prefix));
        }
    }
}

pub fn generate_encoding_table(node: &Option<HuffmanNode>, prefix: Vec<u8>, table: &mut HashMap<u8, Vec<u8>>) {
    if let Some(n) = node.as_ref() {
        if let Some(symbol) = n.symbol {
            table.insert(symbol, prefix);
        } else {
            if let Some(ref left) = n.left {
                let mut left_prefix = prefix.clone();
                left_prefix.push(0);
                generate_encoding_table(&Some(left.as_ref().clone()), left_prefix, table);
            }
            if let Some(ref right) = n.right {
                let mut right_prefix = prefix.clone();
                right_prefix.push(1);
                generate_encoding_table(&Some(right.as_ref().clone()), right_prefix, table);
            }
        }
    }
}



// 生成霍夫曼树
pub fn build_huffman_tree(freqs: &[u16]) -> Option<HuffmanNode> {
    // 创建节点列表，过滤掉频率为0的符号
    let mut nodes: Vec<_> = freqs.iter().enumerate()
        .filter(|&(_, &freq)| freq > 0)
        .map(|(symbol, _)| HuffmanNode::new(Some(symbol as u8)))
        .collect();

    // 当节点数大于1时合并最小的两个节点
    while nodes.len() > 1 {
        // 按照符号排序以确保每次都取最小的两个节点
        nodes.sort_by_key(|node| node.symbol);
        let left = nodes.remove(0);
        let right = nodes.remove(0);

        // 创建一个父节点，将两个子节点添加为其左右子节点
        let mut parent = HuffmanNode::new(None);
        parent.left = Some(Box::new(left));
        parent.right = Some(Box::new(right));

        // 将父节点添加回节点列表
        nodes.push(parent);
    }

    // 确保返回非空树，否则返回 None
    nodes.pop()
}





// Frees the Huffman tree by allowing Rust's ownership system to drop it
pub fn huft_free(tree: Option<HuffmanNode>) {
    // In Rust, memory cleanup is typically automatic,
    // but we can define this function if we want to manually release any resources.
    drop(tree);
}
