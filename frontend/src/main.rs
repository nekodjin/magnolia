mod app;
use app::AppRenderer;

fn main() {
    let renderer = AppRenderer::new();
    renderer.render();
}
