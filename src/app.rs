use imgui::{Ui,ImStr, ImString, ImGuiSetCond_Once,ImGuiSelectableFlags,ImVec2};
use lichen::parse::{Env,DefBlock};
use lichen::var::Var;

pub struct AppState {
    pub exit: bool,
    pub open_file: bool,
    pub env: Option<Env>,

    pub edit_block: Option<(String,String)>,
}
impl Default for AppState {
    fn default() -> Self {
        AppState {
            exit: false,
            open_file: true,
            env: None,

            edit_block: None,
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

        let mut edit_block = None;
        if let Some(ref env) = self.env {
            ui.window(im_str!("Environment Parsed"))
                .build(||{
                    ui.tree_node(im_str!("Def Blocks"))
                        .opened(true, ImGuiSetCond_Once)
                        .build(|| {
                            for d in env.def.keys() {
                                if ui.collapsing_header(im_str!("{:?}",d))
                                    .build() {
                                        let block = env.def.get(d).unwrap();
                                        for (k,v) in block.def.iter() {
                                            let info = im_str!("{:}: {:?}",k,v);
                                            if ui_simple_select(info,ui) {
                                                edit_block = Some((d.to_owned(),
                                                                        k.to_owned()));
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
                                            if ui_simple_select(info,ui) {
                                            }
                                        }
                                    }
                            }
                        });
                });
        
        }

        if edit_block.is_some() {
            self.edit_block = edit_block;
        }

        self.edit_block(ui);
    }

    pub fn edit_block(&mut self, ui: &Ui) {
        if let Some((ref block,ref key)) = self.edit_block.clone() {
            ui.window(im_str!("Def Block {:}", block))
                .build(|| {
                    ui.text(im_str!("Var Name: {:}",key));
                    if let Some(ref mut env) = self.env {
                        
                        if let Some(mut var) = 
                            env.def.get_mut(block).expect("ERROR: Block missing")
                            .def.get_mut(key) {
                                
                                let var_;
                                match var {
                                    &mut Var::Num(ref n) => {
                                        let mut num = n.clone();
                                        ui.input_float(im_str!("Value: {:?}",var), &mut num)
                                            .step(0.125)
                                            .build();
                                        var_ = Var::Num(num);
                                    },
                                    &mut Var::String(ref s) => {
                                        let mut text = ImString::new(s.clone()).unwrap();
                                        text.reserve(1024); // NOTE: reserve arbitrary size
                                        // actual max size might not be this!
                                        
                                        ui.input_text(im_str!("Value: {:?}",var), &mut text)
                                            .build();
                                        var_ = Var::String(text.to_string());
                                    },
                                    &mut Var::Bool(ref b) => {
                                        let mut b = b.clone();
                                        ui.checkbox(im_str!("Value: {:?}",var), &mut b);
                                        var_ = Var::Bool(b);
                                    },
                                    _ => {
                                        ui.text(im_str!("Value: {:?}",var));
                                        var_ = var.clone();
                                    }
                                }

                                *var = var_;
                            }
                    }
                })
        }
    }
}

fn ui_simple_select(info: &ImStr, ui: &Ui) -> bool {
    ui.selectable(info,
                  false,
                  ImGuiSelectableFlags::empty(),
                  ImVec2::new(0.0, 0.0))
}
