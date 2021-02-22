#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{collections::HashMap, fmt};

pub struct DataCellChangeRecord<T>
where
    T: fmt::Display,
{
    row_index: i64,
    column_index: i64,
    old_value: T,
    new_value: T,
}

impl<T> DataCellChangeRecord<T>
where
    T: fmt::Display,
{
    fn new(row_index: i64, column_index: i64, old_value: T, new_value: T) -> Self {
        Self {
            row_index,
            column_index,
            old_value,
            new_value,
        }
    }
}

impl<T> fmt::Display for DataCellChangeRecord<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DataCellChangeRecord {{ rowIndex: {}, colIndex; {}, {}, {} }}",
            self.row_index, self.column_index, self.old_value, self.new_value
        )
    }
}

pub struct DataCollectionChangeRecord {
    index: i64,
    added_count: i64,
    removed_count: i64,
}

impl DataCollectionChangeRecord {
    fn new(index: i64, added_count: i64, removed_count: i64) -> Self {
        Self {
            index,
            added_count,
            removed_count,
        }
    }
}

impl fmt::Display for DataCollectionChangeRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DataCollectionChangeRecord {{ index: {}, added: {}, removed: {} }}",
            self.index, self.added_count, self.removed_count
        )
    }
}

// should impl getters/setters
pub trait TableEntity {}

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
        // return _table._columnIndexByName[columnIndexOrName];
        unimplemented!()
    }

    // /// Creates a new [DataRow] from a list of values.
    // ///
    // ///  Each value in [values] corresponds to a column. If [values] is too short,
    // /// the remaining columns are filled with `null`.
    // DataRow._internal(DataTable table, List values) {
    //   _table = table;
    //   let n = _table._columns.length;
    //   let m = values.length;
    //   let min = m;
    //   if (min > n) min = n;
    //   _cells = values.sublist(0, min);
    //   for (let i = min; i < n; i++) {
    //     _cells.add(null);
    //   }
    // }

    // /// Returns the value of the column specified by [columnIndexOrName].
    // operator [](columnIndexOrName) => _cells[_toIndex(columnIndexOrName)];

    // /// Sets the value of the column specified by [columnIndexOrName].
    // operator []=(columnIndexOrName, value) {
    //   let columnIndex = _toIndex(columnIndexOrName);
    //   let oldValue = _cells[columnIndex];
    //   _cells[columnIndex] = value;
    //   _table._onCellChanged(_index, columnIndex, oldValue, value);
    // }

    // /// Creates a [List] containing all cells in this [DataRow].
    // List toList({bool growable: true}) => _cells.toList(growable: growable);
}

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

    // fn get first() -> E  => _base.first;

    // fn get_last() -> E => _base.last;

    // fn get_single() -> E => _base.single;

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

pub struct DataRowCollection<T> {
    base: Vec<DataRow<T>>,
    //     table: DataTable,
}

impl<T> DataRowCollection<T> {
    pub fn new(table: &DataTable) -> Self {
        //   _table = table;
        Self {
            base: Default::default(),
        }
    }
}

impl<T> DataCollectionBase<DataRow<T>> for DataRowCollection<T> {
    fn release_items(&self, start: i64, end: i64) {
        // while (start < end) {
        //   _base[start]._table = null;
        //   start++;
        // }
        unimplemented!()
    }

    fn update_items(&self, start: i64) {
        // let len = length;
        // while (start < len) {
        //   _base[start]
        //     .._table = _table
        //     .._index = start++;
        // }
        unimplemented!()
    }

    // @override
    // Iterator<E> get iterator => DataCollectionIterator<E>(this);

    // @override
    // E get first => _base.first;

    // @override
    // E get last => _base.last;

    // @override
    // E get single => _base.single;

    // @override
    // i64 get length => _base.length;

    // fn set length(&self, i64 value) {
    //   // TODO: implement.
    //   throw UnimplementedError();
    // }

    // @override
    // E operator [](i64 index) => _base[index];

    // @override
    // operator []=(i64 index, E value) {
    //   // TODO: implement.
    //   throw UnimplementedError();
    // }

    fn add(&self, value: DataRow<T>) {
        // let index = length;
        // _base.add(value);
        // _updateItems(index);
        // _table._onRowsOrColumnsInserted(this, index, 1);
        unimplemented!()
    }

    // fn add_all(&self, iterable: Iterator<E>) {
    // //     // let index = length;
    // //     // _base.addAll(iterable);
    // //     // _updateItems(index);
    // //     // _table._onRowsOrColumnsInserted(this, index, iterable.length);
    // }

    // @override
    // E elementAt(&self, i64 index) => _base[index];

    fn insert(&self, index: i64, value: DataRow<T>) {
        // _base.insert(index, value);
        // _updateItems(index);
        // _table._onRowsOrColumnsInserted(this, index, 1);
        unimplemented!()
    }

    // fn insert_all(&self, index: i64, iterable: Iterable<E>) {
    //     // _base.insertAll(index, iterable);
    //     // _updateItems(index);
    //     // _table._onRowsOrColumnsInserted(this, index, iterable.length);
    // }

    fn remove(&self, element: DataRow<T>) -> bool {
        // let index = _base.indexOf(element);
        // if (index == -1) return false;
        // removeAt(index);
        // return true;
        unimplemented!()
    }

    fn clear(&self) {
        // let len = length;
        // if (len == 0) return;
        // _releaseItems(0, len);
        // _base.clear();
        // _table._onRowsOrColumnsRemoved(this, 0, len);
        unimplemented!()
    }

    fn remove_at(&self, index: i64) -> DataRow<T> {
        // let e = _base.removeAt(index);
        // e._table = null;
        // _updateItems(index);
        // _table._onRowsOrColumnsRemoved(this, index, 1);
        // return e;
        unimplemented!()
    }

    fn remove_last(&self) -> DataRow<T> {
        // let e = _base.removeLast();
        // e._table = null;
        // _table._onRowsOrColumnsRemoved(this, length, 1);
        // return e;
        unimplemented!()
    }

    fn remove_range(&self, start: i64, end: i64) {
        // _releaseItems(start, end);
        // _base.removeRange(start, end);
        // _updateItems(start);
        // _table._onRowsOrColumnsRemoved(this, start, end - start);
        unimplemented!()
    }

    // FIXME: bellow implementation

    // DataRow _toDataRow(value) =>
    //     value is DataRow ? value : DataRow._internal(_table, value);

    // DataRowCollection(DataTable table) : super(table);

    // /// Adds [value] to this collection.
    // ///
    // /// [value] can be a [DataRow] or a [List].
    // fn add(value: E) {
    //     // super.add(_toDataRow(value));
    // }

    // /// Adds all elements of [iterable] to this collection.
    // ///
    // /// Each element in [iterable] can be a [DataRow] or a [List].
    // fn addAll(iterable: Iterable) {
    //     // super.addAll(iterable.map(_toDataRow));
    // }

    // /// Inserts [value] at position [index] in this collection.
    // ///
    // /// [value] can be a [DataRow] or a [List].
    // fn insert(index: i64, value: E) {
    //     // super.insert(index, _toDataRow(value));
    // }

    // /// Inserts all elements of [iterable] at position [index] in this collection.
    // ///
    // /// Each element in [iterable] can be a [DataRow] or a [List].
    // fn insertAll(index: i64, iterable: Iterable) {
    //     // super.insertAll(index, iterable.map(_toDataRow));
    // }
}

pub struct DataColumnCollection<'a> {
    base: Vec<DataColumn<'a>>,
    //     table: DataTable,
}

impl<'a> DataColumnCollection<'a> {
    pub fn new(table: &DataTable) -> Self {
        //   _table = table;
        Self {
            base: Default::default(),
        }
    }
}

impl<'a> DataCollectionBase<DataColumn<'a>> for DataColumnCollection<'a> {
    fn release_items(&self, start: i64, end: i64) {
        // while (start < end) {
        //   _base[start]._table = null;
        //   start++;
        // }
        unimplemented!()
    }

    fn update_items(&self, start: i64) {
        // let len = length;
        // while (start < len) {
        //   _base[start]
        //     .._table = _table
        //     .._index = start++;
        // }
        unimplemented!()
    }

    // @override
    // Iterator<E> get iterator => DataCollectionIterator<E>(this);

    // @override
    // E get first => _base.first;

    // @override
    // E get last => _base.last;

    // @override
    // E get single => _base.single;

    // @override
    // i64 get length => _base.length;

    // fn set length(&self, i64 value) {
    //   // TODO: implement.
    //   throw UnimplementedError();
    // }

    // @override
    // E operator [](i64 index) => _base[index];

    // @override
    // operator []=(i64 index, E value) {
    //   // TODO: implement.
    //   throw UnimplementedError();
    // }

    fn add(&self, value: DataColumn<'a>) {
        // let index = length;
        // _base.add(value);
        // _updateItems(index);
        // _table._onRowsOrColumnsInserted(this, index, 1);
        unimplemented!()
    }

    // fn add_all(&self, iterable: Iterator<DataColumn<'a>>) {
    // //     // let index = length;
    // //     // _base.addAll(iterable);
    // //     // _updateItems(index);
    // //     // _table._onRowsOrColumnsInserted(this, index, iterable.length);
    // }

    // @override
    // E elementAt(&self, i64 index) => _base[index];

    fn insert(&self, index: i64, value: DataColumn<'a>) {
        // _base.insert(index, value);
        // _updateItems(index);
        // _table._onRowsOrColumnsInserted(this, index, 1);
        unimplemented!()
    }

    // fn insert_all(&self, index: i64, iterable: Iterable<DataColumn<'a>>) {
    //     // _base.insertAll(index, iterable);
    //     // _updateItems(index);
    //     // _table._onRowsOrColumnsInserted(this, index, iterable.length);
    // }

    fn remove(&self, element: DataColumn<'a>) -> bool {
        // let index = _base.indexOf(element);
        // if (index == -1) return false;
        // removeAt(index);
        // return true;
        unimplemented!()
    }

    fn clear(&self) {
        // let len = length;
        // if (len == 0) return;
        // _releaseItems(0, len);
        // _base.clear();
        // _table._onRowsOrColumnsRemoved(this, 0, len);
        unimplemented!()
    }

    fn remove_at(&self, index: i64) -> DataColumn<'a> {
        // let e = _base.removeAt(index);
        // e._table = null;
        // _updateItems(index);
        // _table._onRowsOrColumnsRemoved(this, index, 1);
        // return e;
        unimplemented!()
    }

    fn remove_last(&self) -> DataColumn<'a> {
        // let e = _base.removeLast();
        // e._table = null;
        // _table._onRowsOrColumnsRemoved(this, length, 1);
        // return e;
        unimplemented!()
    }

    fn remove_range(&self, start: i64, end: i64) {
        // _releaseItems(start, end);
        // _base.removeRange(start, end);
        // _updateItems(start);
        // _table._onRowsOrColumnsRemoved(this, start, end - start);
        unimplemented!()
    }

    // DataColumnCollection(DataTable table) : super(table);

    // /// Adds a new column given its [name] and [type].
    // fn add2(&self, name: String, dtype: Type) {
    //     // add(DataColumn(name, dtype));
    // }
}

pub struct DataTable<'a> {
    column_index_by_name: HashMap<String, usize>,
    columns: Option<DataColumnCollection<'a>>,
    rows: Option<DataRowCollection<String>>,
    // cellChangeController: StreamController<DataCellChangeRecord>,
    // columnsChangeController: StreamController<DataCollectionChangeRecord>,
    // rowsChangeController: StreamController<DataCollectionChangeRecord>,
}

impl<'a> DataTable<'a> {
    fn on_cell_changed(row_index: i64, column_index: i64, old_value: String, new_value: String) {
        // if (_cellChangeController != null) {
        //   let record =
        //       DataCellChangeRecord(rowIndex, columnIndex, oldValue, newValue);
        //   _cellChangeController.add(record);
        // }
    }

    // fn onRowsOrColumnsInserted(source: DataCollectionBase, index: i64, count: i64) {
    //     // let record = DataCollectionChangeRecord(index, count, 0);
    //     // if (source == _columns) {
    //     //   _insertColumns(index, count);
    //     //   _updateColumnIndexes(index);
    //     //   _columnsChangeController?.add(record);
    //     // } else {
    //     //   _rowsChangeController?.add(record);
    //     // }
    // }

    // fn onRowsOrColumnsRemoved(source: DataCollectionBase, index: i64, count: i64) {
    //     // let record = DataCollectionChangeRecord(index, 0, count);
    //     // if (source == _columns) {
    //     //   _removeColumns(index, count);
    //     //   _updateColumnIndexes(index);
    //     //   _columnsChangeController?.add(record);
    //     // } else {
    //     //   _rowsChangeController?.add(record);
    //     // }
    // }

    fn insert_columns(start: i64, count: i64) {
        // for (let row in _rows) {
        //   row._cells.insertAll(start, List(count));
        // }
    }

    fn remove_columns(start: i64, count: i64) {
        // for (let row in _rows) {
        //   row._cells.removeRange(start, start + count);
        // }
    }

    fn update_column_indexes(start: i64) {
        // let end = _columns.length;
        // while (start < end) {
        //   _columnIndexByName[_columns[start].name] = start++;
        // }
    }

    /// Creates a [DataTable] with optional data [data].
    ///
    /// * 'metadata' - contains row names
    ///
    /// The first row in [data] contains the column names.
    /// The data type of each column is determined by the first non-null value
    /// in that column.
    ///
    /// All values in each column are expected to be of the same type,
    /// and all rows are expected to have the same length.
    // data is optional
    pub fn new(metadata: Vec<&str>, data: Vec<Vec<&str>>) {
        let data_table = DataTable {
            column_index_by_name: Default::default(),
            columns: None,
            rows: None,
        };

        let column_index_by_name = HashMap::<String, usize>::new();
        let rows: DataRowCollection<String> = DataRowCollection::new(&data_table);
        let columns: DataColumnCollection = DataColumnCollection::new(&data_table);

        let col_count = metadata.len();
        let row_count = data.len();

        // first deal with columns
        for col_idx in 0..col_count {
            let name = metadata[col_idx];
            columns.add(DataColumn { index: 0, name });
        }

        // rows.add_all(data);
    }

    // /// The columns in this [DataTable].
    // DataColumnCollection get columns => _columns;

    // /// The rows (without the header row) in this [DataTable].
    // DataRowCollection get rows => _rows;

    // /// Fired when a cell is changed.
    // Stream<DataCellChangeRecord> get onCellChange {
    //   _cellChangeController ??= StreamController.broadcast(
    //       sync: true,
    //       onCancel: () {
    //         _cellChangeController = null;
    //       });
    //   return _cellChangeController.stream;
    // }

    // /// Fired when [columns] are changed.
    // Stream<DataCollectionChangeRecord> get onColumnsChange {
    //   _columnsChangeController ??= StreamController.broadcast(
    //       sync: true,
    //       onCancel: () {
    //         _columnsChangeController = null;
    //       });
    //   return _columnsChangeController.stream;
    // }

    // /// Fired when [rows] are changed.
    // Stream<DataCollectionChangeRecord> get onRowsChange {
    //   _rowsChangeController ??= StreamController.broadcast(
    //       sync: true,
    //       onCancel: () {
    //         _rowsChangeController = null;
    //       });
    //   return _rowsChangeController.stream;
    // }

    // /// Gets the index of the column specified by [name].
    // i64 getColumnIndexByName(String name) {
    //   if (_columnIndexByName.containsKey(name)) {
    //     return _columnIndexByName[name];
    //   }
    //   return -1;
    // }

    // /// Gets the values of the column specified by [columnIndex].
    // Vec<T> getColumnValues<T>(i64 columnIndex) {
    //   let list = <T>[];
    //   for (let row in _rows) {
    //     list.add(row[columnIndex]);
    //   }
    //   return list;
    // }
}
