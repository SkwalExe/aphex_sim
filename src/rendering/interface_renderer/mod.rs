use eframe::egui::Context;
use info_pane::InfoPaneRenderer;
use settings_pane::SettingsPaneRenderer;

use crate::AphexSim;

mod info_pane;
mod settings_pane;

pub(crate) struct InterfaceRenderer;

impl InterfaceRenderer {
    pub(crate) fn render(app: &mut AphexSim, ctx: &Context) {
        SettingsPaneRenderer::render(app, ctx);
        InfoPaneRenderer::render(app, ctx);
    }
}
