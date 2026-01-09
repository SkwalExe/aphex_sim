use eframe::run_native;
use aphex_sim::AphexSim;

fn main() {
    env_logger::init();
    // Use the default window options
    let win_options = eframe::NativeOptions::default();

    // Run our app
    let _ = run_native(
        "AphexSim",
        win_options,
        Box::new(|cc| Ok(Box::new(AphexSim::new(cc)))),
    )
    .inspect_err(|err| {
        println!("Could not start app: {err}");
    });
}
