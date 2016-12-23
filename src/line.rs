use graph::{Graph, Tools, Coord, Padding, HTML, Size};
use entry::Entry;
use axis::{Axes2d, LabelPosition, AxisOption};
use scale::{LinearScale, LinearRoundedScale};

pub struct LineBuilder {
    width: f32,
    height: f32,
    entries: Option<Vec<Entry>>,
}

impl LineBuilder {
    pub fn new() -> LineBuilder {
        LineBuilder {
            width: 500.0,
            height: 500.0,
            entries: None,
        }
    }

    pub fn width(mut self, width: f32) -> LineBuilder {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> LineBuilder {
        self.height = height;
        self
    }

    pub fn entries(mut self, entries: Vec<Entry>) -> LineBuilder {
        self.entries = Some(entries);
        self
    }

    pub fn build(self) -> Line {
        let padding = Padding::with_same(15.0);
        let (width, height) = (self.width, self.height);
        let content = Coord::from_padding(&padding, (0.0, 0.0, width, height));
        let entries = match self.entries {
            Some(e) => e,
            None    => Vec::with_capacity(0),
        };

        let axes = {
            let x_opt = AxisOption {
                scale: Box::new(
                    LinearScale::new(0.0, entries.len() as f32)
                ),
                label_position: LabelPosition::Normal,
            };

            let y_opt = {
                let (min, max) = {
                    let (min, max) = Tools::min_max_entry_values(&entries);
                    ((min as f32).min(0.0), (max as f32).max(0.0))
                };

                AxisOption {
                    scale: Box::new(
                        LinearRoundedScale::new(min, max)
                    ),
                    label_position: LabelPosition::Normal,
                }
            };

            Axes2d::new((content.width, content.height), x_opt, y_opt)
        };

        let body = axes.body();

        Line {
            size: Size { width: width, height: height },
            padding: padding,
            axes: axes,
            body: body,
            entries: entries,
        }
    }
}

pub struct Line {
    size: Size,
    entries: Vec<Entry>,
    padding: Padding,
    body: Coord,
    axes: Axes2d,
}

impl Line {
    fn line_path(&self) -> String {
        let h = self.body.height;

        self.entries
            .iter()
            .enumerate()
            .map(|(i, e)| (self.axes.x.scale.offset(i as f32),
                           h - self.axes.y.scale.offset(e.value as f32)))
            .fold("".to_string(), |acc, (x, y)| {
                let op = if acc.is_empty() {
                    "M"
                } else {
                    "L"
                };
                format!("{}{}{},{}", acc, op, x, y)
            })
    }
}

impl Graph for Line {
    fn into_html(&self) -> HTML {
        let line = self.line_path();

        html! {
            svg width=(self.size.width) height=(self.size.height) xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" {

                g.content transform=(Tools::tr(self.padding.left, self.padding.top)) {
                    (self.axes.render(&self.entries))

                    g.line-box transform=(Tools::tr(self.body.x, self.body.y)) {
                        path.line fill="none" stroke-width="2" stroke="rgb(1,120,111)" d=(line) {}
                    }
                }
            }
        }
    }
}
