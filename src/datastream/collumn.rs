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
//         //   base[start].table = null;
//         //   start++;
//         // }
//         unimplemented!()
//     }

//     fn update_items(&self, start: i64) {
//         // let len = length;
//         // while (start < len) {
//         //   base[start]
//         //     ..table = table
//         //     ..index = start++;
//         // }
//         unimplemented!()
//     }

//     // @override
//     // Iterator<E> get iterator => DataCollectionIterator<E>(this);

//     // @override
//     // E get first => base.first;

//     // @override
//     // E get last => base.last;

//     // @override
//     // E get single => base.single;

//     // @override
//     // i64 get length => base.length;

//     // fn set length(&self, i64 value) {
//     //   // TODO: implement.
//     //   throw UnimplementedError();
//     // }

//     // @override
//     // E operator [](i64 index) => base[index];

//     // @override
//     // operator []=(i64 index, E value) {
//     //   // TODO: implement.
//     //   throw UnimplementedError();
//     // }

//     fn add(&self, value: DataColumn<'a>) {
//         // let index = length;
//         // base.add(value);
//         // updateItems(index);
//         // table.onRowsOrColumnsInserted(this, index, 1);
//         unimplemented!()
//     }

//     // fn add_all(&self, iterable: Iterator<DataColumn<'a>>) {
//     // //     // let index = length;
//     // //     // base.addAll(iterable);
//     // //     // updateItems(index);
//     // //     // table.onRowsOrColumnsInserted(this, index, iterable.length);
//     // }

//     // @override
//     // E elementAt(&self, i64 index) => base[index];

//     fn insert(&self, index: i64, value: DataColumn<'a>) {
//         // base.insert(index, value);
//         // updateItems(index);
//         // table.onRowsOrColumnsInserted(this, index, 1);
//         unimplemented!()
//     }

//     // fn insert_all(&self, index: i64, iterable: Iterable<DataColumn<'a>>) {
//     //     // base.insertAll(index, iterable);
//     //     // updateItems(index);
//     //     // table.onRowsOrColumnsInserted(this, index, iterable.length);
//     // }

//     fn remove(&self, element: DataColumn<'a>) -> bool {
//         // let index = base.indexOf(element);
//         // if (index == -1) return false;
//         // removeAt(index);
//         // return true;
//         unimplemented!()
//     }

//     fn clear(&self) {
//         // let len = length;
//         // if (len == 0) return;
//         // releaseItems(0, len);
//         // base.clear();
//         // table.onRowsOrColumnsRemoved(this, 0, len);
//         unimplemented!()
//     }

//     fn remove_at(&self, index: i64) -> DataColumn<'a> {
//         // let e = base.removeAt(index);
//         // e.table = null;
//         // updateItems(index);
//         // table.onRowsOrColumnsRemoved(this, index, 1);
//         // return e;
//         unimplemented!()
//     }

//     fn remove_last(&self) -> DataColumn<'a> {
//         // let e = base.removeLast();
//         // e.table = null;
//         // table.onRowsOrColumnsRemoved(this, length, 1);
//         // return e;
//         unimplemented!()
//     }

//     fn remove_range(&self, start: i64, end: i64) {
//         // releaseItems(start, end);
//         // base.remove_range(start, end);
//         // updateItems(start);
//         // table.onRowsOrColumnsRemoved(this, start, end - start);
//         unimplemented!()
//     }

//     // DataColumnCollection(DataTable table) : super(table);

//     // /// Adds a new column given its [name] and [type].
//     // fn add2(&self, name: String, dtype: Type) {
//     //     // add(DataColumn(name, dtype));
//     // }
// }