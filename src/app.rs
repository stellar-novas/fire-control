/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct FireControlApp {
	x1: f64,
	y1: f64,

	x2: f64,
	y2: f64,

	velocity: i32,

	// #[serde(skip)] // This how you opt-out of serialization of a field

}

impl Default for FireControlApp {
	fn default() -> Self {
		Self {
			x1: 100.0,
			y1: 8000.0,

			x2: 0.0,
			y2: 0.0,

			velocity: 80
		}
	}
}

impl FireControlApp {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customize the look and feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}

		Default::default()
	}
}

impl eframe::App for FireControlApp {
	/// Called each time the UI needs repainting, which may be many times per second.
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
		// For inspiration and more examples, go to https://emilk.github.io/egui

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			// The top panel is often a good place for a menu bar:

			egui::menu::bar(ui, |ui| {
				// NOTE: no File->Quit on web pages!
				// let is_web = cfg!(target_arch = "wasm32");
				// if !is_web {
				//     ui.menu_button("File", |ui| {
				//         if ui.button("Quit").clicked() {
				//             ctx.send_viewport_cmd(egui::ViewportCommand::Close);
				//         }
				//     });
				//     ui.add_space(16.0);
				// }

				egui::widgets::global_theme_preference_buttons(ui);
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			// The central panel the region left after adding TopPanels and SidePanels
			ui.heading("NCWL Dashboard");
			ui.separator();

			egui::Window::new("Inputs").resizable(false).show(ctx, |ui| {
				egui::Grid::new("grid").show(ui, |ui| {
					ui.label("Current Coordinates");
					ui.add(egui::DragValue::new(&mut self.x1));
					ui.label("X1");
					ui.add(egui::DragValue::new(&mut self.y1));
					ui.label("Y1");
					ui.end_row();
					
					ui.label("Target Coordinates");
					ui.add(egui::DragValue::new(&mut self.x2));
					ui.label("X2");
					ui.add(egui::DragValue::new(&mut self.y2));
					ui.label("Y2");
					ui.end_row();
					
					ui.label("Shell Velocity");
					ui.add(egui::DragValue::new(&mut self.velocity));
					ui.end_row();
				});
				// ui.horizontal(|ui| {
				// 	ui.label("Write something: ");
				// 	ui.text_edit_singleline(&mut self.label);
				// 	ui.separator()
				// });
				//
				// ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
				// if ui.button("Increment").clicked() {
				// 	self.value += 1.0;
				// }
			});
			
			egui::Window::new("Results").resizable(false).show(ctx, |ui| {
				let distance = ((self.x2 - self.x1).powi(2) + (self.y2 - self.y1).powi(2)).sqrt();
				let time = distance / self.velocity as f64;
				// let angle = ((self.y2 - self.y1) / (self.x2 - self.x1)).atan();
				let angle = (self.y2 - self.y1).atan2(self.x2 - self.x1).to_degrees();
				
				egui::Grid::new("grid").show(ui, |ui| {
					ui.label("Distance");
					ui.add(egui::Label::new(format!("{:.2}", distance)));
					ui.end_row();
					
					ui.label("Time");
					ui.add(egui::Label::new(format!("{:.2}", time)));
					ui.end_row();
					
					ui.label("Angle");
					ui.add(egui::Label::new(format!("{:.2}", angle + 90.0)));
					ui.end_row();
				});
				
			});

			ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
				source_code_links(ui);
				egui::warn_if_debug_build(ui);
			});
		});
	}

	/// Called by the framework to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}
}

fn source_code_links(ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		ui.spacing_mut().item_spacing.x = 0.0;
		ui.add(egui::github_link_file!(
            "https://github.com/stellar-novas/fire-control/blob/main/",
            "Source code "
        ));
		ui.separator();
		ui.label(" Powered by ");
		ui.hyperlink_to("egui", "https://github.com/emilk/egui");
		ui.label(" and ");
		ui.hyperlink_to(
			"eframe",
			"https://github.com/emilk/egui/tree/master/crates/eframe",
		);
		ui.label(".");
	});
}
