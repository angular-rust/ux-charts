use std::fmt;

use super::TableEntity;

// class DataCollectionIterator<E extends TableEntity> implements Iterator<E> {
//   let DataCollectionBase<E> iterable;
//   let i64 length;
//   i64 index;
//   E current;

//   DataCollectionIterator(DataCollectionBase<E> iterable)
//       : iterable = iterable,
//         length = iterable.length,
//         index = 0;

//   E get current => current;

//   bool moveNext() {
//     i64 length = iterable.length;
//     if (length != length) {
//       throw ConcurrentModificationError(iterable);
//     }
//     if (index >= length) {
//       current = null;
//       return false;
//     }
//     current = iterable.elementAt(index);
//     index++;
//     return true;
//   }
// }

trait DataCollectionBase<E>
where
    E: TableEntity,
{
    fn release_items(&self, start: i64, end: i64);

    fn update_items(&self, start: i64);

    // fn get_iterator() -> Iterator<E> => DataCollectionIterator<E>(this);

    // fn first() -> E  => base.first;

    // fn last() -> E => base.last;

    // fn single() -> E => base.single;

    // fn get_length() -> usize => base.length;

    // fn set length(&self, i64 value);

    // E operator [](i64 index) => base[index];

    // operator []=(i64 index, E value) {
    //   // TODO: implement.
    //   throw UnimplementedError();
    // }

    fn add(&self, value: E);

    // fn add_all(&self, iterable: Iterator<E>);

    // fn elementAt(&self, i64 index) -> E;

    fn insert(&self, index: i64, value: E);

    // fn insert_all(&self, index: i64, iterable: Iterable<E>);

    fn remove(&self, element: E) -> bool;

    fn clear(&self);

    fn remove_at(&self, index: i64) -> E;

    fn remove_last(&self) -> E;

    fn remove_range(&self, start: i64, end: i64);
}
