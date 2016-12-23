use graph::{Tools, Coord, HTML};
use entry::Entry;
use scale::Scale;

pub struct Axes2d {
    pub x: Axis,
    pub y: Axis,
}

impl Axes2d {
    pub fn new((width, height): (f32, f32), x_opt: AxisOption,
               y_opt: AxisOption) -> Axes2d {

        let label_height = 30.0;
        // TODO: calculate dynamically based on max value length
        let value_width = 30.0;
        let color = "rgb(13,16,17)".to_string();
        let x = value_width;
        let y = 0.0;
        let width = width - x;
        let height = height - label_height;

        let y_axis = Axis {
            x: x,
            y: y,
            width: width,
            height: height,
            color: color.clone(),
            label_position: y_opt.label_position,
            scale: y_opt.scale.with_range(0.0, height),
        };

        let x_axis = Axis {
            x: x,
            y: y,
            width: width,
            height: height,
            color: color.clone(),
            label_position: x_opt.label_position,
            scale: x_opt.scale.with_range(0.0, width),
        };

        Axes2d {
            x: x_axis,
            y: y_axis,
        }
    }

    pub fn body(&self) -> Coord {
        Coord {
            x: self.x.x,
            y: self.y.y,
            width: self.x.width,
            height: self.y.height,
        }
    }

    fn labels(&self, entries: &Vec<Entry>) -> Vec<Label> {
        entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let w = self.x.scale.segment();
                let x = self.x.scale.offset(i as f32);
                let label_x = match self.x.label_position {
                    LabelPosition::InBetween => w / 2.0,
                    LabelPosition::Normal    => 0.0,
                };

                Label {
                    x: x,
                    y: self.x.height,
                    label: Text {
                        x: label_x,
                        y: 17.0,
                        color: "rgb(12,12,12)".to_string(),
                        text: e.label.clone(),
                    },
                    line: Line {
                        x1: 0.0,
                        x2: 0.0,
                        y1: 0.0,
                        y2: -self.y.height,
                        color: if i == 0 {
                            self.y.color.clone()
                        } else {
                            "rgb(203,206,206)".to_string()
                        },
                    },
                }
            })
            .collect()
    }

    fn values(&self) -> Vec<Label> {
        self.y.scale.ticks()
            .iter()
            .map(|v| {
                let y = self.y.height - self.y.scale.offset(*v as f32);
                let y = if y < 0.0 { 0.0 } else { y };

                Label {
                    x: 0.0,
                    y: y,
                    label: Text {
                        x: -5.0,
                        y: 4.0,
                        color: "rgb(12,12,12)".to_string(),
                        text: format!("{}", v),
                    },
                    line: Line {
                        x1: 0.0,
                        x2: self.y.width,
                        y1: 0.0,
                        y2: 0.0,
                        color: if *v == 0 {
                            self.x.color.clone()
                        } else {
                            "rgb(203,206,206)".to_string()
                        },
                    },
                }
            })
            .collect()
    }

    fn vertical_lines(&self, entries: &Vec<Entry>) -> HTML {
        let labels = self.labels(entries);

        html! {
            @for Label { x, y, label, line } in labels {
                g.x-line transform=(Tools::tr(x, y)) {
                    @let Text { x, y, text, color } = label {
                        text.label x=(x) y=(y) text-anchor="middle" fill=(color) (text)
                    }

                    @let Line { x1, x2, y1, y2, color } = line {
                        line x1=(x1) x2=(x2) y1=(y1) y2=(y2) stroke=(color) {}
                    }
                }
            }
        }
    }

    fn horizontal_lines(&self) -> HTML {
        let values = self.values();

        html! {
            @for Label { x, y, label, line } in values {
                g.y-line transform=(Tools::tr(x, y)) {
                    @let Text { x, y, text, color } = label {
                        text.label x=(x) y=(y) text-anchor="end" fill=(color) (text)
                    }

                    @let Line { x1, x2, y1, y2, color } = line {
                        line x1=(x1) x2=(x2) y1=(y1) y2=(y2) stroke=(color) {}
                    }
                }
            }
        }
    }

    pub fn render(&self, entries: &Vec<Entry>) -> HTML {
        html! {
            g.axes transform=(Tools::tr(self.x.x, self.x.y)) {
                (self.horizontal_lines())
                (self.vertical_lines(entries))
            }
        }
    }
}

pub struct Axis {
    color: String,

    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub label_position: LabelPosition,
    pub scale: Box<Scale>,
}

pub struct AxisOption {
    pub scale: Box<Scale>,
    pub label_position: LabelPosition,
}

struct Label {
    x: f32,
    y: f32,
    label: Text,
    line: Line,
}

struct Text {
    x: f32,
    y: f32,
    text: String,
    color: String,
}

struct Line {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    color: String,
}

//#[derive(Debug, Clone)]
pub enum LabelPosition {
    /// Next to separator
    Normal,
    /// Between separators
    InBetween,
}
