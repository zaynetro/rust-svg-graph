use graph::{Graph, Tools, Coord, Padding, HTML, Size};
use entry::Entry;
use axis::{Axes2d, Alignment};
use scale::Scale;

pub struct Line {
    size: Size,
    entries: Vec<Entry>,
    padding: Padding,
    body: Coord,
    axes: Axes2d,
}

impl Line {
    pub fn new(entries: Vec<Entry>) -> Line {
        let padding = Padding::with_same(15.0);
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
            (entries.len() - 1, y_top as usize),
            (-right, 0.0),
            Alignment::UnderSeparator
        );
        let body = axes.body();

        Line {
            size: Size { width: width, height: height },
            padding: padding,
            axes: axes,
            body: body,
            entries: entries,
        }
    }

    fn line_path(&self) -> String {
        let h = self.axes.y.height;

        self.entries
            .iter()
            .enumerate()
            .map(|(i, e)| (self.axes.x.scale.offset(i),
                           h - self.axes.y.scale.offset(e.value as usize)))
            .fold("".to_string(), |acc, (x, y)| {
                let op = if acc.len() == 0 {
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
                    (self.axes.y_axis_html())
                    (self.axes.x_axis_html(&self.entries))

                    g.line-box transform=(Tools::tr(self.body.x, self.body.y)) {
                        path.line fill="none" stroke-width="2" stroke="rgb(1,120,111)" d=(line) {}
                    }
                }
            }
        }
    }
}
