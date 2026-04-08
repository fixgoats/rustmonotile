use nannou::color::{
    BEIGE, BLACK, BLUE, BLUEVIOLET, CYAN, DARKBLUE, GREEN, MAGENTA, MISTYROSE, ORANGE, PLUM, RED,
    STEELBLUE, TURQUOISE, VIOLET, WHEAT, WHITE, YELLOWGREEN,
};
use nannou::geom::{Range, Rect, pt2, pt3};
use nannou::wgpu::Texture;
use nannou::{App, Frame};
use nannou::{app, glam};
use nannou::{geom, prelude};
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use graphtiling::*;

const SQ3: f64 = 1.7320508075688772935;

fn hex_pt(x: f64, y: f64) -> Vec2 {
    v2![x + 0.5 * y, 0.5 * SQ3 * y]
}

const HAT: [Vec2; 13] = [
    v2![0., 0.],
    v2![-0.75, -0.25 * SQ3],
    v2![-0.5, -0.5 * SQ3],
    v2![0.5, -0.5 * SQ3],
    v2![0.75, -0.25 * SQ3],
    v2![1.5, -0.5 * SQ3],
    v2![2.25, -0.25 * SQ3],
    v2![2., 0.],
    v2![1.5, 0.],
    v2![1.5, 0.5 * SQ3],
    v2![0.75, 0.75 * SQ3],
    v2![0.5, 0.5 * SQ3],
    v2![0., 0.5 * SQ3],
];
const HAT1: [Vec2; 13] = [
    v2![0., 0.],
    v2![0.75, -0.25 * SQ3],
    v2![0.5, -0.5 * SQ3],
    v2![-0.5, -0.5 * SQ3],
    v2![-0.75, -0.25 * SQ3],
    v2![-1.5, -0.5 * SQ3],
    v2![-2.25, -0.25 * SQ3],
    v2![-2., 0.],
    v2![-1.5, 0.],
    v2![-1.5, 0.5 * SQ3],
    v2![-0.75, 0.75 * SQ3],
    v2![-0.5, 0.5 * SQ3],
    v2![0., 0.5 * SQ3],
];

fn vec2_to_nannou(v: Vec2) -> geom::Vec2 {
    pt2(v[0] as f32, v[1] as f32)
}

#[derive(Clone, Copy, Debug)]
enum Meta {
    Hexa,
    Tri,
    Para,
    Penta,
    Super,
}

#[derive(Clone, Copy, Debug)]
enum HatType {
    H1,
    H,
}

#[derive(Clone, Copy, Debug)]
struct HatTile {
    t: HatType,
    trans: Affine2,
}

impl HatTile {
    fn new() -> Self {
        Self {
            t: HatType::H,
            trans: Affine2::id(),
        }
    }
    fn points(self) -> [Vec2; 13] {
        let mut h = match self.t {
            HatType::H1 => HAT1,
            _ => HAT,
        };
        h.iter_mut().for_each(|x| *x = self.trans * *x);
        h
    }
}

#[derive(Clone, Debug)]
struct MetaTile {
    t: Meta,
    children: Vec<MetaTile>,
    shape: Vec<Vec2>,
    trans: Affine2,
    width: f64,
}

fn filterpts(pts: Vec<Vec2>, tol: f64) -> Vec<Vec2> {
    let mut ret = Vec::<Vec2>::with_capacity(pts.len());
    for pt in pts {
        let mut matched = false;
        for p in &ret {
            if (*p - pt).norm() < tol {
                matched = true;
            }
        }
        if !matched {
            ret.push(pt);
        }
    }
    ret
}

impl MetaTile {
    fn hats(&self) -> Vec<HatTile> {
        match self.t {
            Meta::Hexa => {
                vec![
                    HatTile {
                        t: HatType::H,
                        trans: self.trans * Affine2::from_rot(-2. * PI / 3.).trans(v2![1., SQ3]),
                    },
                    HatTile {
                        t: HatType::H,
                        trans: self.trans * Affine2::from_rot(-2. * PI / 3.).trans(v2![4.0, SQ3]),
                    },
                    HatTile {
                        t: HatType::H,
                        trans: self.trans
                            * Affine2::from_rot(2. * PI / 3.).trans(v2![2.5, 1.5 * SQ3]),
                    },
                    HatTile {
                        t: HatType::H1,
                        trans: self.trans * Affine2::from_rot(-PI / 3.).trans(v2![2.5, 0.5 * SQ3]),
                    },
                ]
            }
            Meta::Tri => {
                vec![HatTile {
                    t: HatType::H,
                    trans: self.trans.trans(v2![0.5, 0.5 * SQ3]),
                }]
            }
            Meta::Super => self
                .children
                .iter()
                .map(|mt| {
                    let mut ret = mt.hats();
                    ret.iter_mut().for_each(|h| h.trans *= self.trans);
                    ret
                })
                .flatten()
                .collect(),
            _ => {
                vec![
                    HatTile {
                        t: HatType::H,
                        trans: self.trans * Affine2::from_rot(-PI / 3.).trans(v2![0., SQ3]),
                    },
                    HatTile {
                        t: HatType::H,
                        trans: self.trans * Affine2::from_trans(v2![1.5, 0.5 * SQ3]),
                    },
                ]
            }
        }
    }
    fn to_points(&self) -> Vec<Vec2> {
        let hats = self.hats();
        let points: Vec<Vec2> = hats.iter().flat_map(|h| h.points()).collect();

        filterpts(points, 1e-14)
    }
    fn to_polys(&self) -> Vec<[Vec2; 13]> {
        let hats = self.hats();
        hats.iter().map(|h| h.points()).collect()
    }
}

macro_rules! tern {
    ($cond:expr, $true_path:expr, $false_path:expr) => {
        match $cond {
            true => $true_path,
            false => $false_path,
        }
    };
}

fn view(app: &App, model: &Model, frame: Frame) {
    // let yreflection = Affine2::from_cols(pt2(1., 0.), pt2(0., -1.), pt2(0., 1.));
    frame.clear(WHITE);
    let draw = app.draw();

    let colors = [
        PLUM,
        BEIGE,
        BLUE,
        GREEN,
        RED,
        YELLOWGREEN,
        MISTYROSE,
        ORANGE,
    ];
    for i in 0..model.hats.len() {
        let pts = model.hats[i];
        draw.polygon()
            .color(colors[i % 8])
            .stroke(BLACK)
            .points(pts);
    }
    for p in &model.points {
        draw.ellipse().color(BLACK).radius(3.0).xy(*p);
    }
    draw.polyline()
        .color(BLACK)
        .points_closed(model.outline.clone());

    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    ll: Vec2,
    ur: Vec2,
    hats: Vec<[geom::Vec2; 13]>,
    points: Vec<geom::Vec2>,
    outline: Vec<geom::Vec2>,
    window_id: nannou::window::Id,
}

fn hat_to_nannou(poly: &[Vec2; 13]) -> [geom::Vec2; 13] {
    let mut nannou_hat = [geom::Vec2::ZERO; 13];
    for i in 0..13 {
        nannou_hat[i] = vec2_to_nannou(poly[i]);
    }
    nannou_hat
}

fn apply_to_hat(trans: Affine2, poly: &[Vec2; 13]) -> [Vec2; 13] {
    let mut new_hat = [Vec2::zero(); 13];
    for i in 0..13 {
        new_hat[i] = trans * poly[i]
    }
    new_hat
}

fn constructpatch(h: MetaTile, t: MetaTile, p: MetaTile, f: MetaTile) -> MetaTile {
    let rules = vec![
        vec![0],
        vec![2, 0, 0, 2],
        vec![0, 1, 0, 2],
        vec![2, 2, 0, 2],
        vec![0, 3, 0, 2],
        vec![2, 4, 4, 2],
        vec![3, 0, 4, 3],
        vec![3, 2, 4, 3],
        vec![3, 4, 1, 3, 2, 0],
        vec![0, 8, 3, 0],
        vec![2, 9, 2, 0],
        vec![0, 10, 2, 0],
        vec![2, 11, 4, 2],
        vec![0, 12, 0, 2],
        vec![3, 13, 0, 3],
        vec![3, 14, 2, 1],
        vec![0, 15, 3, 4],
        vec![3, 8, 2, 1],
        vec![0, 17, 3, 0],
        vec![2, 18, 2, 0],
        vec![0, 19, 2, 2],
        vec![3, 20, 4, 3],
        vec![2, 20, 0, 2],
        vec![0, 22, 0, 2],
        vec![3, 23, 4, 3],
        vec![3, 23, 0, 3],
        vec![2, 16, 0, 2],
        vec![1, 9, 4, 0, 2, 2],
        vec![3, 4, 0, 3],
    ];

    let mut ret = MetaTile {
        children: vec![],
        width: h.width,
        t: Meta::Super,
        shape: vec![],
        trans: Affine2::id(),
    };
    let shapes = [h, t, p, f];

    for r in rules {
        if r.len() == 1 {
            ret.children.push(shapes[0].clone());
        } else if r.len() == 4 {
            let poly = ret.children[r[1]].shape.clone();
            let transform = ret.children[r[1]].trans;
            let p = transform * poly[(r[2] + 1) % poly.len()];
            let q = transform * poly[r[2]];
            let mut nshp = shapes[r[0]].clone();
            let npoly = &nshp.shape;
            let match_trans =
                Affine2::match_segs(npoly[r[3]], npoly[(r[3] + 1) % npoly.len()], p, q);
            nshp.trans *= match_trans;
            ret.children.push(nshp);
        } else {
            let ch_p = ret.children[r[1]].clone();
            let ch_q = ret.children[r[3]].clone();

            let p = ch_q.trans * ch_q.shape[r[4]];
            let q = ch_p.trans * ch_p.shape[r[2]];
            let mut nshp = shapes[r[0]].clone();
            let npoly = &nshp.shape;
            nshp.trans *= Affine2::match_segs(npoly[r[5]], npoly[(r[5] + 1) % npoly.len()], p, q);

            ret.children.push(nshp);
        }
    }

    return ret;
}

fn construct_metatiles(patch: MetaTile) -> [MetaTile; 4] {
    let bps1 = patch.children[8].to_points()[2];
    let bps2 = patch.children[21].to_points()[2];
    let rbps = Affine2::rot_about(-2.0 * PI / 3.0, bps1) * bps2;

    let p72 = patch.children[7].to_points()[2];
    let p252 = patch.children[25].to_points()[2];

    let llc = intersection(bps1, rbps, patch.children[6].to_points()[2], p72);
    let mut w = patch.children[6].to_points()[2] - llc;

    let mut new_h_outline = vec![llc, bps1];
    w = Affine2::from_rot(-PI / 3.) * w;
    new_h_outline.push(new_h_outline[1] + w);
    new_h_outline.push(patch.children[14].to_points()[2]);
    w = Affine2::from_rot(-PI / 3.) * w;
    new_h_outline.push(new_h_outline[3] - w);
    new_h_outline.push(patch.children[6].to_points()[2]);

    let new_h = MetaTile {
        t: Meta::Super,
        shape: new_h_outline.clone(),
        width: patch.width * 2.,
        children: [0, 9, 16, 27, 26, 6, 1, 8, 10, 15]
            .iter()
            .map(|&ch| patch.children[ch].clone())
            .collect(),
        trans: Affine2::id(),
    };

    let new_p_outline = vec![p72, p72 + bps1 - llc, bps1, llc];
    let new_p = MetaTile {
        t: Meta::Super,
        shape: new_p_outline,
        width: patch.width * 2.,
        children: [7, 2, 3, 4, 28]
            .iter()
            .map(|&ch| patch.children[ch].clone())
            .collect(),
        trans: Affine2::id(),
    };

    let new_f_outline = vec![
        bps2,
        patch.children[24].to_points()[2],
        patch.children[25].to_points()[0],
        p252,
        p252 + llc - bps1,
    ];
    let new_f = MetaTile {
        t: Meta::Super,
        shape: new_f_outline,
        children: [21, 20, 22, 23, 24, 25]
            .iter()
            .map(|&ch| patch.children[ch].clone())
            .collect(),
        trans: Affine2::id(),
        width: patch.width * 2.,
    };

    let aaa = new_h_outline[2];
    let bbb = new_h_outline[1] + new_h_outline[4] - new_h_outline[5];
    let ccc = Affine2::rot_about(-PI / 3., bbb) * aaa;
    let new_t_outline = vec![bbb, ccc, aaa];
    let new_t = MetaTile {
        t: Meta::Super,
        shape: new_t_outline,
        width: patch.width * 2.,
        children: vec![patch.children[11].clone()],
        trans: Affine2::id(),
    };

    return [new_h, new_t, new_p, new_f];
}

fn model(app: &App) -> Model {
    let win_id = app.new_window().size(700, 700).view(view).build().unwrap();

    // let init = MetaTile {
    //     t: Meta::Hexa,
    //     trans: Affine2::id(),
    // };
    // let children = init.descendants();
    // let grandchildren: Vec<MetaTile> = children.iter().flat_map(|mt| mt.descendants()).collect();
    // let points: Vec<Vec2> = filterpts(
    //     grandchildren.iter().flat_map(|mt| mt.to_points()).collect(),
    //     1e-14,
    // );
    // let win = app.window_rect();
    // let bleh = win.x.end;
    // let blah = win.y.end;
    // let scale = bleh.min(blah);

    // let vmin: Vec2 = points
    //     .iter()
    //     .fold(v2![0., 0.], |v, u| v2![v[0].min(u[0]), v[1].min(u[1])]);

    // let vmax: Vec2 = points
    //     .iter()
    //     .fold(v2![0., 0.], |v, u| v2![v[0].max(u[0]), v[1].max(u[1])]);

    // let translation: Vec2 = (vmax + vmin) * 0.5;
    // let lenvec = vmax - vmin;
    // let scalevec = vmax - translation;
    // let max_dim = scalevec.max();
    // let scalemat = mat2![
    //     scale as f64 / (max_dim + 0.05 * lenvec[0]),
    //     0.,
    //     0.,
    //     scale as f64 / (max_dim + 0.05 * lenvec[1])
    // ];
    // let transf = Affine2::from_trans(-translation).matmul(scalemat);
    // let polys = grandchildren
    //     .into_iter()
    //     .flat_map(|mt| {
    //         mt.to_polys()
    //             .iter()
    //             .map(|h| hat_to_nannou(&apply_to_hat(transf, h)))
    //             .collect::<Vec<[geom::Vec2; 13]>>()
    //     })
    //     .collect();
    let init_h = MetaTile {
        t: Meta::Hexa,
        children: vec![],
        shape: vec![
            v2![0., 0.],
            v2![4., 0.],
            v2![4.5, 0.5 * SQ3],
            v2![2.5, 2.5 * SQ3],
            v2![1.5, 2.5 * SQ3],
            v2![-0.5, 0.5 * SQ3],
        ],
        width: 2.,
        trans: Affine2::id(),
    };
    let init_t = MetaTile {
        t: Meta::Tri,
        children: vec![],
        shape: vec![v2![0., 0.], v2![3., 0.], v2![1.5, 1.5 * SQ3]],
        width: 2.,
        trans: Affine2::id(),
    };
    let init_p = MetaTile {
        t: Meta::Para,
        children: vec![],
        shape: vec![v2![0., 0.], v2![4., 0.], v2![3., SQ3], v2![-1., SQ3]],
        width: 2.,
        trans: Affine2::id(),
    };
    let init_f = MetaTile {
        t: Meta::Penta,
        children: vec![],
        shape: vec![
            v2![0., 0.],
            v2![3., 0.],
            v2![3.5, 0.5 * SQ3],
            v2![3., SQ3],
            v2![-1., SQ3],
        ],
        width: 2.,
        trans: Affine2::id(),
    };
    let patch = constructpatch(init_h, init_t, init_p, init_f);
    let tiles = construct_metatiles(patch);
    let points = tiles[0].to_points();
    let win = app.window_rect();
    let bleh = win.x.end;
    let blah = win.y.end;
    let scale = bleh.min(blah);

    let vmin: Vec2 = points
        .iter()
        .fold(v2![0., 0.], |v, u| v2![v[0].min(u[0]), v[1].min(u[1])]);

    let vmax: Vec2 = points
        .iter()
        .fold(v2![0., 0.], |v, u| v2![v[0].max(u[0]), v[1].max(u[1])]);

    let translation: Vec2 = (vmax + vmin) * 0.5;
    let lenvec = vmax - vmin;
    let scalevec = vmax - translation;
    let max_dim = scalevec.max();
    let scalemat = mat2![
        scale as f64 / (max_dim + 0.05 * lenvec[0]),
        0.,
        0.,
        scale as f64 / (max_dim + 0.05 * lenvec[1])
    ];
    let transf = Affine2::from_trans(-translation).matmul(scalemat);
    let polys = tiles[0]
        .clone()
        .to_polys()
        .iter()
        .map(|h| hat_to_nannou(&apply_to_hat(transf, h)))
        .collect::<Vec<[geom::Vec2; 13]>>();
    let outline: Vec<geom::Vec2> = tiles[0]
        .clone()
        .shape
        .into_iter()
        .map(|p| vec2_to_nannou(transf * p))
        .collect();

    Model {
        ll: vmin,
        ur: vmax,
        hats: polys,
        points: points.iter().map(|&v| vec2_to_nannou(transf * v)).collect(),
        outline,
        window_id: win_id,
    }
}

fn main() {
    nannou::app(model).run();
}
