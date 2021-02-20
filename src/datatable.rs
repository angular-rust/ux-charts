#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;

pub struct DataCellChangeRecord<T> {
    row_index: i64,
    column_index: i64,
    old_value: T,
    new_value: T,
}

impl<T> DataCellChangeRecord<T> {
    fn new(row_index: i64, column_index: i64, old_value: T, new_value: T) {
        // this.rowIndex, this.columnIndex, this.oldValue, this.newValue);
    }

    // String to_string() =>
    //     "DataCellChangeRecord { rowIndex: $rowIndex, colIndex; $columnIndex, $oldValue, $newValue }";
}

pub struct DataCollectionChangeRecord {
    index: i64,
    added_count: i64,
    removed_count: i64,
}

impl DataCollectionChangeRecord {
    fn new(index: i64, added_count: i64, removed_count: i64) {}

    // String to_string() =>
    //     "DataCollectionChangeRecord { index: $index, added: $addedCount, removed: $removedCount}";
}

// should impl getters/setters
pub trait TableEntity {}

pub struct DataRow<T> {
    index: i64,
    table: DataTable,
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

pub struct DataType;

pub struct DataColumn {
    index: i64,
    table: DataTable,
    /// The name of the column.
    name: String,
    /// The type of data stored in the column.
    data_type: DataType,
}

impl TableEntity for DataColumn {}

impl DataColumn {
    fn new(name: String, data_type: DataType) {
        unimplemented!()
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

// pub struct DataCollectionBase<E> {
//     base: Vec<E>,
//     table: DataTable,
// }

// E extends _TableEntity
trait DataCollectionBase<E>/* : ListBase<E>*/ {
    fn release_items(start: i64, end: i64) {
        // while (start < end) {
        //   _base[start]._table = null;
        //   start++;
        // }
    }

    fn update_items(start: i64) {
        // let len = length;
        // while (start < len) {
        //   _base[start]
        //     .._table = _table
        //     .._index = start++;
        // }
    }

    fn new(table: DataTable) {
        // : _base = <E>[],
        //   _table = table;
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

    // fn set length(i64 value) {
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

    fn add(value: E) {
        // let index = length;
        // _base.add(value);
        // _updateItems(index);
        // _table._onRowsOrColumnsInserted(this, index, 1);
    }

    // fn addAll(iterable: Iterable<E>) {
    //     // let index = length;
    //     // _base.addAll(iterable);
    //     // _updateItems(index);
    //     // _table._onRowsOrColumnsInserted(this, index, iterable.length);
    // }

    // @override
    // E elementAt(i64 index) => _base[index];

    fn insert(index: i64, value: E) {
        // _base.insert(index, value);
        // _updateItems(index);
        // _table._onRowsOrColumnsInserted(this, index, 1);
    }

    // fn insertAll(index: i64, iterable: Iterable<E>) {
    //     // _base.insertAll(index, iterable);
    //     // _updateItems(index);
    //     // _table._onRowsOrColumnsInserted(this, index, iterable.length);
    // }

    // fn remove(element: E) -> bool {
    //     // let index = _base.indexOf(element);
    //     // if (index == -1) return false;
    //     // removeAt(index);
    //     // return true;
    //     unimplemented!()
    // }

    fn clear() {
        // let len = length;
        // if (len == 0) return;
        // _releaseItems(0, len);
        // _base.clear();
        // _table._onRowsOrColumnsRemoved(this, 0, len);
    }

    fn remove_at(index: i64) -> E {
        // let e = _base.removeAt(index);
        // e._table = null;
        // _updateItems(index);
        // _table._onRowsOrColumnsRemoved(this, index, 1);
        // return e;
        unimplemented!()
    }

    fn remove_last() -> E {
        // let e = _base.removeLast();
        // e._table = null;
        // _table._onRowsOrColumnsRemoved(this, length, 1);
        // return e;
        unimplemented!()
    }

    fn remove_range(start: i64, end: i64) {
        // _releaseItems(start, end);
        // _base.removeRange(start, end);
        // _updateItems(start);
        // _table._onRowsOrColumnsRemoved(this, start, end - start);
        unimplemented!()
    }
}

pub struct DataRowCollection;

impl<T> DataCollectionBase<DataRow<T>> for DataRowCollection {
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

pub struct DataColumnCollection;

impl DataCollectionBase<DataColumn> for DataColumnCollection {
    // DataColumnCollection(DataTable table) : super(table);

    // /// Adds a new column given its [name] and [type].
    // fn add2(name: String, dtype: Type) {
    //     // add(DataColumn(name, dtype));
    // }
}

pub struct DataTable {
    column_index_by_name: HashMap<String, i64>,
    columns: DataColumnCollection,
    rows: DataRowCollection,

    // cellChangeController: StreamController<DataCellChangeRecord>,
    // columnsChangeController: StreamController<DataCollectionChangeRecord>,
    // rowsChangeController: StreamController<DataCollectionChangeRecord>,
}

impl DataTable {
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
    /// The first row in [data] contains the column names.
    /// The data type of each column is determined by the first non-null value
    /// in that column.
    ///
    /// All values in each column are expected to be of the same type,
    /// and all rows are expected to have the same length.
    // data is optional
    fn new(data: Vec<Vec<String>>) {
        // _columnIndexByName = <String, int>{};
        // _rows = DataRowCollection(this);
        // _columns = DataColumnCollection(this);

        // if (data == null) return;

        // let colCount = data.first.length;
        // let rowCount = data.length;

        // for (let colIndex = 0; colIndex < colCount; colIndex++) {
        //   let name = data[0][colIndex];
        //   let type = Object;
        //   for (let rowIndex = 1; rowIndex < rowCount; rowIndex++) {
        //     let value = data[rowIndex][colIndex];
        //     if (value == null) continue;
        //     if (value is String) type = String;
        //     if (value is num) type = num;
        //     if (value is List) type = List;
        //     break;
        //   }
        //   _columns.add2(name, type);
        // }

        // _rows.addAll(data.getRange(1, rowCount));
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
