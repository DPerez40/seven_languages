use std::any::Any;
use crate::library::user::User;

pub trait Borrowable: Any {
    fn borrow_item(&mut self);
    fn return_item(&mut self, user: &mut User);
    fn reserve_item(&mut self, user: String);
    fn status(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn is_borrowed(&self) -> bool;
}
