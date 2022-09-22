use std::fmt::{Debug, DebugList, Formatter};
use std::rc::Rc;

pub struct RcList<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[allow(dead_code)]
impl<T> RcList<T> {

    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn append_new(&self, elem: T) -> Self {
        let new_node = match &self.head {
            None => {
                Node {
                    elem,
                    next: None
                }
            },
            Some(rc) => {
                Node {
                    elem,
                    next: Some(rc.clone())
                }
            }
        };

        Self {
            head: Some(
                Rc::new(new_node)
            )
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn count(&self) -> usize {
        self.reverse_iter().count()
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|rc| &rc.elem)
    }

    pub fn reverse_iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<T> Debug for RcList<T> where T: Debug {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // since RCList's iterator is inverted
        // recursion is used to revert it back to normal order
        fn reverted_append_to_builder<G: Debug>(
            move_iter: &mut Iter<'_, G>,
            result: &mut DebugList,
        ) {
            if let Some(ref_of_t) = move_iter.next() {
                reverted_append_to_builder(move_iter, result);
                result.entry(ref_of_t);
            }
        }

        let mut builder = f.debug_list();
        reverted_append_to_builder(&mut self.reverse_iter(), &mut builder);
        builder.finish()
    }
}

impl<T> Clone for RcList<T> where T: Clone {
    fn clone(&self) -> Self {
        RcList::<T> {
            head: self.head.as_ref().cloned()
        }
    }
}

// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut cur_link = self.head.take();
//         while let Some(mut boxed_node) = cur_link {
//             cur_link = boxed_node.next.take();
//         }
//     }
// }

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use crate::base::Move;
    use super::RcList;

    impl RcList<Move> {
        pub fn toggle_rows(&self) -> RcList<Move> {
            let mut moves: Vec<&Move> = self.reverse_iter().collect();
            moves.reverse();

            moves.iter_mut().fold(
                RcList::new(),
                |rc_list, a_move|rc_list.append_new((*a_move).toggle_rows())
            )
        }
    }

    #[test]
    fn test_basics() {
        let list = RcList::new();

        // Check empty list is empty
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.count(), 0);

        // Populate list
        let child_list = list.append_new(1);
        assert_eq!(list.is_empty(), true);
        assert_eq!(child_list.is_empty(), false);
        assert_eq!(child_list.count(), 1);
    }

    #[test]
    fn test_iter() {
        let list = RcList::new();
        let list1 = list.append_new(1);
        let list2 = list1.append_new(2);
        let list3 = list2.append_new(3);
        let mut reverse_iter = list3.reverse_iter();

        assert_eq!(reverse_iter.next(), Some(&3));
        assert_eq!(reverse_iter.next(), Some(&2));
        assert_eq!(reverse_iter.next(), Some(&1));
        assert!(reverse_iter.next().is_none());
    }

    #[test]
    fn test_toggle_rows() {
        let list = RcList::new();
        let list1 = list.append_new("a1-h8".parse::<Move>().unwrap());
        let list2 = list1.append_new("b1-b2".parse::<Move>().unwrap());
        let list3 = list2.append_new("c8-c1".parse::<Move>().unwrap());

        let toggled_list = list3.toggle_rows();
        let mut reverse_iter = toggled_list.reverse_iter();

        assert_eq!(*reverse_iter.next().unwrap(), "c1-c8".parse::<Move>().unwrap());
        assert_eq!(*reverse_iter.next().unwrap(), "b8-b7".parse::<Move>().unwrap());
        assert_eq!(*reverse_iter.next().unwrap(), "a8-h1".parse::<Move>().unwrap());
        assert!(reverse_iter.next().is_none());
    }

    #[test]
    fn test_peek() {
        let list = RcList::new();
        assert!(list.peek().is_none(), "peek of empty list should be None");
        let list1 = list.append_new(1);
        assert_eq!(list1.peek(), Some(&1));
        let list2 = list1.append_new(2);
        assert_eq!(list2.peek(), Some(&2));
    }
}
