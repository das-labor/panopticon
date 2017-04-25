use uuid::Uuid;
use errors::*;
use {
    Panopticon,
    QPanopticon,
};
use rustc_serialize::json;


#[derive(Clone)]
enum ActionPayload {
    Comment {
        address: u64,
        before: String,
        after: String,
    },
    Rename {
        before: String,
        after: String,
    },
    SetValue {
        variable: String,
        before: String,
        after: String,
    }
}

#[derive(Clone)]
pub struct Action {
    function: Uuid,
    payload: ActionPayload,
}

impl Action {
    fn new(f: Uuid,p: ActionPayload) -> Action {
        Action{
            function: f,
            payload: p,
        }
    }

    pub fn new_comment(panopticon: &mut QPanopticon,func: Uuid, address: u64, comment: String) -> Result<Action> {
        Ok(Self::new(func,ActionPayload::Comment{
            address: address,
            before: panopticon.control_flow_comments.get(&address).cloned().unwrap_or("".to_string()),
            after: comment,
        }))
    }

    pub fn new_rename(panopticon: &mut QPanopticon,func: Uuid, name: String) -> Result<Action> {
        Ok(Self::new(func,ActionPayload::Rename{
            before: panopticon.functions.get(&func).map(|f| f.name.clone()).unwrap_or("".to_string()),
            after: name,
        }))
    }


/*    pub fn new_setvalue(panopticon: &mut Panopticon,func: Uuid, variable: String, value: String) -> Result<Action> {
        Ok(Self::new(func,ActionPayload::SetValue{
            variable: variable.clone(),
            before: panopticon.control_flow_values.get(&(func.to_string(),variable)).cloned().unwrap_or("".to_string()),
            after: value,
        }))
    }*/


    pub fn undo(&self,panopticon: &mut QPanopticon) -> Result<()> {
        match self.payload {
            ActionPayload::Comment { address, ref before, ref after } => {
                debug_assert!(panopticon.control_flow_comments.get(&address).unwrap_or(&"".to_string()) == after);
                panopticon.control_flow_comments.insert(address,before.clone());
                panopticon.update_basic_block(address,&self.function)
            },
            ActionPayload::Rename{ ref before,.. } => {
                if let Some(func) = panopticon.functions.get_mut(&self.function) {
                    func.name = before.clone();
                }

                panopticon.update_sidebar(&self.function)

            },
            ActionPayload::SetValue{ ref variable, ref before, ref after } => {
                /*let key = (self.function.to_string(),variable.clone());
                debug_assert!(panopticon.control_flow_values.get(&key).unwrap_or(&"".to_string()) == after);

                if before == "" {
                    panopticon.control_flow_values.remove(&key);
                } else {
                    panopticon.control_flow_values.insert(key,before.clone());
                }*/
                Self::update_setvalue(panopticon,variable,before)
            }
        }
    }

    pub fn redo(&self,panopticon: &mut QPanopticon) -> Result<()> {
        match self.payload {
            ActionPayload::Comment { address, ref before, ref after } => {
                debug_assert!(panopticon.control_flow_comments.get(&address).unwrap_or(&"".to_string()) == before);
                panopticon.control_flow_comments.insert(address,after.clone());
                panopticon.update_basic_block(address,&self.function)
            },
            ActionPayload::Rename{ ref after,.. } => {
                if let Some(func) = panopticon.functions.get_mut(&self.function) {
                    func.name = after.clone();
                }

                panopticon.update_sidebar(&self.function)
            },
            ActionPayload::SetValue{ ref variable, ref before, ref after } => {
                /*let key = (self.function.to_string(),variable.clone());
                debug_assert!(panopticon.control_flow_values.get(&key).unwrap_or(&"".to_string()) == before);

                if after == "" {
                    panopticon.control_flow_values.remove(&key);
                } else {
                    panopticon.control_flow_values.insert(key,after.clone());
                }*/
                Self::update_setvalue(panopticon,variable,after)
            }
        }
    }

    fn update_setvalue(panopticon: &mut Panopticon,variable: &str, value: &str) -> Result<()> {
        /*let cnt = panopticon.control_flow_nodes.view_data().len();
        for idx in 0..cnt {
            let mut tpl = panopticon.control_flow_nodes.view_data()[idx].clone();
            let mut contents = json::decode::<Vec<CfgMnemonic>>(&FUNCTION.1[tpl.2 as usize].2).unwrap();
            let mut modified = false;

            for mne in contents.iter_mut() {
                for arg in mne.args.iter_mut() {
                    if arg.kind == "variable" && arg.data == variable {
                        if value != "" { arg.display = value.to_string(); }
                        modified = true;
                    }
                }
            }

            if modified {
                panopticon.control_flow_nodes.change_line(idx,tpl.0,tpl.1,tpl.2,tpl.3,json::encode(&contents).unwrap());
            }
        }*/

        Ok(())
    }
}
