pub trait Component {
    fn get_type(&self) -> &str;
    fn get_state(&self) -> &str;
    fn change_state(&mut self, state: &str);
    fn change_val(&mut self, value: f64) {}
}

pub trait Entity {
    fn get_component_names(&self) -> Vec<&str>;
    fn change_component_state(&mut self, comp: &str, val: &str);
    fn get_component_val(&self, comp: &str) -> Option<f64> {
        None
    }
    fn get_component_state(&self, comp: &str) -> Option<&str>;
    fn change_component_val(&mut self, comp: &str, val: f64) {}
}

pub trait System {
    fn components(&self) -> Vec<&str>;
    fn operate(&self, item: &mut dyn Entity);
}

pub fn run(world: &mut Vec<Box<dyn Entity>>, logic: &Vec<Box<dyn System>>) {
    loop {
        for i in 0..world.len() {
            for j in logic {
                if world[i]
                    .get_component_names()
                    .iter()
                    .all(|item| j.components().contains(item as &&str))
                {
                    j.operate(&mut *world[i]);
                }
            }
        }
    }
}
