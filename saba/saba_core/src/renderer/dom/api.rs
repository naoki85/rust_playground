use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;
use crate::renderer::dom::node::{Element, ElementKind, Node, NodeKind};

pub fn get_target_element_node(
    node: Option<Rc<RefCell<Node>>>,
    element_kind: ElementKind,
) -> Option<Rc<RefCell<Node>>> {
    match node {
        Some(n) => {
            if n.borrow().kind() == NodeKind::Element(Element::new(&element_kind.to_string(), Vec::new())) {
                return Some(n.clone())
            }
            let result1 = get_target_element_node(n.borrow().first_child(), element_kind);
            let result2 = get_target_element_node(n.borrow().next_sibling(), element_kind);
            if result1.is_none() && result2.is_none() {
                return None
            }
            if result1.is_none() {
                return result2
            }
            result1
        }
        None => None
    }
}

pub fn get_style_content(root: Rc<RefCell<Node>>) -> String {
    let style_node = match get_target_element_node(Some(root), ElementKind::Style) {
        Some(n) => n,
        None => return "".to_string(),
    };
    let text_node = match style_node.borrow().first_child() {
        Some(n) => n,
        None => return "".to_string(),
    };
    let content = match &text_node.borrow().kind() {
        NodeKind::Text(ref s) => s.clone(),
        _ => "".to_string()
    };
    content
}