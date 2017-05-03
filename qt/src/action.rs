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

    pub fn new_setvalue(panopticon: &mut QPanopticon,func: Uuid, variable: String, value: String) -> Result<Action> {
        Ok(Self::new(func,ActionPayload::SetValue{
            variable: variable.clone(),
            before: panopticon.control_flow_values.get(&func).cloned().unwrap_or("".to_string()),
            after: value,
        }))
    }

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
                debug_assert!(panopticon.control_flow_values.get(&self.function).unwrap_or(&"".to_string()) == after);
                if before == "" {
                    panopticon.control_flow_values.remove(&self.function);
                } else {
                    panopticon.control_flow_values.insert(self.function.clone(),before.clone());
                }
                panopticon.update_basic_block(0,&self.function)
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
                debug_assert!(panopticon.control_flow_values.get(&self.function).unwrap_or(&"".to_string()) == before);
                if after == "" {
                    panopticon.control_flow_values.remove(&self.function);
                } else {
                    panopticon.control_flow_values.insert(self.function.clone(),after.clone());
                }
                panopticon.update_basic_block(0,&self.function)
            }
        }
    }
}
