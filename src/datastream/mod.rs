#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod row;
mod collumn;
mod collection;

use std::{collections::HashMap, fmt};

use ux_primitives::datatable::*;

pub struct DataCellChangeRecord<T>
where
    T: fmt::Display,
{
    pub row_index: i64,
    pub column_index: i64,
    pub old_value: T,
    pub new_value: T,
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
    pub index: usize,
    pub added_count: usize,
    pub removed_count: usize,
}

impl DataCollectionChangeRecord {
    fn new(index: usize, added_count: usize, removed_count: usize) -> Self {
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

#[derive(Clone)]
pub struct DataStream<'a, M, D> 
where
    M: fmt::Display,
    D: fmt::Display,
{
    pub meta: Vec<Channel<'a>>,
    // data is renamed to frames
    pub frames: Vec<DataFrame<M, D>>,
    // column_index_by_name: HashMap<String, usize>,
    // columns: Option<DataColumnCollection<M, D>>,
    // rows: Option<DataRowCollection<M, D>>,
    // cellChangeController: StreamController<DataCellChangeRecord>,
    // columnsChangeController: StreamController<DataCollectionChangeRecord>,
    // rowsChangeController: StreamController<DataCollectionChangeRecord>,
}

impl<'a, M, D> Default for DataStream<'a, M, D> 
where
    M: fmt::Display,
    D: fmt::Display,
{
    fn default() -> Self {
        Self{
            meta: Vec::new(),
            frames: Vec::new(),
        }
    }
}

impl<'a, M, D> DataStream<'a, M, D> 
where
    M: fmt::Display,
    D: fmt::Display,
{
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
    pub fn new(metadata: Vec<Channel>, data: Vec<DataFrame<M, D>>) -> Self {
        // let data_table = DataTable {
        //     // column_index_by_name: Default::default(),
        //     // columns: None,
        //     // rows: None,
        // };

        // let column_index_by_name = HashMap::<String, usize>::new();
        // let rows: DataRowCollection<String> = DataRowCollection::new(&data_table);
        // let columns: DataColumnCollection = DataColumnCollection::new(&data_table);

        // let col_count = metadata.len();
        // let row_count = data.len();

        // // first deal with columns
        // for col_idx in 0..col_count {
        //     let name = metadata[col_idx];
        //     columns.add(DataColumn { index: 0, name });
        // }

        // rows.add_all(data);
        unimplemented!()
    }

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
        //   row._cells.remove_range(start, start + count);
        // }
    }

    fn update_column_indexes(start: i64) {
        // let end = _columns.length;
        // while (start < end) {
        //   _columnIndexByName[_columns[start].name] = start++;
        // }
    }

    // /// The columns in this [DataTable].
    // DataColumnCollection get columns => _columns;

    // /// The rows (without the header row) in this [DataTable].
    // DataRowCollection get rows => _rows;

    // /// Fired when a cell is changed.
    // Stream<DataCellChangeRecord> get onCellChange {
    // ??= - Assign the value if the variable is null
    //   _cellChangeController ??= StreamController.broadcast(
    //       sync: true,
    //       onCancel: () {
    //         _cellChangeController = null;
    //       });
    //   return _cellChangeController.stream;
    // }

    // /// Fired when [columns] are changed.
    // Stream<DataCollectionChangeRecord> get onColumnsChange {
    // ??= - Assign the value if the variable is null
    //   _columnsChangeController ??= StreamController.broadcast(
    //       sync: true,
    //       onCancel: () {
    //         _columnsChangeController = null;
    //       });
    //   return _columnsChangeController.stream;
    // }

    // /// Fired when [rows] are changed.
    // Stream<DataCollectionChangeRecord> get onRowsChange {
    // ??= - Assign the value if the variable is null
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
