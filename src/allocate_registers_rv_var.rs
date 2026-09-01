use std::collections::HashMap;

use crate::liveness_rv_var::{
    RvVarBasicBlockLiveness, RvVarInstrLiveness,
};
use crate::riscv::rv64imfd_reg::IReg;
use crate::riscv_var::location::RvVarLocation;
use petgraph::{graph::NodeIndex, graph::UnGraph};

// todo:
/*
*死定义（dead def）会产生多余干涉边
 一个定义后从不再被用的变量，仍会与它的 live-in 操作数连边。这不会导致错误代码，但会过度约束 → 可能多 spill，降低着色质量。可先做死代码消除，或对「不在任何后续 live 集合里的 def」跳过
*/

fn build_infer_graph(
    block: &RvVarBasicBlockLiveness,
) -> UnGraph<RvVarLocation, RvVarLocation> {
    let mut graph =
        UnGraph::<RvVarLocation, RvVarLocation>::new_undirected();
    let mut graph_nodes = HashMap::new();
    for instr in &block.instrs {
        add_write_live_edge(&mut graph, &mut graph_nodes, instr);
    }
    graph
}

fn link(
    graph: &mut UnGraph<RvVarLocation, RvVarLocation>,
    graph_nodes: &mut HashMap<RvVarLocation, NodeIndex>,
    location1: &RvVarLocation,
    location2: &RvVarLocation,
) {
    if location1 == location2 {
        return;
    }

    if is_float_location(&location1) && is_int_location(&location2) {
        return;
    }

    if is_float_location(&location2) && is_int_location(&location1) {
        return;
    }

    if location1 == &RvVarLocation::IReg(IReg::ZERO)
        || location2 == &RvVarLocation::IReg(IReg::ZERO)
    {
        return;
    }

    let node1 = match graph_nodes.get(location1) {
        Some(idx) => idx.clone(),
        None => {
            let idx = graph.add_node(location1.clone());
            graph_nodes.insert(location1.clone(), idx);
            idx
        },
    };

    let node2 = match graph_nodes.get(location2) {
        Some(idx) => idx.clone(),
        None => {
            let idx = graph.add_node(location2.clone());
            graph_nodes.insert(location2.clone(), idx);
            idx
        },
    };
    let edge_location_name = format!("{}<->{}", location1, location2);

    if !graph.contains_edge(node1, node2) {
        graph.add_edge(
            node1,
            node2,
            RvVarLocation::Dummy(edge_location_name),
        );
    }
}

fn is_float_location(location: &RvVarLocation) -> bool {
    matches!(
        location,
        RvVarLocation::FVar(_) | RvVarLocation::FReg(_)
    )
}

fn is_int_location(location: &RvVarLocation) -> bool {
    matches!(
        location,
        RvVarLocation::IVar(_) | RvVarLocation::IReg(_)
    )
}

fn add_write_live_edge(
    graph: &mut UnGraph<RvVarLocation, RvVarLocation>,
    graph_nodes: &mut HashMap<RvVarLocation, NodeIndex>,
    instr: &RvVarInstrLiveness,
) {
    if let Some(write) = instr.instr.dest_location() {
        if write == RvVarLocation::IReg(IReg::ZERO) {
            return;
        }
        for live in &instr.live {
            link(graph, graph_nodes, &write, live);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::rv64imfd_instr::Rm;
    use crate::riscv::rv64imfd_reg::IReg;
    use crate::riscv_var::instruction::RvVarInstr;
    use crate::riscv_var::location::{fvar, var, x0};
    use std::collections::HashSet;

    type Graph = UnGraph<RvVarLocation, RvVarLocation>;

    fn ivar(name: &str) -> RvVarLocation {
        var(name.to_string())
    }

    fn fv(name: &str) -> RvVarLocation {
        fvar(name.to_string())
    }

    fn locs(locations: &[RvVarLocation]) -> HashSet<RvVarLocation> {
        locations.iter().cloned().collect()
    }

    fn il(
        instr: RvVarInstr,
        live: &[RvVarLocation],
    ) -> RvVarInstrLiveness {
        RvVarInstrLiveness { instr, live: locs(live) }
    }

    fn graph_of(instrs: Vec<RvVarInstrLiveness>) -> Graph {
        let block = RvVarBasicBlockLiveness {
            name: crate::riscv_var::label::Label::new(
                "bb".to_string(),
            ),
            instrs,
            live_out: HashSet::new(),
        };
        build_infer_graph(&block)
    }

    fn node(
        g: &Graph,
        location: &RvVarLocation,
    ) -> Option<NodeIndex> {
        g.node_indices().find(|&i| g.node_weight(i) == Some(location))
    }

    fn has_edge(
        g: &Graph,
        a: &RvVarLocation,
        b: &RvVarLocation,
    ) -> bool {
        match (node(g, a), node(g, b)) {
            (Some(x), Some(y)) => g.contains_edge(x, y),
            _ => false,
        }
    }

    #[test]
    fn int_def_links_live_int_sources() {
        let g = graph_of(vec![il(
            RvVarInstr::Add {
                rd: ivar("a"),
                rs1: ivar("b"),
                rs2: ivar("c"),
            },
            &[ivar("b"), ivar("c")],
        )]);
        assert!(has_edge(&g, &ivar("a"), &ivar("b")));
        assert!(has_edge(&g, &ivar("a"), &ivar("c")));
        assert_eq!(g.edge_count(), 2);
    }

    #[test]
    fn float_def_links_live_float_sources() {
        let g = graph_of(vec![il(
            RvVarInstr::FaddS {
                rd: fv("d"),
                rs1: fv("x"),
                rs2: fv("y"),
                rm: Rm::Rne,
            },
            &[fv("x"), fv("y")],
        )]);
        assert!(has_edge(&g, &fv("d"), &fv("x")));
        assert!(has_edge(&g, &fv("d"), &fv("y")));
        assert_eq!(g.edge_count(), 2);
    }

    #[test]
    fn no_cross_class_edge() {
        // int def reading float source (fmv.x.w)
        let g = graph_of(vec![il(
            RvVarInstr::FmvXW { rd: ivar("d"), rs1: fv("f") },
            &[fv("f")],
        )]);
        assert!(!has_edge(&g, &ivar("d"), &fv("f")));
        assert_eq!(g.edge_count(), 0);

        // float def reading int source (fmv.w.x)
        let g = graph_of(vec![il(
            RvVarInstr::FmvWX { rd: fv("d"), rs1: ivar("i") },
            &[ivar("i")],
        )]);
        assert!(!has_edge(&g, &fv("d"), &ivar("i")));
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn zero_def_produces_nothing() {
        let g = graph_of(vec![il(
            RvVarInstr::Add {
                rd: x0(),
                rs1: ivar("a"),
                rs2: ivar("b"),
            },
            &[ivar("a"), ivar("b")],
        )]);
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn zero_source_is_ignored() {
        let g = graph_of(vec![il(
            RvVarInstr::Add {
                rd: ivar("d"),
                rs1: x0(),
                rs2: ivar("b"),
            },
            &[x0(), ivar("b")],
        )]);
        assert!(has_edge(&g, &ivar("d"), &ivar("b")));
        assert!(!has_edge(&g, &ivar("d"), &x0()));
        assert!(node(&g, &x0()).is_none());
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);
    }

    #[test]
    fn self_edge_is_skipped() {
        let g = graph_of(vec![il(
            RvVarInstr::Add {
                rd: ivar("a"),
                rs1: ivar("a"),
                rs2: ivar("b"),
            },
            &[ivar("a"), ivar("b")],
        )]);
        assert!(has_edge(&g, &ivar("a"), &ivar("b")));
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);
    }

    #[test]
    fn int_var_interferes_with_physical_int_reg() {
        let g = graph_of(vec![il(
            RvVarInstr::Add {
                rd: ivar("a"),
                rs1: RvVarLocation::IReg(IReg::A0),
                rs2: ivar("c"),
            },
            &[RvVarLocation::IReg(IReg::A0), ivar("c")],
        )]);
        assert!(has_edge(
            &g,
            &ivar("a"),
            &RvVarLocation::IReg(IReg::A0)
        ));
        assert!(has_edge(&g, &ivar("a"), &ivar("c")));
        assert_eq!(g.edge_count(), 2);
    }

    #[test]
    fn mixed_int_and_float_stay_separate() {
        let g = graph_of(vec![
            il(
                RvVarInstr::Add {
                    rd: ivar("a"),
                    rs1: ivar("b"),
                    rs2: ivar("c"),
                },
                &[ivar("b"), ivar("c")],
            ),
            il(
                RvVarInstr::FaddS {
                    rd: fv("d"),
                    rs1: fv("x"),
                    rs2: fv("y"),
                    rm: Rm::Rne,
                },
                &[fv("x"), fv("y")],
            ),
        ]);
        assert!(has_edge(&g, &ivar("a"), &ivar("b")));
        assert!(has_edge(&g, &fv("d"), &fv("x")));
        assert!(!has_edge(&g, &ivar("a"), &fv("d")));
        assert!(!has_edge(&g, &ivar("b"), &fv("x")));
        assert_eq!(g.edge_count(), 4);
    }
}
