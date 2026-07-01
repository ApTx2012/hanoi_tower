use eframe::egui;

struct GearTowerGame {
    rods: [Vec<u8>; 3],
    gear_count: u8,
    step_count: u32,
    selected_rod: Option<usize>,
    msg: String,
    win: bool,
}

impl Default for GearTowerGame {
    fn default() -> Self {
        Self {
            rods: [vec![], vec![], vec![]],
            gear_count: 4,
            step_count: 0,
            selected_rod: None,
            msg: "游戏已重置！".to_string(),
            win: false,
        }
    }
}

impl GearTowerGame {
    fn reset(&mut self) {
        self.rods = [vec![], vec![], vec![]];
        for size in (1..=self.gear_count).rev() {
            self.rods[0].push(size);
        }
        self.step_count = 0;
        self.selected_rod = None;
        self.msg = "游戏已重置！".to_string();
        self.win = false;
    }

    fn set_gear_num(&mut self, num: u8) {
        self.gear_count = num;
        self.reset();
    }

    fn move_gear(&mut self, from: usize, to: usize) -> bool {
        if from == to {
            self.msg = "不能移动到同一根柱子".to_string();
            return false;
        }
        let top_gear = match self.rods[from].last() {
            Some(&s) => s,
            None => {
                self.msg = "选中柱子没有可移动齿轮！".to_string();
                return false;
            }
        };
        if let Some(&target_top) = self.rods[to].last() {
            if top_gear > target_top {
                self.msg = format!("禁止：大齿轮{}不能放在小齿轮{}上方", top_gear, target_top);
                return false;
            }
        }
        let gear = self.rods[from].pop().unwrap();
        self.rods[to].push(gear);
        self.step_count += 1;
        self.msg = format!("移动齿轮{}成功，总步数：{}", gear, self.step_count);
        if self.rods[2].len() == self.gear_count as usize {
            self.win = true;
            self.msg = format!("🎉 恭喜通关！总步数：{}", self.step_count);
        }
        true
    }

    fn select_rod(&mut self, idx: usize) {
        if self.rods[idx].is_empty() {
            self.msg = format!("{}柱没有齿轮，无法选中", match idx {0=>"A",1=>"B",2=>"C",_=>""});
            return;
        }
        self.selected_rod = Some(idx);
        self.msg = format!("已选中{}柱，仅可移动最上方最小齿轮", match idx {0=>"A",1=>"B",2=>"C",_=>""});
    }

    fn target_move(&mut self, target_idx: usize) {
        let Some(from_idx) = self.selected_rod else {
            self.msg = "请先点击柱子下方【选中此柱】！".to_string();
            return;
        };
        self.move_gear(from_idx, target_idx);
        self.selected_rod = None;
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1400.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "齿轮堆栈塔",
        native_options,
        Box::new(|creation_ctx| {
            let mut fonts = egui::FontDefinitions::default();
            if let Ok(font_data) = std::fs::read(r"C:\Windows\Fonts\msyh.ttc") {
                fonts.font_data.insert("msyh".into(), egui::FontData::from_owned(font_data));
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "msyh".into());
                fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, "msyh".into());
            }
            creation_ctx.egui_ctx.set_fonts(fonts);
            Box::new(GearTowerGame::default())
        }),
    )
}

impl eframe::App for GearTowerGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("⚙️ 齿轮堆栈塔");
            ui.horizontal(|ui| {
                ui.label("齿轮数量：");
                let mut num = self.gear_count as f32;
                ui.add(egui::Slider::new(&mut num, 1.0..=10.0).integer());
                if num != self.gear_count as f32 {
                    self.set_gear_num(num as u8);
                }
                if ui.button("重置游戏").clicked() {
                    self.reset();
                }
            });
            ui.label(format!("当前总步数：{}", self.step_count));
            let text_color = if self.win { egui::Color32::GREEN } else { egui::Color32::WHITE };
            ui.label(egui::RichText::new(&self.msg).color(text_color));
            ui.separator();

            ui.columns(3, |cols| {
                for rod_idx in 0..3 {
                    let col = &mut cols[rod_idx];
                    if self.selected_rod == Some(rod_idx) {
                        col.style_mut().visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(3.0, egui::Color32::LIGHT_BLUE);
                    }
                    egui::Frame::none()
                        .inner_margin(egui::vec2(10.0, 10.0))
                        .show(col, |ui| {
                            let gears = &self.rods[rod_idx];
                            let max_total = self.gear_count as usize;
                            let exist_cnt = gears.len();
                            let blank_lines = max_total - exist_cnt;
                            for _ in 0..blank_lines {
                                ui.add_sized(egui::vec2(420.0, 30.0), egui::Label::new(""));
                            }

                            for gear_size in gears.iter().rev() {
                                let base_width = 100.0;
                                let width_step = 42.0;
                                let gear_width = base_width + (*gear_size as f32 - 1.0) * width_step;

                                let color = match gear_size {
                                    1 => egui::Color32::from_rgb(120, 190, 210),
                                    2 => egui::Color32::from_rgb(140, 190, 120),
                                    3 => egui::Color32::from_rgb(160, 140, 220),
                                    4 => egui::Color32::from_rgb(240, 180, 80),
                                    5 => egui::Color32::from_rgb(240, 140, 60),
                                    6 => egui::Color32::from_rgb(220, 100, 40),
                                    7 => egui::Color32::from_rgb(200, 60, 30),
                                    8 => egui::Color32::from_rgb(180, 40, 20),
                                    9 => egui::Color32::from_rgb(140, 30, 10),
                                    _ => egui::Color32::from_rgb(100, 20, 0),
                                };
                                ui.horizontal(|h| {
                                    h.add_space((420.0 - gear_width) / 2.0);
                                    h.add_sized(
                                        egui::vec2(gear_width, 30.0),
                                        egui::Button::new(format!("齿轮{}", gear_size)).fill(color)
                                    );
                                });
                            }

                            ui.label(match rod_idx {0 => "【A柱】", 1 => "【B柱】", 2 => "【C柱】", _ => ""});
                            ui.separator();
                            if ui.button("选中此柱").clicked() {
                                self.select_rod(rod_idx);
                            }
                            if ui.button("移动到这里").clicked() {
                                self.target_move(rod_idx);
                            }
                        });
                }
            });

            ui.separator();
            ui.label("游戏规则（与截图一致）：");
            ui.label("1. 堆叠样式：柱子底部是大齿轮，越往上齿轮尺寸越小");
            ui.label("2. 移动限制：一次只能移动柱子最顶端最小的齿轮");
            ui.label("3. 放置限制：只能把小齿轮放在大齿轮上方，不能大齿轮压小齿轮");
            ui.label("4. 通关目标：将A柱所有齿轮完整移动到C柱");

            if self.win {
                egui::Window::new("🎉 通关成功！").show(ctx, |ui| {
                    ui.heading(format!("总移动步数：{}", self.step_count));
                    if ui.button("重新开始游戏").clicked() {
                        self.reset();
                    }
                });
            }
        });
    }
}