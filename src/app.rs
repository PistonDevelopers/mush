use imgui::{Ui,ImGuiSetCond_Once};
use lichen::parse::Env;

pub struct AppState {
    pub exit: bool,
    pub open_file: bool,
    pub env: Option<Env>,
}
impl Default for AppState {
    fn default() -> Self {
        AppState {
            exit: false,
            open_file: true,
            env: None,
        }
    }
}

impl AppState {
    pub fn render (&mut self, ui: &Ui) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"))
                .build(|| {
                    ui.menu_item(im_str!("Open"))
                        .selected(&mut self.open_file)
                        .build();
                    
                    ui.menu_item(im_str!("Exit"))
                        .selected(&mut self.exit)
                        .build();
                });
        });

        if let Some(ref env) = self.env {
            ui.window(im_str!("Environment Parsed"))
                .build(||{
                    ui.tree_node(im_str!("Def Blocks"))
                        .opened(true, ImGuiSetCond_Once)
                        .build(|| {
                            for k in env.def.keys() {
                                if ui.collapsing_header(im_str!("{:?}",k))
                                    .build() {
                                        let block = env.def.get(k).unwrap();
                                        for (k,v) in block.def.iter() {
                                            let info = im_str!("{:}: {:?}",k,v);
                                            if ui.small_button(info) {
                                                
                                            }
                                        }
                                    }
                            }
                        });
                    
                    
                    ui.tree_node(im_str!("Src Blocks"))
                        .opened(true, ImGuiSetCond_Once)
                        .build(|| {
                            for k in env.src.keys() {
                                if ui.collapsing_header(im_str!("{:?}",k))
                                    .build() {
                                        let block = env.src.get(k).unwrap();
                                        for src in block.src.iter() {
                                            let info = im_str!("{:?}",src);
                                            if ui.small_button(info) {
                                                
                                            }
                                        }
                                    }
                            }
                        });
                });
        
        }

    }
}
