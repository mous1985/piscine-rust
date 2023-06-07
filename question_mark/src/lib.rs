pub struct One {
    pub first_layer: Option<Two>,
}

pub struct Two {
    pub second_layer: Option<Three>,
}

pub struct Three {
    pub third_layer: Option<Four>,
}

pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        match &self.first_layer {
            Some(two) => match &two.second_layer {
                Some(three) => match &three.third_layer {
                    Some(four) => four.fourth_layer,
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
}
