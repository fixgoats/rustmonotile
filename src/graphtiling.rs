use log::{debug, error, info, trace, warn};
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! commutative {
    ($op:ident, $opfunc:ident, $my_type:ty, $output_type:ty, $func:ident) => {
        impl $op for $my_type {
            type Output = $output_type;
            fn $opfunc(self, _rhs: $my_type) -> $output_type {
                $func(self, _rhs)
            }
        }
    };
    ($op:ident, $opfunc:ident, $my_type:ty, $other_type:ty, $output_type:ty, $func:ident) => {
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
    };
}

#[derive(Debug)]
pub struct Node<T> {
    pub first_outgoing_edge: Option<usize>,
    pub data: T,
}

#[derive(Debug)]
pub struct Edge<T> {
    pub target: usize,
    pub next_outgoing_edge: Option<usize>,
    pub data: T,
}

#[derive(Debug)]
pub struct Graph<ND, ED> {
    pub nodes: Vec<Node<ND>>,
    pub edges: Vec<Edge<ED>>,
}

pub struct Successors<'graph, ND, ED> {
    pub graph: &'graph Graph<ND, ED>,
    pub current_edge_index: Option<usize>,
}

pub struct EdgeSuccessors<'graph, ND, ED> {
    pub graph: &'graph Graph<ND, ED>,
    pub current_edge_index: Option<usize>,
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
        self.nodes = other
            .into_iter()
            .map(|d| Node {
                first_outgoing_edge: None,
                data: d,
            })
            .collect();
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
pub struct Vec2 {
    //pub data: [f64; 2],
    pub x: f64,
    pub y: f64,
}

#[macro_export]
macro_rules! v2 {
    ( $a:expr, $b:expr ) => {
        Vec2 { x: $a, y: $b }
    };
}

impl Index<usize> for Vec2 {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2: Out of bounds."),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2: Out of bounds."),
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Self) -> Self {
        v2![self[0] + other[0], self[1] + other[1]]
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        v2![-self[0], -self[1]]
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl Mul for Vec2 {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
        other[0] * self[0] + other[1] * self[1]
    }
}

pub fn mymul<T>(lhs: Vec2, _rhs: T) -> Vec2
where
    T: Mul<f64, Output = f64> + Copy,
{
    v2![_rhs * lhs[0], _rhs * lhs[1]]
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: f64) -> Self {
        v2![other * self[0], other * self[1]]
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        v2![other[0] * self, other[1] * self]
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        (1. / other) * self
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    fn div(self, other: Vec2) -> Self {
        v2![self[0] / other[0], self[1] / other[1]]
    }
}

impl Vec2 {
    pub fn zero() -> Self {
        v2![0., 0.]
    }
    pub fn sqnorm(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1]
    }
    pub fn norm(&self) -> f64 {
        self.sqnorm().sqrt()
    }
    pub fn mirror_ax(self, ax: usize) -> Self {
        let mut ret = self;
        ret[ax] *= -1.;
        ret
    }
    pub fn max(&self) -> f64 {
        self[0].max(self[1])
    }
    pub fn min(&self) -> f64 {
        self[0].min(self[1])
    }
}

pub fn intersection(p1: Vec2, q1: Vec2, p2: Vec2, q2: Vec2) -> Vec2 {
    let d = (q2.y - p2.y) * (q1.x - p1.x) - (q2.x - p2.x) * (q1.y - p1.y);
    // let d = (v[1] - u[1]) * (q[0] - p[0]) - (v[0] - u[0]) * (q[1] - p[1]);
    let u_a = ((q2.x - p2.x) * (p1.y - p2.y) - (q2.y - p2.y) * (p1.x - p2.x)) / d;
    // let u_a = ((v[0] - u[0]) * (p[1] - u[1]) - (v[1] - u[1]) * (p[0] - u[0])) / d;
    // const uB = ((q1.x - p1.x) * (p1.y - p2.y) - (q1.y - p1.y) * (p1.x - p2.x)) / d;

    return v2![p1.x + u_a * (q1.x - p1.x), p1.y + u_a * (q1.y - p1.y)];
    // debug!("Value of d: {}", d);
    // debug!("Value of u_a: {}", u_a);
    // p + v2![u_a * (q[0] - p[0]), u_a * (q[1] - p[1])]
}

#[derive(Clone, Copy, Debug)]
pub struct Mat2 {
    pub data: [f64; 4],
}

#[macro_export]
macro_rules! mat2 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        Mat2 {
            data: [$a, $b, $c, $d],
        }
    };

    ($a:expr, $b:expr) => {
        Mat2 {
            data: [$a[0], $a[1], $b[0], $b[1]],
        }
    };
}

impl Index<usize> for Mat2 {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.data[i]
    }
}

impl IndexMut<usize> for Mat2 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.data[i]
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        v2![
            other[0] * self[0] + other[1] * self[1],
            other[0] * self[2] + other[1] * self[3]
        ]
    }
}

impl MulAssign<Mat2> for Vec2 {
    fn mul_assign(&mut self, rhs: Mat2) {
        *self = rhs * (*self)
    }
}

impl Mul<f64> for Mat2 {
    type Output = Mat2;
    fn mul(self, other: f64) -> Mat2 {
        mat2![
            other * self[0],
            other * self[1],
            other * self[2],
            other * self[3]
        ]
    }
}

impl Div<f64> for Mat2 {
    type Output = Mat2;
    fn div(self, other: f64) -> Mat2 {
        mat2![
            self[0] / other,
            self[1] / other,
            self[2] / other,
            self[3] / other
        ]
    }
}

impl Mul<Mat2> for f64 {
    type Output = Mat2;
    fn mul(self, other: Mat2) -> Mat2 {
        mat2![
            other[0] * self,
            other[1] * self,
            other[2] * self,
            other[3] * self
        ]
    }
}

impl Mul for Mat2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            data: [
                other[0] * self[0] + other[2] * self[1],
                other[1] * self[0] + other[3] * self[1],
                other[0] * self[2] + other[2] * self[3],
                other[1] * self[2] + other[3] * self[3],
            ],
        }
    }
}

impl MulAssign for Mat2 {
    fn mul_assign(&mut self, lhs: Self) {
        *self = lhs * *self;
    }
}

impl Add for Mat2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            data: [
                other[0] + self[0],
                other[1] + self[1],
                other[2] + self[2],
                other[3] + self[3],
            ],
        }
    }
}

impl Neg for Mat2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            data: [-self[0], -self[1], -self[2], -self[3]],
        }
    }
}

impl Sub for Mat2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl Mat2 {
    pub fn zero() -> Self {
        mat2![0., 0., 0., 0.]
    }
    pub fn eye() -> Self {
        mat2![1.0, 0., 0., 1.]
    }
    pub fn from_diag(v: Vec2) -> Self {
        mat2![v[0], 0., v[1], 0.]
    }
    pub fn from_rot(ang: f64) -> Self {
        mat2![ang.cos(), -ang.sin(), ang.sin(), ang.cos()]
    }
    pub fn det(&self) -> f64 {
        self[0] * self[3] - self[1] * self[2]
    }
    pub fn inv(&self) -> Mat2 {
        mat2![self[3], -self[1], -self[2], self[0]] / self.det()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Affine2 {
    pub mat: Mat2,
    pub trans: Vec2,
}

#[macro_export]
macro_rules! aff2 {
    ($a:expr, $b:expr) => {
        Affine2 { mat: $a, trans: $b }
    };
}

impl Mul for Affine2 {
    type Output = Self;
    fn mul(self, a: Self) -> Self {
        aff2![self.mat * a.mat, self.mat * a.trans + self.trans]
    }
}

impl MulAssign for Affine2 {
    fn mul_assign(&mut self, a: Self) {
        self.trans = a * self.trans;
    }
}

impl Mul<Vec2> for Affine2 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Vec2 {
        self.mat * v + self.trans
    }
}

impl Add<Vec2> for Affine2 {
    type Output = Affine2;
    fn add(self, v: Vec2) -> Affine2 {
        aff2![self.mat, self.trans + v]
    }
}

impl Affine2 {
    pub fn from_seg(p: Vec2, q: Vec2) -> Self {
        aff2![mat2![q[0] - p[0], p[1] - q[1], q[1] - p[1], q[0] - p[0]], p]
    }
    pub fn from_mat(m: Mat2) -> Self {
        aff2![m, Vec2::zero()]
    }
    pub fn from_rot(ang: f64) -> Self {
        Self::from_mat(Mat2::from_rot(ang))
    }
    pub fn from_trans(v: Vec2) -> Self {
        aff2![Mat2::eye(), v]
    }
    pub fn trans(self, v: Vec2) -> Self {
        aff2![self.mat, self.trans + v]
    }
    pub fn add_trans(&mut self, v: Vec2) {
        self.trans += v
    }
    pub fn matmul(self, m: Mat2) -> Self {
        aff2![m * self.mat, m * self.trans]
    }
    pub fn add_matmul(&mut self, m: Mat2) {
        self.mat *= m;
        self.trans *= m;
    }
    pub fn rot_about(ang: f64, v: Vec2) -> Self {
        Self::from_trans(v) * (Self::from_rot(ang) * Self::from_trans(-v))
    }
    pub fn inv(self) -> Affine2 {
        Affine2 {
            mat: self.mat.inv(),
            trans: -self.mat.inv() * self.trans,
        }
    }
    pub fn id() -> Self {
        Self::from_mat(Mat2::eye())
    }
    // pub fn match_seq(p: Vec2, q: Vec2, u: Vec2, v: Vec2) -> Affine2 {}
    pub fn pretransform(self, v: Vec2) -> Vec2 {
        self.mat * (v + self.trans)
    }
    pub fn match_segs(p: Vec2, q: Vec2, u: Vec2, v: Vec2) -> Affine2 {
        Affine2::from_seg(u, v) * Affine2::from_seg(p, q).inv()
    }
}

// pub fn vec2_sum<'a>(vecs: &'a impl IntoIterator<Item = Vec2>) -> Vec2
// where
// {
//     vecs.into_iter().reduce(|a, b| a + b).unwrap()
// }

// fn centroid<T>(vecs: impl IntoIterator<Item = Vec2> + Clone) -> Vec2
// where
//     T: IntoIterator,
// {
//     let s: Vec2 = vec2_sum(vecs.clone());
//     let size = vecs.into_iter().count() as f64;
//     s / size
// }
//
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub p: Vec2,
    pub t: u8,
}

impl Add<Vec2> for Point {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self {
            p: self.p + other,
            t: self.t,
        }
    }
}

impl Sub<Vec2> for Point {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self {
            p: self.p - other,
            t: self.t,
        }
    }
}

impl Sub for Point {
    type Output = Vec2;
    fn sub(self, other: Self) -> Vec2 {
        self.p - other.p
    }
}

#[macro_export]
macro_rules! p {
    ($a:expr, $b:expr, $c:expr) => {
        Point {
            p: v2![$a, $b],
            t: $c,
        }
    };
    ($v:expr, $c:expr) => {
        Point { p: $v, t: $c }
    };
}

pub fn rot_type_by_n(t: u8, n: u8) -> u8 {
    (t + n) % 6
}

pub fn rot_by_n(t: [u8; 8], n: u8) -> [u8; 8] {
    let mut ret: [u8; 8] = [0; 8];
    for i in 0..8 {
        ret[i] = rot_type_by_n(t[i], n);
    }
    ret
}

pub fn mirror_type(t: u8) -> u8 {
    (5 * t + 1) % 6
}

pub fn mirror(t: [u8; 8]) -> [u8; 8] {
    let mut ret: [u8; 8] = [0; 8];
    for i in 0..8 {
        ret[i] = mirror_type(t[i]);
    }
    ret
}

pub fn euclid_mod(a: i8, b: i8) -> u8 {
    match a < 0 {
        true => ((a + b) % b) as u8,
        false => (a % b) as u8,
    }
}
