use std::{cell::RefCell, rc::Rc};

use nu_protocol::{
    engine::{EngineState, StateWorkingSet},
    Signature,
};

use crate::*;

pub fn create_default_context() -> Rc<RefCell<EngineState>> {
    let engine_state = Rc::new(RefCell::new(EngineState::new()));
    let delta = {
        let engine_state = engine_state.borrow();
        let mut working_set = StateWorkingSet::new(&*engine_state);

        macro_rules! bind_command {
            ($command:expr) => {
                working_set.add_decl(Box::new($command));
            };
        }

        bind_command!(Alias);
        bind_command!(Benchmark);
        bind_command!(BuildString);
        bind_command!(Cd);
        bind_command!(Cp);
        bind_command!(Def);
        bind_command!(Do);
        bind_command!(Each);
        bind_command!(ExportDef);
        bind_command!(External);
        bind_command!(For);
        bind_command!(From);
        bind_command!(FromJson);
        bind_command!(Get);
        bind_command!(Help);
        bind_command!(Hide);
        bind_command!(If);
        bind_command!(Length);
        bind_command!(Let);
        bind_command!(LetEnv);
        bind_command!(Lines);
        bind_command!(Ls);
        bind_command!(Mkdir);
        bind_command!(Module);
        bind_command!(Mv);
        bind_command!(Open);
        bind_command!(Ps);
        bind_command!(Rm);
        bind_command!(Select);
        bind_command!(Sys);
        bind_command!(Table);
        bind_command!(Touch);
        bind_command!(Use);
        bind_command!(Where);
        bind_command!(Wrap);

        // This is a WIP proof of concept
        bind_command!(ListGitBranches);
        bind_command!(Git);
        bind_command!(GitCheckout);

        bind_command!(Source);

        let sig = Signature::build("exit");
        working_set.add_decl(sig.predeclare());
        let sig = Signature::build("vars");
        working_set.add_decl(sig.predeclare());
        let sig = Signature::build("decls");
        working_set.add_decl(sig.predeclare());
        let sig = Signature::build("blocks");
        working_set.add_decl(sig.predeclare());
        let sig = Signature::build("stack");
        working_set.add_decl(sig.predeclare());
        let sig = Signature::build("contents");
        working_set.add_decl(sig.predeclare());

        working_set.render()
    };

    {
        EngineState::merge_delta(&mut *engine_state.borrow_mut(), delta);
    }

    engine_state
}
