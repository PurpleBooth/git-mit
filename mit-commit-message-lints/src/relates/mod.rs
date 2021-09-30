pub use cmd::errors::Error as VcsError;

mod cmd;
mod lib;

pub use self::{
    cmd::{
        get_relate_to_configuration::get_relate_to_configuration,
        set_relates_to::set_relates_to,
    },
    lib::relates_to::RelateTo,
};
