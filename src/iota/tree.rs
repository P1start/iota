use std::cell::RefCell;

use view::View;


pub struct ViewTree<'vt> {
    leaf: Option<&'vt RefCell<View>>,
}

impl<'vt> ViewTree<'vt> {
    pub fn new(v: Option<&'vt RefCell<View>>) -> ViewTree<'vt> {
        ViewTree {
            leaf: v,
        }
    }

    fn draw(&mut self) {
        match self.leaf {
            Some(ref mut l) => return l.draw(),
            None    => {}
        }
    }
}
