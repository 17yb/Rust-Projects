use eframe::egui;
use rand::prelude::*;

fn parse_list(input: &str) -> Vec<String> {
    input.trim()
    .split(',')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect()
}

fn generate_words(syllable_structures: &[String], consonants: &[String], vowels: &[String], finals: &[String], word_count: u32, rng: &mut impl Rng) -> String {
    let mut word = String::new();

    for _ in 0..word_count {
        let syllables = rng.random_range(2..5);

        for _ in 0..syllables {
            let syllable = make_syllable(syllable_structures, consonants, vowels, finals, rng);
            word.push_str(&syllable);
        }
        word.push_str("\n");
    }
    word
}

fn make_syllable(syllable_structures: &[String], consonants: &[String], vowels: &[String], finals: &[String], rng: &mut impl Rng) -> String {
    let mut syllable_structure = String::new();
    let mut text = String::new();

    syllable_structure.push_str(syllable_structures.choose(rng).expect("Syllable structures was empty"));

    for character in syllable_structure.chars() {
        if character == 'C' {
            text.push_str(consonants.choose(rng).expect("Failed to read syllable"));
        } else if character == 'V' {
            text.push_str(vowels.choose(rng).expect("Failed to read syllable"));
        } else if character == 'F' {
            if let Some(f) = finals.choose(rng) {
                text.push_str(f);
            }
        }
    }
    text
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Word Generator", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Default)]
struct MyEguiApp {
    consonants: String,
    vowels: String,
    finals: String,
    syllable_structures: String,
    word_count: String,
    output: String
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Consonants (Comma seperated):");
            ui.text_edit_singleline(&mut self.consonants);
            ui.label("Vowels (Comma seperated):");
            ui.text_edit_singleline(&mut self.vowels);
            ui.label("Finals (Comma seperated):");
            ui.text_edit_singleline(&mut self.finals);
            ui.label("Syllable Structures (Comma seperated):");
            ui.text_edit_singleline(&mut self.syllable_structures);
            ui.label("Word Count:");
            ui.text_edit_singleline(&mut self.word_count);
            if ui.button("Generate").clicked() {
                match self.word_count.trim().parse::<u32>() {
                    Ok(n) => {
                        let consonants = parse_list(&self.consonants);
                        let vowels = parse_list(&self.vowels);
                        let finals = parse_list(&self.finals);
                        let syllable_structures = parse_list(&self.syllable_structures);

                        if consonants.is_empty() || vowels.is_empty() || syllable_structures.is_empty() {
                            self.output = String::from("Please input consonants, vowels, and syllable structures")
                        } else {
                            let mut rng = rand::rng();
                            self.output = generate_words(&syllable_structures, &consonants, &vowels, &finals, n, &mut rng);
                        }
                    }
                    Err(_) => {self.output = String::from("Please enter a whole number");}
                }
            }
            ui.label(&self.output);
       });
   }
}