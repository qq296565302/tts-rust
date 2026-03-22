use crate::config::Config;
use crate::player::AudioPlayer;
use crate::tts::{get_available_voices, generate_output_filename, generate_output_filename_with_index, TtsClient};
use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

enum TaskResult {
    Success(PathBuf),
    Error(String),
}

pub struct TtsApp {
    config: Config,
    text_input: String,
    batch_input: String,
    selected_voice: String,
    status_message: String,
    status_success: bool,
    is_processing: bool,
    pending_tasks: usize,
    generated_files: Vec<PathBuf>,
    selected_file: Option<usize>,
    player: AudioPlayer,
    
    show_config: bool,
    batch_mode: bool,
    
    api_key_input: String,
    api_base_input: String,
    model_input: String,
    output_dir_input: String,
    
    result_sender: Sender<TaskResult>,
    result_receiver: Receiver<TaskResult>,
}

impl TtsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::load();
        let _ = config.ensure_output_dir();
        
        let (sender, receiver) = mpsc::channel();
        
        let api_key_input = config.api_key.clone();
        let api_base_input = config.api_base.clone();
        let model_input = config.model.clone();
        let output_dir_input = config.output_dir.to_string_lossy().to_string();
        
        let valid_voices = get_available_voices();
        let selected_voice = if valid_voices.contains(&config.default_voice.as_str()) {
            config.default_voice.clone()
        } else {
            "default_zh".to_string()
        };
        
        Self {
            selected_voice,
            config,
            text_input: String::new(),
            batch_input: String::new(),
            status_message: String::new(),
            status_success: true,
            is_processing: false,
            pending_tasks: 0,
            generated_files: Vec::new(),
            selected_file: None,
            player: AudioPlayer::new(),
            show_config: false,
            batch_mode: false,
            api_key_input,
            api_base_input,
            model_input,
            output_dir_input,
            result_sender: sender,
            result_receiver: receiver,
        }
    }

    fn save_config(&mut self) {
        self.config.api_key = self.api_key_input.clone();
        self.config.api_base = self.api_base_input.clone();
        self.config.model = self.model_input.clone();
        self.config.output_dir = PathBuf::from(&self.output_dir_input);
        self.config.default_voice = self.selected_voice.clone();
        
        if let Err(e) = self.config.save() {
            self.status_message = format!("保存配置失败: {}", e);
            self.status_success = false;
        } else {
            self.status_message = "配置已保存".to_string();
            self.status_success = true;
        }
    }

    fn start_synthesis(&mut self, text: String, index: Option<usize>) {
        if text.is_empty() {
            self.status_message = "请输入要转换的文字".to_string();
            self.status_success = false;
            return;
        }
        
        if self.api_key_input.is_empty() {
            self.status_message = "请先配置API密钥".to_string();
            self.status_success = false;
            self.show_config = true;
            return;
        }
        
        self.is_processing = true;
        self.pending_tasks += 1;
        self.status_message = format!("正在生成语音... ({} 个任务进行中)", self.pending_tasks);
        self.status_success = true;
        
        let output_path = match index {
            Some(i) => self.config.output_dir.join(generate_output_filename_with_index("wav", i)),
            None => self.config.output_dir.join(generate_output_filename("wav")),
        };
        
        let api_key = self.api_key_input.clone();
        let api_base = self.api_base_input.clone();
        let model = self.model_input.clone();
        let voice = self.selected_voice.clone();
        let sender = self.result_sender.clone();
        
        std::thread::spawn(move || {
            let client = TtsClient::new(api_key, api_base, model);
            let result = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(client.synthesize_to_file(&text, &voice, &output_path));
            
            let msg = match result {
                Ok(_) => TaskResult::Success(output_path),
                Err(e) => TaskResult::Error(e.to_string()),
            };
            
            let _ = sender.send(msg);
        });
    }

    fn check_task_result(&mut self) {
        if let Ok(result) = self.result_receiver.try_recv() {
            self.pending_tasks = self.pending_tasks.saturating_sub(1);
            
            if self.pending_tasks == 0 {
                self.is_processing = false;
            }
            
            match result {
                TaskResult::Success(path) => {
                    self.generated_files.push(path);
                    if self.pending_tasks == 0 {
                        self.status_message = "语音生成成功！".to_string();
                        self.status_success = true;
                    } else {
                        self.status_message = format!("正在生成语音... ({} 个任务进行中)", self.pending_tasks);
                    }
                }
                TaskResult::Error(e) => {
                    self.status_message = format!("生成失败: {}", e);
                    self.status_success = false;
                }
            }
        }
    }

    fn play_selected(&mut self) {
        if let Some(idx) = self.selected_file {
            if let Some(path) = self.generated_files.get(idx) {
                match self.player.play(path) {
                    Ok(()) => {
                        self.status_message = format!("正在播放: {}", path.file_name().unwrap_or_default().to_string_lossy());
                        self.status_success = true;
                    }
                    Err(e) => {
                        self.status_message = format!("播放失败: {}", e);
                        self.status_success = false;
                    }
                }
            }
        } else {
            self.status_message = "请先选择一个文件".to_string();
            self.status_success = false;
        }
    }

    fn open_output_folder(&self) {
        let _ = open::that(&self.config.output_dir);
    }
}

mod style {
    pub const PRIMARY: egui::Color32 = egui::Color32::from_rgb(64, 158, 255);
    pub const SUCCESS: egui::Color32 = egui::Color32::from_rgb(103, 194, 58);
    pub const WARNING: egui::Color32 = egui::Color32::from_rgb(230, 162, 60);
    pub const DANGER: egui::Color32 = egui::Color32::from_rgb(245, 108, 108);
    
    pub const BG_COLOR: egui::Color32 = egui::Color32::from_rgb(245, 247, 250);
    pub const CARD_BG: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);
    pub const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 223, 230);
    pub const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(48, 49, 51);
    pub const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(144, 147, 153);
    pub const TEXT_PLACEHOLDER: egui::Color32 = egui::Color32::from_rgb(192, 196, 204);
}

fn el_card_full_width(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(style::CARD_BG)
        .rounding(4.0)
        .inner_margin(egui::vec2(20.0, 18.0))
        .outer_margin(egui::Margin::symmetric(0.0, 4.0))
        .stroke(egui::Stroke::new(1.0, style::BORDER_COLOR))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add_contents(ui);
        });
}

fn el_button_primary(text: &str) -> egui::Button<'static> {
    egui::Button::new(
        egui::RichText::new(text).color(egui::Color32::WHITE).size(14.0)
    )
    .fill(style::PRIMARY)
    .min_size(egui::vec2(80.0, 32.0))
    .rounding(4.0)
}

fn el_button_success(text: &str) -> egui::Button<'static> {
    egui::Button::new(
        egui::RichText::new(text).color(egui::Color32::WHITE).size(14.0)
    )
    .fill(style::SUCCESS)
    .min_size(egui::vec2(80.0, 32.0))
    .rounding(4.0)
}

fn el_button_danger(text: &str) -> egui::Button<'static> {
    egui::Button::new(
        egui::RichText::new(text).color(egui::Color32::WHITE).size(14.0)
    )
    .fill(style::DANGER)
    .min_size(egui::vec2(80.0, 32.0))
    .rounding(4.0)
}

fn el_button_default(text: &str) -> egui::Button<'static> {
    egui::Button::new(
        egui::RichText::new(text).color(style::TEXT_PRIMARY).size(14.0)
    )
    .fill(style::CARD_BG)
    .stroke(egui::Stroke::new(1.0, style::BORDER_COLOR))
    .min_size(egui::vec2(80.0, 32.0))
    .rounding(4.0)
}

fn el_section_title(ui: &mut egui::Ui, text: &str) {
    ui.horizontal(|ui| {
        ui.add_space(2.0);
        ui.label(egui::RichText::new(text).size(16.0).color(style::TEXT_PRIMARY).strong());
    });
    ui.add_space(12.0);
}

fn el_form_label(ui: &mut egui::Ui, text: &str) {
    ui.label(egui::RichText::new(text).size(14.0).color(style::TEXT_PRIMARY));
}

fn el_text_edit_singleline(value: &mut String, width: f32) -> egui::TextEdit<'_> {
    egui::TextEdit::singleline(value)
        .desired_width(width)
        .min_size(egui::vec2(0.0, 32.0))
}

fn el_text_edit_multiline(value: &mut String, rows: usize) -> egui::TextEdit<'_> {
    egui::TextEdit::multiline(value)
        .desired_width(f32::INFINITY)
        .desired_rows(rows)
        .min_size(egui::vec2(0.0, 100.0))
        .margin(egui::vec2(12.0, 12.0))
}

fn el_select<'a>(
    ui: &mut egui::Ui, 
    selected: &str, 
    options: &[&'static str],
    width: f32,
) -> Option<String> {
    let mut result = None;
    
    let button = egui::Button::new(
        egui::RichText::new(format!("  {}  ", selected))
            .size(14.0)
            .color(style::TEXT_PRIMARY)
    )
    .fill(style::CARD_BG)
    .stroke(egui::Stroke::new(1.0, style::BORDER_COLOR))
    .min_size(egui::vec2(width, 36.0))
    .rounding(4.0);
    
    let menu_id = egui::Id::new("select_menu");
    
    let response = ui.add(button);
    
    if response.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(menu_id));
    }
    
    egui::popup::popup_below_widget(ui, menu_id, &response, |ui| {
        ui.set_min_width(width + 20.0);
        for option in options {
            let is_selected = *option == selected;
            let text = egui::RichText::new(format!("  {}", option))
                .size(14.0)
                .color(if is_selected { egui::Color32::WHITE } else { style::TEXT_PRIMARY });
            
            let mut btn = egui::Button::new(text)
                .fill(if is_selected { style::PRIMARY } else { egui::Color32::TRANSPARENT })
                .min_size(egui::vec2(width, 36.0))
                .rounding(4.0);
            
            if !is_selected {
                btn = btn.stroke(egui::Stroke::NONE);
            }
            
            if ui.add(btn).clicked() {
                result = Some(option.to_string());
                ui.memory_mut(|mem| mem.toggle_popup(menu_id));
            }
        }
    });
    
    result
}

impl eframe::App for TtsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_task_result();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().widgets.noninteractive.bg_fill = style::BG_COLOR;
            ui.visuals_mut().window_fill = style::BG_COLOR;
            ui.visuals_mut().panel_fill = style::BG_COLOR;
            
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            
            egui::TopBottomPanel::top("header")
                .frame(egui::Frame::none()
                    .fill(style::CARD_BG)
                    .inner_margin(egui::vec2(20.0, 15.0))
                    .stroke(egui::Stroke::new(1.0, style::BORDER_COLOR)))
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("TTS 语音生成工具").size(20.0).color(style::TEXT_PRIMARY).strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add(el_button_default("设置")).clicked() {
                                self.show_config = !self.show_config;
                            }
                            if ui.add(el_button_default("打开目录")).clicked() {
                                self.open_output_folder();
                            }
                            let (_batch_text, batch_btn) = if self.batch_mode {
                                ("批量模式", el_button_primary("批量模式"))
                            } else {
                                ("批量模式", el_button_default("批量模式"))
                            };
                            if ui.add(batch_btn).clicked() {
                                self.batch_mode = !self.batch_mode;
                            }
                        });
                    });
                });
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(10.0);
                ui.set_min_width(ui.available_width());
                
                if self.show_config {
                    el_card_full_width(ui, |ui| {
                        el_section_title(ui, "设置");
                        
                        egui::Grid::new("config_grid")
                            .num_columns(2)
                            .spacing([16.0, 16.0])
                            .show(ui, |ui| {
                                el_form_label(ui, "API 密钥:");
                                ui.add(el_text_edit_singleline(&mut self.api_key_input, 400.0).password(true).hint_text("请输入API密钥"));
                                ui.end_row();
                                
                                el_form_label(ui, "API 端点:");
                                ui.add(el_text_edit_singleline(&mut self.api_base_input, 400.0).hint_text("https://api.xiaomimimo.com/v1"));
                                ui.end_row();
                                
                                el_form_label(ui, "模型名称:");
                                ui.add(el_text_edit_singleline(&mut self.model_input, 400.0).hint_text("mimo-v2-tts"));
                                ui.end_row();
                                
                                el_form_label(ui, "输出目录:");
                                ui.horizontal(|ui| {
                                    ui.add(el_text_edit_singleline(&mut self.output_dir_input, 320.0));
                                    if ui.add(el_button_default("浏览")).clicked() {
                                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                            self.output_dir_input = path.to_string_lossy().to_string();
                                        }
                                    }
                                });
                                ui.end_row();
                                
                                el_form_label(ui, "音色:");
                                let voices = get_available_voices();
                                if let Some(new_voice) = el_select(ui, &self.selected_voice, &voices, 200.0) {
                                    self.selected_voice = new_voice;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        if ui.add(el_button_primary("保存配置")).clicked() {
                            self.save_config();
                        }
                    });
                }
                
                el_card_full_width(ui, |ui| {
                    let label_text = if self.batch_mode { "批量输入（每行一段文字）" } else { "输入文字" };
                    el_section_title(ui, label_text);
                    
                    let text_edit = if self.batch_mode {
                        el_text_edit_multiline(&mut self.batch_input, 5)
                    } else {
                        el_text_edit_multiline(&mut self.text_input, 3)
                    };
                    ui.add(text_edit);
                    
                    ui.add_space(16.0);
                    
                    ui.horizontal(|ui| {
                        let button_text = if self.batch_mode { "批量生成" } else { "生成语音" };
                        
                        let button = ui.add_enabled(
                            !self.is_processing, 
                            el_button_primary(button_text)
                        );
                        
                        if button.clicked() {
                            if self.batch_mode {
                                let lines: Vec<String> = self.batch_input
                                    .lines()
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                                
                                if lines.is_empty() {
                                    self.status_message = "请输入要转换的文字".to_string();
                                    self.status_success = false;
                                } else {
                                    for (i, line) in lines.into_iter().enumerate() {
                                        self.start_synthesis(line, Some(i));
                                    }
                                }
                            } else {
                                self.start_synthesis(self.text_input.clone(), None);
                            }
                        }
                        
                        if self.is_processing {
                            ui.add_space(10.0);
                            ui.spinner();
                            ui.label(egui::RichText::new("处理中...").color(style::WARNING).size(14.0));
                        }
                    });
                });
                
                if !self.status_message.is_empty() {
                    let (bg_color, text_color, icon) = if self.status_success {
                        (egui::Color32::from_rgb(240, 249, 235), egui::Color32::from_rgb(103, 194, 58), "✅")
                    } else {
                        (egui::Color32::from_rgb(254, 240, 240), egui::Color32::from_rgb(245, 108, 108), "❌")
                    };
                    
                    egui::Frame::none()
                        .fill(bg_color)
                        .rounding(4.0)
                        .inner_margin(egui::vec2(15.0, 12.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(icon).color(text_color).size(14.0));
                                ui.label(egui::RichText::new(&self.status_message).color(text_color).size(14.0));
                            });
                        });
                    ui.add_space(10.0);
                }
                
                el_card_full_width(ui, |ui| {
                    ui.horizontal(|ui| {
                        el_section_title(ui, "已生成文件");
                        ui.label(egui::RichText::new(format!("({} 个)", self.generated_files.len())).color(style::TEXT_SECONDARY).size(14.0));
                    });
                    
                    egui::ScrollArea::vertical()
                        .max_height(180.0)
                        .show(ui, |ui| {
                            if self.generated_files.is_empty() {
                                ui.add_space(20.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(10.0);
                                    ui.label(egui::RichText::new("暂无文件").color(style::TEXT_PLACEHOLDER).size(14.0));
                                });
                            } else {
                                for (i, path) in self.generated_files.iter().enumerate() {
                                    let is_selected = self.selected_file == Some(i);
                                    let file_name = path.file_name()
                                        .map(|n| n.to_string_lossy().to_string())
                                        .unwrap_or_default();
                                    
                                    let bg_color = if is_selected {
                                        egui::Color32::from_rgb(236, 245, 255)
                                    } else {
                                        style::CARD_BG
                                    };
                                    
                                    let stroke_color = if is_selected { style::PRIMARY } else { style::BORDER_COLOR };
                                    
                                    let btn = egui::Button::new(
                                        egui::RichText::new(format!("🔊  {}", file_name))
                                            .color(style::TEXT_PRIMARY)
                                            .size(14.0)
                                    )
                                    .fill(bg_color)
                                    .stroke(egui::Stroke::new(1.0, stroke_color))
                                    .min_size(egui::vec2(ui.available_width(), 36.0))
                                    .rounding(4.0);
                                    
                                    if ui.add(btn).clicked() {
                                        self.selected_file = Some(i);
                                    }
                                }
                            }
                        });
                    
                    ui.add_space(16.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(el_button_success("播放")).clicked() {
                            self.play_selected();
                        }
                        if ui.add(el_button_danger("停止")).clicked() {
                            self.player.stop();
                            self.status_message.clear();
                        }
                    });
                });
                
                ui.add_space(20.0);
            });
        });
        
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
