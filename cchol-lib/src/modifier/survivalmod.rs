use crate::social::environment::NativeEnvironment;

pub trait SurvivalMod {
    /// Get general survival mod.
    fn survmod(&self) -> i32;
}

pub trait SurvivalModNatEnv {
    fn survmod_in_natenv(&self, native_env: &NativeEnvironment) -> i32;
}