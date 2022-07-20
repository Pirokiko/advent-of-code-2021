// use std::cmp::Ordering;
// use std::iter::{
//     Chain, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, FlatMap, Flatten, Fuse, Inspect,
//     Intersperse, IntersperseWith, Map, MapWhile, Peekable, Product, Rev, Scan, Skip, SkipWhile,
//     StepBy, Sum, Take, TakeWhile, TrustedRandomAccessNoCoerce, Zip,
// };
// use std::ops::{Residual, Try};
//
// type NodeIndex = usize;
// type NodePosition = usize;
//
// struct Node<T> {
//     pub(crate) index: NodeIndex,
//     pub(crate) value: T,
//     prev: Option<NodeIndex>,
//     next: Option<NodeIndex>,
// }
//
// impl<T> Node<T> {
//     pub fn from(
//         index: NodeIndex,
//         value: T,
//         prev: Option<NodeIndex>,
//         next: Option<NodeIndex>,
//     ) -> Node<T> {
//         Node {
//             index,
//             value,
//             prev,
//             next,
//         }
//     }
// }
//
// pub struct LinkedList<T> {
//     nodes: Vec<Node<T>>,
//     head: Option<NodeIndex>,
//     tail: Option<NodeIndex>,
// }
//
// impl<T> LinkedList<T> {
//     pub fn new() -> LinkedList<T> {
//         LinkedList {
//             tail: None,
//             head: None,
//             nodes: vec![],
//         }
//     }
// }
//
// impl<T> LinkedList<T> {
//     pub fn get(&self, index: NodeIndex) -> &Node<T> {
//         &self.nodes[index]
//     }
//
//     pub fn get_mut(&mut self, index: NodeIndex) -> &mut Node<T> {
//         &mut self.nodes[index]
//     }
//
//     pub fn append(&mut self, value: T) {
//         let index = self.nodes.len();
//         self.nodes.push(Node::from(index, value, self.tail, None));
//
//         if self.tail.is_some() {
//             self.nodes[self.tail.unwrap()].next = Some(index);
//         }
//         self.tail = Some(index);
//     }
//
//     pub fn prepend(&mut self, value: T) {
//         let index = self.nodes.len();
//         self.nodes.push(Node::from(index, value, None, self.head));
//
//         if self.head.is_some() {
//             self.nodes[self.head.unwrap()].prev = Some(index);
//         }
//         self.head = Some(index);
//     }
//
//     fn index_for_position(&self, at_position: NodePosition) -> NodeIndex {
//         if at_position > self.nodes.len() {
//             panic!("Position outside of the current number of elements");
//         }
//         let mut position = 0;
//         let mut index = self.head.unwrap();
//         while position < at_position {
//             match self.nodes[index].next {
//                 None => {
//                     panic!("Reached a None, before reaching the position. List appears not connected anymore. Please file a bug, preferably with reproducible input.");
//                 }
//                 Some(n) => {
//                     index = n;
//                 }
//             };
//             position += 1;
//         }
//         index
//     }
//
//     fn node_at_position(&mut self, at_position: NodePosition) -> &mut Node<T> {
//         let index = self.index_for_position(at_position);
//         &mut self.nodes[index]
//     }
//
//     pub fn insert(&mut self, value: T, at_position: NodePosition) {
//         let index = self.index_for_position(at_position);
//         self.before(value, index);
//     }
//
//     pub fn before(&mut self, value: T, before_index: NodeIndex) {
//         let new_index = self.nodes.len();
//
//         let node = &mut self.nodes[before_index];
//         let new_node = Node::from(new_index, value, node.prev, Some(node.index));
//         self.nodes.push(new_node);
//         if node.prev.is_some() {
//             self.get_mut(node.prev.unwrap()).next = Some(new_index);
//             self.nodes[node.prev.unwrap()].next = Some(new_index);
//         }
//         node.prev = Some(new_index);
//     }
//
//     // pub fn iter_mut(&mut self) -> CursorMut<T> {
//     //     CursorMut {
//     //         list: self,
//     //         curr: self.head,
//     //     }
//     // }
//
//     pub fn iter(&self) -> Cursor<T> {
//         Cursor {
//             list: self,
//             curr: self.head,
//         }
//     }
//
//     pub fn iter_mutation<F>(&mut self, mutator: F) -> Mutator<T, F>
//     where
//         F: FnMut(&mut Node<T>),
//     {
//         Mutator {
//             list: self,
//             mutator,
//             curr: self.head,
//         }
//     }
// }
//
// struct Cursor<'list, T> {
//     list: &'list LinkedList<T>,
//     curr: Option<NodeIndex>,
// }
//
// impl<'list, T> Iterator for Cursor<'list, T> {
//     type Item = &'list Node<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.curr {
//             None => None,
//             Some(index) => {
//                 let node = self.list.get(index);
//                 self.curr = node.next;
//                 Some(node)
//             }
//         }
//     }
// }
// //
// // struct CursorMut<'list, T> {
// //     list: &'list mut LinkedList<T>,
// //     curr: Option<NodeIndex>,
// // }
// //
// // impl<'list, T> Iterator for CursorMut<'list, T> {
// //     type Item = &'list mut Node<T>;
// //
// //     fn next(&mut self) -> Option<Self::Item> {
// //         match self.curr {
// //             None => None,
// //             Some(index) => {
// //                 let node = self.list.get_mut(index);
// //                 // self.curr = node.next;
// //                 Some(node)
// //             }
// //         }
// //     }
// // }
//
// struct Mutator<'list, T, F: FnMut(&mut Node<T>)> {
//     list: &'list mut LinkedList<T>,
//     curr: Option<NodeIndex>,
//     mutator: F,
// }
//
// impl<'list, T, F: FnMut(&mut Node<T>)> Iterator for Mutator<'list, T, F> {
//     type Item = &'list mut Node<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.curr {
//             None => None,
//             Some(index) => {
//                 // self.curr = node.next;
//                 Some(self.list.get_mut(index))
//             }
//         }
//     }
// }
//
// struct NodePairs<'list, T> {
//     list: &'list mut LinkedList<T>,
//     curr: Option<&'list mut Node<T>>,
//     prev: Option<&'list mut Node<T>>,
// }
//
// impl<'list, T> Iterator for NodePairs<'list, T> {
//     type Item = (&'list mut Node<T>, &'list mut Node<T>); // Might not be 'list ???
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.curr {
//             None => None,
//             Some(node) => match self.prev {
//                 None => None,
//                 Some(prev_node) => {
//                     self.prev = Some(node);
//                     self.curr = Some(self.list.get_mut(node.next.unwrap()));
//
//                     Some((prev_node, node))
//                 }
//             },
//         }
//     }
// }
//
// impl<T> LinkedList<T> {
//     pub fn iter_pairs_mut(&mut self) -> NodePairs<T> {
//         if self.nodes.len() < 2 {
//             panic!("Too little nodes to act on pairs");
//         }
//         NodePairs {}
//     }
// }
//
// fn testing() {
//     let mut ll = LinkedList::new();
//     ll.append(5);
//     ll.append(6);
//     ll.append(7);
//
//     ll.iter_mutation(|node| {
//         if node.prev.is_some() {
//             ll.get_mut(node.prev.unwrap());
//             ll.before(1, node.index);
//         }
//     });
// }
