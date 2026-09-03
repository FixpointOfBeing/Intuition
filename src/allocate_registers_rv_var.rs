use std::cell::Cell;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::liveness_rv_var::{
    RvVarBasicBlockLiveness, RvVarInstrLiveness,
};
use crate::riscv::rv64imfd_reg::{FReg, XReg};
use crate::riscv_var::location::{self, RvVarLocation, x};
use crate::syntax::Ident;
use petgraph::graph;
use petgraph::{graph::NodeIndex, graph::UnGraph};

struct RvVarLocationGraph {
    ungraph: UnGraph<RvVarLocation, RvVarLocation>,
    location_nodes: HashMap<RvVarLocation, NodeIndex>,
}

impl RvVarLocationGraph {
    fn new() -> Self {
        let ungraph =
            UnGraph::<RvVarLocation, RvVarLocation>::new_undirected();
        let location_nodes = HashMap::new();
        RvVarLocationGraph { ungraph, location_nodes }
    }

    fn link(
        &mut self,
        location1: &RvVarLocation,
        location2: &RvVarLocation,
    ) {
        if location1 == location2 {
            return;
        }

        if is_float_location(location1) && is_x_location(location2) {
            return;
        }

        if is_float_location(location2) && is_x_location(location1) {
            return;
        }

        if location1 == &RvVarLocation::XReg(XReg::ZERO)
            || location2 == &RvVarLocation::XReg(XReg::ZERO)
        {
            return;
        }

        let node1 = match self.location_nodes.get(location1) {
            Some(idx) => idx.clone(),
            None => {
                let idx = self.ungraph.add_node(location1.clone());
                self.location_nodes.insert(location1.clone(), idx);
                idx
            },
        };

        let node2 = match self.location_nodes.get(location2) {
            Some(idx) => idx.clone(),
            None => {
                let idx = self.ungraph.add_node(location2.clone());
                self.location_nodes.insert(location2.clone(), idx);
                idx
            },
        };
        let edge_location_name =
            format!("{}<->{}", location1, location2);

        if !self.ungraph.contains_edge(node1, node2) {
            self.ungraph.add_edge(
                node1,
                node2,
                RvVarLocation::Dummy(edge_location_name),
            );
        }
    }

    fn neighbors(
        &self,
        location: &RvVarLocation,
    ) -> Vec<RvVarLocation> {
        if let Some(idx) = self.location_nodes.get(location) {
            let mut neighbors = Vec::new();
            for adj_idx in self.ungraph.neighbors(*idx) {
                neighbors.push(self.ungraph[adj_idx].clone());
            }

            neighbors
        } else {
            Vec::new()
        }
    }

    fn all_locations(&self) -> Vec<RvVarLocation> {
        self.location_nodes.keys().cloned().collect()
    }
}

// todo:
/*
*死定义（dead def）会产生多余干涉边
 一个定义后从不再被用的变量，仍会与它的 live-in 操作数连边。这不会导致错误代码，但会过度约束 → 可能多 spill，降低着色质量。可先做死代码消除，或对「不在任何后续 live 集合里的 def」跳过
*/

fn build_infer_graph(
    block: &RvVarBasicBlockLiveness,
) -> RvVarLocationGraph {
    let mut graph = RvVarLocationGraph::new();
    for instr in &block.instrs {
        add_write_live_edge(&mut graph, instr);
    }
    graph
}

fn is_float_location(location: &RvVarLocation) -> bool {
    matches!(
        location,
        RvVarLocation::FVar(_) | RvVarLocation::FReg(_)
    )
}

fn is_x_location(location: &RvVarLocation) -> bool {
    matches!(
        location,
        RvVarLocation::XVar(_) | RvVarLocation::XReg(_)
    )
}

fn add_write_live_edge(
    graph: &mut RvVarLocationGraph,
    instr: &RvVarInstrLiveness,
) {
    if let Some(write) = instr.instr.dest_location() {
        if write == RvVarLocation::XReg(XReg::ZERO) {
            return;
        }
        for live in &instr.live {
            graph.link(&write, live);
        }
    }
}

const ALLOCATABLE_XREGS_SIZE: u8 = 26;

fn allocatable_xregs() -> HashSet<RvVarLocation> {
    let regs = vec![
        x(4),
        x(5),
        x(6),
        x(7),
        x(9),
        x(10),
        x(11),
        x(12),
        x(13),
        x(14),
        x(15),
        x(16),
        x(17),
        x(18),
        x(19),
        x(20),
        x(21),
        x(22),
        x(23),
        x(24),
        x(25),
        x(26),
        x(27),
        x(28),
        x(29),
        x(30),
        x(31),
    ];
    regs.into_iter().collect()
}

//  ZERO(x0): 恒为 0。 RA(x1): 返回地址。SP(x2): 栈指针。GP(x3): 全局指针。FP(x8): 帧指针。TP(x4):线程指针。
fn non_allocatable_xregs() -> HashSet<RvVarLocation> {
    let regs = vec![x(0), x(1), x(2), x(3), x(4), x(8)];
    regs.into_iter().collect()
}

type Variable = Ident;

type Color = i8;

#[derive(PartialEq, Eq, Clone)]
struct LocationStaturation {
    staturation: Vec<Color>,
    color: Cell<Color>,
}

impl LocationStaturation {
    fn new() -> Self {
        let color = Cell::new(0);
        let staturation = Vec::new();
        LocationStaturation { color, staturation }
    }
}

impl PartialOrd for LocationStaturation {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LocationStaturation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.staturation.len().cmp(&other.staturation.len())
    }
}

struct RvVarLocationStaturationGraph {
    graph: UnGraph<RvVarLocation, RvVarLocation>,
    location_nodes: HashMap<RvVarLocation, NodeIndex>,
    location_staturation_map:
        HashMap<RvVarLocation, LocationStaturation>,
}

impl RvVarLocationStaturationGraph {
    fn new(location_graph: &RvVarLocationGraph) -> Self {
        let graph =
            UnGraph::<RvVarLocation, RvVarLocation>::new_undirected();
        let location_nodes = HashMap::new();
        let location_staturation_map = HashMap::new();
        RvVarLocationStaturationGraph {
            graph,
            location_nodes,
            location_staturation_map,
        }
    }

    fn add_location(
        &mut self,
        location: &RvVarLocation,
    ) -> NodeIndex {
        let node_idx = self.graph.add_node(location.clone());
        self.location_nodes.insert(location.clone(), node_idx);
        let ls = LocationStaturation::new();
        self.location_staturation_map.insert(location.clone(), ls);
        node_idx
    }

    fn link(
        &mut self,
        location1: &RvVarLocation,
        location2: &RvVarLocation,
    ) {
        if location1 == location2 {
            return;
        }

        let node1 = match self.location_nodes.get(location1) {
            Some(idx) => *idx,
            None => self.add_location(location1),
        };
        let node2 = match self.location_nodes.get(location1) {
            Some(idx) => *idx,
            None => self.add_location(location2),
        };
        let edge_location_name =
            format!("{}<->{}", location1, location2);

        if !self.graph.contains_edge(node1, node2) {
            self.graph.add_edge(
                node1,
                node2,
                RvVarLocation::Dummy(edge_location_name),
            );
        }

        let color1 = self
            .location_staturation_map
            .get(location1)
            .unwrap()
            .color
            .get();
        let color2 = self
            .location_staturation_map
            .get(location2)
            .unwrap()
            .color
            .get();
        if color1 != 0 {
            self.add_staturation(location2, color1);
        }
        if color2 != 0 {
            self.add_staturation(location1, color2);
        }
    }

    fn init(&mut self, location_graph: &RvVarLocationGraph) {
        // 初始化不能分配的x寄存器的saturation和color
        let non_allocatable = non_allocatable_xregs();
        let non_allocatable_xregs_color =
            non_allocatable_xregs_color();

        let all_locations = location_graph.all_locations();
        for location in &all_locations {
            self.add_location(&location);
            if non_allocatable.contains(&location) {
                let color = *non_allocatable_xregs_color
                    .get(&location)
                    .unwrap();
                self.set_color(&location, color);
            };
        }

        for location in all_locations {
            for neighbor in location_graph.neighbors(&location) {
                self.link(&location, &neighbor);
            }
        }
    }

    fn add_staturation(
        &mut self,
        location: &RvVarLocation,
        color: Color,
    ) {
        if let Some(ls) =
            self.location_staturation_map.get_mut(location)
        {
            ls.staturation.push(color);
        }
    }

    fn set_color(&mut self, location: &RvVarLocation, color: Color) {
        if let Some(ls) =
            self.location_staturation_map.get_mut(location)
        {
            ls.color.set(color);
        }
    }
}

fn non_allocatable_xregs_color() -> HashMap<RvVarLocation, Color> {
    let mut color_map = HashMap::<RvVarLocation, Color>::new();
    let mut i = -1;
    for location in non_allocatable_xregs() {
        color_map.insert(location, i);
        i -= 1;
    }
    color_map
}

fn color_graph(
    graph: &RvVarLocationGraph,
    xvars: Vec<Variable>,
    fvars: Vec<Variable>,
) -> (HashMap<Variable, Color>, HashMap<Variable, Color>) {
    todo!()
}

pub fn allocate_registers() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::rv64imfd_instr::Rm;
    use crate::riscv::rv64imfd_reg::XReg;
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
        build_infer_graph(&block).ungraph
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
                rs1: RvVarLocation::XReg(XReg::A0),
                rs2: ivar("c"),
            },
            &[RvVarLocation::XReg(XReg::A0), ivar("c")],
        )]);
        assert!(has_edge(
            &g,
            &ivar("a"),
            &RvVarLocation::XReg(XReg::A0)
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
