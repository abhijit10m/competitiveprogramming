use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(PartialEq, Eq, IntEnum, Copy, Clone, Debug, Sequence, Ord, PartialOrd)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red =  2,
    Violet = 7,
    White = 9,
    Yellow = 4
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    return _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    let resistor_color = ResistorColor::from_int(value);
    if resistor_color.is_ok() {
        match resistor_color.ok().unwrap() {
            ResistorColor::Black => {return "Black".to_string()},
            ResistorColor::Brown => {return "Brown".to_string()},
            ResistorColor::Red => {return "Red".to_string()},
            ResistorColor::Orange => {return "Orange".to_string()},
            ResistorColor::Yellow => {return "Yellow".to_string()},
            ResistorColor::Green => {return "Green".to_string()},
            ResistorColor::Blue => {return "Blue".to_string()},
            ResistorColor::Violet => {return "Violet".to_string()},
            ResistorColor::Grey => {return "Grey".to_string()},
            ResistorColor::White => {return "White".to_string()}
        }
    }
    return "value out of range".to_string()
}

pub fn colors() -> Vec<ResistorColor> {
    // unimplemented!("return a list of all the colors ordered by resistance")
    let mut result = all::<ResistorColor>().collect::<Vec<_>>();
    result.sort();
    return result
}
