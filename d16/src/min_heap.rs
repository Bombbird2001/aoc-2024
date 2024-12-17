// use std::collections::HashMap;
//
// trait Heap<K, V> {
//     fn new() -> Self;
//     fn insert(&mut self, key: K, value: K);
//     fn extract_min(&mut self) -> Option<K>;
//     fn decrease_key(&mut self, key: K, new_value: K);
//     fn peek(&self) -> Option<(K, V)>;
//     fn len(&self) -> usize;
//     fn is_empty(&self) -> bool;
// }
//
// struct MinHeap<'a, K, V> {
//     data: Vec<(&'a K, &'a V)>,
//     key_index_map: HashMap<K, usize>
// }
//
// impl<K, V: Ord> Heap<K, V> for MinHeap<'_, K, V> {
//     fn new() -> Self {
//         MinHeap {
//             data: Vec::new(),
//             key_index_map: HashMap::new()
//         }
//     }
//
//     fn insert(&mut self, key: K, value: V) {
//         if self.key_index_map.contains_key(&key) {
//             return;
//         }
//         self.data.push((&key, &value));
//         self.data.swap(self.data.len() - 1, 0);
//         self.decrease_key(key, value);
//     }
//
//     fn extract_min(&mut self) -> Option<V> {
//         if self.data.is_empty() {
//             return None;
//         }
//         self.data.swap(0, self.data.len() - 1);
//         let res = self.data.pop();
//         self.decrease_key(self.data[0].0, self.data[0].1);
//         res
//     }
//
//     fn decrease_key(&mut self, key: K, new_value: V) {
//         let mut index = *self.key_index_map.get(&key).unwrap();
//         self.data[index] = new_value;
//         let left_child_index = 2 * index + 1;
//         let right_child_index = 2 * index + 2;
//         loop {
//             if left_child_index >= self.data.len() {
//                 break;
//             }
//             if right_child_index >= self.data.len() {
//                 if self.data[left_child_index] < self.data[index] {
//                     self.key_index_map.insert(&self.data[left_child_index].0, index);
//                     self.key_index_map.insert(&key, left_child_index);
//                     self.data.swap(left_child_index, index);
//                 }
//                 break;
//             }
//             let min_child_index = if self.data[left_child_index] < self.data[right_child_index] {
//                 left_child_index
//             } else {
//                 right_child_index
//             };
//             if self.data[min_child_index] < self.data[index] {
//                 self.key_index_map.insert(&self.data[min_child_index].0, index);
//                 self.key_index_map.insert(&key, min_child_index);
//                 self.data.swap(min_child_index, *index);
//                 index = min_child_index;
//             } else {
//                 break;
//             }
//         }
//     }
//
//     fn peek(&self) -> Option<&(&K, &V)> {
//         self.data.first()
//     }
//
//     fn len(&self) -> usize {
//         self.data.len()
//     }
//
//     fn is_empty(&self) -> bool {
//         self.data.is_empty()
//     }
// }