use xilem_core::{Id, MessageResult};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ChangeFlags: u32 {
        const STRUCTURE = 1;
    }
}

impl ChangeFlags {
    // Change flags representing change of tree structure.
    pub fn tree_structure() -> Self {
        ChangeFlags::STRUCTURE
    }
}

pub trait Widget {}

impl Widget for () {}

pub struct Cx<C, Y> {
    _app_context: C,
    _other: Y,
}

pub trait AnyElement {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl AnyElement for Box<dyn AnyElement> {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Widget for Box<dyn AnyElement> {}
pub struct Pod;

impl Pod {
    pub fn new(_el: impl Widget) -> Self {
        Pod
    }
    fn mark(&mut self, flags: ChangeFlags) -> ChangeFlags {
        flags
    }
    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        None
    }
}

xilem_core::generate_view_trait!(View <C, Y>, Widget,  Cx<C, Y>, ChangeFlags;);
xilem_core::generate_viewsequence_trait! {ViewSequence, View <C, Y>, ViewMarker, Widget, Cx<C, Y>, ChangeFlags, Pod;}
xilem_core::generate_anyview_trait! {AnyView, View <C, Y>, ViewMarker, Cx<C, Y>, ChangeFlags, AnyElement, BoxedView;}
xilem_core::generate_memoize_view! {Memoize, MemoizeState, View <C, Y>, ViewMarker, Cx<C, Y>, ChangeFlags, s, memoize}
xilem_core::generate_adapt_view! {View <C, Y>, Cx<C, Y>, ChangeFlags}
xilem_core::generate_adapt_state_view! {View <C, Y>, Cx<C, Y>, ChangeFlags}

struct MyView;

impl<T, C, Y, A> View<T, C, Y, A> for MyView {
    type State = ();
    type Element = ();
    fn build(&self, _cx: &mut Cx<C, Y>) -> (Id, Self::State, Self::Element) {
        (Id::next(), (), ())
    }

    fn rebuild(
        &self,
        _cx: &mut Cx<C, Y>,
        _prev: &Self,
        _id: &mut Id,
        _state: &mut Self::State,
        _element: &mut Self::Element,
    ) -> ChangeFlags {
        ChangeFlags::empty()
    }

    fn message(
        &self,
        _id_path: &[Id],
        _state: &mut Self::State,
        _message: Box<dyn std::any::Any>,
        _app_state: &mut T,
    ) -> MessageResult<A> {
        MessageResult::Nop
    }
}

fn main() {
    println!("Hello, world!");
}
