use crate::{Action, Expr, Resource};
use ockam_core::async_trait;
use ockam_core::compat::boxed::Box;
use ockam_core::compat::vec::Vec;
use ockam_core::Result;

#[async_trait]
pub trait PolicyStorage: Send + Sync + 'static {
    async fn get_policy(&self, r: &Resource, a: &Action) -> Result<Option<Expr>>;
    async fn set_policy(&self, r: &Resource, a: &Action, c: &Expr) -> Result<()>;
    async fn del_policy(&self, r: &Resource, a: &Action) -> Result<()>;
    async fn policies(&self, r: &Resource) -> Result<Vec<(Action, Expr)>>;
}
