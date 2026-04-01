use std::fs::File;
use std::io::Write;
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct Node<T> {
    first_outgoing_edge: Option<usize>,
    data: T,
}

#[derive(Debug)]
pub struct Edge<T> {
    target: usize,
    next_outgoing_edge: Option<usize>,
    data: T,
}

#[derive(Debug)]
pub struct Graph<ND, ED> {
    pub nodes: Vec<Node<ND>>,
    pub edges: Vec<Edge<ED>>,
}

pub struct Successors<'graph, ND, ED> {
    graph: &'graph Graph<ND, ED>,
    current_edge_index: Option<usize>,
}

pub struct EdgeSuccessors<'graph, ND, ED> {
    graph: &'graph Graph<ND, ED>,
    current_edge_index: Option<usize>,
}

impl<ND, ED> Index<usize> for Graph<ND, ED> {
    type Output = Node<ND>;
    fn index<'a>(&'a self, i: usize) -> &'a Node<ND> {
        &self.nodes[i]
    }
}

impl<ND, ED> IndexMut<usize> for Graph<ND, ED> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Node<ND> {
        &mut self.nodes[i]
    }
}


impl<'graph, N, E> Iterator for EdgeSuccessors<'graph, N, E> {
    type Item = &'graph Edge<E>;

    fn next(&mut self) -> Option<&'graph Edge<E>> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge)
            }
        }
    }
}

impl<'graph, N, E> Iterator for Successors<'graph, N, E> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

impl<ND, ED> Graph<ND, ED> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, data: ND) {
        self.nodes.push(Node {
            first_outgoing_edge: None,
            data,
        });
    }

    pub fn nodes_from_data(&mut self, other: Vec<ND>) {
        self.nodes = other.into_iter().map(|d| Node {first_outgoing_edge: None, data: d}).collect();
    }

    pub fn add_edge(&mut self, source: usize, target: usize, data: ED) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(Edge {
            target,
            next_outgoing_edge: node_data.first_outgoing_edge,
            data,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors<'a>(&'a self, source: usize) -> Successors<'a, ND, ED> {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }
    pub fn edge_successors<'a>(&'a self, source: usize) -> EdgeSuccessors<'a, ND, ED> {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        EdgeSuccessors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Vec2 {
    data: [f64;2],
}

macro_rules! v2 {
    ( $a:expr, $b:expr ) => {
        Vec2 {data: [$a, $b]}
    };
}

impl Index<usize> for Vec2 {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.data[i]
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.data[i]
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {data: [self[0] + other[0], self[1] + other[1]]}
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {data: [self[0] - other[0], self[1] - other[1]]}
    }
}

// impl Mul<f64> for Vec2 {
//     type Output = Self;
//     fn mul(self, other: f64) -> Self {
//         Self {data: [other * self[0], other * self[1]]}
//     }
// }

impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {data: [self[0] / other, self[1] / other]}
    }
}

impl Vec2 {
    fn sqnorm(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1]
    }
    fn norm(&self) -> f64 {
        self.sqnorm().sqrt()
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    p: Vec2,
    t: u8,
}

impl Add<Vec2> for Point {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self{p: self.p + other, t: self.t}
    }
}

impl Sub<Vec2> for Point {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self{p: self.p - other, t: self.t}
    }
}

impl Sub for Point {
    type Output = Vec2;
    fn sub(self, other: Self) -> Vec2 {
        self.p - other.p
    }
}


fn mymul<T>(lhs: Vec2, _rhs: T) -> Vec2
where T: Mul<f64, Output=f64> + Copy {
    v2![_rhs * lhs[0], _rhs * lhs[1]]
}

macro_rules! commutative {
    ($op:ident, $opfunc:ident, $my_type:ty, $output_type:ty, $func:ident) => (
        impl $op for $my_type {
            type Output = $output_type;
            fn $opfunc(self, _rhs: $my_type) -> $output_type {
                $func(self, _rhs)
            }
        }
    );
    ($op:ident, $opfunc:ident, $my_type:ty, $other_type:ty, $output_type:ty, $func:ident) => (
        impl $op<$other_type> for $my_type {
            type Output = $output_type;
            fn $opfunc(self, _rhs: $other_type) -> $output_type {
                $func(self, _rhs)
            }
        }
        impl $op<$my_type> for $other_type {
            type Output = $output_type;
            fn $opfunc(self, _rhs: $my_type) -> $output_type {
                $func(_rhs, self)
            }
        }
    )
}

commutative!(Mul, mul, Vec2, f64, Vec2, mymul);

macro_rules! p {
    ($a:expr, $b:expr, $c:expr) => {
        Point {p: v2![$a, $b], t: $c}
    };
    ($v:expr, $c:expr) => {
        Point {p: $v, t: $c}
    };
}

fn initialize_vec_with_length<T>(len: usize) -> Vec<MaybeUninit<T>> {
    std::iter::repeat_with(MaybeUninit::<T>::uninit).take(len).collect()
}

fn rot_type_by_n(t: u8, n: u8) -> u8 {
    (t + n) % 6
}

fn rot_by_n(t: [u8; 8], n: u8) -> [u8; 8] {
    let mut ret: [u8; 8] = [0; 8];
    for i in 0..8 {
        ret[i] = rot_type_by_n(t[i], n);
    }
    ret
}

fn mirror_type(t: u8) -> u8 {
    (5 * t + 1) % 6
}

fn mirror(t: [u8; 8]) -> [u8; 8] {
    let mut ret: [u8; 8] = [0; 8];
    for i in 0..8 {
        ret[i] = mirror_type(t[i]);
    }
    ret
}

fn euclid_mod(a: i8, b: i8) -> u8 {
    match a < 0 {
        true => ((a + b) % b) as u8,
        false => (a % b) as u8,
    }
}

fn main() -> std::io::Result<()>{
    let sq3 = 3.0f64.sqrt();
    let base : [Vec2; 6] = [v2![-0.5, 0.5 * sq3], v2![0.5, 0.5 * sq3], v2![1., 0.], v2![0.5, -0.5 * sq3], v2![-0.5, 0.5 * sq3], v2![-1., 0.]];
    let basis1 = v2![(3. + sq3) / 2., (1. + sq3) / 2.];
    let basis2 = v2![0., 1. + sq3];

    let n = 4;
    // let points: Vec<Point> = (0..100).flat_map(|i| (0..100).map(move |j| Point{p:base[0] + basis1 * (i as f64) + basis2 * (j as f64), t: 0 as u8})).collect();
    let mut g: Graph<Point, u8> = Graph::new();
    {
        let points: Vec<Point> = (0..n).flat_map(|i| (0..n).flat_map(move |j| (0..6).map(move |k| p![base[k] +  (i as f64) * basis1  + (j as f64) * basis2, k as u8]))).collect();
        g.nodes_from_data(points);
    }

    for i in 0..g.nodes.len() {
        for j in (i+1)..g.nodes.len() {
            if (g.nodes[j].data - g.nodes[i].data).sqnorm() > 1.01 {
                g.add_edge(i, j, g.nodes[j].data.t);
                g.add_edge(j, i, g.nodes[i].data.t);
            }
        }
    }
    let mut occupied: Vec<bool> = std::iter::repeat(false).take(g.nodes.len()).collect();
    let hat : [u8; 8] = [0, 1, 2, 3, 5, 0, 4, 3];
    let mirror_hat = mirror(hat);
    let mut hats : Vec<[usize; 8]> = Vec::with_capacity(n * n);
    for i in 0..g.nodes.len() {
        if !occupied[i] {
            println!("Checking tile {}", i);
            for j in 0..8 {
                let necessary_rot = euclid_mod((g.nodes[i].data.t as i8) - (hat[j] as i8), 6);
                println!("Rotating hat by {} to match tile.", necessary_rot);
                let try_hat = rot_by_n(hat, necessary_rot);
                let mut occupied_indices: [usize; 8]= [0; 8];
                let mut matched: [bool; 8]= [false; 8];
                matched[j] = true;
                occupied_indices[j] = i;
                let mut test_site = i;
                for k in j-1..=0 {
                    for edge in g.edge_successors(test_site) {
                        // println!("Target tile is of type {}, current hat tile is of type {}", edge.data, try_hat[k]);
                        if edge.data == try_hat[k] {
                            // println!("Tiles matched, target tile's occupation is: {}", occupied[edge.target]);
                            if occupied[edge.target] {
                                break;
                            } else {
                                test_site = edge.target;
                                matched[k] = true;
                                occupied_indices[k] = test_site;

                            }
                        }
                    }                        
                }
                test_site = i;
                for k in j+1..8 {
                    for edge in g.edge_successors(test_site) {
                        // println!("Target tile is of type {}, current hat tile is of type {}", edge.data, try_hat[k]);
                        if edge.data == try_hat[k]{
                            // println!("Tiles matched, target tile's occupation is: {}", occupied[edge.target]);
                            if occupied[edge.target] {
                                break;
                            } else {
                                test_site = edge.target;
                                matched[k] = true;
                                occupied_indices[k] = test_site;
                            }
                        }
                    }                        
                }
                println!("matched: {:?}", matched);
                if matched.iter().all(|&x| x) {
                    for idx in occupied_indices { occupied[idx] = true; }
                    println!("Occupied indices: {:?}", occupied);
                    hats.push(occupied_indices);
                } else {
                    println!("Unmirrored hat didn't match, trying mirror");
                    let necessary_rot = euclid_mod((g.nodes[i].data.t as i8) - (hat[j] as i8), 6);
                    println!("Rotating hat by {} to match tile.", necessary_rot);
                    let try_hat = rot_by_n(mirror_hat, necessary_rot);
                    let mut occupied_indices: [usize; 8]= [0; 8];
                    let mut matched: [bool; 8]= [false; 8];
                    matched[j] = true;
                    occupied_indices[j] = i;
                    let mut test_site = i;
                    for k in j-1..=0 {
                        for edge in g.edge_successors(test_site) {
                        // println!("Target tile is of type {}, current hat tile is of type {}", edge.data, try_hat[k]);
                        if edge.data == try_hat[k]{
                            if occupied[edge.target] {
                            // println!("Tiles matched, target tile's occupation is: {}", occupied[edge.target]);
                                break;
                                } else {
                                    test_site = edge.target;
                                    matched[k] = true;
                                    occupied_indices[k] = test_site;
                                }
                            }
                        }                        
                    }
                    test_site = i;
                    for k in j+1..8 {
                        for edge in g.edge_successors(test_site) {
                        // println!("Target tile is of type {}, current hat tile is of type {}", edge.data, try_hat[k]);
                        if edge.data == try_hat[k]{
                            // println!("Tiles matched, target tile's occupation is: {}", occupied[edge.target]);
                            if occupied[edge.target] {
                                break;
                                } else {
                                    test_site = edge.target;
                                    matched[k] = true;
                                    occupied_indices[k] = test_site;
                                }
                            }
                        }                        
                    }
                    if matched.iter().all(|&x| x) {
                        for idx in occupied_indices { occupied[idx] = true; }
                        println!("Occupied indices: {:?}", occupied);
                        hats.push(occupied_indices);
                    }
                }
            }
        }
    }
    let mut file = File::create("foo.txt")?;
    for h in hats {
        write!(&mut file, "{} {} {} {} {} {} {} {}\n", h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7])?;
    }
    Ok(())
}
