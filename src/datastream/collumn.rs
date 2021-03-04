use super::TableEntity;

pub struct DataColumn<'a> {
    index: i64,
    // table: DataTable,
    /// The name of the column.
    name: &'a str,
}

impl<'a> TableEntity for DataColumn<'a> {}

impl<'a> DataColumn<'a> {
    fn new(name: &'a str) -> Self {
        Self { index: 0, name }
    }
}

// pub struct DataColumnCollection<M, D> 
// where
//     M: fmt::Display,
// {
//     // base: Vec<DataColumn<'a>>,
//     // table: DataTable<'a, M, D>,
// }

// impl<M, D> DataColumnCollection<M, D> 
// where
//     M: fmt::Display,
// {
//     pub fn new(table: &DataTable<M, D>) -> Self {
      
//         Self {
//             // base: Default::default(),
//             // table
//         }
//     }
// }

// impl<'a, M, D> DataCollectionBase<DataColumn<'a>> for DataColumnCollection<M, D> 
// where
//     M: fmt::Display,
// {
//     fn release_items(&self, start: i64, end: i64) {
//         // while (start < end) {
//         //   _base[start]._table = null;
//         //   start++;
//         // }
//         unimplemented!()
//     }

//     fn update_items(&self, start: i64) {
//         // let len = length;
//         // while (start < len) {
//         //   _base[start]
//         //     .._table = _table
//         //     .._index = start++;
//         // }
//         unimplemented!()
//     }

//     // @override
//     // Iterator<E> get iterator => DataCollectionIterator<E>(this);

//     // @override
//     // E get first => _base.first;

//     // @override
//     // E get last => _base.last;

//     // @override
//     // E get single => _base.single;

//     // @override
//     // i64 get length => _base.length;

//     // fn set length(&self, i64 value) {
//     //   // TODO: implement.
//     //   throw UnimplementedError();
//     // }

//     // @override
//     // E operator [](i64 index) => _base[index];

//     // @override
//     // operator []=(i64 index, E value) {
//     //   // TODO: implement.
//     //   throw UnimplementedError();
//     // }

//     fn add(&self, value: DataColumn<'a>) {
//         // let index = length;
//         // _base.add(value);
//         // _updateItems(index);
//         // _table._onRowsOrColumnsInserted(this, index, 1);
//         unimplemented!()
//     }

//     // fn add_all(&self, iterable: Iterator<DataColumn<'a>>) {
//     // //     // let index = length;
//     // //     // _base.addAll(iterable);
//     // //     // _updateItems(index);
//     // //     // _table._onRowsOrColumnsInserted(this, index, iterable.length);
//     // }

//     // @override
//     // E elementAt(&self, i64 index) => _base[index];

//     fn insert(&self, index: i64, value: DataColumn<'a>) {
//         // _base.insert(index, value);
//         // _updateItems(index);
//         // _table._onRowsOrColumnsInserted(this, index, 1);
//         unimplemented!()
//     }

//     // fn insert_all(&self, index: i64, iterable: Iterable<DataColumn<'a>>) {
//     //     // _base.insertAll(index, iterable);
//     //     // _updateItems(index);
//     //     // _table._onRowsOrColumnsInserted(this, index, iterable.length);
//     // }

//     fn remove(&self, element: DataColumn<'a>) -> bool {
//         // let index = _base.indexOf(element);
//         // if (index == -1) return false;
//         // removeAt(index);
//         // return true;
//         unimplemented!()
//     }

//     fn clear(&self) {
//         // let len = length;
//         // if (len == 0) return;
//         // _releaseItems(0, len);
//         // _base.clear();
//         // _table._onRowsOrColumnsRemoved(this, 0, len);
//         unimplemented!()
//     }

//     fn remove_at(&self, index: i64) -> DataColumn<'a> {
//         // let e = _base.removeAt(index);
//         // e._table = null;
//         // _updateItems(index);
//         // _table._onRowsOrColumnsRemoved(this, index, 1);
//         // return e;
//         unimplemented!()
//     }

//     fn remove_last(&self) -> DataColumn<'a> {
//         // let e = _base.removeLast();
//         // e._table = null;
//         // _table._onRowsOrColumnsRemoved(this, length, 1);
//         // return e;
//         unimplemented!()
//     }

//     fn remove_range(&self, start: i64, end: i64) {
//         // _releaseItems(start, end);
//         // _base.remove_range(start, end);
//         // _updateItems(start);
//         // _table._onRowsOrColumnsRemoved(this, start, end - start);
//         unimplemented!()
//     }

//     // DataColumnCollection(DataTable table) : super(table);

//     // /// Adds a new column given its [name] and [type].
//     // fn add2(&self, name: String, dtype: Type) {
//     //     // add(DataColumn(name, dtype));
//     // }
// }