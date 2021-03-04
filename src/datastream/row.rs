use super::TableEntity;

pub struct DataRow<T> {
    index: i64,
    // table: DataTable,
    /// The list that stores the actual data.
    cells: Vec<T>,
}

impl<T> TableEntity for DataRow<T> {}

impl<T> DataRow<T> {
    /// Converts a column index or name to an index.
    fn to_index(column_index_or_name: i64) -> i64 {
        // if (columnIndexOrName is int) return columnIndexOrName;
        // return table.columnIndexByName[columnIndexOrName];
        unimplemented!()
    }

    // /// Creates a new [DataRow] from a list of values.
    // ///
    // ///  Each value in [values] corresponds to a column. If [values] is too short,
    // /// the remaining columns are filled with `null`.
    // DataRow.internal(DataTable table, List values) {
    //   table = table;
    //   let n = table.columns.length;
    //   let m = values.length;
    //   let min = m;
    //   if (min > n) min = n;
    //   cells = values.sublist(0, min);
    //   for (let i = min; i < n; i++) {
    //     cells.add(null);
    //   }
    // }

    // /// Returns the value of the column specified by [columnIndexOrName].
    // operator [](columnIndexOrName) => cells[_toIndex(columnIndexOrName)];

    // /// Sets the value of the column specified by [columnIndexOrName].
    // operator []=(columnIndexOrName, value) {
    //   let columnIndex = toIndex(columnIndexOrName);
    //   let oldValue = cells[columnIndex];
    //   cells[columnIndex] = value;
    //   table.onCellChanged(index, columnIndex, oldValue, value);
    // }

    // /// Creates a [List] containing all cells in this [DataRow].
    // List toList({bool growable: true}) => cells.toList(growable: growable);
}

// pub struct DataRowCollection<M, D> {
//     base: Vec<DataRow<D>>,
//     //     table: DataTable,
// }

// impl<M, D> DataRowCollection<M, D> {
//     pub fn new(table: &DataTable<M, D>) -> Self {
//         //   table = table;
//         Self {
//             base: Default::default(),
//         }
//     }
// }

// impl<M, D> DataCollectionBase<DataRow<D>> for DataRowCollection<M, D> {
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

//     fn add(&self, value: DataRow<D>) {
//         // let index = length;
//         // base.add(value);
//         // updateItems(index);
//         // table.onRowsOrColumnsInserted(this, index, 1);
//         unimplemented!()
//     }

//     // fn add_all(&self, iterable: Iterator<E>) {
//     // //     // let index = length;
//     // //     // base.addAll(iterable);
//     // //     // updateItems(index);
//     // //     // table.onRowsOrColumnsInserted(this, index, iterable.length);
//     // }

//     // @override
//     // E elementAt(&self, i64 index) => base[index];

//     fn insert(&self, index: i64, value: DataRow<D>) {
//         // base.insert(index, value);
//         // updateItems(index);
//         // table.onRowsOrColumnsInserted(this, index, 1);
//         unimplemented!()
//     }

//     // fn insert_all(&self, index: i64, iterable: Iterable<E>) {
//     //     // base.insertAll(index, iterable);
//     //     // updateItems(index);
//     //     // table.onRowsOrColumnsInserted(this, index, iterable.length);
//     // }

//     fn remove(&self, element: DataRow<D>) -> bool {
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

//     fn remove_at(&self, index: i64) -> DataRow<D> {
//         // let e = base.removeAt(index);
//         // e.table = null;
//         // updateItems(index);
//         // table.onRowsOrColumnsRemoved(this, index, 1);
//         // return e;
//         unimplemented!()
//     }

//     fn remove_last(&self) -> DataRow<D> {
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

//     // FIXME: bellow implementation

//     // DataRow toDataRow(value) =>
//     //     value is DataRow ? value : DataRow.internal(table, value);

//     // DataRowCollection(DataTable table) : super(table);

//     // /// Adds [value] to this collection.
//     // ///
//     // /// [value] can be a [DataRow] or a [List].
//     // fn add(value: E) {
//     //     // self.base.add(toDataRow(value));
//     // }

//     // /// Adds all elements of [iterable] to this collection.
//     // ///
//     // /// Each element in [iterable] can be a [DataRow] or a [List].
//     // fn addAll(iterable: Iterable) {
//     //     // self.base.addAll(iterable.map(toDataRow));
//     // }

//     // /// Inserts [value] at position [index] in this collection.
//     // ///
//     // /// [value] can be a [DataRow] or a [List].
//     // fn insert(index: i64, value: E) {
//     //     // self.base.insert(index, toDataRow(value));
//     // }

//     // /// Inserts all elements of [iterable] at position [index] in this collection.
//     // ///
//     // /// Each element in [iterable] can be a [DataRow] or a [List].
//     // fn insertAll(index: i64, iterable: Iterable) {
//     //     // self.base.insertAll(index, iterable.map(toDataRow));
//     // }
// }