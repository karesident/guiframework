use util::sizes;

#[derive(Clone)]
pub struct BoundingBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl BoundingBox {
    pub fn is_in_bound(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub fn get_center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    pub fn is_enclosed(&self, outer: &BoundingBox) -> bool {
        let smaller_width_and_enclosed = self.width <= outer.width && self.x >= outer.width &&
                                         self.x + self.width <= outer.x + outer.width;
        let smaller_height_and_enclosed = self.height <= outer.height && self.y >= outer.y &&
                                          self.y + self.height <= outer.y + outer.height;

        smaller_width_and_enclosed && smaller_height_and_enclosed
    }

    pub fn rebase_to_outer_box(&mut self, outer: &BoundingBox) -> (i32, i32) {
        if self.width > outer.width {
            self.width = outer.width;
        }

        if self.height > outer.height {
            self.height = outer.height;
        }

        let mut delta_x = 0;
        let mut delta_y = 0;

        if !self.is_enclosed(outer) {
            if self.x < outer.x {
                delta_x = outer.x - self.x;
                self.x = outer.x;
            }

            if self.x + self.width > outer.x + outer.width {
                delta_x = (outer.x + outer.width - self.width) - self.x;
                self.x = outer.x + outer.width - self.width;
            }

            if self.y < outer.y {
                delta_y = outer.y - self.y;
                self.y = outer.y;
            }

            if self.y + self.height > outer.y + outer.height {
                delta_y = (outer.y + outer.height - self.height) - self.y;
                self.y = outer.y + outer.height - self.height;
            }
        }

        (delta_x, delta_y)
    }

    // Returns the actual distance the object was moved.
    pub fn move_in_direction(&mut self,
                             dir_x: i32,
                             dir_y: i32,
                             outer_bounding_box: Option<&BoundingBox>)
                             -> (i32, i32) {
        let pos_x_new = self.x + dir_x;
        let pos_y_new = self.y + dir_y;

        self.x = pos_x_new;
        self.y = pos_y_new;

        // let mut moved_x = 0;
        // let mut moved_y = 0;
        let mut moved_x = dir_x;
        let mut moved_y = dir_y;

        /*
        if pos_x_new < 0 {
            self.x = 0;
            moved_x = dir_x - (0 - pos_x_new);
        } else if pos_x_new + self.width > sizes::MAX_X {
            self.x = sizes::MAX_X - self.width;
            moved_x = dir_x - (pos_x_new - sizes::MAX_X);
        } else {
            self.x = pos_x_new;
            moved_x = dir_x;
        }

        if pos_y_new < 0 {
            self.y = 0;
            moved_y = dir_y - (0 - pos_y_new);
        } else if pos_y_new + self.height > sizes::MAX_Y {
            self.y = sizes::MAX_Y - self.height;
            moved_y = dir_y - (pos_y_new - sizes::MAX_Y);
        } else {
            self.y = pos_y_new;
            moved_y = dir_y;
        }*/

        if let Some(outer_bounding_box) = outer_bounding_box {
            let (delta_x, delta_y) = self.rebase_to_outer_box(&outer_bounding_box);
            moved_x += delta_x;
            moved_y += delta_y;
        }

        (moved_x, moved_y)
    }
}
