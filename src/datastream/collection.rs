use std::fmt;

use super::TableEntity;

// class DataCollectionIterator<E extends TableEntity> implements Iterator<E> {
//   let DataCollectionBase<E> _iterable;
//   let i64 _length;
//   i64 _index;
//   E _current;

//   DataCollectionIterator(DataCollectionBase<E> iterable)
//       : _iterable = iterable,
//         _length = iterable.length,
//         _index = 0;

//   E get current => _current;

//   bool moveNext() {
//     i64 length = _iterable.length;
//     if (_length != length) {
//       throw ConcurrentModificationError(_iterable);
//     }
//     if (_index >= length) {
//       _current = null;
//       return false;
//     }
//     _current = _iterable.elementAt(_index);
//     _index++;
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

    // fn first() -> E  => _base.first;

    // fn last() -> E => _base.last;

    // fn single() -> E => _base.single;

    // fn get_length() -> usize => _base.length;

    // fn set length(&self, i64 value);

    // E operator [](i64 index) => _base[index];

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
