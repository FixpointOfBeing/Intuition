use std::collections::HashMap;

use crate::intu_ir::instruction::Instruction;
use crate::{
    intu_ir::name::Name,
    liveness::{IntuBasicBlock, IntuInstru},
};
use petgraph::data::Build;
use petgraph::{graph::NodeIndex, graph::UnGraph};
fn build_infer_graph(block: &IntuBasicBlock) -> UnGraph<Name, Name> {
    let mut graph = UnGraph::<Name, Name>::new_undirected();
    let mut graph_nodes = HashMap::new();
    for instr in &block.intu_instrs {
        add_write_live_edge(&mut graph, &mut graph_nodes, instr);
    }
    graph
}

fn link(
    graph: &mut UnGraph<Name, Name>,
    graph_nodes: &mut HashMap<Name, NodeIndex>,
    name1: &Name,
    name2: &Name,
) {
    if name1 == name2 {
        return;
    }
    let node1 = match graph_nodes.get(name1) {
        Some(idx) => idx.clone(),
        None => {
            let idx = graph.add_node(name1.clone());
            graph_nodes.insert(name1.clone(), idx);
            idx
        },
    };

    let node2 = match graph_nodes.get(name2) {
        Some(idx) => idx.clone(),
        None => {
            let idx = graph.add_node(name2.clone());
            graph_nodes.insert(name2.clone(), idx);
            idx
        },
    };

    if !graph.contains_edge(node1, node2) {
        graph.add_edge(node1, node2, Name::Name("".to_string()));
    }
}
fn add_write_live_edge(
    graph: &mut UnGraph<Name, Name>,
    graph_nodes: &mut HashMap<Name, NodeIndex>,
    instr: &IntuInstru,
) {
    match &instr.instr {
        Instruction::Add { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::Sub { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::Mul { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::UDiv { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::SDiv { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::URem { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::SRem { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::And { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::Or { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::Xor { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::Shl { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::LShr { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::AShr { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FAdd { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FSub { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FMul { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FDiv { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FRem { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FNeg { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::ExtractValue { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::InsertValue { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::Alloca { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::Load { .. } => {},
        Instruction::Store { .. } => {},
        Instruction::GetElementPtr { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::Trunc { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::ZExt { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::SExt { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FPTrunc { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FPExt { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FPToUI { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FPToSI { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::UIToFP { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::SIToFP { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::PtrToInt { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::IntToPtr { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::BitCast { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::ICmp { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::FCmp { dest, .. } => {
            for name in &instr.live {
                link(graph, graph_nodes, dest, name);
            }
        },
        Instruction::Call { dest, .. } => {
            if let Some(dest1) = dest {
                for name in &instr.live {
                    link(graph, graph_nodes, dest1, name);
                }
            }
        },
    };
}
