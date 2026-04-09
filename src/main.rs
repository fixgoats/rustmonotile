use env_logger::Env;
use log::{debug, error, info, trace, warn};
use nannou::color::{
    BEIGE, BLACK, BLUE, BLUEVIOLET, CYAN, DARKBLUE, FORESTGREEN, GREEN, MAGENTA, MISTYROSE, ORANGE,
    PLUM, RED, STEELBLUE, TURQUOISE, VIOLET, WHEAT, WHITE, YELLOWGREEN,
};
use nannou::geom::{Range, Rect, pt2, pt3};
use nannou::image::imageops::tile;
use nannou::prelude::Rgb;
use nannou::{App, Frame, LoopMode};
use nannou::{geom, prelude};
use std::env;
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
    color: Rgb<u8>,
}

impl HatTile {
    fn new() -> Self {
        Self {
            t: HatType::H,
            trans: Affine2::id(),
            color: Rgb::<u8>::new(0, 0, 0),
        }
    }
    fn points(&self) -> [Vec2; 13] {
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
    fn hats_recursive(&self, trans: Affine2) -> Vec<HatTile> {
        debug!("MetaTile::hats_recursive");
        if self.children.is_empty() {
            match self.t {
                Meta::Hexa => vec![
                    HatTile {
                        t: HatType::H1,
                        trans: trans * Affine2::from_rot(-PI / 3.).trans(v2![2.5, 0.5 * SQ3]),
                        color: BLUEVIOLET,
                    },
                    HatTile {
                        t: HatType::H,
                        trans: trans * Affine2::from_rot(-2. * PI / 3.).trans(v2![1., SQ3]),
                        color: WHEAT,
                    },
                    HatTile {
                        t: HatType::H,
                        trans: trans * Affine2::from_rot(-2. * PI / 3.).trans(v2![4.0, SQ3]),
                        color: WHEAT,
                    },
                    HatTile {
                        t: HatType::H,
                        trans: trans * Affine2::from_rot(2. * PI / 3.).trans(v2![2.5, 1.5 * SQ3]),
                        color: WHEAT,
                    },
                ],
                Meta::Tri => {
                    vec![HatTile {
                        t: HatType::H,
                        trans: trans * Affine2::from_trans(v2![0.5, 0.5 * SQ3]),
                        color: VIOLET,
                    }]
                }
                Meta::Para => {
                    vec![
                        HatTile {
                            t: HatType::H,
                            trans: trans * Affine2::from_trans(v2![1.5, 0.5 * SQ3]),
                            color: FORESTGREEN,
                        },
                        HatTile {
                            t: HatType::H,
                            trans: trans * Affine2::from_rot(-PI / 3.).trans(v2![0., SQ3]),
                            color: FORESTGREEN,
                        },
                    ]
                }
                Meta::Penta => {
                    vec![
                        HatTile {
                            t: HatType::H,
                            trans: trans * Affine2::from_trans(v2![1.5, 0.5 * SQ3]),
                            color: ORANGE,
                        },
                        HatTile {
                            t: HatType::H,
                            trans: trans * Affine2::from_rot(-PI / 3.).trans(v2![0., SQ3]),
                            color: ORANGE,
                        },
                    ]
                }
            }
        } else {
            self.children
                .iter()
                .map(|mt| mt.hats_recursive(trans * mt.trans))
                .flatten()
                .collect()
        }
    }
    fn hats(&self) -> Vec<HatTile> {
        debug!("MetaTile::to_hats");
        self.hats_recursive(self.trans)
    }

    fn to_points(&self) -> Vec<Vec2> {
        let hats = self.hats();
        let points: Vec<Vec2> = hats.iter().flat_map(|h| h.points()).collect();
        filterpts(points, 1e-14)
    }
    fn to_polys(&self) -> Vec<([Vec2; 13], Rgb<u8>)> {
        debug!("MetaTile::to_polys");
        let hats = self.hats();
        debug!("Obtained hats: {:?}", hats);
        hats.iter().map(|h| (h.points(), h.color)).collect()
    }
    fn shapes_recursive(&self, trans: Affine2, shapes: &mut Vec<Vec<Vec2>>) {
        shapes.push((&self.shape).into_iter().map(|p| trans * *p).collect());
        for ch in &self.children {
            ch.shapes_recursive(trans * ch.trans, shapes);
        }
    }
    fn all_shapes(&self) -> Vec<Vec<Vec2>> {
        let mut ret_vec = vec![];
        self.shapes_recursive(self.trans, &mut ret_vec);
        ret_vec
    }
    fn eval_child(&self, i: usize) -> Vec2 {
        self.trans * self.shape[i]
    }
    fn recentre(&mut self) {
        let mut tr = Vec2::zero();
        for p in &self.shape {
            tr += *p;
        }
        let tr = tr / self.shape.len() as f64;
        self.shape = self.shape.clone().into_iter().map(|p| p - tr).collect();
        self.children = self
            .children
            .clone()
            .into_iter()
            .map(|mt| MetaTile {
                t: mt.t,
                children: mt.children.clone(),
                shape: mt.shape.clone(),
                trans: mt.trans.trans(-tr),
            })
            .collect();
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
        let poly = model.hats[i];
        draw.polygon()
            .color(poly.1)
            .stroke(BLACK)
            .stroke_weight(3.0)
            .points(poly.0);
    }
    // for i in 0..model.hats.len() {
    //     let pts = model.hats[i];
    //     draw.text(&format!("{}", i))
    //         .font_size(36)
    //         .color(BLACK)
    //         .xy(pts[0]);
    // }
    for p in &model.points {
        draw.ellipse().color(BLACK).radius(3.0).xy(*p);
    }
    // for i in 0..model.outlines.len() {
    //     draw.polyline()
    //         .color(colors[i % 8])
    //         .weight(2.)
    //         .points_closed(model.outlines[i].clone());
    // }
    // for p in &model.bps1 {
    //     draw.ellipse().color(RED).radius(6.9).xy(*p);
    // }

    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    // ll: Vec2,
    // ur: Vec2,
    hats: Vec<([geom::Vec2; 13], Rgb<u8>)>,
    points: Vec<geom::Vec2>,
    // bps1: [geom::Vec2; 2],
    // outlines: Vec<Vec<geom::Vec2>>,
    // window_id: nannou::window::Id,
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

fn constructpatch(h: MetaTile, t: MetaTile, p: MetaTile, f: MetaTile) -> Vec<MetaTile> {
    let rules: Vec<Vec<usize>> = vec![
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

    let mut ret = vec![];
    let shapes = [h, t, p, f];

    for r in rules {
        debug!("Doing rule: {:?}", r);
        if r.len() == 1 {
            ret.push(shapes[0].clone());
        } else if r.len() == 4 {
            let poly = &ret[r[1]].shape;
            let transform = ret[r[1]].trans;
            let p = transform * poly[(r[2] + 1) % poly.len()];
            let q = transform * poly[r[2]];
            let mut nshp = shapes[r[0]].clone();
            let npoly = &nshp.shape;
            let match_trans =
                Affine2::match_segs(npoly[r[3]], npoly[(r[3] + 1) % npoly.len()], p, q);
            nshp.trans = match_trans;
            ret.push(nshp);
        } else {
            let ch_p = ret[r[1]].clone();
            let ch_q = ret[r[3]].clone();

            let p = ch_q.trans * ch_q.shape[r[4]];
            let q = ch_p.trans * ch_p.shape[r[2]];
            let mut nshp = shapes[r[0]].clone();
            let npoly = &nshp.shape;
            nshp.trans = Affine2::match_segs(npoly[r[5]], npoly[(r[5] + 1) % npoly.len()], p, q);

            ret.push(nshp);
        }
    }

    return ret;
}

fn construct_metatiles(patch: &Vec<MetaTile>) -> [MetaTile; 4] {
    debug!("Constructing MetaTiles.");
    let bps1 = patch[8].eval_child(2);
    debug!("Value of bps1: {:?}", bps1);
    let bps2 = patch[21].eval_child(2);
    debug!("Value of bps2: {:?}", bps2);
    let rbps = Affine2::rot_about(-2.0 * PI / 3.0, bps1) * bps2;
    debug!("Value of rbps: {:?}", rbps);

    let p72 = patch[7].eval_child(2);
    debug!("Value of p72: {:?}", p72);
    let p252 = patch[25].eval_child(2);
    debug!("Value of p252: {:?}", p252);

    let llc = intersection(bps1, rbps, patch[6].eval_child(2), p72);
    debug!("Value of llc: {:?}", llc);
    let mut w = patch[6].eval_child(2) - llc;
    debug!("Initial value of w: {:?}", llc);

    let mut new_h_outline = vec![llc, bps1];
    w = Affine2::from_rot(-PI / 3.) * w;
    new_h_outline.push(new_h_outline[1] + w);
    new_h_outline.push(patch[14].eval_child(2));
    w = Affine2::from_rot(-PI / 3.) * w;
    new_h_outline.push(new_h_outline[3] - w);
    new_h_outline.push(patch[6].eval_child(2));

    let mut new_h = MetaTile {
        t: Meta::Hexa,
        shape: new_h_outline.clone(),
        children: [0, 9, 16, 27, 26, 6, 1, 8, 10, 15]
            .iter()
            .map(|&ch| patch[ch].clone())
            .collect(),
        trans: Affine2::id(),
    };

    let new_p_outline = vec![p72, p72 + bps1 - llc, bps1, llc];
    let mut new_p = MetaTile {
        t: Meta::Para,
        shape: new_p_outline,
        children: [7, 2, 3, 4, 28]
            .iter()
            .map(|&ch| patch[ch].clone())
            .collect(),
        trans: Affine2::id(),
    };

    let new_f_outline = vec![
        bps2,
        patch[24].eval_child(2),
        patch[25].eval_child(0),
        p252,
        p252 + llc - bps1,
    ];
    let mut new_f = MetaTile {
        t: Meta::Penta,
        shape: new_f_outline,
        children: [21, 20, 22, 23, 24, 25]
            .iter()
            .map(|&ch| patch[ch].clone())
            .collect(),
        trans: Affine2::id(),
    };

    let aaa = new_h_outline[2];
    let bbb = new_h_outline[1] + new_h_outline[4] - new_h_outline[5];
    let ccc = Affine2::rot_about(-PI / 3., bbb) * aaa;
    let new_t_outline = vec![bbb, ccc, aaa];
    let mut new_t = MetaTile {
        t: Meta::Tri,
        shape: new_t_outline,
        children: vec![patch[11].clone()],
        trans: Affine2::id(),
    };
    // new_h.recentre();
    // new_p.recentre();
    // new_f.recentre();
    // new_t.recentre();

    return [new_h, new_t, new_p, new_f];
}

fn model(app: &App) -> Model {
    let win_id = app.new_window().size(700, 700).view(view).build().unwrap();

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
        trans: Affine2::id(),
    };
    let init_t = MetaTile {
        t: Meta::Tri,
        children: vec![],
        shape: vec![v2![0., 0.], v2![3., 0.], v2![1.5, 1.5 * SQ3]],
        trans: Affine2::id(),
    };
    let init_p = MetaTile {
        t: Meta::Para,
        children: vec![],
        shape: vec![v2![0., 0.], v2![4., 0.], v2![3., SQ3], v2![-1., SQ3]],
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
        trans: Affine2::id(),
    };
    let mut patch = constructpatch(init_h, init_t, init_p, init_f);
    let mut tiles = construct_metatiles(&patch);
    for _ in 0..5 {
        patch = constructpatch(
            tiles[0].clone(),
            tiles[1].clone(),
            tiles[2].clone(),
            tiles[3].clone(),
        );
        tiles = construct_metatiles(&patch);
    }
    // debug!("Tiles: {:?}", tiles);
    // let patch = constructpatch(
    //     tiles[0].clone(),
    //     tiles[1].clone(),
    //     tiles[2].clone(),
    //     tiles[3].clone(),
    // );
    // let tiles = construct_metatiles(patch);
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
    let question_points = [
        vec2_to_nannou(transf * patch[8].eval_child(2)),
        vec2_to_nannou(transf * patch[21].eval_child(2)),
    ];
    let polys = tiles[0]
        .to_polys()
        .iter()
        .map(|h| (hat_to_nannou(&apply_to_hat(transf, &h.0)), h.1))
        .collect::<Vec<([geom::Vec2; 13], Rgb<u8>)>>();
    let shapes = tiles[0].all_shapes();
    debug!("Obtained {} shapes: {:?}", shapes.len(), shapes);
    // let outlines: Vec<Vec<geom::Vec2>> = shapes
    //     .into_iter()
    //     .map(|shp| shp.iter().map(|p| vec2_to_nannou(transf * *p)).collect())
    //     .collect();

    Model {
        // ll: vmin,
        // ur: vmax,
        hats: polys,
        points: points.iter().map(|&v| vec2_to_nannou(transf * v)).collect(),
        // bps1: question_points,
        // outlines,
        // window_id: win_id,
    }
}

fn main() {
    let env = Env::default()
        .filter_or("RLOG", "none")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    nannou::app(model).loop_mode(LoopMode::rate_fps(10.)).run();
}
