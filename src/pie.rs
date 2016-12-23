use std::f32::consts;

use graph::{Graph, Tools, Coord, Padding, HTML, Size};
use entry::Entry;

pub struct PieBuilder {
    width: f32,
    height: f32,
    entries: Option<Vec<Entry>>,
}

impl PieBuilder {
    pub fn new() -> PieBuilder {
        PieBuilder {
            width: 500.0,
            height: 500.0,
            entries: None,
        }
    }

    pub fn width(mut self, width: f32) -> PieBuilder {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> PieBuilder {
        self.height = height;
        self
    }

    pub fn entries(mut self, entries: Vec<Entry>) -> PieBuilder {
        self.entries = Some(entries);
        self
    }

    pub fn build(self) -> Pie {
        let padding = Padding::with_same(15.0);
        let Padding { top, right, bottom, left } = padding;
        let (width, height) = (self.width, self.height);
        let entries = match self.entries {
            Some(e) => e,
            None    => Vec::with_capacity(0),
        };

        let label_width = 100.0; // TODO: calculate dynamically
        let label_padding_left = 20.0;
        let label_padding_top = 30.0;
        let body = Coord {
            x: left,
            y: top,
            width: width - label_width - label_padding_left - left - right,
            height: height - top - bottom,
        };

        let labels_body = Coord {
            x: body.x + body.width + label_padding_left,
            y: top + label_padding_top,
            width: label_width,
            height: body.height - label_padding_top,
        };

        Pie {
            size: Size { width: width, height: height },
            body: body,
            labels_body: labels_body,
            sum: entries
                .iter()
                .fold(0, |acc, e| acc + e.value),
            entries: entries,
        }
    }
}

pub struct Pie {
    size: Size,
    entries: Vec<Entry>,
    labels_body: Coord,
    body: Coord,
    sum: i32,
}

impl Pie {
    fn angle(&self, v: i32) -> f32 {
        (v as f32) / (self.sum as f32) * 2.0 * consts::PI
    }

    fn color(i: usize) -> String {
        let colors = vec![
            //"rgba(0,0,0,0.5)",
            //"rgba(0,94,255,0.5)",
            //"rgba(255,165,0,0.5)",
            //"rgba(10,199,0,0.5)",
            //"rgba(220,232,0,0.5)",
            //"rgba(232,0,162,0.5)"
            //
            "rgb(237,10,63)",
            "rgb(231,114,0)",
            "rgb(254,216,93)",
            "rgb(1,120,111)",
            "rgb(165,7,44)",
            "rgb(243,184,127)",
            "rgb(66,75,77)",
            "rgb(203,172,74)",
            "rgb(153,201,197)",
            "rgb(129,135,136)",
        ];

        if i >= colors.len() {
            Pie::color(i - colors.len())
        } else {
            colors[i].to_string()
        }
    }

    fn arcs(&self) -> Vec<Arc> {
        let r = if self.body.height > self.body.width {
            self.body.width / 2.0
        } else {
            self.body.height / 2.0
        };
        let text_r = r * 0.85;
        let mut prev_angle = 0.0;

        self.entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let d_angle = self.angle(e.value);
                let next_angle = prev_angle + d_angle;
                let a = (-r * prev_angle.cos(), r * prev_angle.sin());
                let b = (-r * next_angle.cos(), r * next_angle.sin());

                let text_angle = prev_angle + (d_angle / 2.0);
                let (text_x, text_y) = (
                    -text_r * text_angle.cos(),
                     text_r * text_angle.sin()
                );

                prev_angle = next_angle;

                let text = format!("{} ({})", e.label, e.value);

                Arc {
                    path: format!("M{},{}A{},{},0,0,0,{},{}L0,0Z", a.0, a.1, r, r, b.0, b.1),
                    fill: Pie::color(i),
                    text_dx: -(text.len() as f32 * 2.0),
                    text: text,
                    text_x: text_x,
                    text_y: text_y,
                }
            })
            .collect()
    }

    fn labels(&self) -> Vec<Label> {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                Label {
                    fill: Pie::color(i),
                    text: e.label.clone(),
                    x: 0.0,
                    y: (i as f32) * 20.0,
                }
            })
            .collect()
    }
}

impl Graph for Pie {
    fn into_html(&self) -> HTML {
        let center = self.body.center();
        let labels = self.labels();
        let arcs = self.arcs();

        html! {
            svg width=(self.size.width) height=(self.size.height) xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" {

                g.labels transform=(Tools::tr(self.labels_body.x, self.labels_body.y)) {
                    @for Label { fill, text, x, y } in labels {
                        g.label transform=(Tools::tr(x, y)) {
                            rect fill=(fill) x="0" y="-11" width="12" height="12" {}
                            text anchor="start" dx="18" (text)
                        }
                    }
                }

                g.content transform=(Tools::tr(center.0, center.1)) {
                    @for Arc { path, fill, text, text_x, text_y, text_dx } in arcs {
                        g.arc {
                            path stroke="rgba(245,245,245,0.8)" fill=(fill) d=(path) {}
                            text transform=(Tools::tr(text_x, text_y)) dx=(text_dx) anchor="middle" (text)
                        }

                    }
                }
            }
        }
    }
}

struct Arc {
    path: String,
    fill: String,
    text: String,
    text_x: f32,
    text_y: f32,
    text_dx: f32,
}

struct Label {
    fill: String,
    text: String,
    x: f32,
    y: f32,
}
