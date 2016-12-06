use graph::{Tools, Coord, HTML};
use entry::Entry;
use scale::{Scale, LinearScale};

pub struct Axes2d {
    pub x: Axis,
    pub y: Axis,
}

impl Axes2d {
    pub fn new(content: Coord, max: (usize, usize), delta: (f32, f32), alignment: Alignment) -> Axes2d {
        let label_height = 30.0;
        let value_width = 30.0;
        let color = "rgb(13,16,17)".to_string();
        let (x_n, y_n) = max;
        let (dx, dy) = delta;

        let y_axis = {
            let x = 0.0;
            let y = 0.0;
            let width = value_width;
            let height = content.height - y - label_height;

            Axis {
                x: x,
                y: y,
                width: width,
                height: height,
                color: color.clone(),
                alignment: alignment.clone(),
                scale: LinearScale {
                    n: y_n,
                    width: height + dy,
                },
            }
        };

        let x_axis = {
            let x = y_axis.x + y_axis.width;
            let y = y_axis.y + y_axis.height;
            let width = content.width - x;
            let height = label_height;

            Axis {
                x: x,
                y: y,
                width: width,
                height: height,
                color: color.clone(),
                alignment: alignment.clone(),
                scale: LinearScale {
                    n: x_n,
                    width: width + dx,
                },
            }
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

    pub fn labels(&self, scale: &LinearScale, entries: &Vec<Entry>) -> Vec<Label> {
        entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let w = scale.segment(i);
                let x = scale.offset(i);
                let label_x = match self.x.alignment {
                    Alignment::Middle => x + w / 2.0,
                    _ => x
                };

                Label {
                    x: label_x,
                    y: self.x.height / 2.0,
                    color: "rgb(12,12,12)".to_string(),
                    text: e.label.clone(),
                    separator: if i > 0 {
                        Some(Separator {
                            x1: x,
                            x2: x,
                            y1: 0.0,
                            y2: -self.y.height,
                            color: "rgb(203,206,206)".to_string(),
                        })
                    } else {
                        None
                    },
                }
            })
            .collect()
    }

    pub fn values(&self) -> Vec<Label> {
        let x = self.y.y_right();


        self.y.scale.spread_segments()
            .iter()
            .map(|v| {
                let y = self.y.height - self.y.scale.offset(*v as usize);

                Label {
                    x: x - 5.0,
                    y: y,
                    color: "rgb(12,12,12)".to_string(),
                    text: format!("{}", v),
                    separator: Some(Separator {
                        x1: x,
                        x2: x + self.x.width,
                        y1: y,
                        y2: y,
                        color: "rgb(203,206,206)".to_string(),
                    }),
                }
            })
            .collect()
    }

    pub fn x_axis_html(&self, entries: &Vec<Entry>) -> HTML {
        let labels = self.labels(&self.x.scale, entries);

        html! {
            g.x-axis transform=(Tools::tr(self.x.x, self.x.y)) {
                line.axis stroke=(self.x.color) x2=(self.x.width) {}

                @for label in labels {
                    text.label x=(label.x) y=(label.y) text-anchor="middle" (label.text)

                    @match label.separator {
                        Some(Separator { color, x1, x2, y1, y2 }) => {
                            line.separator stroke=(color) x1=(x1) x2=(x2) y1=(y1) y2=(y2) {}
                        },
                        None => {}
                    }
                }
            }
        }
    }

    pub fn y_axis_html(&self) -> HTML {
        let values = self.values();

        html! {
            g.y-axis transform=(Tools::tr(self.y.x, self.y.y)) {
                @let (x1, x2, y1, y2) = self.y.y_line() {
                    line.axis stroke=(self.y.color) x1=(x1) x2=(x2) y1=(y1) y2=(y2) {}
                }

                @for value in values {
                    text.value x=(value.x) y=(value.y) text-anchor="end" (value.text)

                    @match value.separator {
                        Some(Separator { color, x1, x2, y1, y2 }) => {
                            line.separator stroke=(color) x1=(x1) x2=(x2) y1=(y1) y2=(y2) {}
                        },
                        None => {}
                    }
                }
            }
        }
    }
}

//pub struct Axis<S: Scale> {
    //...
    //pub scale: S: Scale,
//}

pub struct Axis {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: String,
    pub alignment: Alignment,
    pub scale: LinearScale,
}

impl Axis {
    pub fn y_line(&self) -> (f32, f32, f32 ,f32) {
        let x1 = self.y_right();
        (x1, x1, self.y, self.y + self.height)
    }

    fn y_right(&self) -> f32 {
        self.x + self.width
    }
}

pub struct Label {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub color: String,
    pub separator: Option<Separator>,
}

pub struct Separator {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
    pub color: String,
}

#[derive(Debug, Clone)]
pub enum Alignment {
    UnderSeparator,
    Middle,
}
