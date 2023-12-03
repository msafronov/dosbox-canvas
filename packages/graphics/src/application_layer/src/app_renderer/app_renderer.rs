extern "C" {
    fn render();
}

#[derive(Clone, Copy)]
pub struct AppRenderer {}

impl AppRenderer {
    pub fn new() -> Self {
        AppRenderer {}
    }

    pub fn render(&self) {
        unsafe {
            render();
        }
    }
}