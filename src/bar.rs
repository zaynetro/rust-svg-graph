use graph::{Graph, Tools, Coord, Padding, HTML, Size};
use entry::Entry;
use axis::{Axes2d, LabelPosition, AxisOption};
use scale::{LinearScale, LinearRoundedScale};

pub struct Bar {
    size: Size,
    entries: Vec<Entry>,
    padding: Padding,
    body: Coord,
    axes: Axes2d,
}

impl Bar {
    pub fn new(entries: Vec<Entry>) -> Bar {
        let padding = Padding::with_same(15.0);
        let (width, height) = (500.0, 500.0);
        let content = Coord::from_padding(&padding, (0.0, 0.0, width, height));

        let axes = {
            let x_opt = AxisOption {
                scale: Box::new(
                    LinearScale::new(0.0, entries.len() as f32)
                ),
                label_position: LabelPosition::InBetween,
            };

            let y_opt = {
                let (min, max) = {
                    let (min, max) = Tools::min_max_entry_values(&entries);
                    if min > 0 {
                        (0, max)
                    } else {
                        (min, max)
                    }
                };

                AxisOption {
                    scale: Box::new(
                        LinearRoundedScale::new(min as f32, max as f32)
                    ),
                    label_position: LabelPosition::Normal,
                }
            };

            Axes2d::new((content.width, content.height), x_opt, y_opt)
        };

        let body = axes.body();

        Bar {
            size: Size { width: width, height: height },
            padding: padding,
            axes: axes,
            body: body,
            entries: entries,
        }
    }

    fn bars(&self) -> Vec<BarColumn> {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let box_w = self.axes.x.scale.segment();
                let dx = box_w / 6.0;
                let w = box_w - 2.0 * dx;
                let h = self.axes.y.scale.offset(e.value as f32);
                let zero = self.axes.y.scale.offset(0.0);
                let bar_height = h - zero;
                let y = if bar_height > 0.0 {
                    self.axes.y.height - zero - bar_height
                } else {
                    self.axes.y.height - zero
                };

                BarColumn {
                    x: self.axes.x.scale.offset(i as f32),
                    y: y,
                    dx: dx,
                    width: w,
                    height: bar_height.abs(),
                    fill: "rgba(131,43,189,0.6)".to_string()
                }
            })
            .collect()
    }
}

impl Graph for Bar {
    fn into_html(&self) -> HTML {
        let bars = self.bars();

        html! {
            svg width=(self.size.width) height=(self.size.height) xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" {

                g.content transform=(Tools::tr(self.padding.left, self.padding.top)) {
                    @for BarColumn { x, y, dx, width, height, fill } in bars {
                        g.bar-box transform=(Tools::tr(self.body.x + x, self.body.y + y)) {
                            rect.bar fill=(fill) x=(dx) width=(width) height=(height) {}
                        }
                    }

                    (self.axes.render(&self.entries))
                }
            }
        }
    }
}

struct BarColumn {
    x: f32,
    y: f32,
    dx: f32,
    width: f32,
    height: f32,
    fill: String
}
