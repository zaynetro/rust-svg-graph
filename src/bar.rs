use graph::{Graph, Tools, Coord, Padding, HTML, Size};
use entry::Entry;
use axis::{Axes2d, Alignment};
use scale::Scale;

pub struct Bar {
    size: Size,
    entries: Vec<Entry>,
    padding: Padding,
    body: Coord,
    axes: Axes2d,
}

impl Bar {
    pub fn new(entries: Vec<Entry>) -> Bar {
        let padding = Padding::with_same(10.0);
        let Padding { top, right, bottom, left } = padding;
        let (width, height) = (500.0, 500.0);
        let content = Coord {
            x: left,
            y: top,
            width: width - left - right,
            height: height - top - bottom,
        };
        let y_top = (1.1 * (Tools::max_entry_value(&entries) as f32)).floor();
        let axes = Axes2d::new(content,
            (entries.len(), y_top as usize),
            (0.0, 0.0),
            Alignment::Middle
        );
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
                let box_w = self.axes.x.scale.segment(i);
                let dx = box_w / 6.0;
                let w = box_w - 2.0 * dx;
                let h = self.axes.y.scale.offset(e.value as usize);

                BarColumn {
                    x: self.axes.x.scale.offset(i),
                    y: self.axes.y.height - h,
                    dx: dx,
                    width: w,
                    height: h,
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

                    (self.axes.x_axis_html(&self.entries))
                    (self.axes.y_axis_html())
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
